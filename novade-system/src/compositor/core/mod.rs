// src/compositor/core/mod.rs
mod display;
mod output;
mod seat;
mod state;
mod window;

// Conditionally compile the test module
#[cfg(test)]
mod state_tests;

pub use display::Display;
pub use output::Output;
pub use seat::Seat;
pub use state::CompositorState;
pub use window::{Window, WindowState}; // also export WindowState
