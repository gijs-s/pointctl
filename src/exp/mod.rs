/// Module containing the explanation mechanisms used for the visualization.
// Sub modules
mod da_silva;
mod driel;
mod normal;
mod explanation;

// Re-export the public facing parts of this module
pub use self::{
    da_silva::DaSilvaExplanation, driel::VanDrielExplanation, normal::NormalExplanation,
};

// Third party imports
use crate::search::{PointContainer2D, PointContainer3D};

// First party imports
use explanation::Explanation;

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Neighborhood {
    K(usize),
    R(f32),
}

pub fn run_da_silva_variance_2d<'a>(
    point_container_2d: &'a PointContainer2D,
    neighborhood_size: Neighborhood,
) -> Vec<DaSilvaExplanation> {
    let da_silva_mechanism =
        da_silva::DaSilvaState::<'a, PointContainer2D>::new(point_container_2d);
    da_silva_mechanism.explain(neighborhood_size)
}

pub fn run_da_silva_variance_3d<'a>(
    point_container_3d: &'a PointContainer3D,
    neighborhood_size: Neighborhood,
) -> Vec<DaSilvaExplanation> {
    let da_silva_mechanism =
        da_silva::DaSilvaState::<'a, PointContainer3D>::new(point_container_3d);
    da_silva_mechanism.explain(neighborhood_size)
}

pub fn run_van_driel_2d<'a>(
    point_container_2d: &'a PointContainer2D,
    neighborhood_size: Neighborhood,
) -> Vec<VanDrielExplanation> {
    // TODO: Remove dummy value
    let theta = 0.95f32;
    let van_driel_mechanism = driel::VanDrielState::<PointContainer2D>::new(point_container_2d, theta);
    van_driel_mechanism.explain(neighborhood_size)
}

pub fn run_van_driel_3d<'a>(
    point_container_3d: &'a PointContainer3D,
    neighborhood_size: Neighborhood,
) -> Vec<VanDrielExplanation> {
    // TODO: Remove dummy value
    let theta = 0.95f32;
    let van_driel_mechanism = driel::VanDrielState::<PointContainer3D>::new(point_container_3d, theta);
    van_driel_mechanism.explain(neighborhood_size)
}