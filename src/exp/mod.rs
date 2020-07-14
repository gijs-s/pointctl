// Module containing the explanation mechanisms used for the visualization.
// TODO: Add prelude?

pub mod da_silva;
// pub mod driel;
pub mod common;

#[derive(Debug, PartialEq)]
pub enum SupportedExplanations {
    DaSilva,
}
