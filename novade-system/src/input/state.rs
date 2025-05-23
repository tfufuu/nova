// src/input/state.rs
use std::collections::{HashMap, HashSet};
use super::event::{InputEvent, KeyState, ButtonState, Modifiers}; // Ensure ButtonState is imported

/// Holds the current state of all input mechanisms.
///
/// This includes information like currently pressed keys, pointer position,
/// pressed mouse buttons, active touch points, and modifier key states.
#[derive(Debug, Clone, PartialEq)]
pub struct InputState {
    /// Set of currently pressed key codes.
    /// Each u32 represents a raw key code.
    pub pressed_keys: HashSet<u32>,
    /// Current pointer X position.
    /// The coordinate system (absolute or relative) is context-dependent.
    pub pointer_x: f64,
    /// Current pointer Y position.
    /// The coordinate system (absolute or relative) is context-dependent.
    pub pointer_y: f64,
    /// Set of currently pressed pointer button codes.
    /// Each u32 represents a raw button code (e.g., BTN_LEFT).
    pub pressed_buttons: HashSet<u32>,
    /// Map of active touch points.
    /// Key is the touch_id, value is a tuple of (x, y) coordinates.
    pub active_touches: HashMap<u32, (f64, f64)>,
    /// Current state of modifier keys (Shift, Ctrl, Alt, Logo).
    pub modifiers: Modifiers,
}

impl Default for InputState {
    /// Creates a default `InputState`.
    ///
    /// Initializes with no pressed keys or buttons, pointer at (0.0, 0.0),
    /// no active touches, and default (all false) modifier states.
    fn default() -> Self {
        Self {
            pressed_keys: HashSet::new(),
            pointer_x: 0.0,
            pointer_y: 0.0,
            pressed_buttons: HashSet::new(),
            active_touches: HashMap::new(),
            modifiers: Modifiers::default(),
        }
    }
}

