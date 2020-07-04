// The attributes based explanation mechanism based on the works of Da Silva et al.

// Multidimensional projections (MPs) are key tools for the analysis of multidimensional data. MPs reduce data dimensionality
// while keeping the original distance structure in the low-dimensional output space, typically shown by a 2D scatterplot. While
// MP techniques grow more precise and scalable, they still do not show how the original dimensions (attributes) influence the
// projection’s layout. In other words, MPs show which points are similar, but not why. We propose a visual approach to describe
// which dimensions contribute mostly to similarity relationships over the projection, thus explain the projection’s layout. For
// this, we rank dimensions by increasing variance over each point-neighborhood, and propose a visual encoding to show the
// least-varying dimensions over each neighborhood. We demonstrate our technique with both synthetic and real-world datasets.

use crate::util::types::PointN;
use super::common::{AnnotatedPoint, ExplanationMechanism, Point};

use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub struct DaSilvaExplanation {
    // Attribute index is the index of which dimension in nD is most important for this point
    attribute_index: i32,
    // The is the confidence we have in said attribute index
    confidence: f32,
}

#[derive(Debug, PartialEq)]
pub struct DaSilvaState {
    // Reference to all points in the dataset.
    points_ref: Vec<Point>,
    // The global dimension ranking for each dimension. Only top 8 will be used to colour encode.
    global_dimension_ranking: HashMap<i32, i32>,
    // The size of the neigborhoud in which to calculate the local metrics.
    neighborhood_size: i32,
}

impl DaSilvaState {
    pub fn new(points: Vec<Point>, neighborhood_size: i32) -> DaSilvaState {
        DaSilvaState {
            points_ref: points,
            global_dimension_ranking: HashMap::new(),
            neighborhood_size,
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

// TODO: Make an abstraction for expanations. this does not seem very clean but so be it.
pub fn explain(input: &Vec<Point>) -> AnnotatedPoint<DaSilvaExplanation> {
    unimplemented!("Not there yet")
    // Calculate the neighbors v_i for point p_i in nD
    // Calculate the local contribution lc_j between each point p_i and all its neighbors v_i for every dimension j
    // Average the contribution for every dimension within the neighborhood
    // Calculate the global contribution of each point (centriod of the nD space and _every_ point in its neighborhood)
    // Normalize the local contribution by dividing by the global contribution (per dimension)
    //
}

// Used for the global explanation, just average the over all dimensions
fn find_centroid(points: &Vec<Point>) -> PointN {
    unimplemented!()
}

// Find the indexes of the n closest neighbor for a point
fn find_neighbors(point_index: usize, neighbor_count: i32, points: &Vec<Point>) -> Vec<usize> {
    unimplemented!()
}

// Given a point index, the set of points and the indices of the neighbors calculate the local contribution
// lc^j_p,r = (p_j - r_j)^2 / ||p-r||^2
// lc_j = Sum over r in neighborhood of lc^j_p,r devided by neighborhood size.
// This function returns a vector of the lc values for each dimension. It corresponds to formula 1 and 2
fn local_contributions(point_index: usize, points: &Vec<Point>, neighbor_indices: Vec<usize>) -> Vec<f32> {
    unimplemented!()
}

// Normalize a local contrib of a dimension using the global contrib of said dimension.
// this function lines up with formulate 3 in the works
fn normalize_rankings(local_contribution: f32, global_contribution: f32) -> f32 {
    unimplemented!()
}
