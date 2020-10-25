/// An abstraction that handles the search of neighborhoods for you.
// Build in imports
use std::cmp::Ordering;

// First party imports
use super::Neighborhood;

// Types to make the code more readable
pub type LocalContributions = Vec<f32>;
pub type GlobalContribution = Vec<f32>;
pub type Ranking = (usize, f32);

pub trait Explanation<T> {
    // Explain the points using the current method, all variables should be set in the state
    fn explain(&self, neighborhood_size: Neighborhood) -> Vec<T>;

    // Normalize a local contrib of a dimension using the global contrib of said dimension.
    // this function lines up with formula 3 in the da silva paper
    #[allow(clippy::useless_conversion, clippy::ptr_arg)]
    fn normalize_rankings(
        local_contributions: LocalContributions,
        global_contributions: &GlobalContribution,
    ) -> LocalContributions {
        // Sum of the local contributions per dimension
        let sum = local_contributions
            .iter()
            .zip(global_contributions)
            .fold(0.0, |acc: f32, (lc_j, gc_j)| acc + (lc_j / gc_j));
        // Normalize each term
        local_contributions
            .iter()
            .zip(global_contributions)
            .map(|(lc_j, gc_j)| (lc_j / gc_j) / sum)
            .collect()
    }

    // From the sorted vector of local contributions and find the dimension than
    // contributes most. Read as: Find the lowest ranking given a the local
    // contribution.
    fn calculate_top_ranking(local_contributions: LocalContributions) -> Ranking {
        local_contributions
            .iter()
            .enumerate()
            .min_by(|(_, &a), (_, &b)| a.partial_cmp(&b).unwrap_or(Ordering::Equal))
            .map(|(index, &f)| (index, f))
            .unwrap()
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use nalgebra::Point3;

//     struct DummyExplanationState {
//         pub rtree: RTree<IndexedPoint3D>,
//     }

//     impl DummyExplanationState {
//         pub fn new(points: Vec<Point3<f32>>) -> Self {
//             let indexed_points = points
//                 .into_iter()
//                 .enumerate()
//                 .map(|(index, point)| IndexedPoint3D {
//                     index,
//                     x: point.x,
//                     y: point.y,
//                     z: point.z,
//                 })
//                 .collect();
//             DummyExplanationState {
//                 rtree: RTree::<IndexedPoint3D>::bulk_load_with_params(indexed_points),
//             }
//         }
//     }

//     impl NeighborhoodExplanationMechanism for DummyExplanationState {
//         fn get_tree(&self) -> &RTree<IndexedPoint3D> {
//             &self.rtree
//         }
//     }

//     #[test]
//     fn calculates_correct_neighbors() {
//         let points_3 = vec![
//             Point3::new(0.0, 0.0, 0.0),
//             Point3::new(1.0, 1.0, 1.0),
//             Point3::new(2.0, 2.0, 2.0),
//             Point3::new(10.0, 10.0, 10.0),
//         ];
//         let mechanism = DummyExplanationState::new(points_3);

//         // Check neighbors
//         assert_eq!(
//             mechanism.find_neighbors_r(
//                 2.0,
//                 IndexedPoint3D {
//                     index: 2,
//                     x: 2.0,
//                     y: 2.0,
//                     z: 2.0
//                 }
//             ),
//             vec![1]
//         );
//         assert_eq!(
//             mechanism.find_neighbors_r(
//                 4.0,
//                 IndexedPoint3D {
//                     index: 2,
//                     x: 2.0,
//                     y: 2.0,
//                     z: 2.0
//                 }
//             ),
//             vec![1, 0]
//         );
//     }
// }
