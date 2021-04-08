/// Module containing the state of the 3D scene

/// Build in imports
use std::collections::HashMap;

/// Third party imports
use kiss3d::camera::{ArcBall, Camera};
use na::Point3;

use super::VisualizationStateInteraction;
/// First party imports
use crate::{
    exp,
    exp::{DaSilvaExplanation, DaSilvaType, NormalExplanation, VanDrielExplanation, VanDrielType},
    search::{Load, PointContainer, PointContainer3D, UIPointData},
    view::{point_renderer::PointRenderer3D, ColorMap, ExplanationMode, PointRendererInteraction},
};

/// The state of the 3D scene
pub struct VisualizationState3D {
    // Camera used by this view.
    pub camera: ArcBall,
    // Data container with all the points.
    pub point_container: PointContainer3D,
    // Used for rendering points.
    pub renderer: PointRenderer3D,
    // color map used by the 3D visualizer
    pub color_maps: HashMap<ExplanationMode, ColorMap>,
    // Explanation being viewed at this moment
    explanation: ExplanationMode,
}

impl VisualizationState3D {
    /// Create the visualizer with actual data.
    pub fn new(point_container: PointContainer3D) -> VisualizationState3D {
        // Create the color map
        let mut color_maps = HashMap::<ExplanationMode, ColorMap>::new();
        color_maps.insert(ExplanationMode::None, ColorMap::default());

        // Create the renderer and add all the points:
        let nn_distance = point_container.find_average_nearest_neighbor_distance();
        let mut point_renderer = PointRenderer3D::new(4.0, nn_distance);
        // This might not be needed when computing the normals
        for p in point_container.point_data.iter() {
            point_renderer.push(p.low, None, ColorMap::default_color());
        }
        point_renderer.sync_gpu_vector();

        VisualizationState3D {
            camera: VisualizationState3D::get_default_camera(),
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
                    ExplanationMode::VanDriel(VanDrielType::MinimalVariance) => {
                        let explanation: VanDrielExplanation = point_data.driel_min.unwrap();
                        color_map.get_color(explanation.dimension, explanation.confidence)
                    }
                    ExplanationMode::VanDriel(VanDrielType::TotalVariance) => {
                        let explanation: VanDrielExplanation = point_data.driel_total.unwrap();
                        color_map.get_color(explanation.dimension, explanation.confidence)
                    }
                    ExplanationMode::Normal => {
                        panic!("The explanation mode should never be set to only normals")
                    }
                };
                let normal = point_data.normal.and_then(|e| {
                    Some(na::Point4::<f32>::new(
                        e.normal.x,
                        e.normal.y,
                        e.normal.z,
                        e.eccentricity,
                    ))
                });
                (point_data.low, normal, color)
            })
            .collect::<Vec<(Point3<f32>, Option<na::Point4<f32>>, Point3<f32>)>>();

        // If all points have a normal we set the render to use normals.
        // TODO: Setting this here might not be that useful, we might want to disable it later.
        // self.renderer.set_shading(points_x_colors.iter().all(|(_, n, _)| n.is_some()));

        for (p, n, c) in points_x_colors {
            self.renderer.push(p, n, c);
        }

        self.renderer.sync_gpu_vector();
    }

    /// Disable the shading, 3D state specific.
    pub fn disable_shading(&mut self) {
        self.renderer.set_shading(false);
        self.color_maps.remove(&ExplanationMode::Normal);
        self.reload_renderer_colors();
    }

    /// Return the default camera for this state
    pub fn get_default_camera() -> ArcBall {
        // Create arcball camera with custom FOV.
        let eye = Point3::new(0.0f32, 0.0, -1.5);
        let at = Point3::new(0.0f32, 0.0f32, 0.0f32);
        let mut cam = ArcBall::new_with_frustrum(std::f32::consts::PI / 3.0, 0.01, 1024.0, eye, at);
        cam.set_dist_step(10.0);
        cam
    }
}

