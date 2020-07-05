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
// - Warning in case of tiny neighborhoods

use super::common::{Distance, Point};
use crate::util::types::PointN;

use std::cmp::Ordering;

// Types to make the code more readable
type NeighborIndices = Vec<usize>;
type LocalContributions = Vec<f32>;
type GlobalContribution = Vec<f32>;
type Ranking = (usize, f32);
type DimensionOrder = Vec<usize>;

#[derive(Debug, PartialEq)]
pub struct DaSilvaExplanation {
    // Attribute index is the index of which dimension in nD is most important for this point
    pub attribute_index: usize,
    // The is the confidence we have in said attribute index
    pub confidence: f32,
}

// Tuple of explanations and the dimension ids sorted by rank.
pub fn explain(
    input: &Vec<Point>,
    neighborhood_size: f32,
) -> (Vec<DaSilvaExplanation>, DimensionOrder) {
    // Calculate the global contribution of each point (centroid of the nD space and
    //_every_ point in its neighborhood)
    let centroid: PointN = find_centroid(&input);
    let global_contribution: GlobalContribution = calculate_global_contribution(centroid, &input);

    // Pre-compute all neighborhoods, each neighborhood consists of all points witing
    // p v_i for point p_i in 3d. The nd neighborhood is {P(p) \in v_i}
    // TODO: Make this much faster!
    let neighborhoods: Vec<NeighborIndices> = (0..input.len())
        .into_iter()
        .map(|i| find_neighbors(i, &input, neighborhood_size))
        .collect();

    // Calculate the ranking vectors, a nd vector containing the IDs and ranks of each dimension.
    let ranking_vectors: Vec<Ranking> = (0..input.len())
        .into_iter()
        .zip(&neighborhoods)
        .map(|(i, n)| {
            // Calculate the local contribution lc_j between each point p_i and all its neighbors
            // v_i for every dimension j. Then average the contribution for every dimension within
            // the neighborhood
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
        .zip(&neighborhoods)
        .map(|(i, n)| calculate_annotation(i, &ranking_vectors, n))
        .collect();

    // TODO: Calculate the global ranking of dimension confidence, useful for the colour encoding.
    (explanations, vec![1])
}

// Used for the global explanation, just average the over all dimensions
fn find_centroid(points: &Vec<Point>) -> PointN {
    points
        .iter()
        // Calculate the sum of all points for each dimension separately
        .fold(
            // Vector containing only zeros as fold state start
            vec![0.0f32; points[0].original.len()],
            // Increment every dimension of the state using each dimension in the state
            |v, p| v.iter().zip(&p.original).map(|(a, b)| a + b).collect(),
        )
        // Iterate over the cumulative point
        .iter()
        // Normalize the point by dividing each dimension by the amount of points.
        // This averages each dimension out.
        .map(|x| x / points.len() as f32)
        .collect::<PointN>()
}

// Find the indexes of each nearest neighbor falling withing the size in 3D.
fn find_neighbors(
    point_index: usize,
    points: &Vec<Point>,
    neighborhood_size: f32,
) -> NeighborIndices {
    let point = &points[point_index];
    // iterate over the points and only keep the once in the neighborhood and are not itself
    points
        .iter()
        // Give every point an index
        .enumerate()
        // Filter out the points that are not near
        .filter(|&(i, p)| {
            p.reduced.distance(&point.reduced) < neighborhood_size && i != point_index
        })
        // Keep only the index
        .map(|(i, _)| i)
        // Collect the indices
        .collect::<Vec<usize>>()
}

// Given 2 points, calculate the contribution of each dimension to the distance.
// corresponds to formula 1. lc_j = (p_j - r_j)^2 / ||p-r||^2 where j is the dim.
fn calculate_distance_contribution(p: &PointN, r: &PointN) -> LocalContributions {
    let dist = p.sq_distance(r);
    p.iter()
        .zip(r)
        .map(|(a, b)| {
            let t = a - b;
            (t * t) / dist
        })
        .collect()
}

// Given a point index, the set of points and the indices of the neighbors calculate the
// average local contribution for each dimension over the neighborhood.
// Corresponds to formula 2. lc_j = Sum over r in neighborhood of lc^j_p,r divided |neighborhood|
fn calculate_local_contributions(
    point_index: usize,
    points: &Vec<Point>,
    neighbor_indices: &NeighborIndices,
) -> LocalContributions {
    // Retrieve a references to the point and neighbors
    let p: &PointN = &points[point_index].original;
    // Calculate the contribution of the distance between the point and all its neighbors
    // The take the cumulative over each dimension. Then divide that by the neigbor size.
    neighbor_indices
        .iter()
        // Calculate the distance contribution between the point and one of its neighbors
        .map(|&index| {
            let r = &points[index].original;
            calculate_distance_contribution(p, r)
        })
        // Fold to collect all the contributions into one single cumulative one.
        .fold(vec![0.0f32; p.len()], |c, lc| {
            c.iter()
                .zip(lc)
                .map(|(&c, x)| c + x)
                .collect::<LocalContributions>()
        })
        .iter()
        // For each dimension normalize using the neighborhood size.
        .map(|&dim| dim / neighbor_indices.len() as f32)
        .collect()
}

// Does the same as calculate_local_contributions but for the entire dataset.
fn calculate_global_contribution(centroid: PointN, points: &Vec<Point>) -> GlobalContribution {
    points
        .iter()
        // Calculate the distance contribution between the centroid and all points.
        .map(|r| calculate_distance_contribution(&centroid, &r.original))
        // Fold to collect all the contributions into one single cumulative one.
        .fold(vec![0.0f32; centroid.len()], |c, lc| {
            c.iter()
                .zip(lc)
                .map(|(&c, x)| c + x)
                .collect::<LocalContributions>()
        })
        .iter()
        // For each dimension normalize using the size of the points set.
        .map(|&dim| dim / points.len() as f32)
        .collect()
}

// Normalize a local contrib of a dimension using the global contrib of said dimension.
// this function lines up with formula 3 in the da silva paper
fn normalize_rankings(
    local_contributions: LocalContributions,
    global_contributions: &GlobalContribution,
) -> LocalContributions {
    let sum = local_contributions
        .iter()
        .zip(global_contributions)
        .fold(0.0, |c: f32, (l, g)| c + (l / g));
    local_contributions
        .iter()
        .zip(global_contributions)
        .map(|(l, g)| (l / g) / sum)
        .collect()
}

// Using the rankings and the neighborhood calculate the the confidence.
// Here the confidence is how many in the neighborhood share the dimension
// of the points ranking.
fn calculate_annotation(
    point_index: usize,
    ranking_vectors: &Vec<Ranking>,
    neighborhood: &NeighborIndices,
) -> DaSilvaExplanation {
    // Retrieve what dimension was chosen for a certain point
    let (point_dim, _) = ranking_vectors[point_index];
    let correct_count = neighborhood
        .iter()
        // Get the ranking vector for each neighbor and only keep the dimension
        .map(|&index| {
            let (dim, _) = ranking_vectors[index];
            dim
        })
        // Only keep the neighbors where the dimension is correct
        .filter(|&dim| dim == point_dim)
        // Count the amount of neighbors left
        .count();

    // TODO: Do we include self in the confidence score? assume no for now.
    DaSilvaExplanation {
        attribute_index: point_dim,
        confidence:
            if neighborhood.len() > 0{
                correct_count as f32 / neighborhood.len() as f32
            } else {
                0.0f32
            },
    }
}

// From the sorted vector of local contributions and find the dimension than
// contributes most. Read as: Find the lowest ranking given a the local
// contribution.
// TODO this seems like a nasty hack. ASsumes we never encounter NaN (at least
// does not handle this correctly)
fn calculate_top_ranking(local_contributions: LocalContributions) -> Ranking {
    local_contributions
        .iter()
        .enumerate()
        .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap_or(Ordering::Equal))
        .map(|(index, &f)| (index, f))
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

    #[test]
    fn calculates_correct_top_ranking() {
        assert_eq!(calculate_top_ranking(vec![0.0f32, 0.8, 0.3]), (1, 0.8));
        assert_eq!(calculate_top_ranking(vec![0.0f32, 0.0, 0.3]), (2, 0.3));
        assert_eq!(calculate_top_ranking(vec![0.0f32, 0.0, 0.0]), (2, 0.0));
    }

    #[test]
    fn calculates_correct_annotation() {
        let rankings: Vec<(usize, f32)> =
            vec![(2, 0.7), (1, 0.4), (1, 0.9), (1, 0.6), (1, 0.4), (3, 0.5)];

        assert_eq!(
            calculate_annotation(0, &rankings, &vec![1, 2, 3, 4]),
            DaSilvaExplanation {
                attribute_index: 2,
                confidence: 0.0
            }
        );

        assert_eq!(
            calculate_annotation(1, &rankings, &vec![2, 3, 4]),
            DaSilvaExplanation {
                attribute_index: 1,
                confidence: 1.0
            }
        );

        assert_eq!(
            calculate_annotation(1, &rankings, &vec![0, 2, 3, 4]),
            DaSilvaExplanation {
                attribute_index: 1,
                confidence: 0.75
            }
        );

        assert_eq!(
            calculate_annotation(1, &rankings, &vec![0, 2, 3, 5]),
            DaSilvaExplanation {
                attribute_index: 1,
                confidence: 0.50
            }
        );
    }
}
