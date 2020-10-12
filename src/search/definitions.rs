extern crate nalgebra as na;

use std::{marker::PhantomData, rc::Rc};

use rstar::RTree;

use crate::exp::{DaSilvaExplanation, VanDrielExplanation};

// use na::{Dim, U2, U3, U4, U5, U6, U7, U8, U9};
use typenum::{Unsigned, U2, U3, U4, U5, U6, U7, U8, U9};


pub struct PointData {
    pub index: usize,
    pub dimensionality: usize,
    pub low: Vec<f32>,
    pub high: Vec<f32>,
    pub normal: Option<na::Point3<f32>>,
    pub driel: Option<VanDrielExplanation>,
    pub silva: Option<DaSilvaExplanation>,
}

/// Trait binding all the dimensions that can be used in the RTreeObject
pub trait D : Unsigned {}
macro_rules! supported_dimension {($($Dim: ident),* $(,)*) => {$(impl D for $Dim {})*}}
supported_dimension!(U2, U3, U4, U5, U6, U7, U8, U9);

/// Trait binding dimensions we consider low dimensionality (reduced)
pub trait LD : D
where LDPoint<Self> : rstar::RTreeObject {}

/// Low dimensional point with reference to data
#[derive(Clone)]
pub struct LDPoint<Dim: LD>
where Self : rstar::RTreeObject {
    pub point: Vec<f32>,
    pub data: Rc<PointData>,
    _phantom: PhantomData<Dim>,
}

impl<Dim> LDPoint<Dim> where Dim : LD, Self : rstar::RTreeObject {
    pub fn new(point: &Vec<f32>, data: Rc<PointData>) -> Self {
        LDPoint::<Dim> {
            point: point.to_vec(),
            data,
            _phantom: PhantomData
        }
    }
}

macro_rules! define_low_point_rtree_object(
    ($($Dim: ident),* $(,)*) => {$(
        impl LD for $Dim {}

        impl rstar::RTreeObject for LDPoint<$Dim>
        where {
            type Envelope = rstar::AABB<[f32; <$Dim as Unsigned>::USIZE]>;
            fn envelope(&self) -> Self::Envelope {
                let mut data: [f32; <$Dim as Unsigned>::USIZE] = Default::default();
                data.copy_from_slice(&self.point[..<$Dim as Unsigned>::USIZE]);
                rstar::AABB::from_point(data)
            }
        }
    )*}
);
define_low_point_rtree_object!(U2, U3);
/// Trait biding the dimensionality of the original points
pub trait HD : D
where HDPoint<Self> : rstar::RTreeObject {}

/// High dimensional point with reference to the data.
#[derive(Clone)]
pub struct HDPoint<Dim: HD>
where Self : rstar::RTreeObject {
    point: Vec<f32>,
    data: Rc<PointData>,
    _phantom: PhantomData<Dim>,
}

impl<Dim> HDPoint<Dim> where Dim : HD, Self : rstar::RTreeObject {
    pub fn new(point: &Vec<f32>, data: Rc<PointData>) -> Self {
        HDPoint::<Dim> {
            point: point.to_vec(),
            data,
            _phantom: PhantomData
        }
    }
}

macro_rules! define_high_point_rtree_object(
    ($($Dim: ident),* $(,)*) => {$(
        impl HD for $Dim {}

        impl rstar::RTreeObject for HDPoint<$Dim>
        where {
            type Envelope = rstar::AABB<[f32; <$Dim as Unsigned>::USIZE]>;
            fn envelope(&self) -> Self::Envelope {
                let mut data: [f32; <$Dim as Unsigned>::USIZE] = Default::default();
                data.copy_from_slice(&self.point[..<$Dim as Unsigned>::USIZE]);
                rstar::AABB::from_point(data)
            }
        }
    )*}
);
define_high_point_rtree_object!(U4, U5, U6, U7, U8, U9);

// TODO: Implement the rstar::Point for [f32; 10] up to 50
// TODO: Extend the rstar::RTreeObject trait from U10 up to U50
