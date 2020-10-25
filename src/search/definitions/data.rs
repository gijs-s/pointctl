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
    pub index: usize,
}

impl<P: Clone + fmt::Debug> IndexedPoint<P> {
    pub fn new(point: P, index: usize) -> Self {
        IndexedPoint::<P> { point, index }
    }
}

pub trait Indexed {
    fn get_index(&self) -> usize;
}

impl<P: Clone + fmt::Debug> Indexed for IndexedPoint<P> {
    fn get_index(&self) -> usize {
        self.index
    }
}

/// Container that stores all the actual data for a single point
pub struct PointData2D {
    pub index: usize,
    pub low: na::Point2<f32>,
    pub high: Vec<f32>,
    pub normal: Option<NormalExplanation>,
    pub driel: Option<VanDrielExplanation>,
    pub silva: Option<DaSilvaExplanation>,
}

impl Into<na::Point2<f32>> for PointData2D {
    fn into(self) -> na::Point2<f32> {
        self.low
    }
}

impl PointData2D {
    pub fn new(
        index: usize,
        low_dimension_point: na::Point2<f32>,
        high_dimension_point: Vec<f32>,
    ) -> Self {
        PointData2D {
            index,
            low: low_dimension_point,
            high: high_dimension_point,
            normal: None,
            driel: None,
            silva: None,
        }
    }
}

/// Container that stores all the actual data for a single point
pub struct PointData3D {
    pub index: usize,
    pub low: na::Point3<f32>,
    pub high: Vec<f32>,
    pub normal: Option<NormalExplanation>,
    pub driel: Option<VanDrielExplanation>,
    pub silva: Option<DaSilvaExplanation>,
}

impl Into<na::Point3<f32>> for PointData3D {
    fn into(self) -> na::Point3<f32> {
        self.low
    }
}

impl PointData3D {
    pub fn new(
        index: usize,
        low_dimension_point: na::Point3<f32>,
        high_dimension_point: Vec<f32>,
    ) -> Self {
        PointData3D {
            index,
            low: low_dimension_point,
            high: high_dimension_point,
            normal: None,
            driel: None,
            silva: None,
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
                    index,
                    low: low_point,
                    high: high.to_vec(),
                    normal: None,
                    driel: None,
                    silva: None,
                }
            })
            .collect::<Vec<PointData2D>>();

        let ref_points_ld = reduced_points_raw
            .into_iter()
            .enumerate()
            .map(|(index, raw_point)| {
                let point = match raw_point[..] {
                    [x, y] => na::Point2::<f32>::new(x, y),
                    _ => exit(14),
                };
                IndexedPoint::<na::Point2<f32>>::new(point, index)
            })
            .collect::<Vec<IndexedPoint<na::Point2<f32>>>>();

        let ref_point_hd = original_points_raw
            .into_iter()
            .enumerate()
            .map(|(index, raw_point)| IndexedPoint::<Vec<f32>>::new(raw_point, index))
            .collect::<Vec<IndexedPoint<Vec<f32>>>>();

        let tree_low = RTree::<IndexedPoint<na::Point2<f32>>>::bulk_load(ref_points_ld);
        let tree_high = VPTree::new(&ref_point_hd);

        PointContainer2D {
            tree_low,
            tree_high,
            dimension_names,
            point_data: data_points,
            dimensionality: dimension_count,
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
                    index,
                    low: low_point,
                    high: high.to_vec(),
                    normal: None,
                    driel: None,
                    silva: None,
                }
            })
            .collect::<Vec<PointData3D>>();

        let points_ld = reduced_points_raw
            .iter()
            .enumerate()
            .map(|(index, raw_point)| {
                let point = match raw_point[..] {
                    [x, y, z] => na::Point3::<f32>::new(x, y, z),
                    _ => exit(14),
                };
                IndexedPoint::<na::Point3<f32>>::new(point, index)
            })
            .collect::<Vec<IndexedPoint<na::Point3<f32>>>>();

        let point_hd = original_points_raw
            .into_iter()
            .enumerate()
            .map(|(index, raw_point)| IndexedPoint::<Vec<f32>>::new(raw_point, index))
            .collect::<Vec<IndexedPoint<Vec<f32>>>>();

        let tree_low = RTree::<IndexedPoint<na::Point3<f32>>>::bulk_load(points_ld);
        let tree_high = VPTree::new(&point_hd);

        PointContainer3D {
            tree_low,
            tree_high,
            dimension_names,
            point_data: data_points,
            dimensionality: dimension_count,
        }
    }
}
