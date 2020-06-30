// Internal module file used to define the common interface with all the explanation mechanisms and datatypes.

use crate::util::types::{Point3, PointN};

#[derive(Debug, PartialEq)]
pub struct Point {
    pub reduced: Point3,
    pub original: PointN,
}

#[derive(Debug, PartialEq)]
pub struct AnnotatedPoint<T> {
    pub reduced: Point3,
    pub original: PointN,
    pub annotation: T
}

pub trait ExplanationMechanism<T> {
    // Initialize a explanation mechanism using the original and reduced data.
    fn init(dataset: Vec<Point>) -> Self;

    // Using this mechanism explain single point
    fn explain(&self, point: Point) -> AnnotatedPoint<T>;

    // TODO: create a method to explain all points at once to prevent recalculating neighborhoods.
}
