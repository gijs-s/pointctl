// Internal module file used to define the common interface with all the explanation mechanisms and datatypes.

use rstar;
use rstar::{RStarInsertionStrategy, RTreeParams};

use crate::util::types::{Point3, PointN};

#[derive(Clone, PartialEq, Debug)]
pub struct PointTuple {
    pub reduced: Point3,
    pub original: PointN,
}

#[derive(Debug, PartialEq)]
pub struct AnnotatedPoint<T> {
    pub point: PointTuple,
    pub annotation: T,
}

impl<T> AnnotatedPoint<T> {
    pub fn annotate(point: PointTuple, annotation: T) -> Self {
        AnnotatedPoint {
            point: point,
            annotation: annotation,
        }
    }
}

///! Used to store this in the rtree, we can not store
/// PointN in here since it is stored on the heap. When
/// we keep de index so we can search for the ND point
/// on the heap after finding the nn in 2/3D.
#[derive(Copy, Clone, PartialEq, Debug)]
pub struct IndexedPoint2D {
    pub index: usize,
    pub x: f32,
    pub y: f32,
}

impl rstar::RTreeObject for IndexedPoint2D {
    type Envelope = rstar::AABB<[f32; 2]>;
    fn envelope(&self) -> Self::Envelope {
        rstar::AABB::from_point([self.x, self.y])
    }
}

impl rstar::PointDistance for IndexedPoint2D {
    fn distance_2(&self, point: &[f32; 2]) -> f32 {
        let x: f32 = point[0] - self.x;
        let y: f32 = point[1] - self.y;
        x.powi(2) + y.powi(2)
    }

    fn contains_point(&self, point: &[f32; 2]) -> bool {
        self.x == point[0] && self.y == point[1]
    }

    fn distance_2_if_less_or_equal(&self, point: &[f32; 2], max_distance_2: f32) -> Option<f32> {
        let t = self.distance_2(point);
        if t <= max_distance_2 {
            Some(t.sqrt())
        } else {
            None
        }
    }
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct IndexedPoint3D {
    pub index: usize,
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl rstar::RTreeObject for IndexedPoint3D {
    type Envelope = rstar::AABB<[f32; 3]>;
    fn envelope(&self) -> Self::Envelope {
        rstar::AABB::from_point([self.x, self.y, self.z])
    }
}

impl rstar::PointDistance for IndexedPoint3D {
    fn distance_2(&self, point: &[f32; 3]) -> f32 {
        let x = point[0] - self.x;
        let y: f32 = point[1] - self.y;
        let z = point[2] - self.z;
        x.powi(2) + y.powi(2) + z.powi(2)
    }

    fn contains_point(&self, point: &[f32; 3]) -> bool {
        self.x == point[0] && self.y == point[1] && self.z == point[2]
    }

    fn distance_2_if_less_or_equal(&self, point: &[f32; 3], max_distance_2: f32) -> Option<f32> {
        let t = self.distance_2(point);
        if t <= max_distance_2 {
            Some(t.sqrt())
        } else {
            None
        }
    }
}

// Distance calculation
pub trait Distance {
    // Euclidean distance
    fn distance(&self, other: &Self) -> f32;

    fn sq_distance(&self, other: &Self) -> f32;
}

impl Distance for Point3 {
    fn distance(&self, other: &Self) -> f32 {
        self.sq_distance(other).sqrt()
    }

    fn sq_distance(&self, other: &Self) -> f32 {
        let x: f32 = &self.x - &other.x;
        let y: f32 = &self.y - &other.y;
        let z: f32 = &self.z - &other.z;
        x * x + y * y + z * z
    }
}

impl Distance for PointN {
    fn distance(&self, other: &Self) -> f32 {
        self.sq_distance(other).sqrt()
    }

    fn sq_distance(&self, other: &Self) -> f32 {
        self.iter()
            .zip(other)
            .map(|(a, b)| {
                let i = a - b;
                i * i
            })
            .fold(0.0f32, |sum, v| sum + v)
    }
}

// RTree parameters
pub struct RTreeParameters2D;
pub struct RTreeParameters3D;

// Custom RTree parameters for 2D points
// TODO: Explain these parameters
impl RTreeParams for RTreeParameters2D {
    const MIN_SIZE: usize = 5;
    const MAX_SIZE: usize = 9;
    const REINSERTION_COUNT: usize = 5;
    type DefaultInsertionStrategy = RStarInsertionStrategy;
}

// Custom RTree parameters for 3D points
// TODO: Explain these parameters
impl RTreeParams for RTreeParameters3D {
    const MIN_SIZE: usize = 10;
    const MAX_SIZE: usize = 20;
    const REINSERTION_COUNT: usize = 5;
    type DefaultInsertionStrategy = RStarInsertionStrategy;
}
