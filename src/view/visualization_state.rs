/// Module containing the current state of the 2D or 3D renderer.
// Build in imports
use crate::exp::NormalExplanation;
use crate::exp;
use std::collections::HashMap;

// Third party imports
use kiss3d::{camera::ArcBall, planar_camera::Sidescroll};
use na::{Point2, Point3};

// First party imports
use crate::{
    exp::{DaSilvaExplanation, DaSilvaType, VanDrielExplanation, VanDrielType},
    search::{Load, PointContainer, PointContainer2D, PointContainer3D},
    view::{
        color_map::ColorMap,
        point_renderer::{PointRenderer2D, PointRenderer3D},
        ExplanationMode,
    },
};

pub struct VisualizationState3D {
    // Camera used by this view. : Create custom camera .
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

    /// Get a reference to the color map that is currently being displayed
    pub fn get_color_map(&self) -> &ColorMap {
        let map: Option<&ColorMap> = self.color_maps.get(&self.explanation);
        match map {
            Some(m) => m,
            None => panic!(
                "There is no color map for the current explanation mode, this should never happen"
            ),
        }
    }

    pub fn set_color_confidence_bounds(&mut self, min: f32, max: f32) {
        if let Some(map) = self.color_maps.get_mut(&self.get_explanation_mode()) {
            map.set_actual_confidence_bounds(min, max);
        };
        self.reload_renderer_colors();
    }

    pub fn toggle_color_confidence_normalization(&mut self) {
        if let Some(map) = self.color_maps.get_mut(&self.get_explanation_mode()) {
            map.toggle_confidence_normalisation();
        };
        self.reload_renderer_colors();
    }

    /// Check if this render has an explanation mode available
    pub fn is_explanation_available(&self, mode: &ExplanationMode) -> bool {
        self.color_maps.contains_key(mode)
    }

    /// Retrieve the explanation mode currently being displayed
    pub fn get_explanation_mode(&self) -> ExplanationMode {
        self.explanation
    }

    /// Set the explanation mode and reload the points in the renderer using the correct coloring mode.
    pub fn set_explanation_mode(&mut self, mode: ExplanationMode) -> bool {
        if mode == ExplanationMode::Normal {
            panic!("Explanation mode should never be set to only normals, this can only be caused by a programming error");
        }

        if self.is_explanation_available(&mode) {
            self.explanation = mode;
            self.reload_renderer_colors();
            true
        } else {
            eprintln!("Color map for {} is not yet loaded", mode.to_str());
            false
        }
    }

    pub fn run_explanation_mode(
        &mut self,
        mode: ExplanationMode,
        neighborhood_size: exp::Neighborhood,
        theta: Option<f32>,
    ) {
        // render mode is already loaded, first remove it
        if self.is_explanation_available(&mode) {
            self.explanation = ExplanationMode::None;
            self.color_maps.remove(&mode);
        }
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
                let normal_explanation = exp::run_normals_calculation(&self.point_container, neighborhood_size);
                self.load(normal_explanation, ());
                // Insert the default color map so "is available" returns true for normals
                self.color_maps.insert(ExplanationMode::Normal, ColorMap::default());
                self.renderer.set_shading(true);
                self.reload_renderer_colors();
            }
            (ExplanationMode::VanDriel(_), None) => {
                panic!("Tried to compute van driel without passing theta")
            }
            (ExplanationMode::None, _) => (),
        }
    }

    // Reload all the points in the renderer using the current rendering mode
    fn reload_renderer_colors(&mut self) {
        // Clear all points and colors from the render
        self.renderer.clear();
        // Get the current color map
        let color_map = self.get_color_map();
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
                    ExplanationMode::VanDriel(VanDrielType::MinimalVariance) => {
                        let explanation: VanDrielExplanation = point_data.driel_min.unwrap();
                        color_map.get_color(explanation.dimension, explanation.confidence)
                    }
                    ExplanationMode::VanDriel(VanDrielType::TotalVariance) => {
                        let explanation: VanDrielExplanation = point_data.driel_total.unwrap();
                        color_map.get_color(explanation.dimension, explanation.confidence)
                    }
                    ExplanationMode::Normal => panic!("The explanation mode should never be set to only normals")
                };
                let normal = point_data.normal.and_then(|e|{
                    Some(na::Point4::<f32>::new(e.normal.x, e.normal.y, e.normal.z, e.eccentricity))
                });
                (point_data.low, normal, color)
            })
            .collect::<Vec<(Point3<f32>, Option<na::Point4<f32>>, Point3<f32>)>>();

        // If all points have a normal we set the render to use normals.
        // TODO: Setting this here might not be that useful, we might want to disable it later.
        // self.renderer.set_shading(points_x_colors.iter().all(|(_, n, _)| n.is_some()));

        for (p, n, c) in points_x_colors {
            self.renderer.push(p, n,c);
        }

        self.renderer.sync_gpu_vector();
    }

    pub fn get_default_camera() -> ArcBall {
        // Create arcball camera with custom FOV.
        let eye = Point3::new(0.0f32, 0.0, -1.5);
        let at = Point3::new(0.0f32, 0.0f32, 0.0f32);
        let mut cam = ArcBall::new_with_frustrum(std::f32::consts::PI / 3.0, 0.01, 1024.0, eye, at);
        cam.set_dist_step(10.0);
        cam
    }
}

