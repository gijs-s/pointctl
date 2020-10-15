/// This module contains the data structure that will be used to store
/// the nd/low-d points and all its annotations. The annotations will
/// contain index, explanations and normals. The search structure will
/// use rtree's in 2/3d and vantage point trees in higher dimensions
/// to quickly find neighbors and accompanying data.


// Sub modules
mod definitions;
mod interface;

// Structs that should be reexported from this module.
pub use self::definitions::data::{PointContainer2D, PointContainer3D};