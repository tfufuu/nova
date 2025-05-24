// src/main.rs

use novade_system::server::Server;
use novade_system::input::{InputEvent, KeyState, Modifiers};
use novade_system::compositor::core::Window; // For creating sample windows

/// Main function to demonstrate Server orchestration, window management, and multi-output concepts.
fn main() {
    println!("NovaDE Advanced Demo Starting...");

    // 1. Initialize Server
    // CompositorState::new() now initializes with a primary and secondary output.
    let mut server = Server::new();
    println!("Server initialized.");

    println!("
--- Initial Output Configuration ---");
    if server.compositor_state.outputs.is_empty() {
        println!("  No outputs initialized by default (should not happen with current CompositorState::new).");
    }
    for output in &server.compositor_state.outputs {
        println!("  Output ID: {}, Name: '{}', Geom: [{}x{} at ({},{})], Primary: {}",
                 output.id, output.name, output.width, output.height, output.x, output.y, output.is_primary);
    }

    // 2. Create sample windows
    println!("
--- Creating Sample Windows ---");
    let window_id_1 = server.compositor_state.next_window_id();
    let sample_window_1 = Window::new(
        window_id_1,
        "Window Alpha".to_string(),
        300, // Initial size, will be overridden
        200,
        10,
        10,
    );
    server.compositor_state.add_window(sample_window_1);
    println!("Added window ID: {}, Title: 'Window Alpha'", window_id_1);

    let window_id_2 = server.compositor_state.next_window_id();
    let sample_window_2 = Window::new(
        window_id_2,
        "Window Beta".to_string(),
        250,
        150,
        50,
        50,
    );
    server.compositor_state.add_window(sample_window_2);
    println!("Added window ID: {}, Title: 'Window Beta'", window_id_2);

    let window_id_3 = server.compositor_state.next_window_id();
    let sample_window_3 = Window::new(
        window_id_3,
        "Window Gamma".to_string(),
        200,
        100,
        90,
        90,
    );
    server.compositor_state.add_window(sample_window_3);
    println!("Added window ID: {}, Title: 'Window Gamma'", window_id_3);

    // 3. Set initial focus
    if server.compositor_state.set_focused_window_for_seat("seat0", Some(window_id_1)) {
        println!("Initial focus set to window ID: {}", window_id_1);
    }

    // 4. Demonstrate Resize and Move on Window Alpha (ID: window_id_1)
    println!("
--- Window Resize & Move Demonstration (Window ID: {}) ---", window_id_1);
    if let Some(win_before) = server.compositor_state.find_window(window_id_1) {
        println!("  Before resize/move: pos=({},{}), size=({}x{})",
                 win_before.x, win_before.y, win_before.width, win_before.height);
    }
    server.compositor_state.resize_window(window_id_1, 400, 350);
    server.compositor_state.move_window(window_id_1, 20, 30);
    if let Some(win_after) = server.compositor_state.find_window(window_id_1) {
        println!("  After resize/move:  pos=({},{}), size=({}x{})",
                 win_after.x, win_after.y, win_after.width, win_after.height);
    }


    // 5. Simulate some input events (these might be processed by the focused window)
    let events_to_simulate = vec![
        InputEvent::Keyboard {
            key_code: 65, // 'A' for Alpha
            state: KeyState::Pressed,
            modifiers: Modifiers { shift: true, ctrl: false, alt: false, logo: false },
        },
    ];
    println!("
Simulating {} input events...", events_to_simulate.len());
    server.run_loop_iteration(events_to_simulate);
    // Window event queues will be processed and printed by Window::process_event_queue

    // 6. Illustrate Window Tiling (should now use multi-output logic)
    println!("
--- Window Tiling Demonstration (Multi-Output Aware) ---");
    println!("Windows before tiling:");
    for window in server.compositor_state.windows.iter() {
        println!("  Window {}: pos=({},{}), size=({}x{}), state={:?}",
                 window.id, window.x, window.y, window.width, window.height, window.state);
    }
    server.compositor_state.tile_windows(); // This will use the new logic
    println!("Windows after tiling (check positions relative to the chosen output):");
    for window in server.compositor_state.windows.iter() {
        println!("  Window {}: pos=({},{}), size=({}x{}), state={:?}",
                 window.id, window.x, window.y, window.width, window.height, window.state);
    }

    // 7. Illustrate Focus Cycling
    println!("
--- Focus Cycling Demonstration (Seat 'seat0') ---");
    for i in 0..server.compositor_state.windows.len() + 1 {
        server.compositor_state.focus_next_window("seat0");
        if let Some(focused_id) = server.compositor_state.seats[0].focused_window {
             if let Some(focused_window) = server.compositor_state.find_window(focused_id) {
                println!("Iteration {}: Focus is on Window ID: {}, Title: '{}'",
                         i + 1, focused_id, focused_window.title);
             } else {
                println!("Iteration {}: Focused ID {} not found.", i + 1, focused_id);
             }
        } else {
            println!("Iteration {}: No window focused.", i + 1);
        }
    }

    println!("
NovaDE Advanced Demo Finished.");
}
