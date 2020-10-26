/// Module containing the explanation mechanisms used for the visualization.
// Sub modules
mod da_silva;
mod driel;
mod explanation;
mod normal;

// Re-export the public facing parts of this module
pub use self::{
    da_silva::{DaSilvaExplanation, DaSilvaType},
    driel::{VanDrielExplanation, VanDrielType},
    normal::NormalExplanation,
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

impl Neighborhood {
    pub fn to_string(&self) -> String {
        match self {
            Neighborhood::K(u) => format!("K: {}", u),
            Neighborhood::R(r) => format!("R: {}", r),
        }
    }
}

pub fn run_da_silva_2d<'a>(
    point_container_2d: &'a PointContainer2D,
    neighborhood_size: Neighborhood,
    method: DaSilvaType,
) -> Vec<DaSilvaExplanation> {
    let da_silva_mechanism =
        da_silva::DaSilvaState::<'a, PointContainer2D>::new(point_container_2d, method);
    da_silva_mechanism.explain(neighborhood_size)
}

pub fn run_da_silva_3d<'a>(
    point_container_3d: &'a PointContainer3D,
    neighborhood_size: Neighborhood,
    method: DaSilvaType,
) -> Vec<DaSilvaExplanation> {
    let da_silva_mechanism =
        da_silva::DaSilvaState::<'a, PointContainer3D>::new(point_container_3d, method);
    da_silva_mechanism.explain(neighborhood_size)
}

pub fn run_van_driel_2d<'a>(
    point_container_2d: &'a PointContainer2D,
    neighborhood_size: Neighborhood,
    theta: f32,
    method: VanDrielType,
) -> Vec<VanDrielExplanation> {
    let van_driel_mechanism =
        driel::VanDrielState::<PointContainer2D>::new(point_container_2d, theta, method);
    van_driel_mechanism.explain(neighborhood_size)
}

pub fn run_van_driel_3d<'a>(
    point_container_3d: &'a PointContainer3D,
    neighborhood_size: Neighborhood,
    theta: f32,
    method: VanDrielType,
) -> Vec<VanDrielExplanation> {
    let van_driel_mechanism =
        driel::VanDrielState::<PointContainer3D>::new(point_container_3d, theta, method);
    van_driel_mechanism.explain(neighborhood_size)
}
