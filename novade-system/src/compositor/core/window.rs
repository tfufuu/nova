// src/compositor/core/window.rs

/// Represents the state of a window (e.g., floating, tiled).
#[derive(Debug, Clone, PartialEq)]
pub enum WindowState {
    /// The window floats freely, managed by the user or specific placement logic.
    Floating,
    /// The window is tiled according to a layout policy.
    Tiled,
    /// The window is minimized and not visible.
    Minimized,
}

/// Represents a client window managed by the compositor.
#[derive(Debug, Clone, PartialEq)]
pub struct Window {
    /// Unique identifier for the window.
    pub id: u32,
    /// Title of the window.
    pub title: String,
    /// Current width of the window in pixels.
    pub width: u32,
    /// Current height of the window in pixels.
    pub height: u32,
    /// X-coordinate of the window's top-left corner.
    pub x: i32,
    /// Y-coordinate of the window's top-left corner.
    pub y: i32,
    /// Current state of the window (e.g., floating, tiled).
    pub state: WindowState,
    /// Optional application identifier for the window.
    pub app_id: Option<String>,
    /// Indicates if the window currently has focus.
    pub focused: bool,
}

impl Window {
    /// Creates a new `Window` with default properties.
    ///
    /// By default, a new window is `Floating`, not `focused`, and has no `app_id`.
    pub fn new(id: u32, title: String, width: u32, height: u32, x: i32, y: i32) -> Self {
        Self {
            id,
            title,
            width,
            height,
            x,
            y,
            state: WindowState::Floating,
            app_id: None,
            focused: false,
        }
    }
}
