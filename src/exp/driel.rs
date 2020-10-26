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

// Third party imports
use indicatif::{ProgressBar, ProgressIterator, ProgressStyle};

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
            .filter(|(_, count)| count != &0usize)
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
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum VanDrielType {
    TotalVariance,
    MinimalVariance,
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

        match self.explanation_type {
            VanDrielType::MinimalVariance => println!("Running Van Driel's PCA min explanation with neighborhood: {}", neighborhood_size.to_string()),
            VanDrielType::TotalVariance => println!("Running VAn Driel's PCA sum explanation with neighborhood: {}", neighborhood_size.to_string()),
        };

        // For each point get the indices of the neighbors
        let neighborhoods = self.point_container.get_neighbor_indices(neighborhood_size);

        // Create a fancy progres bar
        let pb = ProgressBar::new(self.point_container.get_point_count() as u64);
        pb.set_style(ProgressStyle::default_bar()
            .template("[{elapsed_precise}] Calculating annotations [{bar:40.cyan/blue}] {pos}/{len} ({eta} left at {per_sec})")
            .progress_chars("#>-"));

        neighborhoods
            .iter()
            .progress_with(pb)
            .map(|neighborhood| {
                match neighborhood.len() {
                    0usize | 1usize => VanDrielExplanation {
                        dimension: 1,
                        confidence: 0.0f32,
                    },
                    _ => {
                        // Get the eigen values
                        let eigenvalues_sorted = self.get_eigen_values(neighborhood);
                        // total variance calculation from table 1
                        let (dimension, confidence) = match self.explanation_type {
                            VanDrielType::TotalVariance => {
                                let dimension = self.get_dimensionality_total(&eigenvalues_sorted);
                                let conf =
                                    self.get_confidence_total(&eigenvalues_sorted, dimension);
                                (dimension, conf)
                            }
                            VanDrielType::MinimalVariance => {
                                self.get_dimensionality_and_confidence_min(&eigenvalues_sorted)
                            }
                        };
                        VanDrielExplanation {
                            dimension,
                            confidence,
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
        explanation_type: VanDrielType,
    ) -> VanDrielState<'a, PointContainer2D> {
        VanDrielState::<PointContainer2D> {
            point_container,
            explanation_type,
            theta,
        }
    }
}

impl<'a> VanDrielState<'a, PointContainer3D> {
    /// Create a new mechanism
    pub fn new(
        point_container: &'a PointContainer3D,
        theta: f32,
        explanation_type: VanDrielType,
    ) -> VanDrielState<'a, PointContainer3D> {
        VanDrielState::<PointContainer3D> {
            point_container,
            explanation_type,
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
    fn get_dimensionality_total(&self, eigenvalues: &Vec<f32>) -> usize {
        let sum_eigen_value = eigenvalues.iter().sum::<f32>();
        // Check how many dimensions are needed to exceed theta
        for i in 1..=eigenvalues.len() {
            if (eigenvalues.iter().take(i).sum::<f32>() / sum_eigen_value) >= self.theta {
                return i;
            }
        }
        // fallback, we need all dimension, in practice this case is never hit.
        eigenvalues.len()
    }

    /// Get the confidence from the eigenvalues
    fn get_confidence_total(&self, eigenvalues: &Vec<f32>, dimensionality: usize) -> f32 {
        let sum_eigen_value = eigenvalues.iter().sum::<f32>();
        let average_eigen_value = sum_eigen_value / eigenvalues.len() as f32;
        let sum_eigen_value_diff_from_mean = eigenvalues
            .iter()
            .take(dimensionality)
            .map(|v| (v - average_eigen_value).abs())
            .sum::<f32>();

        1.0f32 - (sum_eigen_value_diff_from_mean.max(0.0) / sum_eigen_value)
    }

    /// Get the dimensionality based on the minimal variance
    fn get_dimensionality_and_confidence_min(&self, eigenvalues: &Vec<f32>) -> (usize, f32) {
        let sum_eigen_value = eigenvalues.iter().sum::<f32>();
        // check how many dimensions contribute at least alpha
        let contributing_values: Vec<f32> = eigenvalues
            .iter()
            .filter(|&v| (v / sum_eigen_value) >= self.theta)
            .map(|v| *v)
            .collect();

        let conf = contributing_values.iter().sum::<f32>() / sum_eigen_value;

        (contributing_values.len(), conf)
    }
}
