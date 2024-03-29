//! Module containing the state of the 3D scene

/// Build in imports
use std::collections::HashMap;

/// Third party imports
use kiss3d::planar_camera::{PlanarCamera, Sidescroll};

use na::{Point2, Point3};

use super::VisualizationStateInteraction;
/// First party imports
use crate::{
    exp,
    exp::{DaSilvaExplanation, DaSilvaType, VanDrielExplanation, VanDrielType},
    search::{Load, PointContainer, PointContainer2D, UIPointData},
    view::{point_renderer::PointRenderer2D, ColorMap, ExplanationMode, PointRendererInteraction},
};

pub struct VisualizationState2D {
    // Camera used by this view.
    pub camera: Sidescroll,
    // Data container with all the points.
    pub point_container: PointContainer2D,
    // Used for rendering points.
    pub renderer: PointRenderer2D,
    // color map used by the 2D visualizer
    pub color_maps: HashMap<ExplanationMode, ColorMap>,
    // Explanation being viewed at this moment
    explanation: ExplanationMode,
}

impl VisualizationState2D {
    pub fn new(point_container: PointContainer2D) -> VisualizationState2D {
        // Create the color map
        let mut color_maps = HashMap::<ExplanationMode, ColorMap>::new();
        color_maps.insert(ExplanationMode::None, ColorMap::default());

        // Create the point renderer and insert the points
        let nn_distance = point_container.find_average_nearest_neighbor_distance();
        let mut point_renderer = PointRenderer2D::new(4.0, nn_distance);

        for p in point_container.point_data.iter() {
            point_renderer.push(p.low, ColorMap::default_color());
        }

        VisualizationState2D {
            camera: VisualizationState2D::get_default_camera(),
            point_container,
            renderer: point_renderer,
            color_maps,
            explanation: ExplanationMode::None,
        }
    }

    // Reload all the points in the renderer using the current rendering mode
    fn reload_renderer_colors(&mut self) {
        // Clear all points and colors from the render
        self.renderer.clear();
        // Get the current color map
        let color_map = self.get_current_color_map();
        // Add every point back to the renderer with the correct data.
        let points_x_colors = self
            .point_container
            .point_data
            .iter()
            .map(|point_data| {
                let color = match self.explanation {
                    ExplanationMode::None => ColorMap::default_color(),
                    // Attribute based techniques
                    ExplanationMode::DaSilva(DaSilvaType::Variance) => {
                        let explanation: DaSilvaExplanation = point_data.silva_var.unwrap();
                        color_map.get_color(explanation.attribute_index, explanation.confidence)
                    }
                    ExplanationMode::DaSilva(DaSilvaType::Euclidean) => {
                        let explanation: DaSilvaExplanation = point_data.silva_euclidean.unwrap();
                        color_map.get_color(explanation.attribute_index, explanation.confidence)
                    }
                    ExplanationMode::DaSilva(DaSilvaType::VarianceSingle(attr)) => {
                        let conf = point_data.silva_single[&attr];
                        color_map.get_color(attr, conf)
                    }
                    ExplanationMode::DaSilva(DaSilvaType::EuclideanSingle(attr)) => {
                        let conf = point_data.silva_single[&attr];
                        color_map.get_color(attr, conf)
                    }
                    // Dimensionality based techniques
                    ExplanationMode::VanDriel(VanDrielType::MinimalVariance) => {
                        let explanation: VanDrielExplanation = point_data.driel_min.unwrap();
                        color_map.get_color(explanation.dimension, explanation.confidence)
                    }
                    ExplanationMode::VanDriel(VanDrielType::TotalVariance) => {
                        let explanation: VanDrielExplanation = point_data.driel_total.unwrap();
                        color_map.get_color(explanation.dimension, explanation.confidence)
                    }
                    ExplanationMode::Normal => {
                        panic!("Normals are never present in 2D Point containers")
                    }
                };
                (point_data.low, color)
            })
            .collect::<Vec<(Point2<f32>, Point3<f32>)>>();

        for (p, c) in points_x_colors {
            self.renderer.push(p, c);
        }
    }

    // TODO: Get a good camera that just views all the points
    pub fn get_default_camera() -> Sidescroll {
        let mut cam = Sidescroll::new();
        cam.set_zoom(16.0);
        cam.set_zoom_step(2.7);
        cam
    }
}

impl VisualizationStateInteraction for VisualizationState2D {
    /// Retrieve the current renderer used to show the points
    fn current_render_mode(&self) -> &dyn PointRendererInteraction {
        &self.renderer
    }

    /// Retrieve the current renderer used to show the points as mutable
    fn current_render_mode_mut(&mut self) -> &mut dyn PointRendererInteraction {
        &mut self.renderer
    }

    /// Reset the camera view of the current rendering mode
    fn reset_camera(&mut self) {
        self.camera = Self::get_default_camera()
    }

