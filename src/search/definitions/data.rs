/// Module containing the structs used throughout the search structure

/// Build in imports
use std::rc::Rc;
use std::fmt;

/// Third party imports
use rstar::RTree;
use vpsearch::Tree as VPTree;

/// First party imports
use crate::exp::{DaSilvaExplanation, VanDrielExplanation};
use super::rtree::*;

/// Generic point struct that can be used to
#[derive(Clone)]
pub struct AnnotatedPoint<P: Clone + fmt::Debug> {
    pub point: P,
    pub annotation: Rc<PointData>
}

impl<P : Clone + fmt::Debug> fmt::Debug for AnnotatedPoint<P>
where P : fmt::Debug {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.point.fmt(f)
    }
}

impl<P : Clone + fmt::Debug> AnnotatedPoint<P> {
    pub fn new(point: P, data: Rc<PointData>) -> Self {
        AnnotatedPoint::<P> {
            point,
            annotation: data
        }
    }
}

/// Container that stores all the actual data for a single point
pub struct PointData {
    pub index: usize,
    pub dimensionality: usize,
    pub low: Vec<f32>,
    pub high: Vec<f32>,
    pub normal: Option<na::Point3<f32>>,
    pub driel: Option<VanDrielExplanation>,
    pub silva: Option<DaSilvaExplanation>,
}

impl PointData {
    pub fn new(index: usize, dimensions: usize, low_dimension_point: Vec<f32>, high_dimension_point: Vec<f32>) -> Self {
        PointData {
            index,
            dimensionality: dimensions,
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
pub struct PointContainer2D
{
    // Used for finding low dimensional neighbors
    pub tree_low: RTree<AnnotatedPoint<na::Point2<f32>>>,
    // Used for finding high dimensional neighbors.
    pub tree_high: VPTree<AnnotatedPoint<Vec<f32>>>,
    // Original dimension names
    pub dimension_names: Vec<String>,
    // Used when quickly iterating over all the points in order of index
    pub points: Vec<Rc<PointData>>,
}

/// Data structure used to store the data about all the points,
/// it has build in support for quickly finding all the neighbors
/// in 3D and ND.
pub struct PointContainer3D
{
    // Used for finding low dimensional neighbors
    pub tree_low: RTree<AnnotatedPoint<na::Point3<f32>>>,
    // Used for finding high dimensional neighbors.
    pub tree_high: VPTree<AnnotatedPoint<Vec<f32>>>,
    // Original dimension names
    pub dimension_names: Vec<String>,
    // Used when quickly iterating over all the points in order of index
    pub points: Vec<Rc<PointData>>,
}