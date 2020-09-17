extern crate kiss3d;
extern crate nalgebra as na;

use std::collections::HashMap;

// Third party
use crate::view::RenderMode;
use kiss3d::{
    camera::{ArcBall, Camera},
    light::Light,
    planar_camera::{PlanarCamera, Sidescroll},
    post_processing::PostProcessingEffect,
    renderer::{PlanarRenderer, Renderer},
};
use na::{Point2, Point3};
use rstar::{PointDistance, RTree};

// First party
use crate::{
    exp::{
        common::{
            AnnotatedPoint, IndexedPoint2D, IndexedPoint3D, RTreeParameters2D, RTreeParameters3D,
        },
        da_silva::DaSilvaExplanation,
        driel::VanDrielExplanation,
    },
    util::types::PointN,
    view::{
        color_map::ColorMap,
        point_renderer_2d::PointRenderer2D,
        point_renderer_3d::PointRenderer3D,
        ui::{draw_overlay, WidgetId},
        ExplanationMode,
    },
};

pub trait Load<T> {
    /// Load data into the visualization state
    fn load(&mut self, _: T);
}

pub struct VisualizationState3D {
    // Camera used by this view. : Create custom camera .
    pub camera: ArcBall,
    // Useful when searching points that have been selected or clicked on.
    pub tree: RTree<AnnotatedPoint<IndexedPoint3D>, RTreeParameters3D>,
    // Used for rendering points.
    pub renderer: PointRenderer3D,
    // color map used by the 3D visualizer
    pub color_maps: HashMap<ExplanationMode, ColorMap>,
    // Explanation being viewed at this moment
    explanation: ExplanationMode,
}

impl VisualizationState3D {
    /// Create the visualizer with actual data.
    pub fn new(points: Vec<Point3<f32>>) -> VisualizationState3D {
        // Create the tree
        let annotated_points: Vec<AnnotatedPoint<IndexedPoint3D>> = points
            .iter()
            .enumerate()
            .map(|(index, point)| {
                let point = IndexedPoint3D {
                    index,
                    x: point.x,
                    y: point.y,
                    z: point.z,
                };
                AnnotatedPoint::<IndexedPoint3D> {
                    point,
                    da_silva: None,
                    van_driel: None,
                }
            })
            .collect();
        let rtree =
            RTree::<AnnotatedPoint<IndexedPoint3D>, RTreeParameters3D>::bulk_load_with_params(
                annotated_points,
            );

        // Create the colour map
        let mut color_maps = HashMap::<ExplanationMode, ColorMap>::new();
        color_maps.insert(ExplanationMode::None, ColorMap::default());

        // Create the renderer and add all the points:
        let nn_distance = VisualizationState3D::find_average_nearest_neighbor_distance(&rtree);
        let mut point_renderer = PointRenderer3D::new(4.0, nn_distance);
        for p in points {
            point_renderer.push(p, ColorMap::default_color());
        }

        VisualizationState3D {
            camera: VisualizationState3D::get_default_camera(),
            tree: rtree,
            renderer: point_renderer,
            color_maps: color_maps,
            explanation: ExplanationMode::None,
        }
    }

