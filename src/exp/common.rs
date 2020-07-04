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
