// The attributes based explanation mechanism based on the works of Da Silva et al.

// Multidimensional projections (MPs) are key tools for the analysis of multidimensional data. MPs reduce data dimensionality
// while keeping the original distance structure in the low-dimensional output space, typically shown by a 2D scatterplot. While
// MP techniques grow more precise and scalable, they still do not show how the original dimensions (attributes) influence the
// projection’s layout. In other words, MPs show which points are similar, but not why. We propose a visual approach to describe
// which dimensions contribute mostly to similarity relationships over the projection, thus explain the projection’s layout. For
// this, we rank dimensions by increasing variance over each point-neighborhood, and propose a visual encoding to show the
// least-varying dimensions over each neighborhood. We demonstrate our technique with both synthetic and real-world datasets.

use crate::util::types::Point;
use super::common::{AnnotatedPoint, Explanation};

#[derive(Debug)]
struct ExplanationDaSilva {
    attribute_index: i32,
    confidence: f32,
}

impl Explanation<ExplanationDaSilva> for ExplanationDaSilva {
    // Placeholder explanation for a point.
    fn explain(point: Point) -> AnnotatedPoint<ExplanationDaSilva>{
        AnnotatedPoint {
            point: point,
            annotation: ExplanationDaSilva {
                attribute_index: 1,
                confidence: 0.5,
            },
        }
    }
}