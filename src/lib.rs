pub mod bar;
pub mod chart_container;
pub mod fill_gradient;
pub mod graph;
pub mod grid;
pub mod legend;
pub mod line;
pub mod tooltip;
pub mod types;
pub mod x_axis;
pub mod y_axis;

pub use bar::*;
pub use chart_container::*;
pub use fill_gradient::*;
pub use graph::*;
pub use grid::*;
pub use legend::*;
pub use line::*;
pub use tooltip::*;
pub use types::*;
pub use x_axis::*;
pub use y_axis::*;

// Compile README examples as doctests so they can't drift from the API.
#[cfg(doctest)]
#[doc = include_str!("../README.md")]
struct ReadmeDoctests;
