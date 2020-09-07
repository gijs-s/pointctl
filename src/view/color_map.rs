extern crate nalgebra as na;

// Buildin
use crate::exp::da_silva::DaSilvaExplanation;
use kiss3d::conrod::color::{rgb_bytes, Color};
use na::Point3;
use std::collections::HashMap;

// Everything related to the colors in the visualization
pub struct ColorMap {
    // Map of dimension to a color index
    map: HashMap<usize, usize>,
    // Dimension ranks (inverse map). This maps a color index to the dimension
    // TODO: store dimension name instead
    inverse_map: HashMap<usize, usize>,
    // Min max values for the confidence, used for normalization
    normalization_bounds: (f32, f32),
    // gamma correction value, static for now
    gamma: f32,
}

impl ColorMap {
    // TODO: Create the actual color map
    pub fn new(
        min_confidence: f32,
        max_confidence: f32,
        dimension_ranking: Vec<usize>,
    ) -> ColorMap {
        let mut map = HashMap::<usize, usize>::new();
        let mut inverse_map = HashMap::<usize, usize>::new();
        for (index, &dim) in dimension_ranking.iter().enumerate() {
            map.insert(dim, index);
            inverse_map.insert(index, dim);
        }

        ColorMap {
            map,
            inverse_map,
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
            inverse_map: HashMap::<usize, usize>::new(),
            normalization_bounds: (0.0, 1.0),
            gamma: 2.2,
        }
    }

    /// Check if the color map has been initialized
    pub fn is_initialized(&self) -> bool {
        self.dimension_count() != 0usize
    }

    /// Retrieve the amount of dimensions in the ranking map
    pub fn dimension_count(&self) -> usize {
        self.map.len()
    }

    /// Retrieve the dimension from the rank
    pub fn get_dimension_from_rank(&self, rank: &usize) -> Option<&usize> {
        self.inverse_map.get(rank)
    }

    /// Convert a dimension rank to a color. The ordering is as follows:
    /// Pink, Yellow, Dark red, Green, Blue, Orange, Purple and Crimson brown.clap
    /// Grey is used for all ranks that are not in the top 8.
    pub fn rank_to_color(&self, rank: &usize) -> Point3<f32> {
        match rank {
            0 => Point3::new(0.91243, 0.47774, 0.96863), // f781bf Pink
            1 => Point3::new(0.16667, 0.80000, 1.00000), // ffff33 Yellow
            2 => Point3::new(0.99835, 0.88597, 0.89412), // e41a1c Dark red
            3 => Point3::new(0.32838, 0.57715, 0.68627), // 4daf4a Green
            4 => Point3::new(0.57493, 0.70108, 0.72157), // 377eb8 Blue
            5 => Point3::new(0.08301, 1.00000, 1.00000), // ff7f00 Orange
            6 => Point3::new(0.81177, 0.52147, 0.63922), // 984ea3 Purple
            7 => Point3::new(0.06085, 0.75904, 0.65098), // a65628 Crimson brown
            _ => Point3::new(0.00000, 0.00000, 0.60000), // 999999 Grey
        }
    }

    /// Get a RGB color based on the current pallet
    pub fn get_color(&self, dimension: usize, confidence: f32) -> Point3<f32> {
        // normalize the confidence
        let normalized_conf = confidence
            - self.normalization_bounds.0
                / (self.normalization_bounds.1 - self.normalization_bounds.0);

        // Retrieve the color that used for that dimension
        // First we get the rank of that dimennsion, than we convert that rank to a color.
        let base_color = match self.map.get(&dimension) {
            Some(rank) => self.rank_to_color(rank),
            _ => Point3::new(0.00000, 0.00000, 0.60000), // 999999 Grey
        };

        ColorMap::scale_color(normalized_conf, base_color)
    }

    /// Scale a color in hsv.
    fn scale_color(scale: f32, color: Point3<f32>) -> Point3<f32> {
        let brightness = color.z * scale.cbrt();
        return Point3::new(color.x, color.y, brightness);
    }

    /// Convert a color to one that can be used by the conrod ui
    pub fn get_conrod_color(&self, rank: &usize) -> Color {
        match rank {
            0 => rgb_bytes(247, 129, 191), // f781bf Pink
            1 => rgb_bytes(255, 255, 51),  // ffff33 Yellow
            2 => rgb_bytes(228, 26, 28),   // e41a1c Dark red
            3 => rgb_bytes(77, 175, 74),   // 4daf4a Green
            4 => rgb_bytes(55, 126, 184),  // 377eb8 Blue
            5 => rgb_bytes(255, 127, 0),   // ff7f00 Orange
            6 => rgb_bytes(152, 78, 163),  // 984ea3 Purple
            7 => rgb_bytes(166, 86, 40),   // a65628 Crimson brown
            _ => rgb_bytes(153, 153, 153), // 999999 Grey
        }
    }
}
