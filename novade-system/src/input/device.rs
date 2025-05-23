// src/input/device.rs

/// Represents the type of an input device.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DeviceType {
    /// A keyboard device.
    Keyboard,
    /// A pointer device, such as a mouse or trackpad.
    Pointer,
    /// A touch-based input device.
    Touch,
}

/// Represents an input device.
///
/// This struct holds basic information about an input device,
/// such as its ID, name, and type.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InputDevice {
    /// Unique identifier for the input device.
    pub id: u32,
    /// Human-readable name of the device.
    pub name: String,
    /// The type of the input device.
    pub device_type: DeviceType,
}

impl InputDevice {
    /// Creates a new input device.
    pub fn new(id: u32, name: String, device_type: DeviceType) -> Self {
        Self { id, name, device_type }
    }
}