impl Load<Vec<DaSilvaExplanation>, DaSilvaType> for VisualizationState3D {
    fn load(&mut self, explanations: Vec<DaSilvaExplanation>, mode: DaSilvaType) {
        // Create the color map
        let color_map = ColorMap::from(&explanations);
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

    /// Get a reference to the color map that is currently being displayed
    pub fn get_color_map(&self) -> &ColorMap {
        let map: Option<&ColorMap> = self.color_maps.get(&self.explanation);
        match map {
            Some(m) => m,
            None => panic!(
                "There is no color map for the current explanation mode, this should never happen"
            ),
        }
    }

    pub fn set_color_confidence_bounds(&mut self, min: f32, max: f32) {
        if let Some(map) = self.color_maps.get_mut(&self.get_explanation_mode()) {
            map.set_actual_confidence_bounds(min, max);
        };
        self.reload_renderer_colors();
    }

    pub fn toggle_color_confidence_normalization(&mut self) {
        if let Some(map) = self.color_maps.get_mut(&self.get_explanation_mode()) {
            map.toggle_confidence_normalisation();
        };
        self.reload_renderer_colors();
    }

    /// Check if this render has an explanation mode available
    pub fn is_explanation_available(&self, mode: &ExplanationMode) -> bool {
        self.color_maps.contains_key(mode)
    }

    /// Retrieve the explanation mode currently being displayed
    pub fn get_explanation_mode(&self) -> ExplanationMode {
        self.explanation
    }

    /// Set the explanation mode and reload the points in the renderer using the correct coloring mode.
    pub fn set_explanation_mode(&mut self, mode: ExplanationMode) -> bool {
        if self.is_explanation_available(&mode) {
            self.explanation = mode;
            self.reload_renderer_colors();
            true
        } else {
            eprintln!("Color map for {} is not yet loaded", mode.to_str());
            false
        }
    }

    pub fn run_explanation_mode(
        &mut self,
        mode: ExplanationMode,
        neighborhood_size: exp::Neighborhood,
        theta: Option<f32>,
    ) {
        // render mode is already loaded, first remove it
        if self.is_explanation_available(&mode) {
            self.explanation = ExplanationMode::None;
            self.color_maps.remove(&mode);
        }
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

    // Reload all the points in the renderer using the current rendering mode
    fn reload_renderer_colors(&mut self) {
        // Clear all points and colors from the render
        self.renderer.clear();
        // Get the current color map
        let color_map = self.get_color_map();
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
                    ExplanationMode::VanDriel(VanDrielType::MinimalVariance) => {
                        let explanation: VanDrielExplanation = point_data.driel_min.unwrap();
                        color_map.get_color(explanation.dimension, explanation.confidence)
                    }
                    ExplanationMode::VanDriel(VanDrielType::TotalVariance) => {
                        let explanation: VanDrielExplanation = point_data.driel_total.unwrap();
                        color_map.get_color(explanation.dimension, explanation.confidence)
                    }
                    ExplanationMode::Normal => panic!("Normals are never present in 2D Point containers")
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

impl Load<Vec<DaSilvaExplanation>, DaSilvaType> for VisualizationState2D {
    fn load(&mut self, explanations: Vec<DaSilvaExplanation>, mode: DaSilvaType) {
        // Create the color map
        let color_map = ColorMap::from(&explanations);
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
