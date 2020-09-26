// Module containing the explanation mechanisms used for the visualization.
// TODO: Add prelude?

pub mod common;
pub mod da_silva;
pub mod driel;

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Neighborhood {
    K(usize),
    R(f32)
}
