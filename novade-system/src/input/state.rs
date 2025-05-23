// src/input/state.rs

use std::collections::{HashMap, HashSet};
use super::event::Modifiers; // Use Modifiers from the event module

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
