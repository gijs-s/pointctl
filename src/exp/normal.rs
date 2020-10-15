/// For the process of shading the points we need to calculate PCA in local 3D regions and return the eigenvector
/// of the principal competent with the lowest eigenvalue.
extern crate nalgebra as na;

pub struct NormalExplanation {
    normal: na::Point3<f32>,
}
