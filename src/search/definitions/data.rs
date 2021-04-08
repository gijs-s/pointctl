//! Module containing the structs used throughout the search structure

use std::path::Path;
use std::process::exit;

/// Build in imports
use std::{collections::HashMap, fmt};

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
    pub index: u32,
}

impl<P: Clone + fmt::Debug> IndexedPoint<P> {
    pub fn new(point: P, index: u32) -> Self {
        IndexedPoint::<P> { point, index }
    }
}

pub trait Indexed {
    fn get_index(&self) -> u32;
}

impl<P: Clone + fmt::Debug> Indexed for IndexedPoint<P> {
    fn get_index(&self) -> u32 {
        self.index
    }
}

/// Container that stores all the actual data for a single point
pub struct PointData2D {
    pub index: u32,
    pub low: na::Point2<f32>,
    pub high: Vec<f32>,
    pub driel_min: Option<VanDrielExplanation>,
    pub driel_total: Option<VanDrielExplanation>,
    pub silva_var: Option<DaSilvaExplanation>,
    pub silva_euclidean: Option<DaSilvaExplanation>,
    // Stores the explanation based on a single attribute.
    pub silva_single: HashMap<usize, f32>,
}

impl Into<na::Point2<f32>> for PointData2D {
    fn into(self) -> na::Point2<f32> {
        self.low
    }
}

impl PointData2D {
    pub fn new(
        index: u32,
        low_dimension_point: na::Point2<f32>,
        high_dimension_point: Vec<f32>,
    ) -> Self {
        PointData2D {
            index,
            low: low_dimension_point,
            high: high_dimension_point,
            driel_min: None,
            driel_total: None,
            silva_var: None,
            silva_euclidean: None,
            silva_single: HashMap::new(),
        }
    }
}

/// Container that stores all the actual data for a single point
pub struct PointData3D {
    pub index: u32,
    pub low: na::Point3<f32>,
    pub high: Vec<f32>,
    pub normal: Option<NormalExplanation>,
    pub driel_min: Option<VanDrielExplanation>,
    pub driel_total: Option<VanDrielExplanation>,
    pub silva_var: Option<DaSilvaExplanation>,
    pub silva_euclidean: Option<DaSilvaExplanation>,
    // Stores the explanation based on a single attribute.
    pub silva_single: HashMap<usize, f32>,
}

impl Into<na::Point3<f32>> for PointData3D {
    fn into(self) -> na::Point3<f32> {
        self.low
    }
}

impl PointData3D {
    pub fn new(
        index: u32,
        low_dimension_point: na::Point3<f32>,
        high_dimension_point: Vec<f32>,
    ) -> Self {
        PointData3D {
            index,
            low: low_dimension_point,
            high: high_dimension_point,
            normal: None,
            driel_min: None,
            driel_total: None,
            silva_var: None,
            silva_euclidean: None,
            silva_single: HashMap::new(),
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
    // Projection width of all the points,
    pub projection_width: f32,
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
    // Projection width of all the points,
    pub projection_width: f32,
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
                PointData2D::new(index as u32, low_point, high.to_vec())
            })
            .collect::<Vec<PointData2D>>();

        // Create na::algebra points from the raw data
        let points_ld = reduced_points_raw
            .iter()
            .map(|raw_point| match raw_point[..] {
                [x, y] => na::Point2::<f32>::new(x, y),
                _ => exit(14),
            })
            .collect::<Vec<na::Point2<f32>>>();

        let projection_width = PointContainer2D::calculate_projection_width(&points_ld);

        // Create indexed points from the LD data
        let indexed_ld_points: Vec<IndexedPoint<na::Point2<f32>>> = points_ld
            .into_iter()
            .enumerate()
            .map(|(index, point)| IndexedPoint::<na::Point2<f32>>::new(point, index as u32))
            .collect();

        // Create indexed vectors from the raw HD data
        let ref_point_hd = original_points_raw
            .into_iter()
            .enumerate()
            .map(|(index, raw_point)| IndexedPoint::<Vec<f32>>::new(raw_point, index as u32))
            .collect::<Vec<IndexedPoint<Vec<f32>>>>();

        let tree_low = RTree::<IndexedPoint<na::Point2<f32>>>::bulk_load(indexed_ld_points);
        let tree_high = VPTree::new(&ref_point_hd);

