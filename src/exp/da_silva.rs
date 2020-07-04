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

use std::cmp::Ordering;
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
type Ranking = (usize, f32);
type RankingVector = Vec<(usize, f32)>;

// TODO: Make an abstraction for the explanation, this is not as clean as it should be
// Tuple of explanations and the dimension ids sorted by rank.
pub fn explain(
    input: &Vec<Point>,
    // P
    neighborhood_size: f32,
) -> (Vec<DaSilvaExplanation>, Vec<usize>) {
    // Calculate the global contribution of each point (centriod of the nD space and _every_ point in its neighborhood)
    let centroid: PointN = find_centroid(&input);
    let global_contribution: GlobalContribution = calculate_global_contribution(centroid, &input);

    // Calculate the ranking vectors, a nd vector containing the IDs and ranks of each dimension.
    let ranking_vectors: Vec<Ranking> = (0..input.len())
        .into_iter()
        .map(|i| {
            // Calculate all neighborhoods, each neighborhood consists of all points witing p v_i for point p_i in nD
            let n: NeighborIndices = find_neighbors_nd(i, &input, neighborhood_size);
            // Calculate the local contribution lc_j between each point p_i and all its neighbors v_i for every dimension j
            // Average the contribution for every dimension within the neighborhood
            let lc: LocalContributions = calculate_local_contributions(i, &input, n);
            // Normalize the local contribution by dividing by the global contribution (per dimension)
            let nlc: LocalContributions = normalize_rankings(lc, &global_contribution);
            // Create a ranking vector from the normalized local contribution
            calculate_top_ranking(nlc)
        })
        .collect();

    // Using the ranking vectors calculate the attribute index and the confidence using 3d nn.
    let explanations: Vec<DaSilvaExplanation> = (0..input.len())
        .into_iter()
        .map(|i| {
            let n: NeighborIndices = find_neighbors(i, &input, neighborhood_size);
            calculate_annotation(i, &ranking_vectors, n)
        })
        .collect();

    // TODO: Calculate the global ranking of dimension confidence, useful for the colour encoding.
    (explanations, vec![1])
}

// Used for the global explanation, just average the over all dimensions
fn find_centroid(points: &Vec<Point>) -> PointN {
    unimplemented!()
}

// Find the indexes of each nearest neighbor falling withing the size in nD.
fn find_neighbors_nd(
    point_index: usize,
    points: &Vec<Point>,
    neighborhood_size: f32,
) -> NeighborIndices {
    unimplemented!()
}

// Find the indexes of each nearest neighbor falling withing the size in 3D.
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
    neighbor_indices: NeighborIndices,
) -> LocalContributions {
    unimplemented!()
}

// Does the same as calculate_local_contributions but for the entire dataset.
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

// From the local contribution find the top 1 ranking with the confidence.
// This is simply done by returning the highest ranking dimension for the point
// and the confidence checks how many in the neighborhood share this highest point
fn calculate_annotation(
    point_index: usize,
    ranking_vectors: &Vec<Ranking>,
    neighborhood: NeighborIndices,
) -> DaSilvaExplanation {
    unimplemented!()
}

// From the sorted vector of local contributions and find the dimension than contributes most
// TODO this seems like a nasty hack.
fn calculate_top_ranking(local_contributions: LocalContributions) -> Ranking {
    local_contributions
        .iter()
        .enumerate()
        .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap_or(Ordering::Equal))
        .map(|(index, f)| (index, *f)).unwrap()
}
