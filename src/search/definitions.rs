extern crate nalgebra as na;

use std::rc::Rc;

use rstar::RTree;

use crate::exp::{DaSilvaExplanation, VanDrielExplanation};

use na::{Dim, U3, U4, U5, U6, U7, U8, U9};

pub struct PointData {
    index: usize,
    dimensionality: usize,
    low: na::Point<f32, U3>,
    high: na::DVector<f32>,
    normal: Option<na::Point<f32, U3>>,
    driel: Option<VanDrielExplanation>,
    silva: Option<DaSilvaExplanation>,
}

/// Low dimensional point with reference to data
#[derive(Clone)]
pub struct LDPoint {
    point: na::Point<f32, U3>,
    data: Rc<PointData>,
}

/// Simple RTree object definition so it can be used in
/// the tree structure
impl rstar::RTreeObject for LDPoint {
    type Envelope = rstar::AABB<[f32; 3]>;
    fn envelope(&self) -> Self::Envelope {
        rstar::AABB::from_point([self.point.x, self.point.y, self.point.z])
    }
}

/// High dimensional point with reference to the data
#[derive(Clone)]
pub struct HDPoint<Dim: na::DimName + na::Dim> {
    dim: Dim,
    point: na::DVector<f32>,
    data: Rc<PointData>,
}

/// Move this to other file
macro_rules! dimension_data_definition(
    ($($Dim: ident, $Index: literal);* $(;)*) => {$(
        impl rstar::RTreeObject for HDPoint<$Dim>
        where {
            type Envelope = rstar::AABB<[f32; $Index]>;
            fn envelope(&self) -> Self::Envelope {
                let mut data: [f32; $Index] = Default::default();
                data.copy_from_slice(&self.point.data.as_vec()[..$Index]);
                rstar::AABB::from_point(data)
            }
        }
    )*}
);

dimension_data_definition!(U4, 4; U5, 5; U6, 6; U7, 7; U8, 8; U9, 9);

// TODO: Implement the rstar::Point for U10 up to U35
// TODO: Extend the rstar::RTreeObject trait from U10 up to U35
