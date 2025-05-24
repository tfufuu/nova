// src/compositor/core/state.rs

use crate::input::InputEvent;
use super::output::Output;
use super::window::{Window, WindowState}; // WindowState needs to be in scope
use super::seat::Seat;
use super::display::Display;


/// Manages the overall state of the Wayland compositor.
#[derive(Debug)]
pub struct CompositorState {
    pub running: bool,
    pub display: Display,
    pub outputs: Vec<Output>,
    pub windows: Vec<Window>,
    pub seats: Vec<Seat>,
    next_window_id: u32,
    next_output_id: u32,
}

impl CompositorState {
    /// Creates a new `CompositorState` with initial setup.
    /// Initializes with a default "seat0" and multiple outputs.
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
            1920, 1080, 0, 0, true,
        ));
        // Add a secondary output
        let secondary_output_id = state.next_output_id();
        state.add_output(Output::new(
            secondary_output_id,
            "Secondary-1280x720".to_string(),
            1280, 720, 1920, 0, false,
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
    pub fn add_output(&mut self, output: Output) {
        self.outputs.push(output);
    }

    /// Removes an output by its ID.
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
    pub fn add_window(&mut self, window: Window) {
        self.windows.push(window);
    }

    /// Removes a window by its ID.
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

    /// Sets the focused window for a given seat.
    ///
    /// Also updates the `focused` flag on all windows. Only mapped windows can receive focus.
    /// If `window_id` is `None`, or if it refers to an unmapped or non-existent window,
    /// focus is cleared for the seat (set to `None`).
    ///
    /// # Arguments
    /// * `seat_name` - The name of the seat to update focus for.
    /// * `window_id` - Optional ID of the window to focus.
    ///
    /// # Returns
    /// `true` if the seat was found and focus processing was attempted.
    /// `false` if the seat itself was not found.
    pub fn set_focused_window_for_seat(&mut self, seat_name: &str, window_id: Option<u32>) -> bool {
        let target_window_is_mapped_and_exists = match window_id {
            Some(id) => self.windows.iter().find(|w| w.id == id).map_or(false, |w| w.is_mapped),
            None => true, // Clearing focus (target_id is None) is always allowed from a validity perspective
        };

        let actual_window_id_to_set = if target_window_is_mapped_and_exists {
            window_id
        } else {
            if let Some(attempted_id) = window_id { // Log if an attempt was made to focus an unsuitable window
                 println!("CompositorState: Cannot focus window ID {} on seat '{}' (unmapped or non-existent). Clearing focus.", attempted_id, seat_name);
            }
            None // Clear focus
        };
        
        if let Some(seat) = self.seats.iter_mut().find(|s| s.name == seat_name) {
            seat.focused_window = actual_window_id_to_set;
            for window_to_update in self.windows.iter_mut() {
                window_to_update.focused = Some(window_to_update.id) == actual_window_id_to_set;
            }
            if let Some(id) = actual_window_id_to_set {
                println!("CompositorState: Seat '{}' focus set to mapped window ID: {}", seat_name, id);
            } else {
                println!("CompositorState: Seat '{}' focus cleared.", seat_name);
            }
            true 
        } else {
            println!("CompositorState: Seat '{}' not found for focus setting.", seat_name);
            false
        }
    }

    /// Resizes a specified window to new dimensions.
    pub fn resize_window(&mut self, window_id: u32, new_width: u32, new_height: u32) -> bool {
        if new_width == 0 || new_height == 0 {
            println!("CompositorState: Resize failed for window ID {} - new dimensions ({}, {}) cannot be zero.",
                     window_id, new_width, new_height);
            return false;
        }
        if let Some(window) = self.find_window_mut(window_id) {
            window.width = new_width;
            window.height = new_height;
            println!("CompositorState: Window ID {} resized to {}x{}", window_id, new_width, new_height);
            true
        } else {
            println!("CompositorState: Resize failed - window ID {} not found.", window_id);
            false
        }
    }

    /// Moves a specified window to new coordinates.
    pub fn move_window(&mut self, window_id: u32, new_x: i32, new_y: i32) -> bool {
        if let Some(window) = self.find_window_mut(window_id) {
            window.x = new_x;
            window.y = new_y;
            println!("CompositorState: Window ID {} moved to ({}, {})", window_id, new_x, new_y);
            true
        } else {
            println!("CompositorState: Move failed - window ID {} not found.", window_id);
            false
        }
    }

    /// Arranges all **mapped** windows into a simple horizontal tiling layout on a selected output.
    ///
    /// It prioritizes the primary output. If no primary output exists, it uses the first available output.
    /// If no outputs are defined, it defaults to a 1920x1080 virtual screen at (0,0).
    /// Windows are tiled side-by-side, relative to the selected output's origin and dimensions.
    /// Only mapped windows are considered for tiling. Tiled windows have their state set to `WindowState::Tiled`.
    pub fn tile_windows(&mut self) {
        // Collect mutable references to mapped windows first.
        let mut mapped_windows_refs: Vec<&mut Window> = self.windows.iter_mut().filter(|w| w.is_mapped).collect();
        let num_mapped_windows = mapped_windows_refs.len();

        if num_mapped_windows == 0 {
            println!("CompositorState: No mapped windows to tile.");
            return;
        }

        let target_output = self.outputs.iter().find(|o| o.is_primary)
            .or_else(|| self.outputs.first());

        let (screen_x, screen_y, screen_width, screen_height) = match target_output {
            Some(output) => {
                println!("CompositorState: Tiling on output ID: {}, Name: '{}', Primary: {}", output.id, output.name, output.is_primary);
                (output.x, output.y, output.width, output.height)
            }
            None => {
                println!("CompositorState: No outputs found, tiling on default 1920x1080 screen at (0,0).");
                (0, 0, 1920, 1080)
            }
        };

        let window_width = screen_width / num_mapped_windows as u32;
        let window_height = screen_height;

        for (i, window) in mapped_windows_refs.iter_mut().enumerate() {
            window.x = screen_x + (i as u32 * window_width) as i32;
            window.y = screen_y;
            window.width = window_width;
            window.height = window_height;
            window.state = WindowState::Tiled;
        }
        println!("CompositorState: Mapped windows tiled. Total mapped: {}. On screen area: {}x{} at ({},{}). Each window: {}x{}",
                 num_mapped_windows, screen_width, screen_height, screen_x, screen_y, window_width, window_height);
    }

    /// Changes focus to the next **mapped** window in the list for the specified seat.
    ///
    /// If no window is currently focused on the seat, or if the focused window is unmapped,
    /// it focuses the first mapped window. If the last mapped window is focused, it wraps around
    /// to the first mapped window. If no mapped windows exist, focus is cleared from the seat.
    ///
    /// # Arguments
    /// * `seat_name` - The name of the seat to update focus for.
    ///
    /// # Returns
    /// `true` if focus was successfully set to a mapped window or cleared because no mapped windows exist (and seat was found).
    /// `false` if the seat was not found.
    pub fn focus_next_window(&mut self, seat_name: &str) -> bool {
        let mapped_window_ids: Vec<u32> = self.windows.iter()
                                .filter(|w| w.is_mapped) // Corrected: 'filter'
                                .map(|w| w.id)
                                .collect();

        if mapped_window_ids.is_empty() {
            println!("CompositorState: No mapped windows available to focus on seat '{}'.", seat_name);
            return self.set_focused_window_for_seat(seat_name, None); // Clear focus on the seat
        }

        let seat_exists = self.seats.iter().any(|s| s.name == seat_name);
        if !seat_exists {
            println!("CompositorState: Seat '{}' not found for focus change.", seat_name);
            return false;
        }
        
        let current_focused_id_on_seat = self.seats.iter()
            .find(|s| s.name == seat_name)
            .and_then(|s| s.focused_window);
            
        let mut next_focus_target_idx_in_mapped_list = 0; // Default to the first mapped window

        if let Some(focused_id) = current_focused_id_on_seat {
            // Check if the currently focused window is still in our list of mapped windows
            if let Some(current_mapped_idx) = mapped_window_ids.iter().position(|&id| id == focused_id) {
                // It is mapped, so cycle from its position in the mapped list
                next_focus_target_idx_in_mapped_list = (current_mapped_idx + 1) % mapped_window_ids.len();
            }
            // If current_focused_id_on_seat is Some, but not in mapped_window_ids (e.g., it got unmapped),
            // next_focus_target_idx_in_mapped_list remains 0, effectively focusing the first *actually mapped* window.
        }
        // If no window was focused on the seat (current_focused_id_on_seat is None),
        // next_focus_target_idx_in_mapped_list also remains 0, targeting the first mapped window.

        let new_focused_window_id = mapped_window_ids[next_focus_target_idx_in_mapped_list];
        
        // set_focused_window_for_seat will handle un-focusing other windows
        // and only focusing the target if it's (still) mapped.
        self.set_focused_window_for_seat(seat_name, Some(new_focused_window_id))
    }

    /// Dispatches an input event to the appropriate **mapped** window based on seat focus.
    /// If the focused window exists but is not mapped, the event is not dispatched.
    ///
    /// # Arguments
    /// * `event` - The input event to dispatch.
    /// * `seat_name` - The name of the seat from which the event originates.
    ///
    /// # Returns
    /// `true` if the event was successfully queued to a focused and mapped window,
    /// `false` otherwise (e.g., seat not found, no window focused, or focused window is not mapped).
    pub fn dispatch_input_event(&mut self, event: &InputEvent, seat_name: &str) -> bool {
        let focused_window_id_option = self.seats.iter()
            .find(|s| s.name == seat_name)
            .and_then(|s| s.focused_window);

        if let Some(focused_window_id) = focused_window_id_option {
            // Find the window that is supposed to be focused
            if let Some(window) = self.windows.iter_mut().find(|w| w.id == focused_window_id) {
                // Check if this window is actually mapped
                if window.is_mapped {
                    window.queue_event(event.clone());
                    // println!("CompositorState: Event dispatched to mapped window ID {}", focused_window_id); // Optional: more verbose logging
                    return true;
                } else {
                    // Focused window exists but is not mapped
                    println!("CompositorState: Window ID {} is focused on seat '{}' but is not mapped. Event not dispatched.", focused_window_id, seat_name);
                    return false;
                }
            }
            // If window ID is stale (not found in self.windows), it will fall through to false.
            println!("CompositorState: Focused window ID {} on seat '{}' not found in window list. Event not dispatched.", focused_window_id, seat_name);
        }
        false
    }
}

impl Default for CompositorState {
    fn default() -> Self {
        Self::new()
    }
}
