/// This module contains the data structure that will be used to store
/// the nd/low-d points and all its annotations. The annotations will
/// contain index, explanations and normals. The search structure will
/// use rtree's to quickly find nd or low-d neighbors and all its data

mod definitions;

use std::rc::Rc;
use rstar::RTree;
use nalgebra::dimension::U8;

use definitions::{PointData, LDPoint, HDPoint};

/// Data structure used to store the data about all the points,
/// it has build in support for quickly finding all the neighbors
/// in 2/3D and ND.
pub struct PointContainer {
    // Used for finding low dimensional neighbors
    tree_low: RTree<LDPoint>,
    // Used for finding high dimensional neighbors. TODO: Move away from the static dimensionality
    tree_high: RTree<HDPoint<U8>>,
    // Used when quickly iterating over all the points in order of index
    points: Vec<Rc<PointData>>,
}
