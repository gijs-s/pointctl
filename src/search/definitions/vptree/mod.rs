//! Module containing the code for finding neighbors using vantage point trees
//! in ND. Vantage point trees only store the distances between points so it
//! seems suited well to my needs.

/// Sub modules
mod count;
mod radius;

/// Structs that should be reexported from this module.
pub use self::{count::CountBasedNeighborhood, radius::RadiusBasedNeighborhood};

/// Third party imports
use vpsearch::MetricSpace;

/// First party imports
use super::data::IndexedPoint;
use crate::search::definitions::generic::Distance;

impl MetricSpace for IndexedPoint<Vec<f32>> {
    type UserData = ();
    type Distance = f32;

    fn distance(&self, other: &Self, _: &Self::UserData) -> Self::Distance {
        self.point
            .iter()
            .zip(other.point.iter())
            .map(|(s, o)| (s - o).powi(2))
            .sum::<f32>()
            .sqrt()
    }
}

impl MetricSpace for IndexedPoint<na::Point2<f32>> {
    type UserData = ();
    type Distance = f32;

    fn distance(&self, other: &Self, _: &Self::UserData) -> Self::Distance {
        self.point.distance(&other.point)
    }
}

impl MetricSpace for IndexedPoint<na::Point3<f32>> {
    type UserData = ();
    type Distance = f32;

    fn distance(&self, other: &Self, _: &Self::UserData) -> Self::Distance {
        self.point.distance(&other.point)
    }
}
