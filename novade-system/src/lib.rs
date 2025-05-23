// src/lib.rs
pub mod compositor;
pub mod input;
pub mod window_manager;
pub mod audio_manager;
pub mod ai_integration;
pub mod session_management;

// Optional: Add a simple test function to ensure the library compiles.
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
