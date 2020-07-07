// Internal module file used to define the common interface with all the explanation mechanisms and datatypes.

use rstar;

use crate::util::types::{Point3, PointN};

#[derive(Clone, PartialEq, Debug)]
pub struct PointTuple {
    pub reduced: Point3,
    pub original: PointN,
}

#[derive(Debug, PartialEq)]
pub struct AnnotatedPoint<T> {
    pub point: PointTuple,
    pub annotation: T
}

impl<T> AnnotatedPoint<T>{
    pub fn annotate(point: PointTuple, annotation: T) -> Self {
        AnnotatedPoint {
            point: point,
            annotation: annotation
        }
    }
}

///! Used to store this in the rtree, we can not store
/// PointN in here since it is stored on the heap. When
/// we keep de index so we can search for the ND point
/// on the heap after finding the nn in 3D.
#[derive(Copy, Clone, PartialEq, Debug)]
pub struct IndexedPoint {
    pub index: usize,
    pub point: Point3
}

// implement the rstar point for the IndexedPoint so
// it can be used in a rtree
impl rstar::Point for IndexedPoint {
    type Scalar = f32;
    const DIMENSIONS: usize = 3;

    fn generate(generator: impl Fn(usize) -> Self::Scalar) -> Self
    {
        IndexedPoint {
            // Can't index since we do not now the global state
            index: 0,
            point: Point3::new(generator(0), generator(1), generator(2))
        }
    }

    fn nth(&self, index: usize) -> Self::Scalar
    {
      match index {
        0 => self.point.x,
        1 => self.point.y,
        2 => self.point.z,
        _ => unreachable!()
      }
    }

    fn nth_mut(&mut self, index: usize) -> &mut Self::Scalar
    {
      match index {
        0 => &mut self.point.x,
        1 => &mut self.point.y,
        2 => &mut self.point.z,
        _ => unreachable!()
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
        self.iter().zip(other).map(|(a, b)| {
            let i = a - b;
            i * i
        })
        .fold(0.0f32, |sum, v| sum + v)
    }
}
