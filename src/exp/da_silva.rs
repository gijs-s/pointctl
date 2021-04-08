//! The attributes based explanation mechanism based on the works of Da Silva et al.
// Multidimensional projections (MPs) are key tools for the analysis of multidimensional data. MPs reduce data dimensionality
// while keeping the original distance structure in the low-dimensional output space, typically shown by a 2D scatterplot. While
// MP techniques grow more precise and scalable, they still do not show how the original dimensions (attributes) influence the
// projection’s layout. In other words, MPs show which points are similar, but not why. We propose a visual approach to describe
// which dimensions contribute mostly to similarity relationships over the projection, thus explain the projection’s layout. For
// this, we rank dimensions by increasing variance over each point-neighborhood, and propose a visual encoding to show the
// least-varying dimensions over each neighborhood. We demonstrate our technique with both synthetic and real-world datasets.

// Build in imports
use std::{cmp::Ordering, iter};

// Third party imports
use indicatif::{ParallelProgressIterator, ProgressBar, ProgressStyle};
use rayon::prelude::*;

// First party import
use super::{
    // Import traits from the explanation module
    explanation::Explanation,
    // Import types from the explanation module
    explanation::{GlobalContribution, LocalContributions, Ranking},
    Neighborhood,
};
use crate::{
    search::{Distance, PointContainer, PointContainer2D, PointContainer3D},
    util::math,
};

/// Struct containing the outcome of the da Silva explanation for a single point
#[derive(Debug, PartialEq, Copy, Clone)]
pub struct DaSilvaExplanation {
    // Attribute index is the index of which dimension in nD is most important for this point
    pub attribute_index: usize,
    // The is the confidence we have in said attribute index
    pub confidence: f32,
}

impl DaSilvaExplanation {
    /// Rank the dimensions on how many times they occur
    pub fn calculate_dimension_rankings(explanations: &[DaSilvaExplanation]) -> Vec<usize> {
        if explanations.is_empty() {
            return Vec::<usize>::new();
        }

        let max_dimension_index = explanations
            .iter()
            .map(|exp| exp.attribute_index)
            .max()
            .unwrap()
            + 1;
        let mut ranking_counts = explanations
            .iter()
            .map(|exp| exp.attribute_index)
            // Count the occurrences of each dimension
            .fold(
                vec![0usize; max_dimension_index],
                |mut acc, attribute_index| {
                    acc[attribute_index] += 1;
                    acc
                },
            )
            .into_iter()
            // Add an index to the count of each dimension
            .enumerate()
            .filter(|(_, count)| count != &0usize)
            .collect::<Vec<(usize, usize)>>();

        // Sort desc
        ranking_counts.sort_by(|(_, a), (_, b)| b.cmp(a));
        ranking_counts
            .iter()
            .map(|&(index, _)| index)
            .collect::<Vec<usize>>()
    }

    pub fn confidence_bounds(explanations: &[DaSilvaExplanation]) -> (f32, f32) {
        let min = explanations
            .iter()
            .map(|v| v.confidence)
            .min_by(|a, b| a.partial_cmp(&b).unwrap_or(Ordering::Equal))
            .unwrap();
        let max = explanations
            .iter()
            .map(|v| v.confidence)
            .max_by(|a, b| a.partial_cmp(&b).unwrap_or(Ordering::Equal))
            .unwrap();

        (min, max)
    }
}

/// Enum for the types of explanation metrics used in the da silva paper, Variance is beter.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum DaSilvaType {
    Euclidean,
    Variance,
    EuclideanSingle(usize),
    VarianceSingle(usize),
}

impl ToString for DaSilvaType {
    fn to_string(&self) -> String {
        match self {
            DaSilvaType::Euclidean => "Euclidean".to_string(),
            DaSilvaType::Variance => "Variance".to_string(),
            DaSilvaType::EuclideanSingle(attr) => format!("Euclidean for attribute {}", attr),
            DaSilvaType::VarianceSingle(attr) => format!("Variance for attribute {}", attr),
        }
    }
}

/// Struct containing the state of the da silva mechanism
pub struct DaSilvaState<'a, PC: PointContainer> {
    pub point_container: &'a PC,
    explanation_type: DaSilvaType,
}

