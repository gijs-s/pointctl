extern crate kiss3d;
extern crate nalgebra as na;

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
        common::{IndexedPoint2D, IndexedPoint3D, RTreeParameters2D, RTreeParameters3D},
        da_silva::DaSilvaExplanation,
    },
    util::types::PointN,
    view::{
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
    pub tree: RTree<IndexedPoint3D, RTreeParameters3D>,
    // Used for rendering points.
    pub renderer: PointRenderer3D,
    // color map used by the 3D visualizer
    pub color_map: ColorMap,
}

impl VisualizationState3D {
    pub fn new(
        points: Vec<Point3<f32>>,
        explanations: Vec<DaSilvaExplanation>,
        color_map: ColorMap,
    ) -> VisualizationState3D {
        // Create the tree
        let indexed_points: Vec<IndexedPoint3D> = points
            .iter()
            .enumerate()
            .map(|(index, point)| IndexedPoint3D {
                index,
                x: point.x,
                y: point.y,
                z: point.z,
            })
            .collect();
        let rtree =
            RTree::<IndexedPoint3D, RTreeParameters3D>::bulk_load_with_params(indexed_points);

        // Create the renderer and add all the points:
        let mut point_renderer = PointRenderer3D::new();
        for (&p, e) in points.iter().zip(explanations) {
            let color = color_map.get_color(e.attribute_index, e.confidence);
            point_renderer.push(p, color);
        }


        let nn_distance = IndexedPoint3D::find_average_nearest_neightbor_distance(&rtree);
        // point_renderer.set_blob_size((nn_distance.powi(2) * 2.0).sqrt());
        point_renderer.set_blob_size(nn_distance);


        VisualizationState3D {
            camera: VisualizationState3D::get_default_camera(),
            tree: rtree,
            renderer: point_renderer,
            color_map: color_map,
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
    // Used to denote if the 2d data is present. if not than this state will be empty
    pub initialized: bool,
}

impl VisualizationState2D {
    pub fn new_empty() -> VisualizationState2D {
        VisualizationState2D {
            camera: VisualizationState2D::get_default_camera(),
            tree: RTree::<IndexedPoint2D, RTreeParameters2D>::new_with_params(),
            renderer: PointRenderer2D::new(),
            color_map: ColorMap::new_dummy(),
            initialized: false,
        }
    }

    pub fn new(
        points: Vec<Point2<f32>>,
        explanations: Vec<DaSilvaExplanation>,
        color_map: ColorMap,
    ) -> VisualizationState2D {
        let mut state = VisualizationState2D::new_empty();
        state.initialize(points, explanations, color_map);
        state
    }

    pub fn initialize(
        &mut self,
        points: Vec<Point2<f32>>,
        explanations: Vec<DaSilvaExplanation>,
        color_map: ColorMap,
    ) {
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
        self.tree =
            RTree::<IndexedPoint2D, RTreeParameters2D>::bulk_load_with_params(indexed_points);

        // Initialize the color map
        self.color_map = ColorMap::from_explanations(&explanations, 30);

        // Ensure the renderer is empty.
        self.renderer.clear();

        // Then add all the points.
        for (&p, e) in points.iter().zip(explanations) {
            let color = color_map.get_color(e.attribute_index, e.confidence);
            self.renderer.push(p, color);
        }

        // TODO: Is this even correct?
        let nn_distance = IndexedPoint2D::find_average_nearest_neightbor_distance(&self.tree);
        self.renderer.set_blob_size((nn_distance.powi(2) * 2.0).sqrt());

        self.initialized = true;
    }

    // TODO: Get a good camera that just views all the points
    pub fn get_default_camera() -> Sidescroll {
        let mut cam = Sidescroll::new();
        cam.set_zoom(8.0);
        cam.set_zoom_step(2.7);
        cam
    }
}