    /// Get a reference to the color map that is currently being displayed
    pub fn get_colour_map(&self) -> &ColorMap {
        let map: Option<&ColorMap> = self.color_maps.get(&self.explanation);
        match map {
            Some(m) => m,
            None => panic!(
                "There is no color map for the current explanation mode, this should never happen"
            ),
        }
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

    // Reload all the points in the renderer using the current rendering mode
    fn reload_renderer_colors(&mut self) {
        // Clear all points and colors from the render
        self.renderer.clear();
        // Get the current color map
        let color_map = self.get_colour_map();
        // Add every point back to the renderer with the correct data.
        let points_x_colors = self
            .tree
            .iter()
            .map(|annotated_point| {
                let color = match self.explanation {
                    ExplanationMode::None => ColorMap::default_color(),
                    ExplanationMode::DaSilva => {
                        let explanation: DaSilvaExplanation = annotated_point.da_silva.unwrap();
                        color_map.get_color(explanation.attribute_index, explanation.confidence)
                    }
                    ExplanationMode::VanDriel => {
                        let explanation: VanDrielExplanation = annotated_point.van_driel.unwrap();
                        color_map.get_color(explanation.dimension, explanation.confidence)
                    }
                };
                (annotated_point.point.into(), color)
            })
            .collect::<Vec<(Point3<f32>, Point3<f32>)>>();

        for (p, c) in points_x_colors {
            self.renderer.push(p, c);
        }
    }

    pub fn get_default_camera() -> ArcBall {
        // Create arcball camera with custom FOV.
        let eye = Point3::new(0.0f32, 0.0, -1.5);
        let at = Point3::new(0.0f32, 0.0f32, 0.0f32);
        let mut cam = ArcBall::new_with_frustrum(std::f32::consts::PI / 3.0, 0.01, 1024.0, eye, at);
        cam.set_dist_step(10.0);
        cam
    }

    fn find_average_nearest_neighbor_distance(
        tree: &RTree<AnnotatedPoint<IndexedPoint3D>, RTreeParameters3D>,
    ) -> f32 {
        let mut res = Vec::<f32>::new();
        for query_point in tree.iter() {
            // Get the second nearest neighbor from the query point, the first will be itself.
            let &nn = tree
                .nearest_neighbor_iter(&[
                    query_point.point.x,
                    query_point.point.y,
                    query_point.point.z,
                ])
                .take(2)
                .skip(1)
                .collect::<Vec<&AnnotatedPoint<IndexedPoint3D>>>()
                .first()
                .expect("Could not get nearest neighbor");

            let dist = query_point.distance_2(&[nn.point.x, nn.point.y, nn.point.z]);
            res.push(dist.sqrt());
        }
        let average = res.iter().sum::<f32>() / (res.len() as f32);
        // We draw the blob within a square, to ensure the drawn blob has radius of nn_distance we need to correct it.
        (average.powi(2) * 2.0).sqrt()
    }
}

impl Load<Vec<DaSilvaExplanation>> for VisualizationState3D {
    fn load(&mut self, explanations: Vec<DaSilvaExplanation>) {
        // Create the colour map
        let color_map = ColorMap::from(&explanations);
        self.color_maps.insert(ExplanationMode::DaSilva, color_map);

        // TODO: This assert is just for testing, this should be caught earlier on and have a documented exit code
        assert!(self.tree.size() == explanations.len());

        // Add the annotations to all the points in the search tree
        for ap in self.tree.iter_mut() {
            ap.da_silva = Some(explanations[ap.point.index]);
        }

        self.set_explanation_mode(ExplanationMode::DaSilva);
    }
}

impl Load<Vec<VanDrielExplanation>> for VisualizationState3D {
    fn load(&mut self, explanations: Vec<VanDrielExplanation>) {
        // Create the colour map
        let color_map = ColorMap::from(&explanations);
        self.color_maps.insert(ExplanationMode::VanDriel, color_map);

        // TODO: This assert is just for testing, this should be caught earlier on and have a documented exit code
        assert!(self.tree.size() == explanations.len());

        // Add the annotations to all the points in the search tree
        for ap in self.tree.iter_mut() {
            ap.van_driel = Some(explanations[ap.point.index]);
        }

        self.set_explanation_mode(ExplanationMode::VanDriel);
    }
}

pub struct VisualizationState2D {
    // Camera used by this view.
    pub camera: Sidescroll,
    // Useful when searching points that have been selected or clicked on.
    pub tree: RTree<AnnotatedPoint<IndexedPoint2D>, RTreeParameters2D>,
    // Used for rendering points.
    pub renderer: PointRenderer2D,
    // color map used by the 2D visualizer
    pub color_maps: HashMap<ExplanationMode, ColorMap>,
    // Explanation being viewed at this moment
    explanation: ExplanationMode,
}

impl VisualizationState2D {
    pub fn new(
        points: Vec<Point2<f32>>,
        // explanations: Vec<DaSilvaExplanation>,
    ) -> VisualizationState2D {
        let annotated_points: Vec<AnnotatedPoint<IndexedPoint2D>> = points
            .iter()
            .enumerate()
            .map(|(index, point)| {
                let point = IndexedPoint2D {
                    index,
                    x: point.x,
                    y: point.y,
                };
                AnnotatedPoint::<IndexedPoint2D> {
                    point: point,
                    da_silva: None,
                    van_driel: None,
                }
            })
            .collect();

        // Initialize the search tree
        let rtree =
            RTree::<AnnotatedPoint<IndexedPoint2D>, RTreeParameters2D>::bulk_load_with_params(
                annotated_points,
            );

        // Create the colour map
        let mut color_maps = HashMap::<ExplanationMode, ColorMap>::new();
        color_maps.insert(ExplanationMode::None, ColorMap::default());

        // Create the point renderer and insert the points
        let nn_distance = VisualizationState2D::find_average_nearest_neighbor_distance(&rtree);
        let mut point_renderer = PointRenderer2D::new(4.0, nn_distance);

        for p in points {
            point_renderer.push(p, ColorMap::default_color());
        }

        VisualizationState2D {
            camera: VisualizationState2D::get_default_camera(),
            tree: rtree,
            renderer: point_renderer,
            color_maps: color_maps,
            explanation: ExplanationMode::None,
        }
    }

