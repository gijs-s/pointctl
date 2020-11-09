//! File containing the entire interface with the search structure.
// build in imports
use std::{fmt::Debug, path::Path, process::exit};

// Third party imports
use rstar::RTree;
use vpsearch::Tree as VPTree;

// First party imports
use super::definitions::{Indexed, IndexedPoint, PointContainer2D, PointContainer3D};
use crate::{exp::Neighborhood, filesystem};

/// Functions that are supported by both 2 and 3 dimensional point containers
pub trait PointContainer: Send + Sync {
    const DIMENSIONS: usize;
    type LDPoint: Clone + Debug;
    type IndexedLDPoint: rstar::RTreeObject + Indexed + Clone + Debug;

    /// Get a reference to the low dimension search tree for searching
    /// neighbors
    fn get_tree_low(&self) -> &RTree<Self::IndexedLDPoint>;

    /// Get a reference to the high dimension search tree for searching
    /// neighbors. Will not be used for now.
    fn get_tree_high(&self) -> &VPTree<IndexedPoint<Vec<f32>>>;

    /// Get all the ND points
    fn get_nd_points(&self) -> Vec<&Vec<f32>>;

    /// Get a specific LD point
    fn get_ld_point(&self, index: u32) -> &Self::LDPoint;

    /// Get a specific ND point
    fn get_nd_point(&self, index: u32) -> &Vec<f32>;

    /// Get the amount of points currently stored in this container
    fn get_point_count(&self) -> usize;

    /// Get the dimensionality
    fn get_dimensionality(&self) -> usize;

    /// Get a reference to all neighbors within a certain range. This used the rtree.
    fn find_neighbors_r(&self, r: f32, index: u32) -> Vec<u32>;

    /// Get a reference to the k nearest neighbors.
    fn find_neighbors_k(&self, k: usize, index: u32) -> Vec<u32>;

    /// Find the average nearest neighbor distance in 2/3D to use
    /// as default blob size.
    fn find_average_nearest_neighbor_distance(&self) -> f32;

    /// Retrieve the projection width based on the largest axis aligned bounding box distance
    fn get_projection_width(&self) -> f32;

    fn calculate_projection_width(points: &Vec<Self::LDPoint>) -> f32;

    fn axis_aligned_bounding_box(points: &Vec<Self::LDPoint>) -> (Self::LDPoint, Self::LDPoint);

    /// Get tje neighborhoods for a given point, each consists of all points witing
    /// p v_i for point p_i in ld. The nd neighborhood is {P(p) \in v_i}.
    fn get_neighbor_indices(&self, index: u32, neighborhood_size: Neighborhood) -> Vec<u32> {
        match neighborhood_size {
            Neighborhood::R(size) => {
                self.find_neighbors_r(self.get_projection_width() * size, index)
            }
            Neighborhood::K(size) => self.find_neighbors_k(size, index),
        }
    }

    /// Read the points from the files and check the dimensionality
    fn read_points(
        original_points_path: &Path,
        reduced_points_path: &Path,
    ) -> (Vec<Vec<f32>>, Vec<Vec<f32>>, Vec<String>) {
        // Load in the data
        let (original_points_raw, dimension_count_high, dimension_names) =
            filesystem::read(original_points_path);
        let (reduced_points_raw, dimension_count_low, _) = filesystem::read(reduced_points_path);
        // Check if the amount of points match
        if original_points_raw.len() != reduced_points_raw.len() {
            eprintln!(
                "The reduced and original dataset do not contain the same amount of points.\n\
                {:?} contains {} points while {:?} contains {}.",
                original_points_path.to_str(),
                original_points_raw.len(),
                reduced_points_path.to_str(),
                reduced_points_raw.len()
            );
            exit(18)
        }

        // We only support points reduced to 2 or 3D
        if dimension_count_low != Self::DIMENSIONS {
            eprintln!(
                "Expected data reduced to {}D but got {} dimensions instead",
                Self::DIMENSIONS,
                dimension_count_low
            );
            exit(15)
        }

        println!(
            "Points successfully loaded. {}D reduced to {}D for {} points",
            dimension_count_high,
            dimension_count_low,
            original_points_raw.len()
        );
        (original_points_raw, reduced_points_raw, dimension_names)
    }
}

