extern crate nalgebra as na;

// Move this load trait out of the view state
use crate::view::Load;
use crate::{
    exp::{DaSilvaExplanation, NormalExplanation, VanDrielExplanation},
    filesystem,
};

use rstar::RTree;
use std::{marker::PhantomData, path::Path, process::exit, rc::Rc};

use super::definitions::{HDPoint, LDPoint, PointData, LD};
use typenum::{Unsigned, U8};

/// Data structure used to store the data about all the points,
/// it has build in support for quickly finding all the neighbors
/// in 2D (todo: make this generic to 3D and ND).
pub struct _PointContainer<Low: LD>
where
    LDPoint<Low>: rstar::RTreeObject,
{
    // Used for finding low dimensional neighbors
    tree_low: RTree<LDPoint<Low>>,
    // Used for finding high dimensional neighbors.
    // TODO: Move away from the static dimensionality
    tree_high: RTree<HDPoint<U8>>,
    // Original dimension names
    dimension_names: Vec<String>,
    // Used when quickly iterating over all the points in order of index
    points: Vec<Rc<PointData>>,
}

pub trait PointContainer<Low: LD>
where
    LDPoint<Low>: rstar::RTreeObject,
{
    const LD: usize = <Low as Unsigned>::USIZE;
    /// Create a new point container from 2 files
    fn new(original_points_path: &Path, reduced_points_path: &Path) -> _PointContainer<Low> {
        let (original_points_raw, reduced_points_raw, dimension_names) =
            Self::read_points(original_points_path, reduced_points_path);

        let rc_points = Self::create_reference_points(&original_points_raw, &reduced_points_raw);

        let ref_points = reduced_points_raw
            .iter()
            .zip(&rc_points)
            .map(|(raw, rc)| LDPoint::<Low>::new(raw, rc.clone()))
            .collect::<Vec<LDPoint<Low>>>();

        let tree_low = RTree::<LDPoint<Low>>::bulk_load(ref_points);
        _PointContainer::<Low> {
            tree_low,
            tree_high: RTree::<HDPoint<U8>>::new(),
            dimension_names,
            points: rc_points,
        }
    }

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
        if dimension_count_low != Self::LD {
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

impl<Low> Load<Vec<DaSilvaExplanation>> for _PointContainer<Low>
where
    Low: LD,
    LDPoint<Low>: rstar::RTreeObject,
{
    fn load(&mut self, _explanations: Vec<DaSilvaExplanation>) {
        unimplemented!()
    }
}

impl<Low> Load<Vec<VanDrielExplanation>> for _PointContainer<Low>
where
    Low: LD,
    LDPoint<Low>: rstar::RTreeObject,
{
    fn load(&mut self, _explanations: Vec<VanDrielExplanation>) {
        unimplemented!()
    }
}

impl<Low> Load<Vec<NormalExplanation>> for _PointContainer<Low>
where
    Low: LD,
    LDPoint<Low>: rstar::RTreeObject,
{
    fn load(&mut self, _explanations: Vec<NormalExplanation>) {
        unimplemented!()
    }
}
