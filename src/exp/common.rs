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
/// on the heap after finding the nn in 3D.
#[derive(Copy, Clone, PartialEq, Debug)]
pub struct IndexedPoint {
    pub index: usize,
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

// implement the rstar point for the IndexedPoint so
// it can be used in a rtree
// impl rstar::Point for IndexedPoint {
//     type Scalar = f32;
//     const DIMENSIONS: usize = 3;

//     fn generate(generator: impl Fn(usize) -> Self::Scalar) -> Self
//     {
//         IndexedPoint {
//             // Can't index since we do not now the global state
//             index: 0,
//             x: generator(0),
//             y: generator(1),
//             z: generator(2)
//         }
//     }

//     fn nth(&self, index: usize) -> Self::Scalar
//     {
//       match index {
//         0 => self.x,
//         1 => self.y,
//         2 => self.z,
//         _ => unreachable!()
//       }
//     }

//     fn nth_mut(&mut self, index: usize) -> &mut Self::Scalar
//     {
//       match index {
//         0 => &mut self.x,
//         1 => &mut self.y,
//         2 => &mut self.z,
//         _ => unreachable!()
//       }
//     }
// }

impl rstar::RTreeObject for IndexedPoint {
    type Envelope = rstar::AABB<[f32; 3]>;
    fn envelope(&self) -> Self::Envelope {
        rstar::AABB::from_point([self.x, self.y, self.z])
    }
}

impl rstar::PointDistance for IndexedPoint {
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
