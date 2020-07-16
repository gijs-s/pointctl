extern crate nalgebra as na;

// Buildin
use crate::exp::da_silva::DaSilvaExplanation;
use na::Point3;
use std::collections::HashMap;

// Everything related to the colours in the visualization
pub struct ColorMap {
    // Map of dimension to a colour index
    map: HashMap<usize, usize>,
    // Min max values for the confidence, used for normalization
    normalization_bounds: (f32, f32),
    // gamma correction value, static for now
    gamma: f32,
}

impl ColorMap {
    // TODO: Create the actual colour map
    pub fn new(
        min_confidence: f32,
        max_confidence: f32,
        dimension_ranking: Vec<usize>,
    ) -> ColorMap {
        let mut map = HashMap::<usize, usize>::new();
        for (index, &dim) in dimension_ranking.iter().enumerate() {
            map.insert(dim, index);
        }

        ColorMap {
            map,
            normalization_bounds: (min_confidence, max_confidence),
            gamma: 2.2,
        }
    }

    pub fn from_explanations(
        explanations: &Vec<DaSilvaExplanation>,
        dimension_count: usize,
    ) -> ColorMap {
        ColorMap::new(
            DaSilvaExplanation::min_confidence(&explanations),
            DaSilvaExplanation::max_confidence(&explanations),
            DaSilvaExplanation::calculate_dimension_rankings(dimension_count, &explanations),
        )
    }

    // Create a dummy place holder empty colormap
    pub fn new_dummy() -> ColorMap {
        ColorMap {
            map: HashMap::<usize, usize>::new(),
            normalization_bounds: (0.0, 1.0),
            gamma: 2.2,
        }
    }

    // Get a RGB colour based on the current pallet
    pub fn get_colour(&self, dimension: usize, confidence: f32) -> Point3<f32> {
        let normalized_conf = confidence
            - self.normalization_bounds.0
                / (self.normalization_bounds.1 - self.normalization_bounds.0);
        // TODO: Put in an actual colour map

        let base_color = match self.map.get(&dimension) {
            Some(0) => Point3::new(0.99835, 0.88597, 0.89412), //e41a1c Dark red
            Some(1) => Point3::new(0.57493, 0.70108, 0.72157), //377eb8 Blue
            Some(2) => Point3::new(0.16667, 0.80000, 1.00000), // ffff33 Yellow
            Some(3) => Point3::new(0.91243, 0.47774, 0.96863), // f781bf Pink
            Some(4) => Point3::new(0.81177, 0.52147, 0.63922), // 984ea3 Purple
            Some(5) => Point3::new(0.08301, 1.00000, 1.00000), // ff7f00 Orange
            Some(6) => Point3::new(0.32838, 0.57715, 0.68627), // 4daf4a Green
            Some(7) => Point3::new(0.06085, 0.75904, 0.65098), // a65628 Crimson brown
            _ => Point3::new(0.00000,  0.00000,  0.60000), // 999999 Grey
        };
        ColorMap::scale_color(normalized_conf, base_color)
    }

    // Scale a color in rgb / hsv.
    fn scale_color(scale: f32, color: Point3<f32>) -> Point3<f32> {
        let brightness = color.z * scale.sqrt();
        return Point3::new(color.x, color.y, brightness);
    }
}
