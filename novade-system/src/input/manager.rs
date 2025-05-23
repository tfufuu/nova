// src/input/manager.rs

use crate::input::state::InputState;
use crate::input::event::InputEvent; // Added import

/// Manages the overall input state and processes incoming events.
#[derive(Debug, Default)]
pub struct InputManager {
    /// The current state of all input devices.
    pub input_state: InputState,
}

impl InputManager {
    /// Creates a new `InputManager` with a default input state.
    pub fn new() -> Self {
        Self {
            input_state: InputState::default(),
        }
    }

    /// Processes a simulated raw input event, updating the internal input state.
    ///
    /// This function takes an `InputEvent` (which, in a real scenario, might be
    /// translated from a more raw hardware event) and uses it to update the
    /// `InputState` managed by this `InputManager`.
    ///
    /// # Arguments
    ///
    /// * `event` - The `InputEvent` to process. This event is consumed and returned.
    ///
    /// # Returns
    ///
    /// The same `InputEvent` that was passed in, after it has been used to update
    /// the internal state. This allows the caller to potentially reuse or further
    /// process the event.
    pub fn process_simulated_raw_event(&mut self, event: InputEvent) -> InputEvent {
        self.input_state.update_from_event(&event);
        event
    }
}
