mod data;
/// Module with all the data structures / definitions used throughout the search
/// structures
mod generic;
mod rtree;
mod vptree;

pub use self::{
    data::{Indexed, IndexedPoint, PointContainer2D, PointContainer3D, PointData2D, PointData3D},
    generic::Distance,
};
