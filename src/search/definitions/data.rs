/// Module containing the structs used throughout the search structure

/// Build in imports
use std::fmt;
use std::path::Path;
use std::process::exit;

/// Third party imports
use rstar::RTree;
use vpsearch::Tree as VPTree;

/// First party imports
use crate::{
    exp::{DaSilvaExplanation, NormalExplanation, VanDrielExplanation},
    search::PointContainer,
};

/// Generic point struct that can be used to
#[derive(Clone, Debug)]
pub struct IndexedPoint<P: Clone + fmt::Debug> {
    pub point: P,
    pub index: u32,
}

impl<P: Clone + fmt::Debug> IndexedPoint<P> {
    pub fn new(point: P, index: u32) -> Self {
        IndexedPoint::<P> { point, index }
    }
}

pub trait Indexed {
    fn get_index(&self) -> u32;
}

impl<P: Clone + fmt::Debug> Indexed for IndexedPoint<P> {
    fn get_index(&self) -> u32 {
        self.index
    }
}

/// Container that stores all the actual data for a single point
pub struct PointData2D {
    pub index: u32,
    pub low: na::Point2<f32>,
    pub high: Vec<f32>,
    pub normal: Option<NormalExplanation>,
    pub driel_min: Option<VanDrielExplanation>,
    pub driel_total: Option<VanDrielExplanation>,
    pub silva_var: Option<DaSilvaExplanation>,
    pub silva_euclidean: Option<DaSilvaExplanation>,
}

impl Into<na::Point2<f32>> for PointData2D {
    fn into(self) -> na::Point2<f32> {
        self.low
    }
}

impl PointData2D {
    pub fn new(
        index: u32,
        low_dimension_point: na::Point2<f32>,
        high_dimension_point: Vec<f32>,
    ) -> Self {
        PointData2D {
            index,
            low: low_dimension_point,
            high: high_dimension_point,
            normal: None,
            driel_min: None,
            driel_total: None,
            silva_var: None,
            silva_euclidean: None,
        }
    }
}

/// Container that stores all the actual data for a single point
pub struct PointData3D {
    pub index: u32,
    pub low: na::Point3<f32>,
    pub high: Vec<f32>,
    pub normal: Option<NormalExplanation>,
    pub driel_min: Option<VanDrielExplanation>,
    pub driel_total: Option<VanDrielExplanation>,
    pub silva_var: Option<DaSilvaExplanation>,
    pub silva_euclidean: Option<DaSilvaExplanation>,
}

impl Into<na::Point3<f32>> for PointData3D {
    fn into(self) -> na::Point3<f32> {
        self.low
    }
}

impl PointData3D {
    pub fn new(
        index: u32,
        low_dimension_point: na::Point3<f32>,
        high_dimension_point: Vec<f32>,
    ) -> Self {
        PointData3D {
            index,
            low: low_dimension_point,
            high: high_dimension_point,
            normal: None,
            driel_min: None,
            driel_total: None,
            silva_var: None,
            silva_euclidean: None,
        }
    }
}

/// Data structure used to store the data about all the points,
/// it has build in support for quickly finding all the neighbors
/// in 2D and ND.
pub struct PointContainer2D {
    // Used for finding low dimensional neighbors
    pub tree_low: RTree<IndexedPoint<na::Point2<f32>>>,
    // Used for finding high dimensional neighbors.
    pub tree_high: VPTree<IndexedPoint<Vec<f32>>>,
    // Original dimension names
    pub dimension_names: Vec<String>,
    // Used when quickly iterating over all the points in order of index
    pub point_data: Vec<PointData2D>,
    // The amount of dimensions in the nd data
    pub dimensionality: usize,
    // Projection width of all the points,
    pub projection_width: f32,
}

/// Data structure used to store the data about all the points,
/// it has build in support for quickly finding all the neighbors
/// in 3D and ND.
pub struct PointContainer3D {
    // Used for finding low dimensional neighbors
    pub tree_low: RTree<IndexedPoint<na::Point3<f32>>>,
    // Used for finding high dimensional neighbors.
    pub tree_high: VPTree<IndexedPoint<Vec<f32>>>,
    // Original dimension names
    pub dimension_names: Vec<String>,
    // Used when quickly iterating over all the points in order of index
    pub point_data: Vec<PointData3D>,
    // The amount of dimensions in the nd data
    pub dimensionality: usize,
    // Projection width of all the points,
    pub projection_width: f32,
}

