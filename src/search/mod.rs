//! This module contains the data structure that will be used to store
//! the nd/low-d points and all its annotations. The annotations will
//! contain index, explanations and normals. The search structure will
//! use rtree's in 2/3d and vantage point trees in higher dimensions
//! to quickly find neighbors and accompanying data.

// Sub modules
mod definitions;
mod interface;
mod load;

// Re-export the public facing parts of this module
pub use self::{
    definitions::{
        Distance, IndexedPoint, PointContainer2D, PointContainer3D, PointData2D, PointData3D, UIPointData
    },
    interface::PointContainer,
    load::Load,
};