impl PointContainer for PointContainer2D {
    const DIMENSIONS: usize = 2;
    type LDPoint = na::Point2<f32>;
    type IndexedLDPoint = IndexedPoint<na::Point2<f32>>;

    fn get_tree_low(&self) -> &RTree<Self::IndexedLDPoint> {
        &self.tree_low
    }

    fn get_tree_high(&self) -> &VPTree<IndexedPoint<Vec<f32>>> {
        &self.tree_high
    }

    fn get_nd_points(&self) -> Vec<&Vec<f32>> {
        self.point_data
            .iter()
            .map(|data| &data.high)
            .collect::<Vec<&Vec<f32>>>()
    }

    fn get_ld_point(&self, index: u32) -> &Self::LDPoint {
        &self.point_data[index as usize].low
    }

    fn get_nd_point(&self, index: u32) -> &Vec<f32> {
        &self.point_data[index as usize].high
    }

    /// Getter for the point count
    fn get_point_count(&self) -> usize {
        self.point_data.len()
    }

    /// Getter for the dimensionality
    fn get_dimensionality(&self) -> usize {
        self.dimensionality
    }

    /// Get a reference to all neighbors within a certain range. This used the rtree.
    fn find_neighbors_r(&self, r: f32, index: u32) -> Vec<u32> {
        let point = self.get_ld_point(index);
        let query_point = [point.x, point.y];
        self.tree_low
            .locate_within_distance(query_point, r * r)
            .map(|elem| elem.index)
            .filter(|&i| i != index)
            .collect::<Vec<u32>>()
    }

    /// Get a reference to the k nearest neighbors.
    fn find_neighbors_k(&self, k: usize, index: u32) -> Vec<u32> {
        let point = self.get_ld_point(index);
        let query_point = [point.x, point.y];
        self.tree_low
            .nearest_neighbor_iter(&query_point)
            .take(k + 1)
            .map(|elem| elem.index as u32)
            .filter(|&i| i != index)
            .collect::<Vec<u32>>()
    }

    fn get_projection_width(&self) -> f32 {
        self.projection_width
    }

    /// Calculate the projection width based on the largest axis aligned bounding box distance
    fn calculate_projection_width(points: &Vec<Self::LDPoint>) -> f32 {
        let (min, max) = Self::axis_aligned_bounding_box(points);
        let mut width = f32::NEG_INFINITY;
        for (min_i, max_i) in min.iter().zip(max.iter()) {
            if max_i - min_i > width {
                width = max_i - min_i;
            }
        }
        width
    }

    /// Retrieve the axis aligned bounding box for all the 3D points currently in the state
    fn axis_aligned_bounding_box(points: &Vec<Self::LDPoint>) -> (Self::LDPoint, Self::LDPoint) {
        let mut min = na::Point2::new(f32::INFINITY, f32::INFINITY);
        let mut max = na::Point2::new(f32::NEG_INFINITY, f32::NEG_INFINITY);
        for point in points {
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
        }
        (min, max)
    }

    /// Find the average distance to the first nearest neighbor.
    ///
    /// note: Currently this has an offset to this value to help my rendering engine,
    /// here i draw circles that have radius 5x the average nn distance. This function
    /// already includes this offset
    fn find_average_nearest_neighbor_distance(&self) -> f32 {
        let mut res = Vec::<f32>::new();
        for query_point in self.get_tree_low().iter() {
            // Get the second nearest neighbor from the query point, the first will be itself.
            let &nn = self
                .get_tree_low()
                .nearest_neighbor_iter(&[query_point.point.x, query_point.point.y])
                .take(2)
                .skip(1)
                .collect::<Vec<&Self::IndexedLDPoint>>()
                .first()
                .expect("Could not get nearest neighbor");

            let dist = na::distance(&query_point.point, &nn.point);
            res.push(dist);
        }
        let average = res.iter().sum::<f32>() / (res.len() as f32);
        // We use this to draw round blobs bounded by a square
        (average.powi(2) * 2.0).sqrt() * 5.0
    }
}

