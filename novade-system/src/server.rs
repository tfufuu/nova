// src/server.rs

use crate::compositor::core::CompositorState;
use crate::input::{InputManager, InputEvent};

/// Represents the main server instance, orchestrating compositor and input logic.
#[derive(Debug)] // Default might not be appropriate if state needs specific setup beyond default
pub struct Server {
    /// The state of the compositor (windows, outputs, etc.).
    pub compositor_state: CompositorState,
    /// The manager for input devices and their states.
    pub input_manager: InputManager,
}

impl Server {
    /// Creates a new `Server` instance with default compositor and input states.
    pub fn new() -> Self {
        Self {
            compositor_state: CompositorState::new(), // Assumes CompositorState::new() is preferred over default()
            input_manager: InputManager::new(),   // Assumes InputManager::new() is preferred
        }
    }

    /// Runs a single iteration of the main event loop.
    ///
    /// This method processes a batch of simulated input events, updates compositor state,
    /// and then allows windows to process their queued events.
    ///
    /// # Arguments
    ///
    /// * `simulated_events` - A vector of `InputEvent`s to be processed in this iteration.
    pub fn run_loop_iteration(&mut self, simulated_events: Vec<InputEvent>) {
        println!("Server: Starting loop iteration with {} simulated events.", simulated_events.len());

        // 1. Process all incoming simulated events
        for event in simulated_events {
            println!("Server: Processing event: {:?}", event);
            let processed_event = self.input_manager.process_simulated_raw_event(event);
            println!("Server: Dispatching processed event: {:?}", processed_event);
            let dispatched = self.compositor_state.dispatch_input_event(&processed_event, "seat0");
            if dispatched {
                println!("Server: Event dispatched successfully to focused window on seat0.");
            } else {
                println!("Server: Event dispatch failed or no window focused on seat0.");
            }
        }

        // 2. After all input events are processed and dispatched,
        //    allow windows to process their event queues.
        println!("Server: Allowing windows to process their event queues...");
        for window in self.compositor_state.windows.iter_mut() {
            // In the next step, process_event_queue will be refined to do more.
            // For now, it takes the events and might print them or do nothing.
            // If it were to return events, we might collect them here.
            // E.g., let handled_by_window = window.process_event_queue();
            // println!("Server: Window {} processed {} events.", window.id, handled_by_window.len());
            window.process_event_queue(); // Assuming it modifies itself or prints for now
        }
        println!("Server: Loop iteration finished.");
    }
}

impl Default for Server {
    fn default() -> Self {
        Self::new()
    }
}