/// Allow running the explanation mechanism
impl<'a, PC: PointContainer> Explanation<DaSilvaExplanation> for DaSilvaState<'a, PC> {
    /// Run the da silva explanation mechanism
    fn explain(&self, neighborhood_size: Neighborhood) -> Vec<DaSilvaExplanation> {
        match self.explanation_type {
            DaSilvaType::Variance => println!(
                "Running Da Silva's variance explanation with neighborhood: {}",
                neighborhood_size.to_string()
            ),
            DaSilvaType::Euclidean => println!(
                "Running Da Silva's euclidean explanation with neighborhood: {}",
                neighborhood_size.to_string()
            ),
            DaSilvaType::VarianceSingle(attribute) => println!(
                "Running Da Silva's variance explanation with neighborhood: {} for attribute: {}",
                neighborhood_size.to_string(),
                &attribute
            ),
            DaSilvaType::EuclideanSingle(attribute) => println!(
                "Running Da Silva's euclidean explanation with neighborhood: {} for attribute: {}",
                neighborhood_size.to_string(),
                &attribute
            ),
        };

        // Calculate the global contribution of each point (centroid of the nD space and
        //_every_ point in its neighborhood)
        let global_contribution: GlobalContribution = match self.explanation_type {
            DaSilvaType::Euclidean | DaSilvaType::EuclideanSingle(_) => {
                self.calculate_global_distance_contribution()
            }
            DaSilvaType::Variance | DaSilvaType::VarianceSingle(_) => {
                self.calculate_global_variance()
            }
        };

        // Create a fancy progres bar wen calculating the contributions
        let pb = ProgressBar::new(self.point_container.get_point_count() as u64);
        pb.set_style(ProgressStyle::default_bar()
            .template("[{elapsed_precise}] Calculating contributions [{bar:40.cyan/blue}] {pos}/{len} ({eta} left at {per_sec})")
            .progress_chars("#>-"));

        let ranking_vectors: Vec<Ranking> = (0..self.point_container.get_point_count() as u32)
            .into_par_iter()
            .progress_with(pb)
            .map(|index| {
                let neighborhood = self
                    .point_container
                    .get_neighbor_indices(index as u32, neighborhood_size);
                // Calculate the distance contribution / variance lc_j between each point p_i and all its neighbors
                // v_i for every dimension j. Then average it for every dimension within the neighborhood
                let lc: LocalContributions = match self.explanation_type {
                    DaSilvaType::Euclidean | DaSilvaType::EuclideanSingle(_) => {
                        self.calculate_local_distance_contributions(index, &neighborhood)
                    }
                    DaSilvaType::Variance | DaSilvaType::VarianceSingle(_) => {
                        self.calculate_local_variance(index, &neighborhood)
                    }
                };
                // Normalize the local contribution by dividing by the global contribution (per dimension)
                let nlc: LocalContributions = Self::normalize_rankings(lc, &global_contribution);

                match self.explanation_type {
                    // Create a ranking vector from the normalized local contribution
                    DaSilvaType::Euclidean | DaSilvaType::Variance => {
                        Self::calculate_top_ranking(nlc)
                    }
                    DaSilvaType::EuclideanSingle(attribute)
                    | DaSilvaType::VarianceSingle(attribute) => {
                        Self::calculate_single_ranking(nlc, attribute)
                    }
                }
            })
            .collect();

        match self.explanation_type {
            DaSilvaType::EuclideanSingle(_) | DaSilvaType::VarianceSingle(_) => ranking_vectors
                .into_iter()
                .map(|(attr, contribution)| DaSilvaExplanation {
                    attribute_index: attr,
                    confidence: contribution,
                })
                .collect::<Vec<DaSilvaExplanation>>(),
            _ => {
                // Create a fancy progres bar
                let pb = ProgressBar::new(self.point_container.get_point_count() as u64);
                pb.set_style(ProgressStyle::default_bar()
            .template("[{elapsed_precise}] Calculating annotations [{bar:40.cyan/blue}] {pos}/{len} ({eta} left at {per_sec})")
            .progress_chars("#>-"));

                (0..self.point_container.get_point_count())
                    .into_par_iter()
                    .progress_with(pb)
                    .map(|index| {
                        let neighborhood = self
                            .point_container
                            .get_neighbor_indices(index as u32, neighborhood_size);
                        Self::calculate_annotation(index, &ranking_vectors, &neighborhood)
                    })
                    .collect::<Vec<DaSilvaExplanation>>()
            }
        }
    }
}

