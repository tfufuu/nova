// src/compositor/core/output.rs

/// Represents a display output (e.g., a monitor).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Output {
    /// Unique identifier for the output.
    pub id: u32,
    /// Human-readable name for the output (e.g., "DP-1").
    pub name: String,
    /// Width of the output in pixels.
    pub width: u32,
    /// Height of the output in pixels.
    pub height: u32,
    /// X-coordinate of the output's top-left corner in the global compositor space.
    pub x: i32,
    /// Y-coordinate of the output's top-left corner in the global compositor space.
    pub y: i32,
    /// Whether this output is considered the primary display.
    ///
    /// In a multi-output setup, one output is typically designated as primary.
    /// This can influence default window placement, taskbar location, etc.
    pub is_primary: bool,
}

impl Output {
    /// Creates a new display output.
    pub fn new(id: u32, name: String, width: u32, height: u32, x: i32, y: i32, is_primary: bool) -> Self {
        Self { id, name, width, height, x, y, is_primary }
    }
}
