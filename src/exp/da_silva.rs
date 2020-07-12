//! The attributes based explanation mechanism based on the works of Da Silva et al.
// Multidimensional projections (MPs) are key tools for the analysis of multidimensional data. MPs reduce data dimensionality
// while keeping the original distance structure in the low-dimensional output space, typically shown by a 2D scatterplot. While
// MP techniques grow more precise and scalable, they still do not show how the original dimensions (attributes) influence the
// projection’s layout. In other words, MPs show which points are similar, but not why. We propose a visual approach to describe
// which dimensions contribute mostly to similarity relationships over the projection, thus explain the projection’s layout. For
// this, we rank dimensions by increasing variance over each point-neighborhood, and propose a visual encoding to show the
// least-varying dimensions over each neighborhood. We demonstrate our technique with both synthetic and real-world datasets.
//
// TODO list for this part of the explain module:
// Must haves
// - Implement a basic version of the mechanism
// - Tests for the functions used by the mechanism
// Nice to haves:
// - Use a faster way to calculate nn (maybe approximate nearest neighbors?)
// - Store the data in a more space efficient way
// - Warning in case of tiny neighborhoods

use rstar::{RStarInsertionStrategy, RTree, RTreeParams};

use super::common::{Distance, IndexedPoint};
use crate::util::types::{Point3, PointN};

use rand::{seq::SliceRandom, thread_rng};
use std::cmp::Ordering;

// Types to make the code more readable
type NeighborIndices = Vec<usize>;
type LocalContributions = Vec<f32>;
type GlobalContribution = Vec<f32>;
type Ranking = (usize, f32);
type DimensionOrder = Vec<usize>;

#[derive(Debug, PartialEq)]
pub struct DaSilvaExplanation {
    // Attribute index is the index of which dimension in nD is most important for this point
    pub attribute_index: usize,
    // The is the confidence we have in said attribute index
    pub confidence: f32,
}

// This example uses an rtree with larger internal nodes.
pub struct LargeNodeParameters;
impl RTreeParams for LargeNodeParameters {
    const MIN_SIZE: usize = 10;
    const MAX_SIZE: usize = 30;
    const REINSERTION_COUNT: usize = 5;
    type DefaultInsertionStrategy = RStarInsertionStrategy;
}

pub struct DaSilvaMechanismState<'a> {
    pub rtree: RTree<IndexedPoint, LargeNodeParameters>,
    pub original_points: &'a Vec<PointN>,
    // How large the total projection is. mesured by the two points furthest apart.
    // pub projection_width: f32,
}

