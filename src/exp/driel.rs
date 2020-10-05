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
    common::{Distance, IndexedPoint3D, RTreeParameters3D},
    explanation::{Explanation, NeighborhoodExplanationMechanism},
    explanation::{GlobalContribution, LocalContributions, NeighborIndices},
    Neighborhood,
};
use crate::util::types::PointN;

use std::cmp::Ordering;

/// Struct continaing the outcome of the Van Driel explanation for a single point
#[derive(Debug, PartialEq, Copy, Clone)]
pub struct VanDrielExplanation {
    pub dimension: usize,
    pub confidence: f32,
}

impl VanDrielExplanation {
    /// Rank the dimensions on how many times they occur
    pub fn calculate_dimension_rankings(explanations: &Vec<VanDrielExplanation>) -> Vec<usize> {
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

    pub fn confidence_bounds(explanations: &Vec<VanDrielExplanation>) -> (f32, f32) {
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
    pub original_points: &'a Vec<PointN>,
    pub neighborhood_size: Neighborhood,
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

        // For each point and neighborhood calculate the da silva metric
        let ranking_vectors: Vec<_> = (0..self.get_point_count())
            .into_iter()
            .zip(&neighborhoods)
            .map(|(index, neighborhood)| {
                // TODO expand this
                let eigenvalues_sorted = self.get_eigen_values(neighborhood);
            })
            .collect();

        // TODO: Implement
        unimplemented!()
    }
}

impl<'a> VanDrielState<'a> {
    pub fn new(
        reduced_points: Vec<Point3<f32>>,
        original_points: &'a Vec<PointN>,
        neighborhood_size: Neighborhood,
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
        let rtree = RTree::<IndexedPoint3D>::bulk_load_with_params(indexed_points);
        VanDrielState {
            rtree,
            original_points,
            neighborhood_size,
            explanation_type: VanDrielType::TotalVariance,
        }
    }

    fn get_eigen_values(&self, _neighborhood: &NeighborIndices) -> Vec<f32> {
        unimplemented!()
    }
}