impl PointContainer for PointContainer3D {
    const DIMENSIONS: usize = 3;
    type LDPoint = na::Point3<f32>;
    type IndexedLDPoint = IndexedPoint<na::Point3<f32>>;

    fn get_tree_low(&self) -> &RTree<Self::IndexedLDPoint> {
        &self.tree_low
    }

    fn get_tree_high(&self) -> &VPTree<IndexedPoint<Vec<f32>>> {
        &self.tree_high
    }

    fn get_nd_points(&self) -> Vec<&Vec<f32>> {
        self.point_data
            .iter()
            .map(|data| &data.high)
            .collect::<Vec<&Vec<f32>>>()
    }

    fn get_ld_point(&self, index: u32) -> &Self::LDPoint {
        &self.point_data[index as usize].low
    }

    fn get_nd_point(&self, index: u32) -> &Vec<f32> {
        &self.point_data[index as usize].high
    }

    fn get_point_count(&self) -> usize {
        self.point_data.len()
    }

    fn get_dimensionality(&self) -> usize {
        self.dimensionality
    }

    /// Get a reference to all neighbors within a certain range. This used the rtree.
    fn find_neighbors_r(&self, r: f32, index: u32) -> Vec<u32> {
        let point = self.get_ld_point(index);
        let query_point = [point.x, point.y, point.z];
        self.tree_low
            .locate_within_distance(query_point, r * r)
            .map(|elem| elem.index as u32)
            .filter(|&i| i != index)
            .collect::<Vec<u32>>()
    }

    /// Get a reference to the k nearest neighbors.
    fn find_neighbors_k(&self, k: usize, index: u32) -> Vec<u32> {
        let point = self.get_ld_point(index);
        let query_point = [point.x, point.y, point.z];
        self.tree_low
            .nearest_neighbor_iter(&query_point)
            .take(k + 1)
            .map(|elem| elem.index as u32)
            .filter(|&i| i != index)
            .collect::<Vec<u32>>()
    }

    /// Retrieve the projection width based on the largest axis aligned bounding box distance
    fn get_projection_width(&self) -> f32 {
        self.projection_width
    }

    /// Calculate the projection width based on the largest axis aligned bounding box distance
    fn calculate_projection_width(points: &Vec<Self::LDPoint>) -> f32 {
        let (min, max) = Self::axis_aligned_bounding_box(points);
        let mut width = f32::NEG_INFINITY;
        for (min_i, max_i) in min.iter().zip(max.iter()) {
            if max_i - min_i > width {
                width = max_i - min_i;
            }
        }
        width
    }

