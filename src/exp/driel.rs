// The enhanced attributes based explanation mechanism based on the works of van Driel et al.

// Abstract: Multidimensional projections (MPs) are established tools for exploring the structure of high-dimensional datasets to
// reveal groups of similar observations. For optimal usage, MPs can be augmented with mechanisms that explain what such points have
// in common that makes them similar. We extend the set of such explanatory instruments by two new techniques. First, we compute
// and encode the local dimensionality of the data in the projection, thereby showing areas where the MP can be well explained
// by a few latent variables. Secondly, we compute and display local attribute correlations, thereby helping the user to discover
// alternative explanations for the underlying phenomenon. We implement our explanatory tools using an image-based approach,
// which is efficient to compute, scales well visually for large and dense MP scatterplots, and can handle any projection technique.
// We demonstrate our approach using several datasets.

use super::common::{Point, AnnotatedPoint, ExplanationMechanism};

#[derive(Debug, PartialEq)]
struct ExplanationDriel {
    dimension: i32,
    confidence: f32,
}

#[derive(Debug, PartialEq)]
struct DrielState {
    points_ref: Vec<Point>
}

impl ExplanationMechanism<ExplanationDriel> for DrielState {
    fn new(dataset: Vec<Point>) -> DrielState {
        DrielState { points_ref: dataset}
    }


    // Placeholder explanation for a point.
    fn explain(point: Point) -> AnnotatedPoint<ExplanationDriel>{
        AnnotatedPoint {
            reduced: point.reduced,
            original: point.original,
            annotation: ExplanationDriel {
                dimension: 1,
                confidence: 0.5,
            },
        }
    }
}