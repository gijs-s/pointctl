//! All the code related to the color maps used in the program, these are used to convert a
//! explanation to an actual color.

use std::cmp::Ordering;

// Buildin
use crate::exp::{DaSilvaExplanation, VanDrielExplanation};
use bimap::BiHashMap;
use kiss3d::conrod::color::{rgba, Color, Rgba};
use na::Point3;

#[derive(Debug, PartialEq, Clone)]
enum ColoringMode {
    // Points are in categories
    Categorical,
    // Points are in ordinal classes
    Ordinal,
    // Points are in a single class, base color only on confidence
    Single,
}

/// Everything related to the colors in the visualization
#[derive(Debug, PartialEq, Clone)]
pub struct ColorMap {
    // Map of dimension index to a color index
    map: BiHashMap<usize, usize>,
    // Dimension rank overrides, we allow the user to override the actual mapping
    // from the UI. This will change the mapping from dimension to a rank.
    rank_overrides: BiHashMap<usize, usize>,
    // Min max values for the confidence, used for normalization
    static_normalization_bounds: (f32, f32),
    // Min max values actually used to render the images, this can be set to
    // 'focus' in on a certain confidence range.
    normalization_bounds: (f32, f32),
    pub use_normalization: bool,
    mode: ColoringMode,
}

impl From<&Vec<f32>> for ColorMap {
    fn from(values: &Vec<f32>) -> Self {
        let min = values
            .iter()
            .min_by(|a, b| a.partial_cmp(&b).unwrap_or(Ordering::Equal))
            .unwrap();
        let max = values
            .iter()
            .max_by(|a, b| a.partial_cmp(&b).unwrap_or(Ordering::Equal))
            .unwrap();

        ColorMap::new(*min, *max, Vec::new(), ColoringMode::Single)
    }
}

impl From<&Vec<DaSilvaExplanation>> for ColorMap {
    fn from(explanations: &Vec<DaSilvaExplanation>) -> Self {
        let (min, max) = DaSilvaExplanation::confidence_bounds(&explanations);
        ColorMap::new(
            min,
            max,
            DaSilvaExplanation::calculate_dimension_rankings(&explanations),
            ColoringMode::Categorical,
        )
    }
}

impl From<&Vec<VanDrielExplanation>> for ColorMap {
    fn from(explanations: &Vec<VanDrielExplanation>) -> Self {
        let (min, max) = VanDrielExplanation::confidence_bounds(&explanations);
        let mut map = ColorMap::new(
            min,
            max,
            VanDrielExplanation::calculate_dimension_rankings(&explanations),
            ColoringMode::Ordinal,
        );
        map.toggle_confidence_normalisation();
        map
    }
}

impl Default for ColorMap {
    /// Create a dummy default color map that is completely empty
    fn default() -> Self {
        ColorMap {
            map: BiHashMap::<usize, usize>::new(),
            rank_overrides: BiHashMap::<usize, usize>::new(),
            normalization_bounds: (0.0, 1.0),
            static_normalization_bounds: (0.0, 1.0),
            use_normalization: true,
            mode: ColoringMode::Categorical,
        }
    }
}

