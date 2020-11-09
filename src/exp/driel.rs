//! The enhanced attributes based explanation mechanism based on the works of van Driel et al.
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
use indicatif::{ParallelProgressIterator, ProgressBar, ProgressStyle};
use rayon::prelude::*;

// First party imports
use super::{explanation::Explanation, Neighborhood};
use crate::{
    search::{PointContainer, PointContainer2D, PointContainer3D},
    util::math,
};

/// Struct containing the outcome of the Van Driel explanation for a single point
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

/// Enum for the types of thresholding used in the Van Driel explanation mechanism
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
    explanation_type: VanDrielType,
}

/// Allow running the explanation mechanism
impl<'a, PC: PointContainer> Explanation<VanDrielExplanation> for VanDrielState<'a, PC> {
    /// Run the da silva explanation mechanism
    fn explain(&self, neighborhood_size: Neighborhood) -> Vec<VanDrielExplanation> {
        match self.explanation_type {
            VanDrielType::MinimalVariance => println!(
                "Running Van Driel's PCA min explanation with neighborhood: {}",
                neighborhood_size.to_string()
            ),
            VanDrielType::TotalVariance => println!(
                "Running Van Driel's PCA sum explanation with neighborhood: {}",
                neighborhood_size.to_string()
            ),
        };

        // Create a fancy progress bar
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
    fn get_eigen_values(&self, neighborhood_indices: Vec<u32>) -> Vec<f32> {
        // TODO: Clone is bad mkay move this into the trait!
        let neighbor_points: Vec<Vec<f32>> = neighborhood_indices
            .iter()
            .map(|index| self.point_container.get_nd_point(*index).clone())
            .collect();
        // Get eigen values
        let mut eigenvalues = math::eigen_values_from_points(&neighbor_points).unwrap();
        // Get the absolute eigen values keeping only the finite values
        // let mut eigenvalues: Vec<f32> = eigenvalues
        //     .into_iter()
        //     .map(|v| if v.is_finite() { v } else { 0.0 })
        //     .collect();
        // Sort in descending order
        eigenvalues.sort_by(|a, b| b.partial_cmp(&a).unwrap_or(Ordering::Equal));
        // println!("{:?}", eigenvalues);
        eigenvalues
    }

    /// Get the dimensionality from the eigenvalues
    ///
    /// Dim_θ = min k s.t.
    ///  1. 0 > k ≤ n
    ///  2. (sum_{i=1}^k λ_i / sum_{i=1}^n λ_i) ≥ θ
    ///  2. λi ≥ λj where 1 ≤ i < n and i < j ≤ n
    ///
    /// Here dimensionality is determined by the count of the k largest eigenvalues
    /// which sum up to more than θ percent of the total variance in a neighborhood.
    fn get_dimensionality_total(&self, eigenvalues: &Vec<f32>) -> usize {
        let sum_eigen_value = eigenvalues.iter().sum::<f32>();
        // Check how many dimensions are needed to exceed theta
        for i in 1..=eigenvalues.len() {
            if eigenvalues.iter().take(i).sum::<f32>() / sum_eigen_value >= self.theta {
                return i;
            }
        }
        // fallback, we need all dimension, in practice this case is never hit.
        eigenvalues.len()
    }

    /// Get the confidence from the eigenvalues
    ///
    /// Conf_θ = 1 – ( (sum_{i=1}^k λ_i / sum_{i=1}^n λ_i) / θ )
    ///     iff λi ≥ λj where 1 ≤ i < n and i < j ≤ n
    ///
    /// Here the sum of the first k eigenvalues explains AT LEAST theta, by construction.
    /// So, if that sum explains EXACTLY theta, then you get conf_θ = 1, meaning, a
    /// perfect explanation of exactly theta percent of the variance. If that sum explains
    /// more, well, the confidence drops since you deviate from the exact explanation.
    fn get_confidence_total(&self, eigenvalues: &Vec<f32>, dimensionality: usize) -> f32 {
        let sum_eigen_value = eigenvalues.iter().sum::<f32>();
        let sum_eigen_value_to_n = eigenvalues.iter().take(dimensionality).sum::<f32>();

        let theta_offset = (sum_eigen_value_to_n / sum_eigen_value) - self.theta;
        1.0f32 - theta_offset
    }

    /// Get the dimensionality and confidence based on the minimal variance
    ///
    /// Dim_θ = | { λi / (sum_{j=1}^n λj) ≥ θ | 1 ≤ i ≤ n} |
    /// Conf_θ = (sum_{i=1}^Dim_θ λi) / (sum_{j=1}^n λj)
    ///     iff λi ≥ λj where 1 ≤ i < n and i < j ≤ n
    ///
    /// Dimensionality is given by how many of the top eigenvalues are larger than a given
    /// threshold θ of the total variance. Confidence is given by how much of the total
    /// variance the top k eigenvalues explain.
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
