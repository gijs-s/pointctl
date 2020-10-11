// The enhanced attributes based explanation mechanism based on the works of van Driel et al.

// Abstract: Multidimensional projections (MPs) are established tools for exploring the structure of high-dimensional datasets to
// reveal groups of similar observations. For optimal usage, MPs can be augmented with mechanisms that explain what such points have
// in common that makes them similar. We extend the set of such explanatory instruments by two new techniques. First, we compute
// and encode the local dimensionality of the data in the projection, thereby showing areas where the MP can be well explained
// by a few latent variables. Secondly, we compute and display local attribute correlations, thereby helping the user to discover
// alternative explanations for the underlying phenomenon. We implement our explanatory tools using an image-based approach,
// which is efficient to compute, scales well visually for large and dense MP scatterplots, and can handle any projection technique.
// We demonstrate our approach using several datasets.

use nalgebra::Point3;
use rstar::RTree;

use super::{
    common::{Distance, IndexedPoint3D},
    explanation::{Explanation, NeighborhoodExplanationMechanism},
    explanation::{GlobalContribution, LocalContributions, NeighborIndices},
    Neighborhood,
};
use crate::util::{math, types::PointN};

use std::cmp::Ordering;

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

        let max_dimension_index = explanations.iter().map(|exp| exp.dimension).max().unwrap();
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

enum VanDrielType {
    TotalVariance,
    MinimalVariance,
}

/// Struct containing the state of the van driel explanation mechanism
pub struct VanDrielState<'a> {
    pub rtree: RTree<IndexedPoint3D>,
    pub original_points: &'a [PointN],
    // theta value uses in the calculation
    pub theta: f32,
    explanation_type: VanDrielType,
}

impl<'a> NeighborhoodExplanationMechanism for VanDrielState<'a> {
    fn get_tree(&self) -> &RTree<IndexedPoint3D> {
        &self.rtree
    }
}

impl<'a> Explanation<VanDrielExplanation> for VanDrielState<'a> {
    /// Run the da silva explanation mechanism
    #[allow(unused_variables)]
    fn explain(&self, neighborhood_size: Neighborhood) -> Vec<VanDrielExplanation> {
        // For each point get the indices of the neighbors
        let neighborhoods = self.get_neighbor_indices(neighborhood_size);

        (0..self.get_point_count())
            .zip(&neighborhoods)
            .map(|(index, neighborhood)| {
                // TODO expand this
                let eigenvalues_sorted = self.get_eigen_values(neighborhood);
                // total variance calculation from table 1
                let dimensionality = self.get_dimensionality(&eigenvalues_sorted);
                VanDrielExplanation {
                    dimension: dimensionality,
                    confidence: self.get_confidence(&eigenvalues_sorted, dimensionality),
                }
            })
            .collect::<Vec<VanDrielExplanation>>()
    }
}

impl<'a> VanDrielState<'a> {
    pub fn new(
        reduced_points: Vec<Point3<f32>>,
        original_points: &'a [PointN],
        theta: f32,
    ) -> VanDrielState<'a> {
        let indexed_points: Vec<IndexedPoint3D> = reduced_points
            .into_iter()
            .enumerate()
            .map(|(index, point)| IndexedPoint3D {
                index,
                x: point.x,
                y: point.y,
                z: point.z,
            })
            .collect();
        VanDrielState::new_with_indexed_point(indexed_points, original_points, theta)
    }

    pub fn new_with_indexed_point(
        indexed_points: Vec<IndexedPoint3D>,
        original_points: &'a [PointN],
        theta: f32,
    ) -> VanDrielState<'a> {
        let rtree = RTree::<IndexedPoint3D>::bulk_load_with_params(indexed_points);
        VanDrielState {
            rtree,
            original_points,
            explanation_type: VanDrielType::TotalVariance,
            theta,
        }
    }

    /// Get the min/max normalized eigen vectors of the neighborhood sorted desc
    fn get_eigen_values(&self, neighborhood_indices: &[usize]) -> Vec<f32> {
        // TODO: Clone is bad mkay
        let neighbor_points: Vec<Vec<f32>> = neighborhood_indices
            .iter()
            .map(|index| self.original_points[*index].clone())
            .collect();
        // Get eigen values
        let eigenvalues = math::eigen_values_from_points(&neighbor_points).unwrap();
        // Get the absolute eigen values
        let mut abs_eigenvalues: Vec<f32> = eigenvalues.into_iter().map(|v| v.abs()).collect();
        // Sort in descending order
        abs_eigenvalues.sort_by(|a, b| b.partial_cmp(&a).unwrap_or(Ordering::Equal));
        abs_eigenvalues
    }

    /// Get the dimensionality from the eigenvalues
    fn get_dimensionality(&self, eigenvalues: &Vec<f32>) -> usize {
        let sum_eigen_value = eigenvalues.iter().sum::<f32>();
        // Check how many dimensions are needed to exceed theta
        for i in 1..eigenvalues.len() {
            if (eigenvalues.iter().take(i).sum::<f32>() / sum_eigen_value) >= self.theta {
                return i;
            }
        }
        // fallback, we need all dimension
        eigenvalues.len()
    }

    /// Get the confidence from the eigenvalues
    fn get_confidence(&self, eigenvalues: &Vec<f32>, dimensionality: usize) -> f32 {
        let sum_eigen_value = eigenvalues.iter().sum::<f32>();
        let average_eigen_value = sum_eigen_value / eigenvalues.len() as f32;
        1.0f32
            - (eigenvalues
                .iter()
                .take(dimensionality)
                .map(|v| v - average_eigen_value)
                .sum::<f32>()
                / sum_eigen_value)
    }
}
