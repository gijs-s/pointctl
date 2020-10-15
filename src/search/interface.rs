/// File containing the entire interface with the search structure.

// build in imports
use std::{path::Path, process::exit, rc::Rc};

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
use super::definitions::data::{AnnotatedPoint, PointData, PointContainer2D, PointContainer3D};

/// Functions that are supported by both 2 and 3 dimensional point containers
pub trait PointContainer {
    const DIMENSIONS: usize;

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
                "Expected data reduced to 2D but got {} dimensions instead",
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
    fn create_reference_points(
        original_points_raw: &Vec<Vec<f32>>,
        reduced_points_raw: &Vec<Vec<f32>>,
    ) -> Vec<Rc<PointData>> {
        let dimension_count = original_points_raw.first().unwrap().len();
        original_points_raw
            .iter()
            .zip(reduced_points_raw)
            .enumerate()
            .map(|(index, (high, low))| {
                Rc::new(PointData {
                    index,
                    dimensionality: dimension_count,
                    low: low.to_vec(),
                    high: high.to_vec(),
                    normal: None,
                    driel: None,
                    silva: None,
                })
            })
            .collect::<Vec<Rc<PointData>>>()
    }
}

impl PointContainer for PointContainer2D {
    const DIMENSIONS: usize = 2;
}

impl PointContainer2D {
    /// Create a new point container from 2 files
    fn new(original_points_path: &Path, reduced_points_path: &Path) -> PointContainer2D {
        let (original_points_raw, reduced_points_raw, dimension_names) =
            Self::read_points(original_points_path, reduced_points_path);

        let rc_points = Self::create_reference_points(&original_points_raw, &reduced_points_raw);

        let ref_points_ld = reduced_points_raw
            .iter()
            .zip(&rc_points)
            .map(|(raw, rc)| {
                let point = match raw[..] {
                    [x, y] => na::Point2::<f32>::new(x,y),
                    _ => exit(14),
                };
                AnnotatedPoint::<na::Point2<f32>>::new(point, rc.clone())
            })
            .collect::<Vec<AnnotatedPoint<na::Point2<f32>>>>();

        let ref_point_hd = original_points_raw
            .into_iter()
            .zip(&rc_points)
            .map(|(raw, rc)| AnnotatedPoint::<Vec<f32>>::new(raw, rc.clone()))
            .collect::<Vec<AnnotatedPoint::<Vec<f32>>>>();

        let tree_low = RTree::<AnnotatedPoint<na::Point2<f32>>>::bulk_load(ref_points_ld);
        let tree_high = VPTree::new(&ref_point_hd);

        PointContainer2D {
            tree_low,
            tree_high,
            dimension_names,
            points: rc_points,
        }
    }
}

impl PointContainer for PointContainer3D {
    const DIMENSIONS: usize = 3;
}

impl PointContainer3D {
    /// Create a new point container from 2 files
    fn new(original_points_path: &Path, reduced_points_path: &Path) -> PointContainer3D {
        let (original_points_raw, reduced_points_raw, dimension_names) =
            Self::read_points(original_points_path, reduced_points_path);

        let rc_points = Self::create_reference_points(&original_points_raw, &reduced_points_raw);

        let ref_points_ld = reduced_points_raw
            .iter()
            .zip(&rc_points)
            .map(|(raw, rc)| {
                let point = match raw[..] {
                    [x, y, z] => na::Point3::<f32>::new(x,y,z),
                    _ => exit(14),
                };
                AnnotatedPoint::<na::Point3<f32>>::new(point, rc.clone())
            })
            .collect::<Vec<AnnotatedPoint<na::Point3<f32>>>>();

        let ref_point_hd = original_points_raw
            .into_iter()
            .zip(&rc_points)
            .map(|(raw, rc)| AnnotatedPoint::<Vec<f32>>::new(raw, rc.clone()))
            .collect::<Vec<AnnotatedPoint::<Vec<f32>>>>();

        let tree_low = RTree::<AnnotatedPoint<na::Point3<f32>>>::bulk_load(ref_points_ld);
        let tree_high = VPTree::new(&ref_point_hd);

        PointContainer3D {
            tree_low,
            tree_high,
            dimension_names,
            points: rc_points,
        }
    }
}