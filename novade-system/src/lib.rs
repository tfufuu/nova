// src/lib.rs

// Make modules public so they can be used by main.rs or other library users.
pub mod compositor;
pub mod input;
pub mod server; // Added server module
// pub mod window_manager; // Assuming these are still planned
// pub mod audio_manager;
// pub mod ai_integration;
// pub mod session_management;

// Re-export key types for easier access if this is intended as a library.
// For now, direct use via `crate::module::Type` in main.rs is also fine.
pub use server::Server; // Added Server re-export

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
