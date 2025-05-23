// src/compositor/core/seat.rs

/// Represents a user input context (e.g., keyboard, pointer, touch).
#[derive(Debug, Clone, PartialEq)]
pub struct Seat {
    /// Name of the seat (e.g., "seat0").
    pub name: String,
    /// ID of the window that currently has input focus for this seat, if any.
    pub focused_window: Option<u32>,
    // Future additions: keyboard state, pointer state, capabilities.
}

impl Seat {
    /// Creates a new `Seat`.
    pub fn new(name: String) -> Self {
        Self { name, focused_window: None }
    }
}
