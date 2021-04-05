//! Helper for generating synthetic datasets from scratch.

use rand::{rngs::ThreadRng, Rng};
use rand_distr::{Distribution, Normal};

/// Create a vector of points that each have 0 zero axis.
///
/// Noise determines the variance of the normal dist used for noise.
/// A value between 0.00 and 0.05 is advisable.
pub fn generate_cube(points: i32, noise: f32) -> Vec<Vec<f32>> {
    let normal: Normal<f32> = Normal::new(0.0, noise).unwrap();
    let mut res: Vec<Vec<f32>> = Vec::new();
    let mut rng: ThreadRng = ThreadRng::default();

    let update_interval = points / 10;

    for i in 0..points {
        if i % update_interval == 0 && i != 0 {
            println!("Generated {} points", i)
        }
        // Get two random coordinates
        let (rand_1, rand_2): (f32, f32) = rng.gen();
        // Add zero coordinate and noise
        let mut coords: Vec<f32> = vec![
            rand_1 + normal.sample(&mut rng),
            rand_2 + normal.sample(&mut rng),
            normal.sample(&mut rng),
        ];
        // Rotate the vector so the zero axis differs
        coords.rotate_left(rng.gen_range(0..3));
        // Add new point to the result.
        res.push(coords);
    }
    res
}

pub fn generate_hyper_cube(points: i32, noise: f32) -> Vec<Vec<f32>> {
    let normal: Normal<f32> = Normal::new(0.0, noise).unwrap();
    let mut res: Vec<Vec<f32>> = Vec::new();
    let mut rng: ThreadRng = ThreadRng::default();

    let update_interval = points / 10;

    for i in 0..points {
        if i % update_interval == 0 && i != 0 {
            println!("Generated {} points", i)
        }
        // Get three random coordinates
        let (rand_1, rand_2, rand_3): (f32, f32, f32) = rng.gen();
        // Add zero coordinate and noise
        let mut coords: Vec<f32> = vec![
            rand_1 + normal.sample(&mut rng),
            rand_2 + normal.sample(&mut rng),
            rand_3 + normal.sample(&mut rng),
            normal.sample(&mut rng),
        ];
        // Rotate the vector so the zero axis differs
        coords.rotate_left(rng.gen_range(0..4));
        // Add new point to the result.
        res.push(coords);
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generate_correct_number() {
        assert_eq!(0, generate_cube(0, 0.0).len());
        assert_eq!(10, generate_cube(10, 0.0).len());
        assert_eq!(10_000, generate_cube(10_000, 0.0).len());
    }
}