impl PointContainer2D {
    /// Create a new point container from 2 files
    pub fn new(original_points_path: &Path, reduced_points_path: &Path) -> PointContainer2D {
        let (original_points_raw, reduced_points_raw, dimension_names) =
            Self::read_points(original_points_path, reduced_points_path);

        let dimension_count = original_points_raw.first().unwrap().len();

        let data_points = original_points_raw
            .iter()
            .zip(&reduced_points_raw)
            .enumerate()
            .map(|(index, (high, low))| {
                let low_point = match low[..] {
                    [x, y] => na::Point2::<f32>::new(x, y),
                    _ => panic!("Point have an incorrect length"),
                };

                PointData2D {
                    index: index as u32,
                    low: low_point,
                    high: high.to_vec(),
                    normal: None,
                    driel_min: None,
                    driel_total: None,
                    silva_var: None,
                    silva_euclidean: None,
                }
            })
            .collect::<Vec<PointData2D>>();

        // Create na::algebra points from the raw data
        let points_ld = reduced_points_raw
            .iter()
            .map(|raw_point| match raw_point[..] {
                    [x, y] => na::Point2::<f32>::new(x, y),
                    _ => exit(14),
                }
            )
            .collect::<Vec<na::Point2<f32>>>();

        let projection_width = PointContainer2D::calculate_projection_width(&points_ld);

        // Create indexed points from the LD data
        let indexed_ld_points: Vec<IndexedPoint::<na::Point2<f32>>> = points_ld
            .into_iter()
            .enumerate()
            .map(|(index, point)| IndexedPoint::<na::Point2<f32>>::new(point, index as u32))
            .collect();

        // Create indexed vectors from the raw HD data
        let ref_point_hd = original_points_raw
            .into_iter()
            .enumerate()
            .map(|(index, raw_point)| IndexedPoint::<Vec<f32>>::new(raw_point, index as u32))
            .collect::<Vec<IndexedPoint<Vec<f32>>>>();

        let tree_low = RTree::<IndexedPoint<na::Point2<f32>>>::bulk_load(indexed_ld_points);
        let tree_high = VPTree::new(&ref_point_hd);

        PointContainer2D {
            tree_low,
            tree_high,
            dimension_names,
            point_data: data_points,
            dimensionality: dimension_count,
            projection_width
        }
    }
}

impl PointContainer3D {
    /// Create a new point container from 2 files
    pub fn new(original_points_path: &Path, reduced_points_path: &Path) -> PointContainer3D {
        let (original_points_raw, reduced_points_raw, dimension_names) =
            Self::read_points(original_points_path, reduced_points_path);

        let dimension_count = original_points_raw.first().unwrap().len();

        let data_points = original_points_raw
            .iter()
            .zip(&reduced_points_raw)
            .enumerate()
            .map(|(index, (high, low))| {
                let low_point = match low[..] {
                    [x, y, z] => na::Point3::<f32>::new(x, y, z),
                    _ => panic!("Point have an incorrect length"),
                };

                PointData3D {
                    index: index as u32,
                    low: low_point,
                    high: high.to_vec(),
                    normal: None,
                    driel_min: None,
                    driel_total: None,
                    silva_var: None,
                    silva_euclidean: None,
                }
            })
            .collect::<Vec<PointData3D>>();

        let points_ld = reduced_points_raw
            .iter()
            .map(|raw_point| match raw_point[..] {
                    [x, y, z] => na::Point3::<f32>::new(x, y, z),
                    _ => exit(14),
                }
            )
            .collect::<Vec<na::Point3<f32>>>();

        let projection_width = PointContainer3D::calculate_projection_width(&points_ld);

        // Create indexed points from the LD data
        let indexed_ld_points: Vec<IndexedPoint::<na::Point3<f32>>> = points_ld
            .into_iter()
            .enumerate()
            .map(|(index, point)| IndexedPoint::<na::Point3<f32>>::new(point, index as u32))
            .collect();

        // Create indexed vectors from the raw HD data
        let point_hd = original_points_raw
            .into_iter()
            .enumerate()
            .map(|(index, raw_point)| IndexedPoint::<Vec<f32>>::new(raw_point, index as u32))
            .collect::<Vec<IndexedPoint<Vec<f32>>>>();


        let tree_low = RTree::<IndexedPoint<na::Point3<f32>>>::bulk_load(indexed_ld_points);
        let tree_high = VPTree::new(&point_hd);

        PointContainer3D {
            tree_low,
            tree_high,
            dimension_names,
            point_data: data_points,
            dimensionality: dimension_count,
            projection_width
        }
    }
}
