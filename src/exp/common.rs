// Internal module file used to define the common interface with all the explanation mechanisms and datatypes.

use space;

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

pub struct Euclidean<'a>(pub &'a Point3);

impl<'a> space::MetricPoint for Euclidean<'a> {
    fn distance(&self, rhs: &Self) -> u32 {
        space::f32_metric({
            let x: f32 = &self.0.x - &rhs.0.x;
            let y: f32 = &self.0.y - &rhs.0.y;
            let z: f32 = &self.0.z - &rhs.0.z;
            let t: f32 = x * x + y * y + z * z;
            t.sqrt()
        })
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
