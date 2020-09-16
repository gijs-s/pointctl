// The enhanced attributes based explanation mechanism based on the works of van Driel et al.

// Abstract: Multidimensional projections (MPs) are established tools for exploring the structure of high-dimensional datasets to
// reveal groups of similar observations. For optimal usage, MPs can be augmented with mechanisms that explain what such points have
// in common that makes them similar. We extend the set of such explanatory instruments by two new techniques. First, we compute
// and encode the local dimensionality of the data in the projection, thereby showing areas where the MP can be well explained
// by a few latent variables. Secondly, we compute and display local attribute correlations, thereby helping the user to discover
// alternative explanations for the underlying phenomenon. We implement our explanatory tools using an image-based approach,
// which is efficient to compute, scales well visually for large and dense MP scatterplots, and can handle any projection technique.
// We demonstrate our approach using several datasets.

use rstar::RTree;

use super::common::{Distance, IndexedPoint3D, RTreeParameters3D};

use crate::util::types::{Point3, PointN};
use std::cmp::Ordering;

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

pub struct DrielState<'a> {
    pub rtree: RTree<IndexedPoint3D, RTreeParameters3D>,
    pub original_points: &'a Vec<PointN>,
}

impl<'a> DrielState<'a> {
    pub fn new(reduced_points: Vec<Point3>, original_points: &'a Vec<PointN>) -> DrielState<'a> {
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
        let rtree =
            RTree::<IndexedPoint3D, RTreeParameters3D>::bulk_load_with_params(indexed_points);
        DrielState {
            rtree,
            original_points,
        }
    }

    #[allow(unused_variables)]
    pub fn explain(
        &self,
        neighborhood_size: f32,
        neighborhood_bound: usize,
    ) -> Vec<VanDrielExplanation> {
        unimplemented!("Explanation method for van driel is not ready yet")
    }
}
