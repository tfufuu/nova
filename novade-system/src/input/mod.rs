// src/input/mod.rs

pub mod event;
pub mod device;
pub mod state;
pub mod manager; // Added manager module

pub use event::{
    ButtonState, InputEvent, KeyState, Modifiers,
};
pub use device::{DeviceType, InputDevice};
pub use state::InputState;
pub use manager::InputManager; // Re-export InputManager
