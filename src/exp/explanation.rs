/// An abstraction that handles the search of neighborhoods for you.
use nalgebra::Point3;
use rstar::RTree;

use super::{
    common::{Distance, IndexedPoint3D, RTreeParameters3D},
    Neighborhood,
};

use std::cmp::Ordering;

// Types to make the code more readable
pub type NeighborIndices = Vec<usize>;
pub type LocalContributions = Vec<f32>;
pub type GlobalContribution = Vec<f32>;
pub type Ranking = (usize, f32);

pub trait Explanation<T> {
    // Explain the points using the current method, all variables should be set in the state
    fn explain(&self, neighborhood_size: Neighborhood) -> Vec<T>;

    // Normalize a local contrib of a dimension using the global contrib of said dimension.
    // this function lines up with formula 3 in the da silva paper
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

pub trait NeighborhoodExplanationMechanism {
    fn get_tree(&self) -> &RTree<IndexedPoint3D>;

    fn get_point_count(&self) -> usize {
        self.get_tree().size()
    }

    /// Pre-compute all neighborhoods, each neighborhood consists of all points witing
    /// p v_i for point p_i in 3d. The nd neighborhood is {P(p) \in v_i}. Note that we
    /// do this for every (unordered) element in the rtree so we sort after.
    fn get_neighbor_indices(&self, neighborhood_size: Neighborhood) -> Vec<Vec<usize>> {
        let projection_width = self.projection_width();
        let mut indexed_neighborhoods: Vec<(usize, NeighborIndices)> = self
            .get_tree()
            .iter()
            .map(|indexed_point| {
                let neighbors = match neighborhood_size {
                    Neighborhood::R(size) => {
                        self.find_neighbors_r(projection_width * size, *indexed_point)
                    }
                    Neighborhood::K(size) => self.find_neighbors_k(size, *indexed_point),
                };
                (indexed_point.index, neighbors)
            })
            .collect();

        // Sort the neighborhoods again based on index
        indexed_neighborhoods.sort_by(|(a, _), (b, _)| a.partial_cmp(b).unwrap());
        // Remove the index from the sorted neighborhood
        indexed_neighborhoods.into_iter().map(|(_, n)| n).collect()
    }

    /// Get a reference to all neighbors within a certain range. This used the rtree.
    fn find_neighbors_r(&self, r: f32, indexed_point: IndexedPoint3D) -> NeighborIndices {
        let query_point = [indexed_point.x, indexed_point.y, indexed_point.z];
        self.get_tree()
            .locate_within_distance(query_point, r * r)
            .map(|elem| elem.index)
            .filter(|&index| index != indexed_point.index)
            .collect::<NeighborIndices>()
    }

    /// Get a reference to the k nearest neighbors.
    fn find_neighbors_k(&self, k: usize, indexed_point: IndexedPoint3D) -> NeighborIndices {
        let query_point = [indexed_point.x, indexed_point.y, indexed_point.z];
        self.get_tree()
            .nearest_neighbor_iter(&query_point)
            .take(k + 1)
            .map(|elem| elem.index)
            .filter(|&index| index != indexed_point.index)
            .collect::<NeighborIndices>()
    }

    /// Retrieve the projection width based on the largest axis aligned bounding box distance
    /// TODO: this is a temporary hack, using a convex hull would be nicer
    fn projection_width(&self) -> f32 {
        let (min, max) = self.axis_aligned_bounding_box();
        let mut width = f32::NEG_INFINITY;
        for (min_i, max_i) in min.iter().zip(max.iter()) {
            if max_i - min_i > width {
                width = max_i - min_i;
            }
        }
        width
    }

    /// Get the axis aligned bounding box of all points.
    fn axis_aligned_bounding_box(&self) -> (Point3<f32>, Point3<f32>) {
        let mut min = Point3::new(f32::INFINITY, f32::INFINITY, f32::INFINITY);
        let mut max = Point3::new(f32::NEG_INFINITY, f32::NEG_INFINITY, f32::NEG_INFINITY);
        for point in self.get_tree().iter() {
            // Update the min and max values
            if point.x < min.x {
                min.x = point.x
            } else if point.x > max.x {
                max.x = point.x
            }
            if point.y < min.y {
                min.y = point.y
            } else if point.y > max.y {
                max.y = point.y
            }
            if point.z < min.z {
                min.z = point.z
            } else if point.z > max.z {
                max.z = point.z
            }
        }
        (min, max)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use nalgebra::Point3;

    struct DummyExplanationState {
        pub rtree: RTree<IndexedPoint3D>,
    }

    impl DummyExplanationState {
        pub fn new(points: Vec<Point3<f32>>) -> Self {
            let indexed_points = points
                .into_iter()
                .enumerate()
                .map(|(index, point)| IndexedPoint3D {
                    index,
                    x: point.x,
                    y: point.y,
                    z: point.z,
                })
                .collect();
            DummyExplanationState {
                rtree: RTree::<IndexedPoint3D>::bulk_load_with_params(indexed_points),
            }
        }
    }

    impl NeighborhoodExplanationMechanism for DummyExplanationState {
        fn get_tree(&self) -> &RTree<IndexedPoint3D> {
            &self.rtree
        }
    }

    #[test]
    fn calculates_correct_neighbors() {
        let points_3 = vec![
            Point3::new(0.0, 0.0, 0.0),
            Point3::new(1.0, 1.0, 1.0),
            Point3::new(2.0, 2.0, 2.0),
            Point3::new(10.0, 10.0, 10.0),
        ];
        let mechanism = DummyExplanationState::new(points_3);

        // Check neighbors
        assert_eq!(
            mechanism.find_neighbors_r(
                2.0,
                IndexedPoint3D {
                    index: 2,
                    x: 2.0,
                    y: 2.0,
                    z: 2.0
                }
            ),
            vec![1]
        );
        assert_eq!(
            mechanism.find_neighbors_r(
                4.0,
                IndexedPoint3D {
                    index: 2,
                    x: 2.0,
                    y: 2.0,
                    z: 2.0
                }
            ),
            vec![1, 0]
        );
    }
}
