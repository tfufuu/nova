// src/lib.rs

pub mod client; // Added client module
pub mod compositor;
pub mod input;
pub mod server;

// Re-export key types
pub use client::{Client, ClientRequest, ServerEvent}; // Added re-exports
pub use server::Server;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
