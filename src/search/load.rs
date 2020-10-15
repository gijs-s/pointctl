/// Trait / implementation of for loading in explanations into the data structure

// First party imports
use super::definitions::{PointContainer2D, PointContainer3D};
use crate::exp::{DaSilvaExplanation, VanDrielExplanation};

/// TODO is this even needed? Mutation could be done while running the calculation.
pub trait Load<T> {
    /// Load data into the visualization state
    fn load(&mut self, _: T);
}

/// Loading in the Da silva into the 2D data container
impl Load<Vec<DaSilvaExplanation>> for PointContainer2D {
    fn load(&mut self, explanations: Vec<DaSilvaExplanation>) {
        // TODO: This assert is just for testing, this should be caught earlier
        // on and have a documented exit code
        assert!(self.point_data.len() == explanations.len());
        for (point_data, exp) in self.point_data.iter_mut().zip(explanations.into_iter()) {
            point_data.silva = Some(exp);
        }
    }
}

/// Loading in Van Driel into the 2D data container
impl Load<Vec<VanDrielExplanation>> for PointContainer2D {
    fn load(&mut self, explanations: Vec<VanDrielExplanation>) {
        // TODO: This assert is just for testing, this should be caught earlier
        // on and have a documented exit code
        assert!(self.point_data.len() == explanations.len());
        for (point_data, exp) in self.point_data.iter_mut().zip(explanations.into_iter()) {
            point_data.driel = Some(exp);
        }
    }
}

/// Loading in the Da silva into the 3D data container
impl Load<Vec<DaSilvaExplanation>> for PointContainer3D {
    fn load(&mut self, explanations: Vec<DaSilvaExplanation>) {
        // TODO: This assert is just for testing, this should be caught earlier
        // on and have a documented exit code
        assert!(self.point_data.len() == explanations.len());
        for (point_data, exp) in self.point_data.iter_mut().zip(explanations.into_iter()) {
            point_data.silva = Some(exp);
        }
    }
}

/// Loading in Van Driel into the 3D data container
impl Load<Vec<VanDrielExplanation>> for PointContainer3D {
    fn load(&mut self, explanations: Vec<VanDrielExplanation>) {
        // TODO: This assert is just for testing, this should be caught earlier
        // on and have a documented exit code
        assert!(self.point_data.len() == explanations.len());
        for (point_data, exp) in self.point_data.iter_mut().zip(explanations.into_iter()) {
            point_data.driel = Some(exp);
        }
    }
}
