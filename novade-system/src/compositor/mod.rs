// src/compositor/mod.rs
pub mod core; // Ensure the core module is declared

#[derive(Debug, thiserror::Error)] // Using thiserror
pub enum CompositorError {
    #[error("Initialization failed: {0}")]
    InitializationFailed(String),
    // Add other error variants as needed later
}

pub type CompositorResult<T> = Result<T, CompositorError>;
