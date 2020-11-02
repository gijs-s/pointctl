/// For the process of shading the points we need to calculate PCA in local 3D regions and return the eigenvector
/// of the principal competent with the lowest eigenvalue.

// Third party imports
use indicatif::{ParallelProgressIterator, ProgressBar, ProgressStyle};
use rayon::prelude::*;

// First party imports
use super::{explanation::Explanation, Neighborhood};
use crate::{
    search::{PointContainer, PointContainer3D},
    util::math,
};

#[allow(dead_code)]
pub struct NormalExplanation {
    normal: na::Point3<f32>,
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
                let minor_eigen_vector = self.get_minor_eigen_vector(neighborhood);
                NormalExplanation {
                    normal: minor_eigen_vector
                }
            })
            .collect::<Vec<NormalExplanation>>()
    }
}

#[allow(dead_code)]
impl<'a> NormalState<'a> {
    /// Create a new mechanism
    pub fn new(point_container: &'a PointContainer3D) -> NormalState<'a> {
        NormalState { point_container }
    }

    fn get_minor_eigen_vector(&self, neighborhood_indices: Vec<u32>) -> na::Point3<f32> {
        // TODO: Clone is bad mkay move this into the trait!
        let neighbor_points: Vec<Vec<f32>> = neighborhood_indices
            .iter()
            .map(|index| {
                let p: &na::Point3<f32> = self.point_container.get_ld_point(*index);
                vec![p.x, p.y, p.z]
            })
            .collect();
        let cov_matrix = math::covariance_matrix(&neighbor_points).expect("Could not calculate the covariance matrix");
        let (_values, _vectors) = math::eigen_values(cov_matrix).expect("Could not calculate the eigen values");
        unimplemented!("Retrieval of the min eigenvalue is not yet ready")
    }
}