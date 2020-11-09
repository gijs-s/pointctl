//! Contains a generic distance calculation trait.

// Distance calculation
pub trait Distance {
    // Euclidean distance
    fn distance(&self, other: &Self) -> f32;
    // Squared euclidean distance
    fn sq_distance(&self, other: &Self) -> f32;
}

impl Distance for na::Point2<f32> {
    fn distance(&self, other: &Self) -> f32 {
        self.sq_distance(other).sqrt()
    }

    fn sq_distance(&self, other: &Self) -> f32 {
        let x: f32 = self.x - other.x;
        let y: f32 = self.y - other.y;
        x * x + y * y
    }
}

impl Distance for na::Point3<f32> {
    fn distance(&self, other: &Self) -> f32 {
        self.sq_distance(other).sqrt()
    }

    fn sq_distance(&self, other: &Self) -> f32 {
        let x: f32 = self.x - other.x;
        let y: f32 = self.y - other.y;
        let z: f32 = self.z - other.z;
        x * x + y * y + z * z
    }
}

impl Distance for [f32] {
    fn distance(&self, other: &Self) -> f32 {
        self.sq_distance(other).sqrt()
    }

    fn sq_distance(&self, other: &Self) -> f32 {
        self.iter()
            .zip(other)
            .map(|(a, b)| (a - b).powi(2))
            .sum::<f32>()
    }
}