impl<'a> DaSilvaMechanismState<'a> {
    pub fn new(
        reduced_points: Vec<Point3>,
        original_points: &'a Vec<PointN>,
    ) -> DaSilvaMechanismState<'a> {
        let indexed_points: Vec<IndexedPoint> = reduced_points
            .into_iter()
            .enumerate()
            .map(|(index, point)| IndexedPoint {
                index,
                x: point.x,
                y: point.y,
                z: point.z,
            })
            .collect();
        let rtree =
            RTree::<IndexedPoint, LargeNodeParameters>::bulk_load_with_params(indexed_points);
        DaSilvaMechanismState {
            rtree,
            original_points,
        }
    }

    pub fn explain(
        &self,
        neighborhood_size: f32,
        neighborhood_bound: usize,
    ) -> (Vec<DaSilvaExplanation>, DimensionOrder) {
        // Calculate the global contribution of each point (centroid of the nD space and
        //_every_ point in its neighborhood)
        let centroid: PointN = Self::find_centroid(&self.original_points);
        let global_contribution: GlobalContribution =
            Self::calculate_global_contribution(centroid, &self.original_points);

        // Pre-compute all neighborhoods, each neighborhood consists of all points witing
        // p v_i for point p_i in 3d. The nd neighborhood is {P(p) \in v_i}. Note that we
        // do this for every (unorderd) element in the rtree so we sort after.
        let mut rng = thread_rng();
        let mut indexed_neighborhoods: Vec<(usize, NeighborIndices)> = self
            .rtree
            .iter()
            .map(|indexed_point| {
                let neighbors = self.find_neighbors(neighborhood_size, *indexed_point);
                // limit the neigborhoods size by the bound. If it exceeds this bound we take random
                // samples without replacement.
                if neighbors.len() < neighborhood_bound {
                    (indexed_point.index, neighbors)
                } else {
                    // sample from the original neighbors.
                    let neighbors_samples = neighbors
                        .choose_multiple(&mut rng, neighborhood_bound)
                        .cloned()
                        .collect();
                    (indexed_point.index, neighbors_samples)
                }
            })
            .collect();
        indexed_neighborhoods.sort_by(|(a, _), (b, _)| a.partial_cmp(b).unwrap());
        let neighborhoods: Vec<NeighborIndices> =
            indexed_neighborhoods.into_iter().map(|(_, n)| n).collect();

        let ranking_vectors: Vec<Ranking> = (0..self.original_points.len())
            .into_iter()
            .zip(&neighborhoods)
            .map(|(index, neighborhood)| {
                // Calculate the local contribution lc_j between each point p_i and all its neighbors
                // v_i for every dimension j. Then average the contribution for every dimension within
                // the neighborhood
                let lc: LocalContributions =
                    Self::calculate_local_contributions(index, self.original_points, neighborhood);
                // Normalize the local contribution by dividing by the global contribution (per dimension)
                let nlc: LocalContributions = Self::normalize_rankings(lc, &global_contribution);
                // Create a ranking vector from the normalized local contribution
                Self::calculate_top_ranking(nlc)
            })
            .collect();

        let explanation: Vec<DaSilvaExplanation> = (0..self.original_points.len())
            .into_iter()
            .zip(&neighborhoods)
            .map(|(index, neighborhood)| {
                Self::calculate_annotation(index, &ranking_vectors, neighborhood)
            })
            .collect();

        let dimension_rankings = self.calculate_dimension_rankings(&explanation);

        (explanation, dimension_rankings)
    }

    // Get a reference to all neighbors within a certain range. This used the rtree.
    fn find_neighbors(
        &self,
        neighborhood_size: f32,
        indexed_point: IndexedPoint,
    ) -> NeighborIndices {
        let query_point = [indexed_point.x, indexed_point.y, indexed_point.z];
        self.rtree
            .locate_within_distance(query_point, neighborhood_size * neighborhood_size)
            .map(|elem| elem.index)
            .filter(|&index| index != indexed_point.index)
            .collect::<NeighborIndices>()
    }

    // Used for the global explanation, just average the over all dimensions
    pub fn find_centroid(points: &Vec<PointN>) -> PointN {
        points
            .iter()
            // Calculate the sum of all points for each dimension separately
            .fold(
                // Vector containing only zeros as fold state start
                vec![0.0f32; points[0].len()],
                // Increment every dimension of the state using each dimension in the state
                |v, p| v.iter().zip(p).map(|(a, b)| a + b).collect(),
            )
            // Iterate over the cumulative point
            .iter()
            // Normalize the point by dividing each dimension by the amount of points.
            // This averages each dimension out.
            .map(|x| x / points.len() as f32)
            .collect::<PointN>()
    }

    // Given 2 points, calculate the contribution of each dimension to the distance.
    // corresponds to formula 1. lc_j = (p_j - r_j)^2 / ||p-r||^2 where j is the dim.
    fn calculate_distance_contribution(p: &PointN, r: &PointN) -> LocalContributions {
        let dist = p.sq_distance(r);
        p.iter()
            .zip(r)
            .map(|(a, b)| {
                let t = a - b;
                (t * t) / dist
            })
            .collect()
    }

    // Given a point index, the set of points and the indices of the neighbors calculate the
    // average local contribution for each dimension over the neighborhood.
    // Corresponds to formula 2. lc_j = Sum over r in neighborhood of lc^j_p,r divided |neighborhood|
    fn calculate_local_contributions(
        point_index: usize,
        original_points: &Vec<PointN>,
        neighbor_indices: &NeighborIndices,
    ) -> LocalContributions {
        // Retrieve a references to the point and neighbors
        let p: &PointN = &original_points[point_index];
        // Calculate the contribution of the distance between the point and all its neighbors
        // The take the cumulative over each dimension. Then divide that by the neigbor size.
        neighbor_indices
            .iter()
            // Calculate the distance contribution between the point and one of its neighbors
            .map(|&index| {
                let r = &original_points[index];
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
            .map(|&dim| dim / neighbor_indices.len() as f32)
            .collect()
    }

    // Does the same as calculate_local_contributions but for the entire dataset.
    fn calculate_global_contribution(centroid: PointN, points: &Vec<PointN>) -> GlobalContribution {
        points
            .iter()
            // Calculate the distance contribution between the centroid and all points.
            .map(|r| Self::calculate_distance_contribution(&centroid, r))
            // Fold to collect all the contributions into one single cumulative one.
            .fold(vec![0.0f32; centroid.len()], |c, lc| {
                c.iter()
                    .zip(lc)
                    .map(|(&c, x)| c + x)
                    .collect::<LocalContributions>()
            })
            .iter()
            // For each dimension normalize using the size of the points set.
            .map(|&dim| dim / points.len() as f32)
            .collect()
    }

    // Normalize a local contrib of a dimension using the global contrib of said dimension.
    // this function lines up with formula 3 in the da silva paper
    fn normalize_rankings(
        local_contributions: LocalContributions,
        global_contributions: &GlobalContribution,
    ) -> LocalContributions {
        // Sum of the local contributions per dimension
        let sum = local_contributions
            .iter()
            .zip(global_contributions)
            .fold(0.0, |c: f32, (l, g)| c + (l / g));
        // Normalize each term
        local_contributions
            .iter()
            .zip(global_contributions)
            .map(|(l, g)| (l / g) / sum)
            .collect()
    }

    // Using the rankings and the neighborhood calculate the the confidence.
    // Here the confidence is how many in the neighborhood share the dimension
    // of the points ranking.
    fn calculate_annotation(
        point_index: usize,
        ranking_vectors: &Vec<Ranking>,
        neighborhood: &NeighborIndices,
    ) -> DaSilvaExplanation {
        // Retrieve what dimension was chosen for a certain point
        let (point_dim, _) = ranking_vectors[point_index];
        let correct_count = neighborhood
            .iter()
            // Get the ranking vector for each neighbor and only keep the dimension
            .map(|&index| {
                let (dim, _) = ranking_vectors[index];
                dim
            })
            // Only keep the neighbors where the dimension is correct
            .filter(|&dim| dim == point_dim)
            // Count the amount of neighbors left
            .count();

        // TODO: Do we include self in the confidence score? assume no for now.
        DaSilvaExplanation {
            attribute_index: point_dim,
            confidence: if neighborhood.len() > 0 {
                correct_count as f32 / neighborhood.len() as f32
            } else {
                0.0f32
            },
        }
    }

    // From the sorted vector of local contributions and find the dimension than
    // contributes most. Read as: Find the lowest ranking given a the local
    // contribution.
    // TODO this seems like a nasty hack. ASsumes we never encounter NaN (at least
    // does not handle this correctly)
    pub fn calculate_top_ranking(local_contributions: LocalContributions) -> Ranking {
        local_contributions
            .iter()
            .enumerate()
            .min_by(|(_, &a), (_, &b)| a.partial_cmp(&b).unwrap_or(Ordering::Equal))
            .map(|(index, &f)| (index, f))
            .unwrap()
    }

    pub fn calculate_dimension_rankings(
        &self,
        explanations: &Vec<DaSilvaExplanation>,
    ) -> Vec<usize> {
        let dimension_count = self.original_points.first().unwrap().len();
        let mut ranking_counts = explanations
            .iter()
            .map(|exp| exp.attribute_index)
            // Count the occurrences of each dimension
            .fold(vec![0usize; dimension_count], |mut acc, attribute_index| {
                acc[attribute_index] += 1;
                acc
            })
            .into_iter()
            // Add an index to the count of each dimension
            .enumerate()
            .collect::<Vec<(usize, usize)>>();

        // Sort desc
        ranking_counts.sort_by(|(_, a), (_, b)| b.cmp(a));
        ranking_counts
            .iter()
            .map(|&(index, _)| index)
            .collect::<Vec<usize>>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::types::{Point3, PointN};

    #[test]
    fn calculates_correct_neighbors() {
        let points_3 = vec![
            Point3::new(0.0, 0.0, 0.0),
            Point3::new(1.0, 1.0, 1.0),
            Point3::new(2.0, 2.0, 2.0),
            Point3::new(10.0, 10.0, 10.0),
        ];
        let points_n = Vec::<PointN>::new();
        let mechanism = DaSilvaMechanismState::new(points_3, &points_n);

        // Check neighbors
        assert_eq!(
            mechanism.find_neighbors(
                2.0,
                IndexedPoint {
                    index: 2,
                    x: 2.0,
                    y: 2.0,
                    z: 2.0
                }
            ),
            vec![1]
        );
        assert_eq!(
            mechanism.find_neighbors(
                4.0,
                IndexedPoint {
                    index: 2,
                    x: 2.0,
                    y: 2.0,
                    z: 2.0
                }
            ),
            vec![1, 0]
        );
    }

    #[test]
    fn calculates_correct_centroid() {
        let original_data: Vec<PointN> = vec![
            vec![1.0, 1.0, -1.0],
            vec![-1.0, 0.0, 1.0],
            vec![0.0, 2.0, 0.0],
        ];
        assert_eq!(
            DaSilvaMechanismState::find_centroid(&original_data),
            vec![0.0, 1.0, 0.0]
        );
    }

    #[test]
    fn calculates_correct_top_ranking() {
        assert_eq!(
            DaSilvaMechanismState::calculate_top_ranking(vec![0.8f32, 0.1, 0.3]),
            (1, 0.1)
        );
        assert_eq!(
            DaSilvaMechanismState::calculate_top_ranking(vec![0.0f32, 0.0, 0.3]),
            (0, 0.0)
        );
        assert_eq!(
            DaSilvaMechanismState::calculate_top_ranking(vec![0.3f32, 0.3, 0.0]),
            (2, 0.0)
        );
    }

    #[test]
    fn calculates_correct_annotation() {
        let rankings: Vec<(usize, f32)> =
            vec![(2, 0.7), (1, 0.4), (1, 0.9), (1, 0.6), (1, 0.4), (3, 0.5)];

        assert_eq!(
            DaSilvaMechanismState::calculate_annotation(0, &rankings, &vec![1, 2, 3, 4]),
            DaSilvaExplanation {
                attribute_index: 2,
                confidence: 0.0
            }
        );

        assert_eq!(
            DaSilvaMechanismState::calculate_annotation(1, &rankings, &vec![2, 3, 4]),
            DaSilvaExplanation {
                attribute_index: 1,
                confidence: 1.0
            }
        );

        assert_eq!(
            DaSilvaMechanismState::calculate_annotation(1, &rankings, &vec![0, 2, 3, 4]),
            DaSilvaExplanation {
                attribute_index: 1,
                confidence: 0.75
            }
        );

        assert_eq!(
            DaSilvaMechanismState::calculate_annotation(1, &rankings, &vec![0, 2, 3, 5]),
            DaSilvaExplanation {
                attribute_index: 1,
                confidence: 0.50
            }
        );
    }

    #[test]
    fn calculates_correct_dimension_rankings() {
        // Add single dummy points, used for finding the dimensionality
        let points_n = vec![vec![0.0f32, 0.0, 0.0]];
        let mechanism = DaSilvaMechanismState::new(vec![], &points_n);

        let indexes = vec![0, 1, 1, 1, 2, 2];
        let explanations = indexes
            .into_iter()
            .map(|index| DaSilvaExplanation {
                attribute_index: index,
                confidence: 0.5,
            })
            .collect::<Vec<_>>();

        assert_eq!(
            mechanism.calculate_dimension_rankings(&explanations),
            vec![1, 2, 0]
        )
    }
}
