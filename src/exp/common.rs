// Internal module file used to define the common interface with all the explanation mechanisms and datatypes.

use crate::util::types::Point;

#[derive(Debug)]
pub struct AnnotatedPoint<T> {
    pub point: Point,
    pub annotation: T
}

pub trait Explanation<T> {
    fn explain(point: Point) -> AnnotatedPoint<T>;
}