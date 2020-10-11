// Module containing the explanation mechanisms used for the visualization.
// TODO: Add prelude?

pub use self::{
    common::{
        AnnotatedPoint, IndexedPoint2D, IndexedPoint3D, RTreeParameters2D, RTreeParameters3D,
    },
    da_silva::DaSilvaExplanation,
    driel::VanDrielExplanation,
};

mod common;
mod da_silva;
mod driel;
mod explanation;

use crate::util::types::PointN;
use explanation::Explanation;
use nalgebra::{Point2, Point3};

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Neighborhood {
    K(usize),
    R(f32),
}

pub fn run_da_silva_variance(
    reduced_points: Vec<Point3<f32>>,
    original_points: &[PointN],
    neighborhood_size: Neighborhood,
) -> Vec<DaSilvaExplanation> {
    let da_silva_mechanism = da_silva::DaSilvaState::new(reduced_points, original_points);
    da_silva_mechanism.explain(neighborhood_size)
}

pub fn run_da_silva_variance_indexed(
    indexed_points: Vec<IndexedPoint3D>,
    original_points: &[PointN],
    neighborhood_size: Neighborhood,
) -> Vec<DaSilvaExplanation> {
    let da_silva_mechanism =
        da_silva::DaSilvaState::new_with_indexed_point(indexed_points, original_points);
    da_silva_mechanism.explain(neighborhood_size)
}

pub fn run_van_driel(
    reduced_points: Vec<Point3<f32>>,
    original_points: &[PointN],
    neighborhood_size: Neighborhood,
) -> Vec<VanDrielExplanation> {
    // TODO: Remove dummy value
    let theta = 0.5f32;
    let van_driel_mechanism = driel::VanDrielState::new(reduced_points, original_points, theta);
    van_driel_mechanism.explain(neighborhood_size)
}

pub fn run_van_driel_indexed(
    indexed_points: Vec<IndexedPoint3D>,
    original_points: &[PointN],
    neighborhood_size: Neighborhood,
) -> Vec<VanDrielExplanation> {
    // TODO: Remove dummy value
    let theta = 0.5f32;
    let van_driel_mechanism = driel::VanDrielState::new_with_indexed_point(indexed_points, original_points, theta);
    van_driel_mechanism.explain(neighborhood_size)
}
