// src/compositor/core/state.rs

use crate::input::InputEvent;
use super::display::Display; // Display is used in CompositorState::new()
use super::output::Output;
use super::seat::Seat;
use super::window::{Window, WindowState}; // Ensure WindowState is imported and used

/// Manages the overall state of the Wayland compositor.
///
/// This struct holds all the core components like displays, outputs,
/// windows, and seats, and provides methods to manage them.
#[derive(Debug)] // Default derive removed, custom Default impl provided below
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
    /// Creates a new `CompositorState` with initial setup.
    ///
    /// Initializes with a default "seat0", a primary output (1920x1080 at 0,0),
    /// and a secondary output (1280x720 at 1920,0).
    /// `running` is set to true, and ID counters are initialized.
    pub fn new() -> Self {
        let mut state = Self {
            running: true,
            display: Display::new("default_display".to_string()),
            outputs: Vec::new(),
            windows: Vec::new(),
            seats: vec![Seat::new("seat0".to_string())],
            next_window_id: 1,
            next_output_id: 1,
        };

        // Add a primary output
        let primary_output_id = state.next_output_id();
        state.add_output(Output::new(
            primary_output_id,
            "Primary-1920x1080".to_string(),
            1920,
            1080,
            0,  // x position
            0,  // y position
            true, // is_primary
        ));

        // Add a secondary output
        let secondary_output_id = state.next_output_id();
        state.add_output(Output::new(
            secondary_output_id,
            "Secondary-1280x720".to_string(),
            1280,
            720,
            1920, // x position (to the right of the primary)
            0,    // y position (aligned at the top)
            false, // is_primary
        ));
        
        state
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
                for window_mut in self.windows.iter_mut() {
                    window_mut.focused = window_mut.id == id;
                }
            } else {
                for window_mut in self.windows.iter_mut() {
                    window_mut.focused = false;
                }
            }
            true
        } else {
            false
        }
    }

    /// Dispatches an input event to the appropriate window based on seat focus.
    ///
    /// # Arguments
    ///
    /// * `event` - The input event to dispatch.
    /// * `seat_name` - The name of the seat from which the event originates.
    ///
    /// # Returns
    ///
    /// `true` if the event was successfully queued to a focused window,
    /// `false` otherwise (e.g., seat not found, no window focused, or focused window not found).
    pub fn dispatch_input_event(&mut self, event: &InputEvent, seat_name: &str) -> bool {
        if let Some(seat) = self.seats.iter().find(|s| s.name == seat_name) {
            if let Some(focused_window_id) = seat.focused_window {
                if let Some(window) = self.windows.iter_mut().find(|w| w.id == focused_window_id) {
                    window.queue_event(event.clone());
                    return true;
                }
            }
        }
        false
    }

    /// Arranges all managed windows into a simple horizontal tiling layout on a selected output.
    ///
    /// It prioritizes the primary output. If no primary output exists, it uses the first available output.
    /// If no outputs are defined, it defaults to a 1920x1080 virtual screen at (0,0).
    /// Windows are tiled side-by-side, relative to the selected output's origin and dimensions.
    /// All tiled windows will have their state set to `WindowState::Tiled`.
    pub fn tile_windows(&mut self) {
        let num_windows = self.windows.len();
        if num_windows == 0 {
            println!("CompositorState: No windows to tile.");
            return;
        }

        // Determine the target output for tiling
        let target_output = self.outputs.iter().find(|o| o.is_primary)
            .or_else(|| self.outputs.first()); // Fallback to the first output if no primary

        let (screen_x, screen_y, screen_width, screen_height) = match target_output {
            Some(output) => {
                println!("CompositorState: Tiling on output ID: {}, Name: '{}', Primary: {}", output.id, output.name, output.is_primary);
                (output.x, output.y, output.width, output.height)
            }
            None => {
                println!("CompositorState: No outputs found, tiling on default 1920x1080 screen at (0,0).");
                (0, 0, 1920, 1080) // Default virtual screen
            }
        };

        let window_width = screen_width / num_windows as u32;
        let window_height = screen_height;

        for (i, window) in self.windows.iter_mut().enumerate() {
            window.x = screen_x + (i as u32 * window_width) as i32;
            window.y = screen_y;
            window.width = window_width;
            window.height = window_height;
            window.state = WindowState::Tiled;
        }
        println!("CompositorState: Windows tiled. Total: {}. On screen area: {}x{} at ({},{}). Each window: {}x{}",
                 num_windows, screen_width, screen_height, screen_x, screen_y, window_width, window_height);
    }

    /// Changes focus to the next window in the list for the specified seat.
    ///
    /// If no window is currently focused, it focuses the first window.
    /// If the last window is focused, it wraps around to the first window.
    /// Handles cases with no windows or only one window.
    ///
    /// # Arguments
    /// * `seat_name` - The name of the seat to update focus for.
    ///
    /// # Returns
    /// `true` if focus was changed or set, `false` if no windows exist or seat not found.
    pub fn focus_next_window(&mut self, seat_name: &str) -> bool {
        if self.windows.is_empty() {
            println!("CompositorState: No windows to focus.");
            return false;
        }

        let seat_index = match self.seats.iter().position(|s| s.name == seat_name) {
            Some(idx) => idx,
            None => {
                println!("CompositorState: Seat '{}' not found for focus change.", seat_name);
                return false;
            }
        };

        let current_focused_id = self.seats[seat_index].focused_window;
        let mut next_window_index = 0; 

        if let Some(focused_id) = current_focused_id {
            if let Some(current_idx) = self.windows.iter().position(|w| w.id == focused_id) {
                next_window_index = (current_idx + 1) % self.windows.len();
            }
        }
        
        for window in self.windows.iter_mut() {
            window.focused = false;
        }

        if let Some(window_to_focus) = self.windows.get_mut(next_window_index) {
            window_to_focus.focused = true;
            self.seats[seat_index].focused_window = Some(window_to_focus.id);
            println!("CompositorState: Focus changed on seat '{}' to window ID: {}", seat_name, window_to_focus.id);
            return true;
        }
        
        self.seats[seat_index].focused_window = None; 
        println!("CompositorState: Could not set next focus on seat '{}'.", seat_name);
        false
    }

    /// Resizes a specified window to new dimensions.
    ///
    /// # Arguments
    /// * `window_id` - The ID of the window to resize.
    /// * `new_width` - The new width for the window. Must be greater than 0.
    /// * `new_height` - The new height for the window. Must be greater than 0.
    ///
    /// # Returns
    /// `true` if the window was found and successfully resized, `false` otherwise
    /// (e.g., window not found, or `new_width` or `new_height` is 0).
    pub fn resize_window(&mut self, window_id: u32, new_width: u32, new_height: u32) -> bool {
        if new_width == 0 || new_height == 0 {
            println!("CompositorState: Resize failed for window ID {} - new dimensions ({}, {}) cannot be zero.",
                     window_id, new_width, new_height);
            return false;
        }

        if let Some(window) = self.find_window_mut(window_id) {
            window.width = new_width;
            window.height = new_height;
            // Optionally, update window state if resizing implies something (e.g., exiting tiled mode)
            // For now, just resize. If it was tiled, it might look odd until retiled.
            // window.state = WindowState::Floating; // Example if resize breaks tiling
            println!("CompositorState: Window ID {} resized to {}x{}", window_id, new_width, new_height);
            true
        } else {
            println!("CompositorState: Resize failed - window ID {} not found.", window_id);
            false
        }
    }

    /// Moves a specified window to new coordinates.
    ///
    /// # Arguments
    /// * `window_id` - The ID of the window to move.
    /// * `new_x` - The new x-coordinate for the window's top-left corner.
    /// * `new_y` - The new y-coordinate for the window's top-left corner.
    ///
    /// # Returns
    /// `true` if the window was found and successfully moved, `false` otherwise.
    pub fn move_window(&mut self, window_id: u32, new_x: i32, new_y: i32) -> bool {
        if let Some(window) = self.find_window_mut(window_id) {
            window.x = new_x;
            window.y = new_y;
            // Optionally, update window state if moving implies something
            // window.state = WindowState::Floating; // Example if move breaks tiling
            println!("CompositorState: Window ID {} moved to ({}, {})", window_id, new_x, new_y);
            true
        } else {
            println!("CompositorState: Move failed - window ID {} not found.", window_id);
            false
        }
    }
}

// Make sure Default calls the updated new()
impl Default for CompositorState {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod state_tests;