    fn find_average_nearest_neighbor_distance(
        tree: &RTree<AnnotatedPoint<IndexedPoint2D>, RTreeParameters2D>,
    ) -> f32 {
        let mut res = Vec::<f32>::new();
        for query_point in tree.iter() {
            // Get the second nearest neighbor from the query point, the first will be itself.
            let &nn = tree
                .nearest_neighbor_iter(&[query_point.point.x, query_point.point.y])
                .take(2)
                .skip(1)
                .collect::<Vec<&AnnotatedPoint<IndexedPoint2D>>>()
                .first()
                .expect("Could not get nearest neighbor");

            let dist = query_point.distance_2(&[nn.point.x, nn.point.y]);
            res.push(dist.sqrt());
        }
        let average = res.iter().sum::<f32>() / (res.len() as f32);
        // We draw the blob within a square, to ensure the drawn blob has radius of nn_distance we need to correct it.
        (average.powi(2) * 2.0).sqrt()
    }

    /// Get a reference to the color map that is currently being displayed
    pub fn get_colour_map(&self) -> &ColorMap {
        let map: Option<&ColorMap> = self.color_maps.get(&self.explanation);
        match map {
            Some(m) => m,
            None => panic!(
                "There is no color map for the current explanation mode, this should never happen"
            ),
        }
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

    // Reload all the points in the renderer using the current rendering mode
    fn reload_renderer_colors(&mut self) {
        // Clear all points and colors from the render
        self.renderer.clear();
        // Get the current color map
        let color_map = self.get_colour_map();
        // Add every point back to the renderer with the correct data.
        let points_x_colors = self
            .tree
            .iter()
            .map(|annotated_point| {
                let color = match self.explanation {
                    ExplanationMode::None => ColorMap::default_color(),
                    ExplanationMode::DaSilva => {
                        let explanation: DaSilvaExplanation = annotated_point.da_silva.unwrap();
                        color_map.get_color(explanation.attribute_index, explanation.confidence)
                    }
                    ExplanationMode::VanDriel => {
                        let explanation: VanDrielExplanation = annotated_point.van_driel.unwrap();
                        color_map.get_color(explanation.dimension, explanation.confidence)
                    }
                };
                (annotated_point.point.into(), color)
            })
            .collect::<Vec<(Point2<f32>, Point3<f32>)>>();

        for (p, c) in points_x_colors {
            self.renderer.push(p, c);
        }
    }

    // TODO: Get a good camera that just views all the points
    pub fn get_default_camera() -> Sidescroll {
        let mut cam = Sidescroll::new();
        cam.set_zoom(8.0);
        cam.set_zoom_step(2.7);
        cam
    }
}

impl Load<Vec<DaSilvaExplanation>> for VisualizationState2D {
    fn load(&mut self, explanations: Vec<DaSilvaExplanation>) {
        // Create the colour map
        let color_map = ColorMap::from(&explanations);
        self.color_maps.insert(ExplanationMode::DaSilva, color_map);

        // TODO: This assert is just for testing, this should be caught earlier on and have a documented exit code
        assert!(self.tree.size() == explanations.len());

        // Add the annotations to all the points in the search tree
        for ap in self.tree.iter_mut() {
            ap.da_silva = Some(explanations[ap.point.index]);
        }

        self.set_explanation_mode(ExplanationMode::DaSilva);
    }
}

impl Load<Vec<VanDrielExplanation>> for VisualizationState2D {
    fn load(&mut self, explanations: Vec<VanDrielExplanation>) {
        // Create the colour map
        let color_map = ColorMap::from(&explanations);
        self.color_maps.insert(ExplanationMode::VanDriel, color_map);

        // TODO: This assert is just for testing, this should be caught earlier on and have a documented exit code
        assert!(self.tree.size() == explanations.len());

        // Add the annotations to all the points in the search tree
        for ap in self.tree.iter_mut() {
            ap.van_driel = Some(explanations[ap.point.index]);
        }

        self.set_explanation_mode(ExplanationMode::VanDriel);
    }
}