impl<'a> DaSilvaState<'a, PointContainer2D> {
    /// Create a new mechanism
    pub fn new(
        point_container: &'a PointContainer2D,
        explanation_type: DaSilvaType,
    ) -> DaSilvaState<'a, PointContainer2D> {
        DaSilvaState::<PointContainer2D> {
            point_container,
            explanation_type,
        }
    }
}

impl<'a> DaSilvaState<'a, PointContainer3D> {
    /// Create a new mechanism
    pub fn new(
        point_container: &'a PointContainer3D,
        explanation_type: DaSilvaType,
    ) -> DaSilvaState<'a, PointContainer3D> {
        DaSilvaState::<PointContainer3D> {
            point_container,
            explanation_type,
        }
    }
}

impl<'a, PC: PointContainer> DaSilvaState<'a, PC> {
    /// From the sorted vector of local contributions and find the dimension than
    /// contributes most. Read as: Find the lowest ranking given a the local
    /// contribution.
    fn calculate_top_ranking(local_contributions: LocalContributions) -> Ranking {
        local_contributions
            .iter()
            .enumerate()
            .min_by(|(_, &a), (_, &b)| a.partial_cmp(&b).unwrap_or(Ordering::Equal))
            .map(|(index, &f)| (index, f))
            .unwrap()
    }

    /// Based on the attribute retrieve its contribution
    fn calculate_single_ranking(
        local_contributions: LocalContributions,
        attribute: usize,
    ) -> Ranking {
        let conf = local_contributions[attribute];
        (attribute, conf)
    }
}

impl<'a, PC: PointContainer> DaSilvaState<'a, PC> {
    /// Using the rankings and the neighborhood calculate the the confidence.
    /// Here the confidence is how many in the neighborhood share the dimension
    /// of the points ranking.
    fn calculate_annotation(
        point_index: usize,
        ranking_list: &[Ranking],
        neighborhood_indices: &[u32],
    ) -> DaSilvaExplanation {
        // Retrieve what dimension was chosen for a certain point
        let (point_dim, _) = ranking_list[point_index];
        let correct_count = neighborhood_indices
            .iter()
            // Get the ranking vector for each neighbor and only keep the dimension
            .map(|&index| {
                let (dim, _) = ranking_list[index as usize];
                dim
            })
            // Only keep the neighbors where the dimension is correct
            .filter(|&dim| dim == point_dim)
            // Count the amount of neighbors left
            .count();

        // TODO: Do we include self in the confidence score? assume no for now.
        DaSilvaExplanation {
            attribute_index: point_dim,
            confidence: if neighborhood_indices.is_empty() {
                0.0f32
            } else {
                correct_count as f32 / neighborhood_indices.len() as f32
            },
        }
    }
}

// Functions used for the variance type of explanation
impl<'a, PC: PointContainer> DaSilvaState<'a, PC> {
    /// Calculate the variance over all point per dimension.
    /// TODO: This is horrible code please fix
    fn calculate_global_variance(&self) -> GlobalContribution {
        // This is basically a transpose the points and then take the variance
        self.point_container
            .get_nd_points()
            .iter()
            // Transpose the points
            .fold(
                vec![Vec::new(); self.point_container.get_dimensionality()],
                |mut acc, &p| {
                    for (acc_j, &p_j) in acc.iter_mut().zip(p) {
                        acc_j.push(p_j)
                    }
                    acc
                },
            )
            .iter()
            // Get the variance per dimension
            .map(|acc_j| math::variance(acc_j).unwrap())
            .collect()
    }

    /// Calculate the variance in a set of a neighborhood _including_ the point
    /// This function is a lot faster but does not yield the correct results yet.
    #[allow(dead_code)]
    fn calculate_local_variance_fast(
        &self,
        point_index: u32,
        neighbor_indices: &[u32],
    ) -> LocalContributions {
        let point_count = neighbor_indices.len();
        let dimensions = self.point_container.get_dimensionality();
        let point_matrix = na::DMatrix::<f32>::from_iterator(
            dimensions,
            point_count + 1,
            // Retrieve the actual point using the index
            neighbor_indices
                .iter()
                .chain(iter::once(&point_index))
                .map(|&index| self.point_container.get_nd_point(index).iter())
                .flatten()
                .cloned(),
        );

        let variance = point_matrix.column_variance();
        Vec::from(variance.as_slice())
    }