    /// Retrieve the axis aligned bounding box for all the 2D points currently in the state
    fn axis_aligned_bounding_box(points: &Vec<Self::LDPoint>) -> (Self::LDPoint, Self::LDPoint) {
        let mut min = na::Point3::new(f32::INFINITY, f32::INFINITY, f32::INFINITY);
        let mut max = na::Point3::new(f32::NEG_INFINITY, f32::NEG_INFINITY, f32::NEG_INFINITY);
        for point in points {
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

    /// Find the average distance to the first nearest neighbor.
    ///
    /// note: Currently this has an offset to this value to help my rendering engine,
    /// here i draw circles that have radius 5x the average nn distance. This function
    /// already includes this offset
    fn find_average_nearest_neighbor_distance(&self) -> f32 {
        let mut res = Vec::<f32>::new();
        for query_point in self.get_tree_low().iter() {
            // Get the second nearest neighbor from the query point, the first will be itself.
            let &nn = self
                .get_tree_low()
                .nearest_neighbor_iter(&[
                    query_point.point.x,
                    query_point.point.y,
                    query_point.point.z,
                ])
                .take(2)
                .skip(1)
                .collect::<Vec<&Self::IndexedLDPoint>>()
                .first()
                .expect("Could not get nearest neighbor");

            let dist = na::distance(&query_point.point, &nn.point);
            res.push(dist);
        }
        let average = res.iter().sum::<f32>() / (res.len() as f32);
        // We use this to draw round blobs bounded by a square
        (average.powi(2) * 2.0).sqrt() * 5.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;
    use vpsearch::Tree as VPTree;

    #[test]
    // Here we will calculate the average distance to the first nearest neighbor
    fn find_average_nearest_neightbor_distance_2d_one_line() {
        let indexed_points = vec![
            IndexedPoint::<na::Point2<f32>> {
                index: 0,
                point: na::Point2::<f32>::new(0.0, 0.0),
            },
            IndexedPoint::<na::Point2<f32>> {
                index: 1,
                point: na::Point2::<f32>::new(4.0, 0.0),
            },
            IndexedPoint::<na::Point2<f32>> {
                index: 2,
                point: na::Point2::<f32>::new(7.0, 0.0),
            },
            IndexedPoint::<na::Point2<f32>> {
                index: 3,
                point: na::Point2::<f32>::new(9.0, 0.0),
            },
            IndexedPoint::<na::Point2<f32>> {
                index: 4,
                point: na::Point2::<f32>::new(10.0, 0.0),
            },
        ];

        // The average nearest neighbor distance is based on 5 points
        // | Point | Nearest Neightbor | Distance to neighbor |
        // | 0     | 1                 | 4                    |
        // | 1     | 2                 | 3                    |
        // | 2     | 3                 | 2                    |
        // | 3     | 4                 | 1                    |
        // | 4     | 3                 | 1                    |

        let point_container = PointContainer2D {
            tree_low: RTree::<IndexedPoint<na::Point2<f32>>>::bulk_load_with_params(indexed_points),
            tree_high: VPTree::new(vec![].as_slice()),
            dimension_names: vec![],
            point_data: vec![],
            dimensionality: 0,
            projection_width: 0f32,
        };

        let expected = (4.0f32 + 3.0f32 + 2.0f32 + 1.0f32 + 1.0f32) / 5.0f32;
        let actual = point_container.find_average_nearest_neighbor_distance();
        // offset used to get a better starting point for the blob size
        let offset_expected = (expected.powi(2) * 2.0).sqrt() * 5.0;
        assert_relative_eq!(actual, offset_expected, epsilon = 1.0e-4);
    }

    #[test]
    // Here we will calculate the average distance to the first nearest neighbor
    fn find_average_nearest_neightbor_distance_2d_xy() {
        let indexed_points = vec![
            IndexedPoint::<na::Point2<f32>> {
                index: 0,
                point: na::Point2::<f32>::new(0.0, 0.0),
            },
            IndexedPoint::<na::Point2<f32>> {
                index: 1,
                point: na::Point2::<f32>::new(4.0, 4.0),
            },
            IndexedPoint::<na::Point2<f32>> {
                index: 2,
                point: na::Point2::<f32>::new(7.0, 1.0),
            },
            IndexedPoint::<na::Point2<f32>> {
                index: 3,
                point: na::Point2::<f32>::new(9.0, 4.0),
            },
            IndexedPoint::<na::Point2<f32>> {
                index: 4,
                point: na::Point2::<f32>::new(9.0, 5.0),
            },
        ];

        // The average nearest neighbor distance is based on 5 points
        // | Point | Nearest Neightbor | Distance to neighbor |
        // | 0     | 1                 | sqrt 32              |
        // | 1     | 2                 | sqrt 18              |
        // | 2     | 3                 | sqrt 13              |
        // | 3     | 4                 | sqrt 1               |
        // | 4     | 3                 | sqrt 1               |

        let point_container = PointContainer2D {
            tree_low: RTree::<IndexedPoint<na::Point2<f32>>>::bulk_load_with_params(indexed_points),
            tree_high: VPTree::new(vec![].as_slice()),
            dimension_names: vec![],
            point_data: vec![],
            dimensionality: 0,
            projection_width: 0f32,
        };
        let expected =
            (32.0f32.sqrt() + 18.0f32.sqrt() + 13.0f32.sqrt() + 1.0f32 + 1.0f32) / 5.0f32;
        let actual = point_container.find_average_nearest_neighbor_distance();
        // offset used to get a better starting point for the blob size
        let offset_expected = (expected.powi(2) * 2.0).sqrt() * 5.0;
        assert_relative_eq!(actual, offset_expected, epsilon = 1.0e-4);
    }
}
