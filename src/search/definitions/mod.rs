/// Module with all the data structures / definitions used throughout the search
/// structures
mod generic;
mod rtree;
mod vptree;
mod data;

pub use self::data::{IndexedPoint, PointContainer2D, PointContainer3D, PointData};