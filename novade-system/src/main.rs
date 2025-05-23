// src/main.rs

use novade_system::server::Server;
use novade_system::input::{InputEvent, KeyState, Modifiers};
use novade_system::compositor::core::Window; // For creating sample windows

/// Main function to demonstrate Server orchestration and window management.
fn main() {
    println!("NovaDE Server Orchestration & Window Management Demo Starting...");

    // 1. Initialize Server
    let mut server = Server::new();
    println!("Server initialized.");

    // 2. Create sample windows
    let window_id_1 = server.compositor_state.next_window_id();
    let sample_window_1 = Window::new(
        window_id_1,
        "Window Alpha".to_string(),
        600, // Initial size, will be overridden by tiling
        400,
        10,  // Initial position, will be overridden
        10,
    );
    server.compositor_state.add_window(sample_window_1);
    println!("Added window with ID: {}", window_id_1);

    let window_id_2 = server.compositor_state.next_window_id();
    let sample_window_2 = Window::new(
        window_id_2,
        "Window Beta".to_string(),
        500,
        300,
        50,
        50,
    );
    server.compositor_state.add_window(sample_window_2);
    println!("Added window with ID: {}", window_id_2);

    let window_id_3 = server.compositor_state.next_window_id();
    let sample_window_3 = Window::new(
        window_id_3,
        "Window Gamma".to_string(),
        400,
        200,
        90,
        90,
    );
    server.compositor_state.add_window(sample_window_3);
    println!("Added window with ID: {}", window_id_3);


    // 3. Set initial focus (e.g., to the first window)
    if server.compositor_state.set_focused_window_for_seat("seat0", Some(window_id_1)) {
        println!("Initial focus set to window ID: {}", window_id_1);
    } else {
        println!("Failed to set initial focus.");
    }

    // 4. Simulate some input events
    let events_to_simulate = vec![
        InputEvent::Keyboard {
            key_code: 65, // 'A' for Alpha
            state: KeyState::Pressed,
            modifiers: Modifiers { shift: true, ctrl: false, alt: false, logo: false },
        },
        InputEvent::Keyboard {
            key_code: 70, // 'F' key (handled by Window::process_event_queue)
            state: KeyState::Pressed,
            modifiers: Modifiers::default(),
        },
    ];
    println!("\nSimulating {} input events...", events_to_simulate.len());
    server.run_loop_iteration(events_to_simulate);
    // Window event queues will be processed and printed by Window::process_event_queue

    // 5. Illustrate Window Tiling
    println!("\n--- Window Tiling Demonstration ---");
    println!("Windows before tiling:");
    for window in server.compositor_state.windows.iter() {
        println!("  Window {}: pos=({},{}), size=({}x{}), state={:?}",
                 window.id, window.x, window.y, window.width, window.height, window.state);
    }
    server.compositor_state.tile_windows();
    println!("Windows after tiling:");
    for window in server.compositor_state.windows.iter() {
        println!("  Window {}: pos=({},{}), size=({}x{}), state={:?}",
                 window.id, window.x, window.y, window.width, window.height, window.state);
    }

    // 6. Illustrate Focus Cycling
    println!("\n--- Focus Cycling Demonstration (Seat 'seat0') ---");
    for i in 0..server.compositor_state.windows.len() + 1 { // Cycle a bit more to show wrap
        server.compositor_state.focus_next_window("seat0");
        if let Some(focused_id) = server.compositor_state.seats[0].focused_window {
             if let Some(focused_window) = server.compositor_state.find_window(focused_id) {
                println!("Iteration {}: Focus is on Window ID: {}, Title: '{}'",
                         i + 1, focused_id, focused_window.title);
             } else {
                println!("Iteration {}: Focused ID {} not found (should not happen if consistent).", i + 1, focused_id);
             }
        } else {
            println!("Iteration {}: No window focused.", i + 1);
        }
    }

    // Simulate an event for the newly focused window
    if server.compositor_state.seats[0].focused_window.is_some() {
        let final_key_event = InputEvent::Keyboard {
            key_code: 88, // 'X'
            state: KeyState::Pressed,
            modifiers: Modifiers { ctrl: true, shift: false, alt: false, logo: false },
        };
        println!("\nSimulating one more event for currently focused window: {:?}", final_key_event);
        server.run_loop_iteration(vec![final_key_event]);
    }


    println!("\nNovaDE Server Orchestration & Window Management Demo Finished.");
}
