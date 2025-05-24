// src/input/manager.rs

use crate::input::state::InputState;
use crate::input::event::{InputEvent, KeyState, Modifiers}; // Added KeyState and Modifiers

// Define assumed key codes for C and V.
// These are based on ASCII, but in a real system might come from xkbcommon or similar.
const KEY_C: u32 = 67;
const KEY_V: u32 = 86;

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

        // Check for clipboard shortcuts
        if let InputEvent::Keyboard { key_code, state, modifiers } = event {
            if modifiers.ctrl && state == KeyState::Pressed {
                if key_code == KEY_C {
                    println!("InputManager: Detected Ctrl+C shortcut.");
                    return InputEvent::CopyShortcut;
                } else if key_code == KEY_V {
                    println!("InputManager: Detected Ctrl+V shortcut.");
                    return InputEvent::PasteShortcut;
                }
            }
        }
        event
    }
}