    /// Calculate the variance in a set of a neighborhood _including_ the point
    fn calculate_local_variance(
        &self,
        point_index: u32,
        neighbor_indices: &[u32],
    ) -> LocalContributions {
        // Create the accumulator using the search point an put each dimension into a singleton
        let accumulator: Vec<Vec<f32>> = self
            .point_container
            .get_nd_point(point_index)
            .clone()
            .iter()
            .map(|&v| vec![v])
            .collect();

        neighbor_indices
            .iter()
            // Retrieve the actual point using the index
            .map(|&index| self.point_container.get_nd_point(index))
            // Fold to collect all the contributions into one single cumulative one.
            // Transpose the points. Use search point as initial value for the accumulator
            .fold(accumulator, |mut acc, p| {
                for (acc_j, &p_j) in acc.iter_mut().zip(p) {
                    acc_j.push(p_j);
                }
                acc
            })
            .iter()
            .map(|acc_j| math::variance(acc_j).unwrap())
            .collect()
    }
}

// Functions used by the distance based metric
impl<'a, PC: PointContainer> DaSilvaState<'a, PC> {
    /// Given 2 points, calculate the contribution of distance for each dimension.
    /// corresponds to formula 1. lc_j = (p_j - r_j)^2 / ||p-r||^2 where j is the dim.
    fn calculate_distance_contribution(p: &[f32], r: &[f32]) -> LocalContributions {
        // ||p - r||^2
        let dist = p.sq_distance(r);
        p.iter()
            .zip(r)
            // (p_j - r_j)^2 / ||p - r||^2
            .map(|(p_j, r_j)| (p_j - r_j).powi(2) / dist)
            .collect::<LocalContributions>()
    }

    /// Used for the global explanation, just average the over all dimensions
    pub fn find_centroid(&self) -> Vec<f32> {
        self.point_container
            .get_nd_points()
            .iter()
            // Calculate the sum of all points for each dimension separately
            .fold(
                // Vector containing only zeros as fold state start
                vec![0.0f32; self.point_container.get_dimensionality()],
                // Increment every dimension of the state using each dimension in the state
                |v, &p| v.iter().zip(p).map(|(a, b)| a + b).collect(),
            )
            // Iterate over the cumulative point
            .iter()
            // Normalize the point by dividing each dimension by the amount of points.
            // This averages each dimension out.
            .map(|x| x / (self.point_container.get_point_count() as f32))
            .collect::<Vec<f32>>()
    }

    /// Calculate the distance contribution of all points from the centriod of the entire data set
    fn calculate_global_distance_contribution(&self) -> GlobalContribution {
        let centroid: Vec<f32> = self.find_centroid();
        self.point_container
            .get_nd_points()
            .iter()
            // Calculate the distance contribution between the centroid and all points.
            .map(|r| Self::calculate_distance_contribution(&centroid, r))
            // Fold to collect all the contributions into one single cumulative one.
            .fold(vec![0.0f32; centroid.len()], |acc, lc| {
                acc.iter()
                    .zip(lc)
                    .map(|(&acc_j, lc_j)| acc_j + lc_j)
                    .collect::<LocalContributions>()
            })
            .iter()
            // For each dimension normalize using the size of the points set.
            .map(|&dim| dim / (self.point_container.get_point_count() as f32))
            .collect()
    }

