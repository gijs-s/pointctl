// Module containing the explanation mechanisms used for the visualization.

pub mod da_silva;
// pub mod driel;
pub mod common;

#[derive(Debug, PartialEq)]
pub enum SupportedExplanations {
    DaSilva
}