// The enhanced attributes based explanation mechanism based on the works of van Driel et al.

// Abstract: Multidimensional projections (MPs) are established tools for exploring the structure of high-dimensional datasets to
// reveal groups of similar observations. For optimal usage, MPs can be augmented with mechanisms that explain what such points have
// in common that makes them similar. We extend the set of such explanatory instruments by two new techniques. First, we compute
// and encode the local dimensionality of the data in the projection, thereby showing areas where the MP can be well explained
// by a few latent variables. Secondly, we compute and display local attribute correlations, thereby helping the user to discover
// alternative explanations for the underlying phenomenon. We implement our explanatory tools using an image-based approach,
// which is efficient to compute, scales well visually for large and dense MP scatterplots, and can handle any projection technique.
// We demonstrate our approach using several datasets.

// Build in imports
use std::cmp::Ordering;

// First party imports
use super::{explanation::Explanation, Neighborhood};
use crate::{
    search::{PointContainer, PointContainer2D, PointContainer3D},
    util::math,
};

/// Struct continaing the outcome of the Van Driel explanation for a single point
#[derive(Debug, PartialEq, Copy, Clone)]
pub struct VanDrielExplanation {
    pub dimension: usize,
    pub confidence: f32,
}

impl VanDrielExplanation {
    /// Rank the dimensions on how many times they occur
    pub fn calculate_dimension_rankings(explanations: &[VanDrielExplanation]) -> Vec<usize> {
        if explanations.is_empty() {
            return Vec::<usize>::new();
        }

        let max_dimension_index = explanations.iter().map(|exp| exp.dimension).max().unwrap() + 1;
        let mut ranking_counts = explanations
            .iter()
            .map(|exp| exp.dimension)
            // Count the occurrences of each dimension
            .fold(vec![0usize; max_dimension_index], |mut acc, dim| {
                acc[dim] += 1;
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

    pub fn confidence_bounds(explanations: &[VanDrielExplanation]) -> (f32, f32) {
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

#[allow(dead_code)]
enum VanDrielType {
    TotalVariance,
    MinimalVariance,
    Ratio,
}

/// Struct containing the state of the van driel explanation mechanism
pub struct VanDrielState<'a, PC: PointContainer> {
    pub point_container: &'a PC,
    // theta value uses in the calculation
    pub theta: f32,
    #[allow(dead_code)]
    explanation_type: VanDrielType,
}

/// Allow running the explanation mechanism
impl<'a, PC: PointContainer> Explanation<VanDrielExplanation> for VanDrielState<'a, PC> {
    /// Run the da silva explanation mechanism
    fn explain(&self, neighborhood_size: Neighborhood) -> Vec<VanDrielExplanation> {
        // For each point get the indices of the neighbors
        let neighborhoods = self.point_container.get_neighbor_indices(neighborhood_size);

        neighborhoods
            .iter()
            .map(|neighborhood| {
                match neighborhood.len() {
                    0usize => VanDrielExplanation {
                        dimension: 1,
                        confidence: 0.0f32,
                    },
                    _ => {
                        // Get the eigen values
                        let eigenvalues_sorted = self.get_eigen_values(neighborhood);
                        // total variance calculation from table 1
                        let dimensionality = self.get_dimensionality(&eigenvalues_sorted);
                        VanDrielExplanation {
                            dimension: dimensionality,
                            confidence: self.get_confidence(&eigenvalues_sorted, dimensionality),
                        }
                    }
                }
            })
            .collect::<Vec<VanDrielExplanation>>()
    }
}

impl<'a> VanDrielState<'a, PointContainer2D> {
    /// Create a new mechanism
    pub fn new(
        point_container: &'a PointContainer2D,
        theta: f32,
    ) -> VanDrielState<'a, PointContainer2D> {
        VanDrielState::<PointContainer2D> {
            point_container,
            explanation_type: VanDrielType::TotalVariance,
            theta,
        }
    }
}

impl<'a> VanDrielState<'a, PointContainer3D> {
    /// Create a new mechanism
    pub fn new(
        point_container: &'a PointContainer3D,
        theta: f32,
    ) -> VanDrielState<'a, PointContainer3D> {
        VanDrielState::<PointContainer3D> {
            point_container,
            explanation_type: VanDrielType::TotalVariance,
            theta,
        }
    }
}

impl<'a, PC: PointContainer> VanDrielState<'a, PC> {
    /// Get the min/max normalized eigen vectors of the neighborhood sorted desc
    fn get_eigen_values(&self, neighborhood_indices: &[usize]) -> Vec<f32> {
        // TODO: Clone is bad mkay move this into the trait!
        let neighbor_points: Vec<Vec<f32>> = neighborhood_indices
            .iter()
            .map(|index| self.point_container.get_nd_point(*index).clone())
            .collect();
        // Get eigen values
        let eigenvalues = math::eigen_values_from_points(&neighbor_points).unwrap();
        // Get the absolute eigen values
        let mut eigenvalues: Vec<f32> = eigenvalues.into_iter().map(|v| v.abs()).collect();
        // Sort in descending order
        eigenvalues.sort_by(|a, b| b.partial_cmp(&a).unwrap_or(Ordering::Equal));
        eigenvalues
    }

    /// Get the dimensionality from the eigenvalues
    fn get_dimensionality(&self, eigenvalues: &Vec<f32>) -> usize {
        let sum_eigen_value = eigenvalues.iter().sum::<f32>();
        // Check how many dimensions are needed to exceed theta
        for i in 1..=eigenvalues.len() {
            if (eigenvalues.iter().take(i).sum::<f32>() / sum_eigen_value) >= self.theta {
                return i - 1;
            }
        }
        // fallback, we need all dimension, in practice this case is never hit.
        eigenvalues.len() - 1
    }

    /// Get the confidence from the eigenvalues
    fn get_confidence(&self, eigenvalues: &Vec<f32>, dimensionality: usize) -> f32 {
        let sum_eigen_value = eigenvalues.iter().sum::<f32>();
        let average_eigen_value = sum_eigen_value / eigenvalues.len() as f32;
        let sum_eigen_value_diff_from_mean = eigenvalues
            .iter()
            .take(dimensionality + 1)
            .map(|v| (v - average_eigen_value).abs())
            .sum::<f32>();

        1.0f32 - (sum_eigen_value_diff_from_mean / sum_eigen_value)
    }
}
