// src/compositor/mod.rs
pub mod core; // Ensure the core module is declared

// #[derive(Debug, thiserror::Error)] // Commented out due to thiserror being disabled
#[derive(Debug)] // Basic Debug
pub enum CompositorError {
    // #[error("Initialization failed: {0}")] // Commented out
    InitializationFailed(String),
    // Add other error variants as needed later
}
// Manual Display if needed, or skip for now.

pub type CompositorResult<T> = Result<T, CompositorError>;
