//! Trait / implementation of for loading in explanations into the data structure
// First party imports
use std::{collections::HashMap, hash::Hash};

use super::definitions::{PointContainer2D, PointContainer3D};
use crate::exp::{
    DaSilvaExplanation, DaSilvaType, NormalExplanation, VanDrielExplanation, VanDrielType,
};

/// TODO is this even needed? Mutation could be done while running the calculation.
pub trait Load<T, S: Hash> {
    /// Load data into the visualization state
    fn load(&mut self, _: T, _: S);
}

/// Loading in the Da silva into the 2D data container
impl Load<Vec<DaSilvaExplanation>, DaSilvaType> for PointContainer2D {
    fn load(&mut self, explanations: Vec<DaSilvaExplanation>, mode: DaSilvaType) {
        // TODO: This assert is just for testing, this should be caught earlier
        // on and have a documented exit code
        assert!(self.point_data.len() == explanations.len());
        for (point_data, exp) in self.point_data.iter_mut().zip(explanations.into_iter()) {
            match mode {
                DaSilvaType::Variance => {
                    // The single explanations are no longer correct, remove them
                    point_data.silva_single = HashMap::new();
                    point_data.silva_var = Some(exp);
                }
                DaSilvaType::Euclidean => {
                    // The single explanations are no longer correct, remove them
                    point_data.silva_single = HashMap::new();
                    point_data.silva_euclidean = Some(exp);
                }
                DaSilvaType::EuclideanSingle(attribute)
                | DaSilvaType::VarianceSingle(attribute) => {
                    point_data.silva_single.insert(attribute, exp.confidence);
                }
            };
        }
    }
}

/// Loading in Van Driel into the 2D data container
impl Load<Vec<VanDrielExplanation>, VanDrielType> for PointContainer2D {
    fn load(&mut self, explanations: Vec<VanDrielExplanation>, mode: VanDrielType) {
        // TODO: This assert is just for testing, this should be caught earlier
        // on and have a documented exit code
        assert!(self.point_data.len() == explanations.len());
        for (point_data, exp) in self.point_data.iter_mut().zip(explanations.into_iter()) {
            match mode {
                VanDrielType::MinimalVariance => point_data.driel_min = Some(exp),
                VanDrielType::TotalVariance => point_data.driel_total = Some(exp),
            };
        }
    }
}

/// Loading in the Da silva into the 3D data container
impl Load<Vec<DaSilvaExplanation>, DaSilvaType> for PointContainer3D {
    fn load(&mut self, explanations: Vec<DaSilvaExplanation>, mode: DaSilvaType) {
        // TODO: This assert is just for testing, this should be caught earlier
        // on and have a documented exit code
        assert!(self.point_data.len() == explanations.len());
        for (point_data, exp) in self.point_data.iter_mut().zip(explanations.into_iter()) {
            match mode {
                DaSilvaType::Variance => {
                    // The single explanations are no longer correct, remove them
                    point_data.silva_single = HashMap::new();
                    point_data.silva_var = Some(exp);
                }
                DaSilvaType::Euclidean => {
                    // The single explanations are no longer correct, remove them
                    point_data.silva_single = HashMap::new();
                    point_data.silva_euclidean = Some(exp);
                }
                DaSilvaType::EuclideanSingle(attribute)
                | DaSilvaType::VarianceSingle(attribute) => {
                    point_data.silva_single.insert(attribute, exp.confidence);
                }
            };
        }
    }
}

/// Loading in Van Driel into the 3D data container
impl Load<Vec<VanDrielExplanation>, VanDrielType> for PointContainer3D {
    fn load(&mut self, explanations: Vec<VanDrielExplanation>, mode: VanDrielType) {
        // TODO: This assert is just for testing, this should be caught earlier
        // on and have a documented exit code
        assert!(self.point_data.len() == explanations.len());
        for (point_data, exp) in self.point_data.iter_mut().zip(explanations.into_iter()) {
            match mode {
                VanDrielType::MinimalVariance => point_data.driel_min = Some(exp),
                VanDrielType::TotalVariance => point_data.driel_total = Some(exp),
            };
        }
    }
}

/// Loading in Normal into the 3D data container
impl Load<Vec<NormalExplanation>, ()> for PointContainer3D {
    fn load(&mut self, explanations: Vec<NormalExplanation>, _mode: ()) {
        // TODO: This assert is just for testing, this should be caught earlier
        // on and have a documented exit code
        assert!(self.point_data.len() == explanations.len());
        for (point_data, exp) in self.point_data.iter_mut().zip(explanations.into_iter()) {
            point_data.normal = Some(exp);
        }
    }
}