    /// Given a point index, the set of points and the indices of the neighbors calculate the
    /// average local contribution for each dimension over the neighborhood.
    /// Corresponds to formula 2. lc_j = Sum over r in neighborhood of lc^j_p,r divided |neighborhood|
    fn calculate_local_distance_contributions(
        &self,
        point_index: u32,
        neighbor_indices: &[u32],
    ) -> LocalContributions {
        // Retrieve a references to the point and neighbors
        let p: &Vec<f32> = self.point_container.get_nd_point(point_index);
        // Calculate the contribution of the distance between the point and all its neighbors
        // The take the cumulative over each dimension. Then divide that by the neigbor size.
        neighbor_indices
            .iter()
            // Calculate the distance contribution between the point and one of its neighbors
            .map(|&index| {
                let r = self.point_container.get_nd_point(index);
                Self::calculate_distance_contribution(p, r)
            })
            // Fold to collect all the contributions into one single cumulative one.
            .fold(vec![0.0f32; p.len()], |c, lc| {
                c.iter()
                    .zip(lc)
                    .map(|(&c, x)| c + x)
                    .collect::<LocalContributions>()
            })
            .iter()
            // For each dimension normalize using the neighborhood size.
            .map(|&dim| dim / (neighbor_indices.len() as f32))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use rstar::RTree;
    use vpsearch::Tree as VPTree;

    use super::*;
    use crate::search::{IndexedPoint, PointContainer2D, PointContainer3D, PointData3D};

    #[test]
    fn calculates_correct_centroid() {
        let point_data: Vec<PointData3D> = vec![
            vec![1.0, 1.0, -1.0],
            vec![-1.0, 0.0, 1.0],
            vec![0.0, 2.0, 0.0],
        ]
        .into_iter()
        .map(|v| PointData3D::new(0, na::Point3::<f32>::new(0.0, 0.0, 0.0), v))
        .collect();

        let point_container = PointContainer3D {
            tree_low: RTree::<IndexedPoint<na::Point3<f32>>>::new(),
            tree_high: VPTree::new(vec![].as_slice()),
            dimension_names: vec![],
            point_data: point_data,
            dimensionality: 3,
            projection_width: 0f32,
        };

        let state = DaSilvaState::<PointContainer3D>::new(&point_container, DaSilvaType::Variance);

        assert_eq!(state.find_centroid(), vec![0.0, 1.0, 0.0]);
    }

    #[test]
    fn calculates_correct_top_ranking() {
        assert_eq!(
            DaSilvaState::<PointContainer2D>::calculate_top_ranking(vec![0.8f32, 0.1, 0.3]),
            (1, 0.1)
        );
        assert_eq!(
            DaSilvaState::<PointContainer2D>::calculate_top_ranking(vec![0.0f32, 0.0, 0.3]),
            (0, 0.0)
        );
        assert_eq!(
            DaSilvaState::<PointContainer2D>::calculate_top_ranking(vec![0.3f32, 0.3, 0.0]),
            (2, 0.0)
        );
    }

    #[test]
    fn calculates_correct_annotation() {
        let rankings: Vec<(usize, f32)> =
            vec![(2, 0.7), (1, 0.4), (1, 0.9), (1, 0.6), (1, 0.4), (3, 0.5)];

        assert_eq!(
            DaSilvaState::<PointContainer2D>::calculate_annotation(0, &rankings, &vec![1, 2, 3, 4]),
            DaSilvaExplanation {
                attribute_index: 2,
                confidence: 0.0
            }
        );

        assert_eq!(
            DaSilvaState::<PointContainer2D>::calculate_annotation(1, &rankings, &vec![2, 3, 4]),
            DaSilvaExplanation {
                attribute_index: 1,
                confidence: 1.0
            }
        );

        assert_eq!(
            DaSilvaState::<PointContainer2D>::calculate_annotation(1, &rankings, &vec![0, 2, 3, 4]),
            DaSilvaExplanation {
                attribute_index: 1,
                confidence: 0.75
            }
        );

        assert_eq!(
            DaSilvaState::<PointContainer2D>::calculate_annotation(1, &rankings, &vec![0, 2, 3, 5]),
            DaSilvaExplanation {
                attribute_index: 1,
                confidence: 0.50
            }
        );
    }

    #[test]
    fn calculates_correct_dimension_rankings() {
        // Add single dummy points, used for finding the dimensionality

        let indexes = vec![0, 1, 1, 1, 2, 2];
        let explanations = indexes
            .into_iter()
            .map(|index| DaSilvaExplanation {
                attribute_index: index,
                confidence: 0.5,
            })
            .collect::<Vec<_>>();

        assert_eq!(
            DaSilvaExplanation::calculate_dimension_rankings(&explanations),
            vec![1, 2, 0]
        )
    }
}
