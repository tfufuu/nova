// src/compositor/core/display.rs

/// Represents global display properties or manages outputs.
///
/// This is a high-level placeholder for now. It might hold configurations
/// like global scale factor, or be responsible for managing the list of outputs.
#[derive(Debug, Default)]
pub struct Display {
    /// Name for the display configuration (e.g., "primary_display_config").
    pub name: String,
}

impl Display {
    /// Creates a new `Display` configuration.
    pub fn new(name: String) -> Self {
        Self { name }
    }
}
