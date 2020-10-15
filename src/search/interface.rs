/// File containing the entire interface with the search structure.

// build in imports
use std::{path::Path, process::exit, rc::Rc, fmt::Debug};

// Third party imports
use rstar::RTree;
use vpsearch::Tree as VPTree;

// First party imports
use crate::{
    // Move this load trait out of the view state
    view::Load,
    exp::{DaSilvaExplanation, NormalExplanation, VanDrielExplanation},
    filesystem,
};
use super::definitions::{IndexedPoint, PointData, PointContainer2D, PointContainer3D};

/// Functions that are supported by both 2 and 3 dimensional point containers
pub trait PointContainer {
    const DIMENSIONS: usize;
    type LDPoint : rstar::RTreeObject + Clone + Debug;


    /// Get a reference to the low dimension search tree for searching
    /// neighbors
    fn get_tree_low(&self) -> &RTree<Self::LDPoint>;

    /// Get a reference to the high dimension search tree for searching
    /// neighbors. Will not be used for now.
    fn get_tree_high(&self) -> &VPTree<IndexedPoint<Vec<f32>>>;

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

    /// Create new version of the PointData objects wrapped in a reference counter
    fn create_data_points(
        original_points_raw: &Vec<Vec<f32>>,
        reduced_points_raw: &Vec<Vec<f32>>,
    ) -> Vec<PointData> {
        let dimension_count = original_points_raw.first().unwrap().len();
        original_points_raw
            .iter()
            .zip(reduced_points_raw)
            .enumerate()
            .map(|(index, (high, low))| {
                PointData {
                    index,
                    dimensionality: dimension_count,
                    low: low.to_vec(),
                    high: high.to_vec(),
                    normal: None,
                    driel: None,
                    silva: None,
                }
            })
            .collect::<Vec<PointData>>()
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
}

impl PointContainer2D {
    /// Create a new point container from 2 files
    fn new(original_points_path: &Path, reduced_points_path: &Path) -> PointContainer2D {
        let (original_points_raw, reduced_points_raw, dimension_names) =
            Self::read_points(original_points_path, reduced_points_path);

        let data_points = Self::create_data_points(&original_points_raw, &reduced_points_raw);

        let ref_points_ld = reduced_points_raw
            .iter()
            .enumerate()
            .map(|(index, raw_point)| {
                let point = match raw_point[..] {
                    [x, y] => na::Point2::<f32>::new(x,y),
                    _ => exit(14),
                };
                IndexedPoint::<na::Point2<f32>>::new(point, index)
            })
            .collect::<Vec<IndexedPoint<na::Point2<f32>>>>();

        let ref_point_hd = original_points_raw
            .into_iter()
            .enumerate()
            .map(|(index, raw_point)| IndexedPoint::<Vec<f32>>::new(raw_point, index))
            .collect::<Vec<IndexedPoint::<Vec<f32>>>>();

        let tree_low = RTree::<IndexedPoint<na::Point2<f32>>>::bulk_load(ref_points_ld);
        let tree_high = VPTree::new(&ref_point_hd);

        PointContainer2D {
            tree_low,
            tree_high,
            dimension_names,
            point_data: data_points,
        }
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
}

impl PointContainer3D {
    /// Create a new point container from 2 files
    fn new(original_points_path: &Path, reduced_points_path: &Path) -> PointContainer3D {
        let (original_points_raw, reduced_points_raw, dimension_names) =
            Self::read_points(original_points_path, reduced_points_path);

        let data_points = Self::create_data_points(&original_points_raw, &reduced_points_raw);

        let ref_points_ld = reduced_points_raw
            .iter()
            .enumerate()
            .map(|(index, raw_point)| {
                let point = match raw_point[..] {
                    [x, y, z] => na::Point3::<f32>::new(x,y,z),
                    _ => exit(14),
                };
                IndexedPoint::<na::Point3<f32>>::new(point, index)
            })
            .collect::<Vec<IndexedPoint<na::Point3<f32>>>>();

        let ref_point_hd = original_points_raw
            .into_iter()
            .enumerate()
            .map(|(index, raw_point)| IndexedPoint::<Vec<f32>>::new(raw_point, index))
            .collect::<Vec<IndexedPoint::<Vec<f32>>>>();

        let tree_low = RTree::<IndexedPoint<na::Point3<f32>>>::bulk_load(ref_points_ld);
        let tree_high = VPTree::new(&ref_point_hd);

        PointContainer3D {
            tree_low,
            tree_high,
            dimension_names,
            point_data: data_points,
        }
    }
}