        PointContainer2D {
            tree_low,
            tree_high,
            dimension_names,
            point_data: data_points,
            dimensionality: dimension_count,
            projection_width,
        }
    }

    /// Get the point closest to these world coordinates. Returns none if there are no points
    pub fn get_closest_point(&self, x: f32, y: f32) -> Option<&PointData2D> {
        self.tree_low
            .nearest_neighbor_iter(&[x, y])
            .map(|ind| ind.index)
            .next()
            .and_then(|index| Some(&self.point_data[index as usize]))
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

                PointData3D::new(index as u32, low_point, high.to_vec())
            })
            .collect::<Vec<PointData3D>>();

        let points_ld = reduced_points_raw
            .iter()
            .map(|raw_point| match raw_point[..] {
                [x, y, z] => na::Point3::<f32>::new(x, y, z),
                _ => exit(14),
            })
            .collect::<Vec<na::Point3<f32>>>();

        let projection_width = PointContainer3D::calculate_projection_width(&points_ld);

        // Create indexed points from the LD data
        let indexed_ld_points: Vec<IndexedPoint<na::Point3<f32>>> = points_ld
            .into_iter()
            .enumerate()
            .map(|(index, point)| IndexedPoint::<na::Point3<f32>>::new(point, index as u32))
            .collect();

        // Create indexed vectors from the raw HD data
        let point_hd = original_points_raw
            .into_iter()
            .enumerate()
            .map(|(index, raw_point)| IndexedPoint::<Vec<f32>>::new(raw_point, index as u32))
            .collect::<Vec<IndexedPoint<Vec<f32>>>>();

        let tree_low = RTree::<IndexedPoint<na::Point3<f32>>>::bulk_load(indexed_ld_points);
        let tree_high = VPTree::new(&point_hd);

        PointContainer3D {
            tree_low,
            tree_high,
            dimension_names,
            point_data: data_points,
            dimensionality: dimension_count,
            projection_width,
        }
    }

    /// Compute the distance from a point to a ray
    fn distance_to_line(
        ray_origin: &na::Point3<f32>,
        ray_direction: &na::Vector3<f32>,
        point: &na::Point3<f32>,
    ) -> f32 {
        // Get the direction from the origin to the point
        let v: na::Vector3<f32> = point - ray_origin;
        let t: f32 = v.dot(ray_direction);
        // Point projected onto the line, closest point along the line from A
        let p: na::Point3<f32> = ray_origin + t * ray_direction;
        na::distance(&p, point)
    }

    /// Get the the point closest to the line, note that this linear scan is a bit slow
    pub fn get_closest_point(
        &self,
        ray_origin: na::Point3<f32>,
        ray_direction: na::Vector3<f32>,
    ) -> Option<&PointData3D> {
        // Distance and point index
        let mut closest: (f32, Option<usize>) = (f32::INFINITY, None);
        for indexed_point in self.get_tree_low().iter() {
            let distance = Self::distance_to_line(&ray_origin, &ray_direction, &indexed_point.point);
            if distance < closest.0 {
                closest = (distance, Some(indexed_point.index as usize));
            }
        }
        // If index is not none get the point data
        closest.1.and_then(|index| Some(&self.point_data[index]))
    }
}

/// Easy to use generic format for showing a tooltip in the frontend
pub struct UIPointData {
    pub index: u32,
    pub x: f32,
    pub y: f32,
    pub z: Option<f32>,
    pub high: Vec<f32>,
    pub driel_min: Option<VanDrielExplanation>,
    pub driel_total: Option<VanDrielExplanation>,
    pub silva_var: Option<DaSilvaExplanation>,
    pub silva_euclidean: Option<DaSilvaExplanation>,
}

impl From<&PointData2D> for UIPointData {
    fn from(p: &PointData2D) -> Self {
        UIPointData {
            index: p.index,
            x: p.low.x,
            y: p.low.y,
            z: None,
            high: p.high.clone(),
            driel_min: p.driel_min,
            driel_total: p.driel_total,
            silva_var: p.silva_var,
            silva_euclidean: p.silva_euclidean,
        }
    }
}

impl From<&PointData3D> for UIPointData {
    fn from(p: &PointData3D) -> Self {
        UIPointData {
            index: p.index,
            x: p.low.x,
            y: p.low.y,
            z: Some(p.low.z),
            high: p.high.clone(),
            driel_min: p.driel_min,
            driel_total: p.driel_total,
            silva_var: p.silva_var,
            silva_euclidean: p.silva_euclidean,
        }
    }
}

impl UIPointData {
    pub fn ui_string(&self, attribute_names: Vec<String>) -> String {
        // Add the first line, index and coords.
        let mut res = format!("Index: {}", self.index);
        res.push_str(&match &self.z {
            Some(z) => format!(" ({:.3}, {:.3}, {:.3})", self.x, self.y, z),
            None => format!(" ({:.3}, {:.3})", self.x, self.y),
        });
        // Show all the explanations.
        if let Some(expl) = self.silva_var {
            res.push_str(&format!(
                "\nAttribute-based (Variance):\n  Confidence {:.3}, Attribute: {}",
                expl.confidence, attribute_names[expl.attribute_index]
            ));
        }
        if let Some(expl) = self.silva_euclidean {
            res.push_str(&format!(
                "\nAttribute-based (Euclidean):\n  Confidence {:.3}, Attribute: {}",
                expl.confidence, attribute_names[expl.attribute_index]
            ));
        }
        if let Some(expl) = self.driel_min {
            res.push_str(&format!(
                "\nDimensionality-based (min):\n  Confidence {:.3}, Dimensions: {}",
                expl.confidence, expl.dimension
            ));
        }
        if let Some(expl) = self.driel_total {
            res.push_str(&format!(
                "\nDimensionality-based (total):\n  Confidence {:.3}, Dimensions: {}",
                expl.confidence, expl.dimension
            ));
        }
        // Show the values in the original dataset
        res.push_str("\n\nOriginal values:");
        for (value, name) in self.high.iter().zip(attribute_names) {
            res.push_str(&format!("\n {:.4} - {}", value, name));
        }
        res
    }
}