    /// Run the explanation for this state and load it.
    fn run_explanation_mode(
        &mut self,
        mode: ExplanationMode,
        neighborhood_size: exp::Neighborhood,
        theta: Option<f32>,
    ) {
        // Remove the entries that already exist before computing the new entry.
        self.explanation = ExplanationMode::None;
        self.color_maps.retain(|&k, _| match (k, mode) {
            // If the explanation is the same as it was before we remove it.
            (existing, new) if existing == new => false,
            // Remove the single explanations if we recompute the complete images
            (
                ExplanationMode::DaSilva(DaSilvaType::VarianceSingle(_)),
                ExplanationMode::DaSilva(DaSilvaType::Variance),
            ) => false,
            (
                ExplanationMode::DaSilva(DaSilvaType::EuclideanSingle(_)),
                ExplanationMode::DaSilva(DaSilvaType::Euclidean),
            ) => false,
            // All other cases should be preserved
            (_, _) => true,
        });

        match (mode, theta) {
            (ExplanationMode::DaSilva(method), _) => {
                let da_silva_explanation =
                    exp::run_da_silva_2d(&self.point_container, neighborhood_size, method);
                self.load(da_silva_explanation, method);
                self.set_explanation_mode(mode);
            }
            (ExplanationMode::VanDriel(method), Some(t)) => {
                let van_driel_explanation =
                    exp::run_van_driel_2d(&self.point_container, neighborhood_size, t, method);
                self.load(van_driel_explanation, method);
                self.set_explanation_mode(mode);
            }
            (ExplanationMode::VanDriel(_), None) => {
                panic!("Tried to compute van driel without passing theta")
            }
            (_, _) => (),
        }
    }

    /// Check if a given explanation mode is already loaded for the state
    fn is_explanation_available(&self, mode: &ExplanationMode) -> bool {
        self.color_maps.contains_key(mode)
    }

    /// Get the current explanation mode
    fn get_explanation_mode(&self) -> ExplanationMode {
        self.explanation
    }
    /// Set the explanation mode and reload the points in the renderer using
    /// the correct coloring mode.
    fn set_explanation_mode(&mut self, mode: ExplanationMode) -> bool {
        if self.is_explanation_available(&mode) {
            self.explanation = mode;
            self.reload_renderer_colors();
            true
        } else {
            eprintln!("Color map for {} is not yet loaded", mode.to_string());
            false
        }
    }

    /// Get the color map that is currently in use
    fn get_current_color_map(&self) -> &ColorMap {
        let map: Option<&ColorMap> = self.color_maps.get(&self.explanation);
        match map {
            Some(m) => m,
            None => panic!(
                "There is no color map for the current explanation mode, this should never happen"
            ),
        }
    }
    /// Set the confidence bounds on the current color map
    fn set_color_map_confidence_bounds(&mut self, min: f32, max: f32) {
        if let Some(map) = self.color_maps.get_mut(&self.get_explanation_mode()) {
            map.set_actual_confidence_bounds(min, max);
            self.reload_renderer_colors();
        };
    }
    /// Toggle the confidence normalization in the current color map
    fn toggle_color_map_confidence_normalization(&mut self) {
        if let Some(map) = self.color_maps.get_mut(&self.get_explanation_mode()) {
            map.toggle_confidence_normalisation();
            self.reload_renderer_colors();
        };
    }

    /// Set an override to from rank to a dimension in the current color map
    fn set_rank_dimension_override(&mut self, rank: usize, dimension: usize) {
        if let Some(map) = self.color_maps.get_mut(&self.get_explanation_mode()) {
            map.set_rank_override(rank, dimension);
            self.reload_renderer_colors();
        };
    }

    /// Reset the overrides made for the current color map
    fn reset_rank_overrides(&mut self) {
        if let Some(map) = self.color_maps.get_mut(&self.get_explanation_mode()) {
            map.clear_rank_overrides();
            self.reload_renderer_colors();
        };
    }

    /// Get the point count of the state
    fn get_point_count(&self) -> usize {
        self.point_container.get_point_count()
    }

    /// Scale the current camera step size
    fn scale_camera_step(&mut self, scale: f32) {
        self.camera.set_zoom_step(self.camera.zoom_step() * scale);
    }

    /// Get the tooltip for the point closest to the cursor position
    fn get_point_tooltip(
        &self,
        cursor_x: f32,
        cursor_y: f32,
        window_size: na::Vector2<f32>,
    ) -> Option<UIPointData> {
        let screen_pos = Point2::<f32>::new(cursor_x, cursor_y);
        let world_pos = self.camera.unproject(&screen_pos, &window_size);
        self.point_container
            .get_closest_point(world_pos.x, world_pos.y)
            .and_then(|point| Some(UIPointData::from(point)))
    }
}

impl Load<Vec<DaSilvaExplanation>, DaSilvaType> for VisualizationState2D {
    fn load(&mut self, explanations: Vec<DaSilvaExplanation>, mode: DaSilvaType) {
        // Create the color map
        let color_map = match mode {
            DaSilvaType::Euclidean | DaSilvaType::Variance => ColorMap::from(&explanations),
            DaSilvaType::EuclideanSingle(_) | DaSilvaType::VarianceSingle(_) => {
                let conf_values = &explanations
                    .iter()
                    .map(|e| e.confidence)
                    .collect::<Vec<f32>>();
                ColorMap::from(conf_values)
            }
        };

        self.color_maps
            .insert(ExplanationMode::DaSilva(mode), color_map);

        self.point_container.load(explanations, mode);

        self.set_explanation_mode(ExplanationMode::DaSilva(mode));
    }
}

impl Load<Vec<VanDrielExplanation>, VanDrielType> for VisualizationState2D {
    fn load(&mut self, explanations: Vec<VanDrielExplanation>, mode: VanDrielType) {
        // Create the color map
        let color_map = ColorMap::from(&explanations);
        self.color_maps
            .insert(ExplanationMode::VanDriel(mode), color_map);
        self.point_container.load(explanations, mode);

        self.set_explanation_mode(ExplanationMode::VanDriel(mode));
    }
}
