extern crate nalgebra as na;

// Buildin
use crate::exp::da_silva::DaSilvaExplanation;
use na::Point3;
use std::collections::HashMap;

// Everything related to the colours in the visualization
pub struct ColorMap {
    // Map of dimension to colour
    map: HashMap<usize, Point3<f32>>,
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
        _dimension_ranking: Vec<usize>,
    ) -> ColorMap {
        ColorMap {
            map: HashMap::<usize, Point3<f32>>::new(),
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
            map: HashMap::<usize, Point3<f32>>::new(),
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
            Some(c) => c.clone(),
            None => Point3::new(0.0f32, 0.0, 0.0),
        };
        ColorMap::scale_color(normalized_conf, base_color)
    }

    // Scale a color in rgb / hsv.
    fn scale_color(_scale: f32, color: Point3<f32>) -> Point3<f32> {
        // TODO: add scaling function to dim a color.
        return color;
        // unimplemented!()
    }
}