impl VisualizationStateInteraction for VisualizationState3D {
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
        self.camera = Self::get_default_camera();
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
                    exp::run_da_silva_3d(&self.point_container, neighborhood_size, method);
                self.load(da_silva_explanation, method);
                self.set_explanation_mode(mode);
            }
            (ExplanationMode::VanDriel(method), Some(t)) => {
                let van_driel_explanation =
                    exp::run_van_driel_3d(&self.point_container, neighborhood_size, t, method);
                self.load(van_driel_explanation, method);
                self.set_explanation_mode(mode);
            }
            (ExplanationMode::Normal, _) => {
                let normal_explanation =
                    exp::run_normals_calculation(&self.point_container, neighborhood_size);
                self.load(normal_explanation, ());
                // Insert the default color map so "is available" returns true for normals
                self.color_maps
                    .insert(ExplanationMode::Normal, ColorMap::default());
                self.renderer.set_shading(true);
                self.reload_renderer_colors();
            }
            (ExplanationMode::VanDriel(_), None) => {
                panic!("Tried to compute van driel without passing theta")
            }
            (ExplanationMode::None, _) => (),
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

    /// Set the explanation mode of the state to `mode`
    fn set_explanation_mode(&mut self, mode: ExplanationMode) -> bool {
        if mode == ExplanationMode::Normal {
            panic!("Explanation mode should never be set to only normals, this can only be caused by a programming error");
        }

        if self.is_explanation_available(&mode) {
            self.explanation = mode;
            self.reload_renderer_colors();
            true
        } else {
            eprintln!("Color map for {} is not yet loaded", mode.to_string());
            false
        }
    }

    /// Get a reference to the color map that is currently being displayed
    fn get_current_color_map(&self) -> &ColorMap {
        let map: Option<&ColorMap> = self.color_maps.get(&self.explanation);
        match map {
            Some(m) => m,
            None => panic!(
                "There is no color map for the current explanation mode, this should never happen"
            ),
        }
    }

    /// Set the confidence bounds  on the color map currently being used.
    fn set_color_map_confidence_bounds(&mut self, min: f32, max: f32) {
        if let Some(map) = self.color_maps.get_mut(&self.get_explanation_mode()) {
            map.set_actual_confidence_bounds(min, max);
        };
        self.reload_renderer_colors();
    }

    /// Toggle the confidence normalization in the current color map
    fn toggle_color_map_confidence_normalization(&mut self) {
        if let Some(map) = self.color_maps.get_mut(&self.get_explanation_mode()) {
            map.toggle_confidence_normalisation();
        };
        self.reload_renderer_colors();
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
        self.camera.set_dist_step(self.camera.dist_step() * scale);
    }

    /// Get the tooltip for the point closest to the cursor position
    fn get_point_tooltip(
        &self,
        cursor_x: f32,
        cursor_y: f32,
        window_size: na::Vector2<f32>,
    ) -> Option<UIPointData> {
        let screen_pos = na::Point2::<f32>::new(cursor_x, cursor_y);
        let (ray_origin, ray_direction): (na::Point3<f32>, na::Vector3<f32>) =
            self.camera.unproject(&screen_pos, &window_size);
        self.point_container
            .get_closest_point(ray_origin, ray_direction)
            .and_then(|point| Some(UIPointData::from(point)))
    }
}

impl Load<Vec<DaSilvaExplanation>, DaSilvaType> for VisualizationState3D {
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

impl Load<Vec<VanDrielExplanation>, VanDrielType> for VisualizationState3D {
    fn load(&mut self, explanations: Vec<VanDrielExplanation>, mode: VanDrielType) {
        // Create the color map
        let color_map = ColorMap::from(&explanations);
        self.color_maps
            .insert(ExplanationMode::VanDriel(mode), color_map);
        self.point_container.load(explanations, mode);
        self.set_explanation_mode(ExplanationMode::VanDriel(mode));
    }
}

impl Load<Vec<NormalExplanation>, ()> for VisualizationState3D {
    fn load(&mut self, explanations: Vec<NormalExplanation>, _: ()) {
        self.point_container.load(explanations, ());
    }
}