impl InputState {
    /// Updates the input state based on an incoming event.
    ///
    /// # Arguments
    ///
    /// * `event` - The input event to process.
    pub fn update_from_event(&mut self, event: &InputEvent) {
        // Always update modifiers from the event if it carries them
        match event {
            InputEvent::Keyboard { modifiers, .. } |
            InputEvent::PointerMotion { modifiers, .. } |
            InputEvent::PointerButton { modifiers, .. } |
            InputEvent::PointerAxis { modifiers, .. } |
            InputEvent::TouchDown { modifiers, .. } |
            InputEvent::TouchMotion { modifiers, .. } |
            InputEvent::TouchUp { modifiers, .. } => {
                self.modifiers = *modifiers;
            }
        }

        match event {
            InputEvent::Keyboard { key_code, state, .. } => {
                match state {
                    KeyState::Pressed => {
                        self.pressed_keys.insert(*key_code);
                    }
                    KeyState::Released => {
                        self.pressed_keys.remove(key_code);
                    }
                }
            }
            InputEvent::PointerMotion { delta_x, delta_y, .. } => {
                // Assuming delta_x and delta_y are relative movements
                // If they were absolute, it would be self.pointer_x = delta_x;
                self.pointer_x += delta_x;
                self.pointer_y += delta_y;
            }
            InputEvent::PointerButton { button_code, state, .. } => {
                match state {
                    ButtonState::Pressed => {
                        self.pressed_buttons.insert(*button_code);
                    }
                    ButtonState::Released => {
                        self.pressed_buttons.remove(button_code);
                    }
                }
            }
            InputEvent::PointerAxis { .. } => {
                // For now, PointerAxis mainly updates modifiers.
                // Actual scroll accumulation could be added here if needed.
            }
            InputEvent::TouchDown { touch_id, x, y, .. } => {
                self.active_touches.insert(*touch_id, (*x, *y));
            }
            InputEvent::TouchMotion { touch_id, x, y, .. } => {
                if self.active_touches.contains_key(touch_id) {
                    self.active_touches.insert(*touch_id, (*x, *y));
                }
            }
            InputEvent::TouchUp { touch_id, .. } => {
                self.active_touches.remove(touch_id);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*; // Imports InputState
    use crate::input::event::{InputEvent, KeyState, ButtonState, Modifiers}; // Imports event types
    use std::collections::HashSet;

    fn default_modifiers() -> Modifiers {
        Modifiers { shift: false, ctrl: false, alt: false, logo: false }
    }

    #[test]
    fn test_keyboard_event_update() {
        let mut state = InputState::default();
        let modifiers_with_shift = Modifiers { shift: true, ..default_modifiers() };

        // Press 'A' key (e.g., key_code 65)
        let event_a_press = InputEvent::Keyboard {
            key_code: 65,
            state: KeyState::Pressed,
            modifiers: modifiers_with_shift,
        };
        state.update_from_event(&event_a_press);
        assert!(state.pressed_keys.contains(&65));
        assert_eq!(state.modifiers, modifiers_with_shift);

        // Release 'A' key
        let event_a_release = InputEvent::Keyboard {
            key_code: 65,
            state: KeyState::Released,
            modifiers: default_modifiers(), // Shift released
        };
        state.update_from_event(&event_a_release);
        assert!(!state.pressed_keys.contains(&65));
        assert_eq!(state.modifiers, default_modifiers());
    }

    #[test]
    fn test_pointer_motion_update() {
        let mut state = InputState::default();
        let modifiers_with_ctrl = Modifiers { ctrl: true, ..default_modifiers() };
        let event = InputEvent::PointerMotion {
            delta_x: 10.5,
            delta_y: -5.0,
            modifiers: modifiers_with_ctrl,
        };
        state.update_from_event(&event);
        assert_eq!(state.pointer_x, 10.5);
        assert_eq!(state.pointer_y, -5.0);
        assert_eq!(state.modifiers, modifiers_with_ctrl);

        // Subsequent motion
        let event2 = InputEvent::PointerMotion {
            delta_x: -0.5,
            delta_y: 1.0,
            modifiers: modifiers_with_ctrl, // Ctrl still held
        };
        state.update_from_event(&event2);
        assert_eq!(state.pointer_x, 10.0); // 10.5 - 0.5
        assert_eq!(state.pointer_y, -4.0); // -5.0 + 1.0
    }

    #[test]
    fn test_pointer_button_update() {
        let mut state = InputState::default();
        let modifiers_with_alt = Modifiers { alt: true, ..default_modifiers() };

        // Press button 1
        let event_press = InputEvent::PointerButton {
            button_code: 1,
            state: ButtonState::Pressed,
            modifiers: modifiers_with_alt,
        };
        state.update_from_event(&event_press);
        assert!(state.pressed_buttons.contains(&1));
        assert_eq!(state.modifiers, modifiers_with_alt);

        // Release button 1
        let event_release = InputEvent::PointerButton {
            button_code: 1,
            state: ButtonState::Released,
            modifiers: default_modifiers(), // Alt released
        };
        state.update_from_event(&event_release);
        assert!(!state.pressed_buttons.contains(&1));
        assert_eq!(state.modifiers, default_modifiers());
    }

    #[test]
    fn test_pointer_axis_update() {
        let mut state = InputState::default();
        let modifiers_with_logo = Modifiers { logo: true, ..default_modifiers() };
        let event = InputEvent::PointerAxis {
            horizontal: 1.0,
            vertical: -1.0,
            modifiers: modifiers_with_logo,
        };
        state.update_from_event(&event);
        // Axis events currently only update modifiers in this InputState impl.
        assert_eq!(state.modifiers, modifiers_with_logo);
        // Assert other state fields remain unchanged (e.g. pointer_x, pressed_keys)
        assert_eq!(state.pointer_x, 0.0);
        assert!(state.pressed_keys.is_empty());
    }

    #[test]
    fn test_touch_events_update() {
        let mut state = InputState::default();
        let touch_id = 1;
        let modifiers_touch = Modifiers { shift: true, ctrl: true, ..default_modifiers() };

        // Touch down
        let event_down = InputEvent::TouchDown {
            touch_id,
            x: 100.0,
            y: 150.0,
            modifiers: modifiers_touch,
        };
        state.update_from_event(&event_down);
        assert!(state.active_touches.contains_key(&touch_id));
        assert_eq!(state.active_touches.get(&touch_id), Some(&(100.0, 150.0)));
        assert_eq!(state.modifiers, modifiers_touch);

        // Touch motion
        let event_motion = InputEvent::TouchMotion {
            touch_id,
            x: 110.0,
            y: 155.0,
            modifiers: modifiers_touch, // Modifiers held
        };
        state.update_from_event(&event_motion);
        assert_eq!(state.active_touches.get(&touch_id), Some(&(110.0, 155.0)));

        // Touch up
        let event_up = InputEvent::TouchUp {
            touch_id,
            modifiers: default_modifiers(), // Modifiers released
        };
        state.update_from_event(&event_up);
        assert!(!state.active_touches.contains_key(&touch_id));
        assert_eq!(state.modifiers, default_modifiers());
    }

    #[test]
    fn test_modifier_priority() {
        // Test that modifiers from the latest event are always used.
        let mut state = InputState::default();
        let mod_shift = Modifiers { shift: true, ..default_modifiers() };
        let mod_ctrl = Modifiers { ctrl: true, ..default_modifiers() };

        let event_key_shift = InputEvent::Keyboard { key_code: 65, state: KeyState::Pressed, modifiers: mod_shift };
        state.update_from_event(&event_key_shift);
        assert_eq!(state.modifiers, mod_shift);

        // A subsequent motion event with different modifiers should update state.modifiers
        let event_motion_ctrl = InputEvent::PointerMotion { delta_x: 0.0, delta_y: 0.0, modifiers: mod_ctrl };
        state.update_from_event(&event_motion_ctrl);
        assert_eq!(state.modifiers, mod_ctrl);
    }
}
