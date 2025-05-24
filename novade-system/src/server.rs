// src/server.rs

use crate::compositor::core::{CompositorState, Window}; // Window needs to be in scope
use crate::input::{InputManager, InputEvent};
use crate::client::{Client, ClientRequest, ServerEvent}; // ClientRequest, ServerEvent needed

/// Represents the main server instance, orchestrating compositor and input logic.
#[derive(Debug)]
pub struct Server {
    /// The state of the compositor (windows, outputs, etc.).
    pub compositor_state: CompositorState,
    /// The manager for input devices and their states.
    pub input_manager: InputManager,
    /// List of connected clients.
    pub clients: Vec<Client>,
    /// Counter for generating unique client IDs.
    next_client_id: u32,
}

impl Server {
    /// Creates a new `Server` instance with default states.
    pub fn new() -> Self {
        Self {
            compositor_state: CompositorState::new(),
            input_manager: InputManager::new(),
            clients: Vec::new(),
            next_client_id: 1,
        }
    }

    /// Adds a new client to the server.
    ///
    /// A new client instance is created with a unique ID, added to the server's
    /// list of clients, and its ID is returned.
    ///
    /// # Returns
    /// The ID of the newly added client.
    pub fn add_client(&mut self) -> u32 {
        let client_id = self.next_client_id;
        self.next_client_id += 1;
        let new_client = Client::new(client_id);
        self.clients.push(new_client);
        println!("Server: Client added with ID: {}", client_id);
        client_id
    }

    /// Processes a request from a client.
    ///
    /// # Arguments
    /// * `request` - The `ClientRequest` to process.
    ///
    /// # Returns
    /// An `Option<ServerEvent>` which might contain an event to be sent back
    /// to the client or broadcast, or `None` if the request generates no immediate event
    /// or is invalid.
    pub fn process_client_request(&mut self, request: ClientRequest) -> Option<ServerEvent> {
        println!("Server: Received client request: {:?}", request);
        match request {
            ClientRequest::CreateWindow { client_id, title, initial_width, initial_height } => {
                // Verify client_id exists
                if !self.clients.iter().any(|c| c.id == client_id) {
                    eprintln!("Server Error: Attempt to create window for non-existent client ID: {}", client_id);
                    return None;
                }

                let window_id = self.compositor_state.next_window_id();
                
                // Define initial position (e.g., fixed, cascaded, or from layout manager in future)
                // For now, let's use a simple fixed offset or a small cascade.
                let num_windows = self.compositor_state.windows.len() as i32;
                let initial_x = 50 + num_windows * 20; // Simple cascade
                let initial_y = 50 + num_windows * 20; // Simple cascade

                let new_window = Window::new(
                    window_id,
                    client_id,
                    title,
                    initial_width,  // Window::new handles 0 width/height
                    initial_height, // Window::new handles 0 width/height
                    initial_x,
                    initial_y,
                );
                let geometry = (new_window.x, new_window.y, new_window.width, new_window.height);
                
                self.compositor_state.add_window(new_window);
                println!("Server: Window {} created for client {} at ({},{}) size {}x{}",
                         window_id, client_id, geometry.0, geometry.1, geometry.2, geometry.3);

                Some(ServerEvent::WindowCreated {
                    window_id,
                    client_id,
                    initial_geometry: geometry,
                })
            }
            // Handle other ClientRequest variants here in the future
            // _ => {
            //     println!("Server: Received unhandled client request type.");
            //     None
            // }
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
            window.process_event_queue();
        }
        println!("Server: Loop iteration finished.");
    }
}

impl Default for Server { fn default() -> Self { Self::new() } } // Ensure this is present
