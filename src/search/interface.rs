/// File containing the entire interface with the search structure.
// build in imports
use std::{fmt::Debug, path::Path, process::exit};

// Third party imports
use indicatif::{ProgressBar, ProgressIterator, ProgressStyle};
use rstar::RTree;
use vpsearch::Tree as VPTree;

// First party imports
use super::definitions::{Indexed, IndexedPoint, PointContainer2D, PointContainer3D};
use crate::{exp::Neighborhood, filesystem};

/// Functions that are supported by both 2 and 3 dimensional point containers
pub trait PointContainer {
    const DIMENSIONS: usize;
    type LDPoint: rstar::RTreeObject + Indexed + Clone + Debug;

    /// Get a reference to the low dimension search tree for searching
    /// neighbors
    fn get_tree_low(&self) -> &RTree<Self::LDPoint>;

    /// Get a reference to the high dimension search tree for searching
    /// neighbors. Will not be used for now.
    fn get_tree_high(&self) -> &VPTree<IndexedPoint<Vec<f32>>>;

    fn get_nd_points(&self) -> Vec<&Vec<f32>>;

    fn get_nd_point(&self, index: usize) -> &Vec<f32>;

    fn get_point_count(&self) -> usize;

    fn get_dimensionality(&self) -> usize;

    /// Get a reference to all neighbors within a certain range. This used the rtree.
    fn find_neighbors_r(&self, r: f32, indexed_point: &Self::LDPoint) -> Vec<usize>;

    /// Get a reference to the k nearest neighbors.
    fn find_neighbors_k(&self, k: usize, indexed_point: &Self::LDPoint) -> Vec<usize>;

    /// Find the average nearest neighbor distance in 2/3D to use
    /// as default blob size.
    fn find_average_nearest_neighbor_distance(&self) -> f32;

    /// Retrieve the projection width based on the largest axis aligned bounding box distance
    fn projection_width(&self) -> f32;

