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

use super::common::{Distance, Point};
use crate::util::types::PointN;

use std::cmp::Ordering;

// Types to make the code more readable
type NeighborIndices = Vec<usize>;
type LocalContributions = Vec<f32>;
type GlobalContribution = Vec<f32>;
type Ranking = (usize, f32);

#[derive(Debug, PartialEq)]
pub struct DaSilvaExplanation {
    // Attribute index is the index of which dimension in nD is most important for this point
    attribute_index: i32,
    // The is the confidence we have in said attribute index
    confidence: f32,
}

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
    points
        .iter()
        .fold(vec![0.0f32; points[0].original.len()], |v, p| {
            v.iter().zip(&p.original).map(|(a, b)| a + b).collect()
        })
        .iter()
        .map(|x| x / points.len() as f32)
        .collect::<PointN>()
}

// Find the indexes of each nearest neighbor falling withing the size in nD.
fn find_neighbors_nd(
    point_index: usize,
    points: &Vec<Point>,
    neighborhood_size: f32,
) -> NeighborIndices {
    let point = &points[point_index];
    points
        .iter()
        .enumerate()
        .filter(|(i, p)| {
            p.original.distance(&point.original) < neighborhood_size && *i != point_index
        })
        .map(|(i, _)| i)
        .collect::<Vec<usize>>()
}

// Find the indexes of each nearest neighbor falling withing the size in 3D.
fn find_neighbors(
    point_index: usize,
    points: &Vec<Point>,
    neighborhood_size: f32,
) -> NeighborIndices {
    let point = &points[point_index];
    points
        .iter()
        .enumerate()
        .filter(|(i, p)| {
            p.reduced.distance(&point.reduced) < neighborhood_size && *i != point_index
        })
        .map(|(i, _)| i)
        .collect::<Vec<usize>>()
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

// Using the rankings and the neighboorhood calculate the the confidence.
// Here the confidence is how many in the neighborhood share the dimension
// of the points ranking.
fn calculate_annotation(
    point_index: usize,
    ranking_vectors: &Vec<Ranking>,
    neighborhood: NeighborIndices,
) -> DaSilvaExplanation {
    unimplemented!()
}

// From the sorted vector of local contributions and find the dimension than contributes most.
// Read as: Find the lowest ranking given a the local contribution.
// TODO this seems like a nasty hack. ASsumes we never encounter NaN (at least does not handle this correctly)
fn calculate_top_ranking(local_contributions: LocalContributions) -> Ranking {
    local_contributions
        .iter()
        .enumerate()
        .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap_or(Ordering::Equal))
        .map(|(index, f)| (index, *f))
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::types::{Point3, PointN};

    #[test]
    fn calculates_correct_neighbors() {
        let points = vec![
            Point {
                reduced: Point3::new(0.0, 0.0, 0.0),
                original: vec![0.0, 0.0],
            },
            Point {
                reduced: Point3::new(1.0, 1.0, 1.0),
                original: vec![1.0, 1.0],
            },
            Point {
                reduced: Point3::new(2.0, 2.0, 2.0),
                original: vec![2.0, 2.0],
            },
            Point {
                reduced: Point3::new(10.0, 10.0, 10.0),
                original: vec![10.0, 10.0],
            },
        ];
        // Check size in 3D
        assert_eq!(find_neighbors(0, &points, 2.0).len(), 1);
        // Check size in ND
        assert_eq!(find_neighbors_nd(0, &points, 3.0).len(), 2);
    }

    #[test]
    fn calculates_correct_centroid() {
        let original_data: Vec<PointN> = vec![
            vec![1.0, 1.0, -1.0],
            vec![-1.0, 0.0, 1.0],
            vec![0.0, 2.0, 0.0],
        ];
        let points: Vec<Point> = original_data
            .iter()
            .map(|f| Point {
                reduced: Point3::new(0.0, 0.0, 0.0),
                original: f.to_owned(),
            })
            .collect();
        assert_eq!(find_centroid(&points), vec![0.0, 1.0, 0.0]);
    }
}
