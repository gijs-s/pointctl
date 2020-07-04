//! The attributes based explanation mechanism based on the works of Da Silva et al.
// Multidimensional projections (MPs) are key tools for the analysis of multidimensional data. MPs reduce data dimensionality
// while keeping the original distance structure in the low-dimensional output space, typically shown by a 2D scatterplot. While
// MP techniques grow more precise and scalable, they still do not show how the original dimensions (attributes) influence the
// projection’s layout. In other words, MPs show which points are similar, but not why. We propose a visual approach to describe
// which dimensions contribute mostly to similarity relationships over the projection, thus explain the projection’s layout. For
// this, we rank dimensions by increasing variance over each point-neighborhood, and propose a visual encoding to show the
// least-varying dimensions over each neighborhood. We demonstrate our technique with both synthetic and real-world datasets.
//
// TODO list for this part of the explain module:
// Must haves
// - Implement a basic version of the mechanism
// - Tests for the functions used by the mechanism
// Nice to haves:
// - Use a faster way to calculate nn (maybe approximate nearest neighbors?)
// - Store the data in a more space efficient way

use super::common::Point;
use crate::util::types::PointN;

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
    // The size of the neigborhood in which to calculate the local metrics.
    neighborhood_size: f32,
}

// Types to make the code more readable
type NeighborIndices = Vec<usize>;
type LocalContributions = Vec<f32>;
type GlobalContribution = Vec<f32>;

// TODO: Make an abstraction for the explanation, this is not as clean as it should be
pub fn explain(
    input: &Vec<Point>,
    // P
    neighborhood_size: f32,
) -> Vec<DaSilvaExplanation> {
    // Calculate the global contribution of each point (centriod of the nD space and _every_ point in its neighborhood)
    let centroid: PointN = find_centroid(&input);
    let global_contribution: GlobalContribution = calculate_global_contribution(centroid, &input);

    // Calculate all neighborhoods, each neighborhood consists of all points witing p v_i for point p_i in nD
    let neighborhoods: Vec<NeighborIndices> = (0..input.len())
        .into_iter()
        .map(|i| find_neighbors(i, &input, neighborhood_size))
        .collect();

    // Calculate the normalized local contributions.
    let local_contributions: Vec<LocalContributions> = neighborhoods
        .iter()
        .enumerate()
        .map(|(i, n)| {
            // Calculate the local contribution lc_j between each point p_i and all its neighbors v_i for every dimension j
            // Average the contribution for every dimension within the neighborhood
            let lc: LocalContributions = calculate_local_contributions(i, &input, n);
            // Normalize the local contribution by dividing by the global contribution (per dimension)
            normalize_rankings(lc, &global_contribution)
        })
        .collect();

    // Using the local contributions and the neighborhoods calculate the top ranking dimension and its
    // confidence within the neighborhood.
    neighborhoods
        .iter()
        .enumerate()
        .map(|(i, n)| calculate_annotation(i, &local_contributions, n))
        .collect::<Vec<DaSilvaExplanation>>()
}

// Used for the global explanation, just average the over all dimensions
fn find_centroid(points: &Vec<Point>) -> PointN {
    unimplemented!()
}

// Find the indexes of the n closest neighbor for a point
fn find_neighbors(
    point_index: usize,
    points: &Vec<Point>,
    neighborhood_size: f32,
) -> NeighborIndices {
    unimplemented!()
}

// Given a point index, the set of points and the indices of the neighbors calculate the local contribution
// lc^j_p,r = (p_j - r_j)^2 / ||p-r||^2
// lc_j = Sum over r in neighborhood of lc^j_p,r devided by neighborhood size.
// This function returns a vector of the lc values for each dimension. It corresponds to formula 1 and 2
fn calculate_local_contributions(
    point_index: usize,
    points: &Vec<Point>,
    neighbor_indices: &NeighborIndices,
) -> LocalContributions {
    unimplemented!()
}

fn calculate_global_contribution(centroid: PointN, points: &Vec<Point>) -> GlobalContribution {
    unimplemented!()
}

// Normalize a local contrib of a dimension using the global contrib of said dimension.
// this function lines up with formulate 3 in the works
fn normalize_rankings(
    local_contributions: LocalContributions,
    global_contributions: &GlobalContribution,
) -> LocalContributions {
    unimplemented!()
}

// From the local contribution find the top 1 ranking with the confidence
fn calculate_annotation(
    point_index: usize,
    local_contributions: &Vec<LocalContributions>,
    neighborhood: &NeighborIndices,
) -> DaSilvaExplanation {
    unimplemented!()
}