impl ColorMap {
    fn new(
        min_confidence: f32,
        max_confidence: f32,
        dimension_ranking: Vec<usize>,
        mode: ColoringMode,
    ) -> ColorMap {
        // Maps a rank to a dimension and vise versa
        let map: BiHashMap<usize, usize> = match mode {
            ColoringMode::Categorical => dimension_ranking.into_iter().enumerate().collect(),
            // For the ordinal mapping we need to find the 8 dimensions that are
            // used the most, then we will have to sort these based on the actual
            // dimension numbers.
            ColoringMode::Ordinal => {
                // XXX: get 8?
                let mut rankings: Vec<usize> = dimension_ranking.into_iter().collect();
                rankings.sort_by(|a, b| a.cmp(&b));
                rankings.into_iter().enumerate().collect()
            }
            ColoringMode::Single => vec![(0, 0), (1, 1)].into_iter().collect(),
        };

        ColorMap {
            map,
            rank_overrides: BiHashMap::<usize, usize>::new(),
            normalization_bounds: (min_confidence, max_confidence),
            static_normalization_bounds: (min_confidence, max_confidence),
            use_normalization: true,
            mode,
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

    /// Return if the color map is only used for one attribute
    pub fn single_attribute_mode(&self) -> bool {
        self.mode == ColoringMode::Single
    }

    /// Retrieve the dimension from the rank (0 indexed)
    pub fn get_dimension_from_rank(&self, rank: &usize) -> Option<&usize> {
        // Retrieve the value from the overlay, if it is not present get it from
        // the inverse map
        match self.rank_overrides.get_by_right(rank) {
            None => match self.map.get_by_left(rank) {
                Some(dim) => match self.rank_overrides.get_by_left(dim) {
                    Some(_) => None,
                    None => Some(dim),
                },
                None => None,
            },
            v => v,
        }
    }

    /// Retrieve the rank for a dimension
    pub fn get_rank_from_dimension(&self, dim: &usize) -> Option<&usize> {
        // retrieve value form the overlay, if it is not present get it from the
        // inverse map
        match self.rank_overrides.get_by_left(dim) {
            // If the rank is not also pointed towards in the overlay.
            None => match self.map.get_by_right(dim) {
                Some(rank) => match self.rank_overrides.get_by_right(rank) {
                    Some(_) => None,
                    None => Some(rank),
                },
                None => None,
            },
            // If the override index rank is out of bounds, consider it the be no color
            // TODO: Is this needed?
            Some(r) if r == &self.map.len() => {
                println!("--> {:?}", self.map);
                None
            }
            v => v,
        }
    }

    /// Add an override for the standard color map from a color to rank.
    pub fn set_rank_override(&mut self, rank: usize, dim: usize) {
        self.rank_overrides.insert(dim, rank);
    }

    /// Returns if rank overrides are being used an override from the rank
    pub fn uses_rank_overrides(&self) -> bool {
        self.rank_overrides.len() != 0usize
    }

    /// Clear the overrides to the rank
    pub fn clear_rank_overrides(&mut self) {
        self.rank_overrides.clear();
    }

    /// Convert a dimension rank to a color. The ordering is as follows:
    /// Pink, Yellow, Dark red, Green, Blue, Orange, Purple and Crimson brown.
    /// Grey is used for all ranks that are not in the top 8. When in ordinal
    /// we simply use the rank as an index.
    pub fn rank_to_color(&self, rank: &usize) -> Point3<f32> {
        match self.mode {
            // Ordinal color mode
            ColoringMode::Ordinal => {
                // Rank is zero based, we only color up to rank 7.
                match rank {
                    r if r > &7usize => Self::default_color(),
                    r => {
                        let color_float: f32 = *r as f32;
                        let colored_dimensions = (self.dimension_count()).min(8usize) as f32 - 1f32;
                        if colored_dimensions == 0f32 {
                            Point3::new(2f32 / 3f32, 1f32, 1f32)
                        } else {
                            let hue = (2f32 / 3f32)
                                - (color_float * ((2f32 / 3f32) / colored_dimensions));
                            if hue.is_nan() || hue < 0.0f32 || hue > 1.032 {
                                println!(
                                    "Faulty hue found, index: {}, colored_dims: {}",
                                    r, colored_dimensions
                                )
                            }
                            Point3::new(hue, 1f32, 1f32)
                        }
                    }
                }
            }

            // Categorical color mode
            ColoringMode::Categorical => match rank {
                0 => Point3::new(0.91243, 0.47774, 0.96863), // f781bf Pink
                1 => Point3::new(0.16667, 0.80000, 1.00000), // ffff33 Yellow
                2 => Point3::new(0.99835, 0.88597, 0.89412), // e41a1c Dark red
                3 => Point3::new(0.32838, 0.57715, 0.68627), // 4daf4a Green
                4 => Point3::new(0.57493, 0.70108, 0.72157), // 377eb8 Blue
                5 => Point3::new(0.08301, 1.00000, 1.00000), // ff7f00 Orange
                6 => Point3::new(0.81177, 0.52147, 0.63922), // 984ea3 Purple
                7 => Point3::new(0.06085, 0.75904, 0.65098), // a65628 Crimson brown
                // Project explainer color map from http://mbostock.github.io/protovis/docs/color.html
                // 0 => Point3::new(0.753875, 0.455026, 0.741176), // Purple
                // 1 => Point3::new(0.167741, 0.820105, 0.741176), // Gold
                // 2 => Point3::new(0.028205, 0.464285, 0.549019), // Brown
                // 3 => Point3::new(0.083006, 1.000000, 1.000000), // Dark orange
                // 4 => Point3::new(0.884259, 0.475770, 0.890196), // Pink
                // 5 => Point3::new(0.515398, 0.888888, 0.811764), // Blue
                // 6 => Point3::new(0.001700, 0.390438, 0.984313), // Light red
                // 7 => Point3::new(0.382352, 0.093406, 0.713725), // Light green
                // 8 => Point3::new(0.568232, 0.827777, 0.705882), // Dark Blue
                _ => Self::default_color(), // 999999 Grey
            },
            // This should only be used by the UI functions
            ColoringMode::Single => match rank {
                0 => Point3::new(0.0, 1.0, 1.0), // ff0000 Red
                1 => Point3::new(0.3, 1.0, 1.0), // 00ff00 Green
                _ => Self::default_color(),      // 999999 Grey
            },
        }
    }

    /// Get a RGB color based on the current pallet
    pub fn get_color(&self, dimension: usize, confidence: f32) -> Point3<f32> {
        let normalized_conf = match self.use_normalization {
            false => confidence,
            true => {
                // normalize the confidence
                let normalized_conf = (confidence - self.normalization_bounds.0)
                    / (self.normalization_bounds.1 - self.normalization_bounds.0);

                // The user can override the normalization_bounds so we need to clamp it
                normalized_conf.max(0.0).min(1.0)
            }
        };

        match self.mode {
            ColoringMode::Single => ColorMap::get_ordinal_shade(normalized_conf),
            _ => {
                // Retrieve the color that used for that dimension
                // First we get the rank of that dimension, than we convert that rank to a color.
                let base_color = match self.get_rank_from_dimension(&dimension) {
                    Some(rank) => self.rank_to_color(rank),
                    _ => Point3::new(0.00000, 0.00000, 0.60000), // 999999 Grey
                };

                ColorMap::scale_color(normalized_conf, base_color)
            }
        }
    }

    /// When using only the confidence get a color between green and red that denotes the confidence
    fn get_ordinal_shade(conf: f32) -> Point3<f32> {
        // In HSV space 0 is red and 0.3 is green.
        Point3::new(0.3 - 0.3 * conf, 1.0, 1.0)
    }

    /// Scale a color in hsv.
    fn scale_color(scale: f32, color: Point3<f32>) -> Point3<f32> {
        // TODO: Is the brightness scale correct?
        // TODO: Make this scaling changeable
        let brightness = color.z * scale;
        Point3::new(color.x, color.y, brightness)
    }

    /// Convert a color to one that can be used by the conrod ui
    pub fn get_conrod_color(&self, rank: &usize) -> Color {
        let p = self.rank_to_color(rank);
        let (r, g, b) = hsv2rgb(p.x, p.y, p.z);
        Color::Rgba(r, g, b, 1.0f32)
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

    pub fn get_static_confidence_bounds(&self) -> (f32, f32) {
        self.static_normalization_bounds
    }

    pub fn get_actual_confidence_bounds(&self) -> (f32, f32) {
        self.normalization_bounds
    }

    pub fn set_actual_confidence_bounds(&mut self, min: f32, max: f32) {
        let lower = min.max(self.static_normalization_bounds.0);
        let upper = max.min(self.static_normalization_bounds.1);
        self.normalization_bounds = (lower, upper);
    }

    pub fn toggle_confidence_normalisation(&mut self) {
        self.use_normalization = !self.use_normalization;
    }
}

// Convert hue, saturation and value to rgb
fn hsv2rgb(hue: f32, sat: f32, val: f32) -> (f32, f32, f32) {
    // Ensure the hue does not exceed 1
    let hue_360 = (hue * 360f32) % 360f32;

    let c = val * sat;
    let x = c * (1f32 - ((hue_360 / 60f32) % 2f32 - 1f32).abs());
    let m = val - c;

    let (r, g, b) = match hue_360 {
        h if h >= 0f32 && h < 60f32 => (c, x, 0f32),
        h if h >= 60f32 && h < 120f32 => (x, c, 0f32),
        h if h >= 120f32 && h < 180f32 => (0f32, c, x),
        h if h >= 180f32 && h < 240f32 => (0f32, x, c),
        h if h >= 240f32 && h < 300f32 => (x, 0f32, c),
        h if h >= 300f32 => (c, 0f32, x),
        _ => panic!("Actually unreachable: {}", hue_360),
    };
    (r + m, g + m, b + m)
}
