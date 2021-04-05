//! Retrieve the normal for the process of shading the points. Here we need to calculate PCA in local 3D \
//! regions and return the eigenvector of the principal competent with the lowest eigenvalue.

// Build in imports
use std::cmp::Ordering;

// Third party imports
use indicatif::{ParallelProgressIterator, ProgressBar, ProgressStyle};
use rayon::prelude::*;

// First party imports
use super::{explanation::Explanation, Neighborhood};
use crate::{
    search::{PointContainer, PointContainer3D},
    util::math,
};

/// Struct containing the output of the normal explanation for a single point
#[derive(Debug, PartialEq, Copy, Clone)]
pub struct NormalExplanation {
    pub normal: na::Point3<f32>,
    pub eccentricity: f32,
}

/// Struct containing the state of the van driel explanation mechanism
pub struct NormalState<'a> {
    pub point_container: &'a PointContainer3D,
}

impl<'a> Explanation<NormalExplanation> for NormalState<'a> {
    /// Retrieve the normals from the dataset
    fn explain(&self, neighborhood_size: Neighborhood) -> Vec<NormalExplanation> {
        // Create a fancy progress bar
        let pb = ProgressBar::new(self.point_container.get_point_count() as u64);
        pb.set_style(ProgressStyle::default_bar()
            .template("[{elapsed_precise}] Calculating normals [{bar:40.cyan/blue}] {pos}/{len} ({eta} left at {per_sec})")
            .progress_chars("#>-"));

        (0..self.point_container.get_point_count())
            .into_par_iter()
            .progress_with(pb)
            .map(|index| {
                let neighborhood = self
                    .point_container
                    .get_neighbor_indices(index as u32, neighborhood_size);
                self.get_explanation(neighborhood)
            })
            .collect::<Vec<NormalExplanation>>()
    }
}

impl<'a> NormalState<'a> {
    /// Create a new mechanism
    pub fn new(point_container: &'a PointContainer3D) -> NormalState<'a> {
        NormalState { point_container }
    }

    fn get_explanation(&self, neighborhood_indices: Vec<u32>) -> NormalExplanation {
        // TODO: Clone is bad mkay move this into the trait!
        let neighbor_points: Vec<Vec<f32>> = neighborhood_indices
            .iter()
            .map(|index| {
                let p: &na::Point3<f32> = self.point_container.get_ld_point(*index);
                vec![p.x, p.y, p.z]
            })
            .collect();

        match neighbor_points.len() {
            // We cannot do anything without a neighborhood
            0 => NormalExplanation {
                normal: na::Point3::<f32>::new(1f32, 0f32, 0f32),
                eccentricity: 1f32,
            },
            _ => {
                // Get the covariance matrix and the eigen values / vectors
                let cov_matrix = math::covariance_matrix(&neighbor_points)
                    .expect("Could not calculate the covariance matrix");
                let (values, vectors) =
                    math::eigen_values(cov_matrix).expect("Could not calculate the eigen values");

                // Return the minimal 3D eigen vector.
                let (index, _) = values
                    .iter()
                    .enumerate()
                    .min_by(|(_, val_a), (_, val_b)| {
                        val_a.partial_cmp(&val_b).unwrap_or(Ordering::Equal)
                    })
                    .unwrap();

                let min = values.iter().fold(f32::INFINITY, |a, &b| a.min(b));
                let max = values.iter().fold(f32::NEG_INFINITY, |a, &b| a.max(b));

                NormalExplanation {
                    normal: na::Point3::<f32>::new(
                        vectors[(0, index)],
                        vectors[(1, index)],
                        vectors[(2, index)],
                    ),
                    eccentricity: min / max,
                }
            }
        }
    }
}
