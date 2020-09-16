// Internal module file used to define the common interface with all the explanation mechanisms and datatypes.
extern crate nalgebra as na;

use rstar;
use rstar::{PointDistance, RTree, RTreeObject, RStarInsertionStrategy, RTreeParams, Envelope};
use na::{Point2, Point3};


use super::{da_silva::DaSilvaExplanation, driel::VanDrielExplanation};
use crate::util::types::{PointN};

/// Used to store this in the rtree, we can not store PointN in here since it is stored on the heap.
/// When we keep de index so we can search for the ND point on the heap after finding the nn in 2/3D.
#[derive(Copy, Clone, PartialEq, Debug)]
pub struct IndexedPoint2D {
    pub index: usize,
    pub x: f32,
    pub y: f32,
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct IndexedPoint3D {
    pub index: usize,
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

/// This is an abstaction on the indexed point that allows me to annotate these points.
#[derive(Debug, PartialEq)]
pub struct AnnotatedPoint<P> {
    pub point: P,
    pub da_silva: Option<DaSilvaExplanation>,
    pub van_driel: Option<VanDrielExplanation>,
}

impl Into<Point2<f32>> for IndexedPoint2D{
    fn into(self: Self) -> Point2<f32> {
        Point2::<f32>::new(self.x, self.y)
    }
}

impl Into<Point3<f32>> for IndexedPoint3D{
    fn into(self: Self) -> Point3<f32> {
        Point3::<f32>::new(self.x, self.y, self.z)
    }
}

impl rstar::RTreeObject for IndexedPoint2D {
    type Envelope = rstar::AABB<[f32; 2]>;
    fn envelope(&self) -> Self::Envelope {
        rstar::AABB::from_point([self.x, self.y])
    }
}

impl rstar::RTreeObject for IndexedPoint3D {
    type Envelope = rstar::AABB<[f32; 3]>;
    fn envelope(&self) -> Self::Envelope {
        rstar::AABB::from_point([self.x, self.y, self.z])
    }
}

impl<T> rstar::RTreeObject for AnnotatedPoint<T>
where
    T: rstar::RTreeObject,
{
    type Envelope = T::Envelope;
    fn envelope(&self) -> Self::Envelope {
        self.point.envelope()
    }
}

impl rstar::PointDistance for IndexedPoint2D {
    fn distance_2(&self, point: &[f32; 2]) -> f32 {
        let x: f32 = point[0] - self.x;
        let y: f32 = point[1] - self.y;
        x.powi(2) + y.powi(2)
    }

    fn contains_point(&self, point: &[f32; 2]) -> bool {
        self.x == point[0] && self.y == point[1]
    }

    fn distance_2_if_less_or_equal(
        &self,
        point: &[f32; 2],
        max_distance_2: f32,
    ) -> Option<f32> {
        let t = self.distance_2(point);
        if t <= max_distance_2 {
            Some(t.sqrt())
        } else {
            None
        }
    }
}

impl rstar::PointDistance for AnnotatedPoint<IndexedPoint2D> {
    fn distance_2(&self, point: &[f32; 2]) -> f32 {
        let x: f32 = point[0] - self.point.x;
        let y: f32 = point[1] - self.point.y;
        x.powi(2) + y.powi(2)
    }

    fn contains_point(&self, point: &[f32; 2]) -> bool {
        self.point.x == point[0] && self.point.y == point[1]
    }

    fn distance_2_if_less_or_equal(
        &self,
        point: &[f32; 2],
        max_distance_2: f32,
    ) -> Option<f32> {
        let t = self.distance_2(point);
        if t <= max_distance_2 {
            Some(t.sqrt())
        } else {
            None
        }
    }
}

impl rstar::PointDistance for IndexedPoint3D {
    fn distance_2(&self, point: &[f32; 3]) -> f32 {
        let x = point[0] - self.x;
        let y: f32 = point[1] - self.y;
        let z = point[2] - self.z;
        x.powi(2) + y.powi(2) + z.powi(2)
    }

    fn contains_point(&self, point: &[f32; 3]) -> bool {
        self.x == point[0] && self.y == point[1] && self.z == point[2]
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

impl rstar::PointDistance for AnnotatedPoint<IndexedPoint3D> {
    fn distance_2(&self, point: &[f32; 3]) -> f32 {
        let x = point[0] - self.point.x;
        let y: f32 = point[1] - self.point.y;
        let z = point[2] - self.point.z;
        x.powi(2) + y.powi(2) + z.powi(2)
    }

    fn contains_point(&self, point: &[f32; 3]) -> bool {
        self.point.x == point[0] && self.point.y == point[1] && self.point.z == point[2]
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

// Distance calculation
pub trait Distance {
    // Euclidean distance
    fn distance(&self, other: &Self) -> f32;

    fn sq_distance(&self, other: &Self) -> f32;
}

impl Distance for Point3<f32> {
    fn distance(&self, other: &Self) -> f32 {
        self.sq_distance(other).sqrt()
    }

    fn sq_distance(&self, other: &Self) -> f32 {
        let x: f32 = &self.x - &other.x;
        let y: f32 = &self.y - &other.y;
        let z: f32 = &self.z - &other.z;
        x * x + y * y + z * z
    }
}

impl Distance for PointN {
    fn distance(&self, other: &Self) -> f32 {
        self.sq_distance(other).sqrt()
    }

    fn sq_distance(&self, other: &Self) -> f32 {
        self.iter()
            .zip(other)
            .map(|(a, b)| {
                let i = a - b;
                i * i
            })
            .fold(0.0f32, |sum, v| sum + v)
    }
}

// RTree parameters
pub struct RTreeParameters2D;
pub struct RTreeParameters3D;

// Custom RTree parameters for 2D points
// TODO: Explain these parameters
impl RTreeParams for RTreeParameters2D {
    const MIN_SIZE: usize = 5;
    const MAX_SIZE: usize = 9;
    const REINSERTION_COUNT: usize = 3;
    type DefaultInsertionStrategy = RStarInsertionStrategy;
}

// Custom RTree parameters for 3D points
// TODO: Explain these parameters
impl RTreeParams for RTreeParameters3D {
    const MIN_SIZE: usize = 10;
    const MAX_SIZE: usize = 20;
    const REINSERTION_COUNT: usize = 3;
    type DefaultInsertionStrategy = RStarInsertionStrategy;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    // Here we will calculate the average distance to the first nearest neighbor
    fn find_average_nearest_neightbor_distance_2d_one_line() {
        let indexed_points = vec![
            IndexedPoint2D {
                index: 0,
                x: 0.0,
                y: 0.0,
            },
            IndexedPoint2D {
                index: 1,
                x: 4.0,
                y: 0.0,
            },
            IndexedPoint2D {
                index: 2,
                x: 7.0,
                y: 0.0,
            },
            IndexedPoint2D {
                index: 3,
                x: 9.0,
                y: 0.0,
            },
            IndexedPoint2D {
                index: 4,
                x: 10.0,
                y: 0.0,
            },
        ];

        // The average nearest neighbor distance is based on 5 points
        // | Point | Nearest Neightbor | Distance to neighbor |
        // | 0     | 1                 | 4                    |
        // | 1     | 2                 | 3                    |
        // | 2     | 3                 | 2                    |
        // | 3     | 4                 | 1                    |
        // | 4     | 3                 | 1                    |

        let tree =
            RTree::<IndexedPoint2D, RTreeParameters2D>::bulk_load_with_params(indexed_points);
        let expected = (4.0f32 + 3.0f32 + 2.0f32 + 1.0f32 + 1.0f32) / 5.0f32;
        let actual = IndexedPoint2D::find_average_nearest_neightbor_distance(&tree);
        assert_eq!(actual, expected);
    }

    #[test]
    // Here we will calculate the average distance to the first nearest neighbor
    fn find_average_nearest_neightbor_distance_2d_xy() {
        let indexed_points = vec![
            IndexedPoint2D {
                index: 0,
                x: 0.0,
                y: 0.0,
            },
            IndexedPoint2D {
                index: 1,
                x: 4.0,
                y: 4.0,
            },
            IndexedPoint2D {
                index: 2,
                x: 7.0,
                y: 1.0,
            },
            IndexedPoint2D {
                index: 3,
                x: 9.0,
                y: 4.0,
            },
            IndexedPoint2D {
                index: 4,
                x: 9.0,
                y: 5.0,
            },
        ];

        // The average nearest neighbor distance is based on 5 points
        // | Point | Nearest Neightbor | Distance to neighbor |
        // | 0     | 1                 | sqrt 32              |
        // | 1     | 2                 | sqrt 18              |
        // | 2     | 3                 | sqrt 13              |
        // | 3     | 4                 | sqrt 1               |
        // | 4     | 3                 | sqrt 1               |

        let tree =
            RTree::<IndexedPoint2D, RTreeParameters2D>::bulk_load_with_params(indexed_points);
        let expected =
            (32.0f32.sqrt() + 18.0f32.sqrt() + 13.0f32.sqrt() + 1.0f32 + 1.0f32) / 5.0f32;
        let actual = IndexedPoint2D::find_average_nearest_neightbor_distance(&tree);
        // Transform to int to work around floating point inaccuracies
        assert_eq!((actual * 1000000f32) as i64, (expected * 1000000f32) as i64);
    }
}
