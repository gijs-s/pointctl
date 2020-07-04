// Internal module file used to define the common interface with all the explanation mechanisms and datatypes.

use crate::util::types::{Point3, PointN};

#[derive(Debug, PartialEq)]
pub struct Point {
    pub reduced: Point3,
    pub original: PointN,
}

#[derive(Debug, PartialEq)]
pub struct AnnotatedPoint<T> {
    pub point: Point,
    pub annotation: T
}

impl<T> AnnotatedPoint<T>{
    pub fn annotate(point: Point, annotation: T) -> Self {
        AnnotatedPoint {
            point: point,
            annotation: annotation
        }
    }
}

pub trait Distance {
    // Euclidean distance
    fn distance(&self, other: &Self) -> f32;
}

impl Distance for Point3 {
    fn distance(&self, other: &Self) -> f32 {
        let x: f32 = &self.x - &other.x;
        let y: f32 = &self.y - &other.y;
        let z: f32 = &self.z - &other.z;
        let t = x * x + y * y + z * z;
        t.sqrt()
    }
}

impl Distance for PointN {
    fn distance(&self, other: &Self) -> f32 {
        self.iter().zip(other).map(|(a, b)| {
            let i = a - b;
            i * i
        })
        .fold(0.0f32, |sum, v| sum + v)
        .sqrt()
    }
}
