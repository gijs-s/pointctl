extern crate nalgebra as na;

// Buildin
use crate::exp::{da_silva::DaSilvaExplanation, driel::VanDrielExplanation};
use kiss3d::conrod::color::{rgb_bytes, rgba, Color, Rgba};
use na::Point3;
use std::collections::HashMap;

/// Everything related to the colors in the visualization
#[derive(Debug, PartialEq, Clone)]
pub struct ColorMap {
    // Map of dimension to a color index
    map: HashMap<usize, usize>,
    // Dimension ranks (inverse map). This maps a color index to the dimension
    // TODO: store dimension name instead
    inverse_map: HashMap<usize, usize>,
    // Min max values for the confidence, used for normalization
    normalization_bounds: (f32, f32),
}

impl From<&Vec<DaSilvaExplanation>> for ColorMap {
    fn from(explanations: &Vec<DaSilvaExplanation>) -> Self {
        let (min, max) = DaSilvaExplanation::confidence_bounds(&explanations);
        ColorMap::new(
            min,
            max,
            DaSilvaExplanation::calculate_dimension_rankings(&explanations),
        )
    }
}

impl From<&Vec<VanDrielExplanation>> for ColorMap {
    fn from(explanations: &Vec<VanDrielExplanation>) -> Self {
        let (min, max) = VanDrielExplanation::confidence_bounds(&explanations);
        ColorMap::new(
            min,
            max,
            VanDrielExplanation::calculate_dimension_rankings(&explanations),
        )
    }
}

impl Default for ColorMap {
    /// Create a dummy default color map that is completely empty
    fn default() -> Self {
        ColorMap {
            map: HashMap::<usize, usize>::new(),
            inverse_map: HashMap::<usize, usize>::new(),
            normalization_bounds: (0.0, 1.0),
        }
    }
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
        }
    }

    /// Check if the color map has been initialized
    pub fn is_initialized(&self) -> bool {
        self.dimension_count() != 0usize
    }

    /// Retrieve the amount of dimensions in the ranking map
    pub fn dimension_count(&self) -> usize {
        assert_eq!(self.map.len(), self.inverse_map.len());
        self.map.len()
    }

    /// Retrieve the dimension from the rank (0 indexed)
    pub fn get_dimension_from_rank(&self, rank: &usize) -> Option<&usize> {
        assert_eq!(self.map.len(), self.inverse_map.len());
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
        let normalized_conf = (confidence - self.normalization_bounds.0)
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
        // TODO: Is the brightness scale correct?
        // TODO: Make this scaling changeable
        let brightness = color.z * scale;
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

    pub fn get_conrod_color_with_gamma(&self, rank: &usize, gamma: f32) -> Color {
        let color = self.get_conrod_color(&rank);
        ColorMap::gamma_correct(&color, gamma)
    }

    // Preform the gamma correction calculation for a conrod color. This is used for the UI
    pub fn gamma_correct(color: &Color, gamma: f32) -> Color {
        let scale = 1.0f32 / gamma;
        let Rgba(r, g, b, a) = color.to_rgb();
        rgba(r.powf(scale), g.powf(scale), b.powf(scale), a)
    }

    // Return a gray point
    pub fn default_color() -> Point3<f32> {
        Point3::new(0.00000, 0.00000, 0.60000)
    }
}
