// src/compositor/core/state.rs

use super::display::Display;
use super::output::Output;
use super::seat::Seat;
use super::window::Window;

/// Manages the overall state of the Wayland compositor.
///
/// This struct holds all the core components like displays, outputs,
/// windows, and seats, and provides methods to manage them.
#[derive(Debug, Default)]
pub struct CompositorState {
    /// Indicates if the compositor is currently running.
    pub running: bool,
    /// Represents the main display configuration.
    pub display: Display,
    /// A list of all outputs (e.g., monitors).
    pub outputs: Vec<Output>,
    /// A list of all windows managed by the compositor.
    pub windows: Vec<Window>,
    /// A list of all seats (user input contexts).
    pub seats: Vec<Seat>,
    /// Counter for the next available window ID.
    next_window_id: u32,
    /// Counter for the next available output ID.
    next_output_id: u32,
}

impl CompositorState {
    /// Creates a new `CompositorState` with default values.
    ///
    /// Initializes with a default display, an empty list of outputs and windows,
    /// a default seat ("seat0"), and sets `running` to true.
    /// ID counters are initialized to 1.
    pub fn new() -> Self {
        Self {
            running: true,
            display: Display::new("default_display".to_string()),
            outputs: Vec::new(),
            windows: Vec::new(),
            seats: vec![Seat::new("seat0".to_string())],
            next_window_id: 1,
            next_output_id: 1,
        }
    }

    /// Returns the next available output ID and increments the internal counter.
    pub fn next_output_id(&mut self) -> u32 {
        let id = self.next_output_id;
        self.next_output_id += 1;
        id
    }

    /// Adds a new output to the compositor state.
    ///
    /// The ID for the output should typically be generated using `next_output_id()`.
    pub fn add_output(&mut self, output: Output) {
        self.outputs.push(output);
    }

    /// Removes an output by its ID.
    ///
    /// Returns `true` if an output was removed, `false` otherwise.
    pub fn remove_output(&mut self, output_id: u32) -> bool {
        if let Some(index) = self.outputs.iter().position(|o| o.id == output_id) {
            self.outputs.remove(index);
            true
        } else {
            false
        }
    }

    /// Returns the next available window ID and increments the internal counter.
    pub fn next_window_id(&mut self) -> u32 {
        let id = self.next_window_id;
        self.next_window_id += 1;
        id
    }

    /// Adds a new window to the compositor state.
    ///
    /// The ID for the window should typically be generated using `next_window_id()`.
    pub fn add_window(&mut self, window: Window) {
        self.windows.push(window);
    }

    /// Removes a window by its ID.
    ///
    /// Returns `true` if a window was removed, `false` otherwise.
    pub fn remove_window(&mut self, window_id: u32) -> bool {
        if let Some(index) = self.windows.iter().position(|w| w.id == window_id) {
            self.windows.remove(index);
            true
        } else {
            false
        }
    }

    /// Finds a mutable reference to a window by its ID.
    pub fn find_window_mut(&mut self, window_id: u32) -> Option<&mut Window> {
        self.windows.iter_mut().find(|w| w.id == window_id)
    }

    /// Finds an immutable reference to a window by its ID.
    pub fn find_window(&self, window_id: u32) -> Option<&Window> {
        self.windows.iter().find(|w| w.id == window_id)
    }

    /// Sets the focused window for a specific seat.
    ///
    /// If `window_id` is `Some`, the specified window will be marked as focused,
    /// and all other windows will be marked as not focused.
    /// If `window_id` is `None`, all windows will be marked as not focused.
    /// Returns `true` if the seat was found and updated, `false` otherwise.
    pub fn set_focused_window_for_seat(&mut self, seat_name: &str, window_id: Option<u32>) -> bool {
        if let Some(seat) = self.seats.iter_mut().find(|s| s.name == seat_name) {
            seat.focused_window = window_id;
            if let Some(id) = window_id {
                // Set this window to focused, others to not focused
                for window in self.windows.iter_mut() {
                    window.focused = window.id == id;
                }
            } else {
                // No window focused
                for window in self.windows.iter_mut() {
                    window.focused = false;
                }
            }
            true
        } else {
            false
        }
    }
}
// The #[cfg(test)] mod tests { ... } block has been removed.
