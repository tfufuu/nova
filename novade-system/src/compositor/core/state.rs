// src/compositor/core/state.rs
pub struct CompositorState {
    // Placeholder field, e.g., a counter or a simple boolean
    pub running: bool,
}

impl CompositorState {
    pub fn new() -> Self {
        Self { running: true }
    }
}

impl Default for CompositorState {
    fn default() -> Self {
        Self::new()
    }
}