    /// Pre-compute all neighborhoods, each neighborhood consists of all points witing
    /// p v_i for point p_i in 3d. The nd neighborhood is {P(p) \in v_i}. Note that we
    /// do this for every (unordered) element in the rtree so we sort after.
    fn get_neighbor_indices(&self, neighborhood_size: Neighborhood) -> Vec<Vec<usize>> {
        let projection_width = self.projection_width();

        let pb = ProgressBar::new(self.get_point_count() as u64);
        pb.set_style(ProgressStyle::default_bar()
            .template("[{elapsed_precise}] Calculating neighbors [{bar:40.cyan/blue}] {pos}/{len} ({eta} left at {per_sec})")
            .progress_chars("#>-"));

        let mut indexed_neighborhoods: Vec<(usize, Vec<usize>)> = self
            .get_tree_low()
            .iter()
            .progress_with(pb)
            .map(|indexed_point| {
                let neighbors = match neighborhood_size {
                    Neighborhood::R(size) => {
                        self.find_neighbors_r(projection_width * size, indexed_point)
                    }
                    Neighborhood::K(size) => self.find_neighbors_k(size, indexed_point),
                };
                (indexed_point.get_index(), neighbors)
            })
            .collect();

        // Sort the neighborhoods again based on index
        indexed_neighborhoods.sort_by(|(a, _), (b, _)| a.partial_cmp(b).unwrap());
        // Remove the index from the sorted neighborhood
        indexed_neighborhoods.into_iter().map(|(_, n)| n).collect()
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
    type LDPoint = IndexedPoint<na::Point2<f32>>;

    fn get_tree_low(&self) -> &RTree<Self::LDPoint> {
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

    fn get_nd_point(&self, index: usize) -> &Vec<f32> {
        &self.point_data[index].high
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
    fn find_neighbors_r(
        &self,
        r: f32,
        indexed_point: &IndexedPoint<na::Point2<f32>>,
    ) -> Vec<usize> {
        let query_point = [indexed_point.point.x, indexed_point.point.y];
        self.tree_low
            .locate_within_distance(query_point, r * r)
            .map(|elem| elem.index)
            .filter(|&index| index != indexed_point.index)
            .collect::<Vec<usize>>()
    }

    /// Get a reference to the k nearest neighbors.
    fn find_neighbors_k(
        &self,
        k: usize,
        indexed_point: &IndexedPoint<na::Point2<f32>>,
    ) -> Vec<usize> {
        let query_point = [indexed_point.point.x, indexed_point.point.y];
        self.tree_low
            .nearest_neighbor_iter(&query_point)
            .take(k + 1)
            .map(|elem| elem.index)
            .filter(|&index| index != indexed_point.index)
            .collect::<Vec<usize>>()
    }

    /// Retrieve the projection width based on the largest axis aligned bounding box distance
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

    /// Find the average nearest neighbor distance
    fn find_average_nearest_neighbor_distance(&self) -> f32 {
        let mut res = Vec::<f32>::new();
        for query_point in self.get_tree_low().iter() {
            // Get the second nearest neighbor from the query point, the first will be itself.
            let &nn = self
                .get_tree_low()
                .nearest_neighbor_iter(&[query_point.point.x, query_point.point.y])
                .take(2)
                .skip(1)
                .collect::<Vec<&Self::LDPoint>>()
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
    type LDPoint = IndexedPoint<na::Point3<f32>>;

    fn get_tree_low(&self) -> &RTree<Self::LDPoint> {
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

    fn get_nd_point(&self, index: usize) -> &Vec<f32> {
        &self.point_data[index].high
    }

    fn get_point_count(&self) -> usize {
        self.point_data.len()
    }

    fn get_dimensionality(&self) -> usize {
        self.dimensionality
    }

    /// Get a reference to all neighbors within a certain range. This used the rtree.
    fn find_neighbors_r(
        &self,
        r: f32,
        indexed_point: &IndexedPoint<na::Point3<f32>>,
    ) -> Vec<usize> {
        let query_point = [
            indexed_point.point.x,
            indexed_point.point.y,
            indexed_point.point.z,
        ];
        self.tree_low
            .locate_within_distance(query_point, r * r)
            .map(|elem| elem.index)
            .filter(|&index| index != indexed_point.index)
            .collect::<Vec<usize>>()
    }

    /// Get a reference to the k nearest neighbors.
    fn find_neighbors_k(
        &self,
        k: usize,
        indexed_point: &IndexedPoint<na::Point3<f32>>,
    ) -> Vec<usize> {
        let query_point = [
            indexed_point.point.x,
            indexed_point.point.y,
            indexed_point.point.z,
        ];
        self.tree_low
            .nearest_neighbor_iter(&query_point)
            .take(k + 1)
            .map(|elem| elem.index)
            .filter(|&index| index != indexed_point.index)
            .collect::<Vec<usize>>()
    }

    /// Retrieve the projection width based on the largest axis aligned bounding box distance
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
                .collect::<Vec<&Self::LDPoint>>()
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

impl PointContainer3D {
    /// todo: Clean this up
    fn axis_aligned_bounding_box(&self) -> (na::Point3<f32>, na::Point3<f32>) {
        let mut min = na::Point3::new(f32::INFINITY, f32::INFINITY, f32::INFINITY);
        let mut max = na::Point3::new(f32::NEG_INFINITY, f32::NEG_INFINITY, f32::NEG_INFINITY);
        for point_data in self.point_data.iter() {
            let point = point_data.low;
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

impl PointContainer2D {
    /// todo: Clean this up
    fn axis_aligned_bounding_box(&self) -> (na::Point2<f32>, na::Point2<f32>) {
        let mut min = na::Point2::new(f32::INFINITY, f32::INFINITY);
        let mut max = na::Point2::new(f32::NEG_INFINITY, f32::NEG_INFINITY);
        for point_data in self.point_data.iter() {
            let point = point_data.low;
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
        };
        let expected =
            (32.0f32.sqrt() + 18.0f32.sqrt() + 13.0f32.sqrt() + 1.0f32 + 1.0f32) / 5.0f32;
        let actual = point_container.find_average_nearest_neighbor_distance();
        // offset used to get a better starting point for the blob size
        let offset_expected = (expected.powi(2) * 2.0).sqrt() * 5.0;
        assert_relative_eq!(actual, offset_expected, epsilon = 1.0e-4);
    }
}
