//! Module containing the implementations required to store indexed 2d/3d
//! points in an r*-tree.

/// Third party imports
use rstar;

/// First party imports
use super::data::IndexedPoint;

/// Trait that allows use to store the annotated 2d points in an rtree
impl rstar::RTreeObject for IndexedPoint<na::Point2<f32>> {
    type Envelope = rstar::AABB<[f32; 2]>;
    fn envelope(&self) -> Self::Envelope {
        rstar::AABB::from_point([self.point.x, self.point.y])
    }
}

/// Trait that allows use to store the annotated 3d points in an rtree
impl rstar::RTreeObject for IndexedPoint<na::Point3<f32>> {
    type Envelope = rstar::AABB<[f32; 3]>;
    fn envelope(&self) -> Self::Envelope {
        rstar::AABB::from_point([self.point.x, self.point.y, self.point.z])
    }
}

/// Trait used for fast distance calculations between annotated 2d points
impl rstar::PointDistance for IndexedPoint<na::Point2<f32>> {
    fn distance_2(&self, point: &[f32; 2]) -> f32 {
        let x: f32 = point[0] - self.point.x;
        let y: f32 = point[1] - self.point.y;
        x.powi(2) + y.powi(2)
    }

    fn contains_point(&self, point: &[f32; 2]) -> bool {
        // Exact floating point comparisons can cause trouble
        // so I introduce a small margin of error
        let error = 1.0e-6;
        (self.point.x - point[0]) < error && (self.point.y - point[1]) < error
    }

    fn distance_2_if_less_or_equal(&self, point: &[f32; 2], max_distance_2: f32) -> Option<f32> {
        let t = self.distance_2(point);
        if t <= max_distance_2 {
            Some(t.sqrt())
        } else {
            None
        }
    }
}

/// Trait used for fast distance calculations between annotated 3d points
impl rstar::PointDistance for IndexedPoint<na::Point3<f32>> {
    fn distance_2(&self, point: &[f32; 3]) -> f32 {
        let x = point[0] - self.point.x;
        let y = point[1] - self.point.y;
        let z = point[2] - self.point.z;
        x.powi(2) + y.powi(2) + z.powi(2)
    }

    fn contains_point(&self, point: &[f32; 3]) -> bool {
        // Exact floating point comparisons can cause trouble
        // so I introduce a small margin of error
        let error = 1.0e-6;
        (self.point.x - point[0]) < error
            && (self.point.y - point[1]) < error
            && (self.point.z - point[2]) < error
    }

    fn distance_2_if_less_or_equal(&self, point: &[f32; 3], max_distance_2: f32) -> Option<f32> {
        let t = self.distance_2(point);
        if t <= max_distance_2 {
            Some(t.sqrt())
        } else {
            None
        }
    }
}
