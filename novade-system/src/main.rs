// src/main.rs
// Ensure the library crate is referred to by its name (usually the project name, e.g., "novade_system")
use novade_system::compositor::core::CompositorState; 
// If Cargo.toml has a different name for the lib, use that.
// If 'novade_system' is not found, try 'novade_system_lib' or check Cargo.toml name field.
// The worker should determine the correct library name (it's 'novade_system' based on previous steps).

fn main() {
    println!("NovaDE Starting...");

    // Instantiate CompositorState
    let state = CompositorState::new();

    if state.running {
        println!("CompositorState initialized: running = true");
    } else {
        println!("CompositorState initialized: running = false");
    }

    // Example of using the error type, though we won't trigger an error here.
    // This is more to ensure the types are accessible.
    // use novade_system::compositor::CompositorError;
    // let _example_result: Result<(), CompositorError> = Ok(());

    println!("NovaDE finished placeholder execution.");
}
