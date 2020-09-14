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
        common::{IndexedPoint2D, IndexedPoint3D, AnnotatedPoint, RTreeParameters2D, RTreeParameters3D},
        da_silva::DaSilvaExplanation,
    },
    util::types::PointN,
    view::{
        ExplanationMode,
        color_map::ColorMap,
        point_renderer_2d::PointRenderer2D,
        point_renderer_3d::PointRenderer3D,
        ui::{draw_overlay, WidgetId},
    },
};

pub struct VisualizationState3D {
    // Camera used by this view. : Create custom camera .
    pub camera: ArcBall,
    // Useful when searching points that have been selected or clicked on.
    pub tree: RTree<AnnotatedPoint<IndexedPoint3D>, RTreeParameters3D>,
    // Amount of dimensions in the original data
    pub dimension_count: usize,
    // Used for rendering points.
    pub renderer: PointRenderer3D,
    // color map used by the 3D visualizer
    pub color_maps: HashMap<ExplanationMode, ColorMap>,
    // Explanation being viewed at this moment
    pub explanation: ExplanationMode
}

impl VisualizationState3D {
    /// Create the visualizer with actual data.
    pub fn new(
        points: Vec<Point3<f32>>,
        original_points: &Vec<PointN>,
    ) -> VisualizationState3D {
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
                AnnotatedPoint::<IndexedPoint3D> { point, da_silva: None, van_driel: None}
            }
            )
            .collect();
        let rtree =
            RTree::<AnnotatedPoint::<IndexedPoint3D>, RTreeParameters3D>::bulk_load_with_params(annotated_points);

        let nn_distance = IndexedPoint3D::find_average_nearest_neighbor_distance(&rtree);
        // We draw the blob within a square, to ensure the drawn blob has radius of nn_distance we need to correct it.
        let corrected_distance = (nn_distance.powi(2) * 2.0).sqrt();

        // Create the colour map
        let dimension_count = original_points.first().unwrap().len();
        let color_maps = HashMap::<ExplanationMode, ColorMap>::new();
        color_maps.insert(ExplanationMode::None, ColorMap::new_dummy());

        // Create the renderer and add all the points:
        let mut point_renderer = PointRenderer3D::new(4.0, corrected_distance);
        for &p in points.iter() {
            point_renderer.push(p, ColorMap::default_color());
        }

        VisualizationState3D {
            camera: VisualizationState3D::get_default_camera(),
            tree: rtree,
            renderer: point_renderer,
            dimension_count: dimension_count,
            color_maps: color_maps,
            explanation: ExplanationMode::None
        }
    }

    pub fn load_explanations(&mut self, explanations: Vec<DaSilvaExplanation>) {
        // Create the colour map
        let color_map = ColorMap::from_da_silva(&explanations, self.dimension_count);
        self.color_maps.insert(ExplanationMode::DaSilva, color_map);

        // TODO: This assert is just for testing, this should be caught earlier on and have a documented exit code
        assert!(self.tree.size() == explanations.len());

        // Add the annotations to all the points in the search tree
        self.tree.iter_mut().map(|ap| {
            ap.da_silva = explanations[ap.point.index];
        });
    }

    // Set the explanation mode and reload the points in the renderer using the correct coloring mode.
    fn set_explanation_mode(&self, mode: ExplanationMode) -> bool {

    }

    // Reload all the points in the renderer using the current rendering mode
    fn reload_renderer(&mut self){

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

pub struct VisualizationState2D {
    // Camera used by this view.
    pub camera: Sidescroll,
    // Useful when searching points that have been selected or clicked on.
    pub tree: RTree<IndexedPoint2D, RTreeParameters2D>,
    // Used for rendering points. TODO BUILD THIS
    pub renderer: PointRenderer2D,
    // color map used by the 3D visualizer
    pub color_map: ColorMap,
}

impl VisualizationState2D {
    pub fn new(
        points: Vec<Point2<f32>>,
        explanations: Vec<DaSilvaExplanation>,
        original_points: &Vec<PointN>,
    ) -> VisualizationState2D {
        let indexed_points: Vec<IndexedPoint2D> = points
            .iter()
            .enumerate()
            .map(|(index, point)| IndexedPoint2D {
                index,
                x: point.x,
                y: point.y,
            })
            .collect();

        // Initialize the search tree
        let rtree =
            RTree::<IndexedPoint2D, RTreeParameters2D>::bulk_load_with_params(indexed_points);

        // Find the blob size based on the average first nearest neighbor distance
        // We draw the blob within a square, to ensure the drawn blob has radius of nn_distance we need to correct it.
        let nn_distance = IndexedPoint2D::find_average_nearest_neightbor_distance(&rtree);
        let corrected_distance = (nn_distance * nn_distance * 2.0).sqrt();

        // Create the colour map
        let dimension_count = original_points.first().unwrap().len();
        let color_map = ColorMap::from_explanations(&explanations, dimension_count);

        // Create the point renderer and insert the points
        let mut point_renderer = PointRenderer2D::new(4.0, corrected_distance);

        for (&p, e) in points.iter().zip(explanations) {
            let color = color_map.get_color(e.attribute_index, e.confidence);
            point_renderer.push(p, color);
        }

        VisualizationState2D {
            camera: VisualizationState2D::get_default_camera(),
            tree: rtree,
            renderer: point_renderer,
            color_map: color_map,
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
