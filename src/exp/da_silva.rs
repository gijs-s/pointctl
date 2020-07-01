// The attributes based explanation mechanism based on the works of Da Silva et al.

// Multidimensional projections (MPs) are key tools for the analysis of multidimensional data. MPs reduce data dimensionality
// while keeping the original distance structure in the low-dimensional output space, typically shown by a 2D scatterplot. While
// MP techniques grow more precise and scalable, they still do not show how the original dimensions (attributes) influence the
// projection’s layout. In other words, MPs show which points are similar, but not why. We propose a visual approach to describe
// which dimensions contribute mostly to similarity relationships over the projection, thus explain the projection’s layout. For
// this, we rank dimensions by increasing variance over each point-neighborhood, and propose a visual encoding to show the
// least-varying dimensions over each neighborhood. We demonstrate our technique with both synthetic and real-world datasets.

// use crate::util::types::{Point3, PointN};
use super::common::{AnnotatedPoint, ExplanationMechanism, Point};

use std::collections::HashMap;

#[derive(Debug, PartialEq)]
struct DaSilvaExplanation {
    attribute_index: i32,
    confidence: f32,
}

#[derive(Debug, PartialEq)]
struct DaSilvaState {
    // Reference to all points in the dataset.
    points_ref: Vec<Point>,
    // The global dimension ranking for each dimension. Only top 8 will be used to colour encode.
    global_dimension_ranking: HashMap<i32, i32>,
    neighborhood_size: i32,
}

impl DaSilvaState {
    pub fn new(points: Vec<Point>, neighborhood_size: i32) -> DaSilvaState {
        DaSilvaState {
            points_ref: points,
            global_dimension_ranking: HashMap::new(),
            neighborhood_size: neighborhood_size,
        }
    }
}

impl ExplanationMechanism<DaSilvaExplanation> for DaSilvaState {
    fn init(dataset: Vec<Point>) -> DaSilvaState {
        DaSilvaState::new(dataset, 20)
    }

    // Placeholder explanation for a point.
    fn explain(&self, point: Point) -> AnnotatedPoint<DaSilvaExplanation> {
        AnnotatedPoint {
            reduced: point.reduced,
            original: point.original,
            annotation: DaSilvaExplanation {
                attribute_index: 1,
                confidence: 0.5,
            },
        }
    }
}
