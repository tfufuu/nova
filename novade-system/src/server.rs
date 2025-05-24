// src/server.rs

use crate::clipboard::Clipboard;
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
    /// The server's clipboard instance.
    pub clipboard: Clipboard,
}

impl Server {
    /// Creates a new `Server` instance with default states.
    pub fn new() -> Self {
        Self {
            compositor_state: CompositorState::new(),
            input_manager: InputManager::new(),
            clients: Vec::new(),
            next_client_id: 1,
            clipboard: Clipboard::new(),
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
            ClientRequest::CopyText { client_id, text } => {
                if !self.clients.iter().any(|c| c.id == client_id) {
                    eprintln!("Server Error: CopyText request from non-existent client ID: {}", client_id);
                    return None;
                }
                self.set_clipboard_data(text);
                Some(ServerEvent::TextCopied { client_id })
            }
            ClientRequest::PasteTextRequest { client_id } => {
                if !self.clients.iter().any(|c| c.id == client_id) {
                    eprintln!("Server Error: PasteTextRequest from non-existent client ID: {}", client_id);
                    return None;
                }
                let text = self.get_clipboard_data();
                Some(ServerEvent::PasteTextResponse { client_id, text })
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
            println!("Server: Processing raw event: {:?}", event);
            let processed_event = self.input_manager.process_simulated_raw_event(event);

            match processed_event {
                InputEvent::CopyShortcut => {
                    // In a real scenario, we'd try to get data from the "focused" window.
                    // For now, we simulate this with a predefined string.
                    let data_to_copy = "Simulated copied text from active window".to_string();
                    self.set_clipboard_data(data_to_copy);
                    println!("Server: Detected CopyShortcut, data set to clipboard.");
                    // This event is handled by the server, not dispatched to windows.
                }
                InputEvent::PasteShortcut => {
                    let clipboard_content = self.get_clipboard_data();
                    if let Some(data) = clipboard_content {
                        println!("Server: Detected PasteShortcut, data: '{}' would be sent to active window.", data);
                        // In a real scenario, this data would be sent to the focused window.
                        // For now, we just print it.
                    } else {
                        println!("Server: Detected PasteShortcut, no data in clipboard.");
                    }
                    // This event is handled by the server, not dispatched to windows.
                }
                _ => {
                    // If it's not a server-handled shortcut, dispatch it to the compositor
                    println!("Server: Dispatching processed event to compositor: {:?}", processed_event);
                    let dispatched = self.compositor_state.dispatch_input_event(&processed_event, "seat0");
                    if dispatched {
                        println!("Server: Event dispatched successfully to focused window on seat0.");
                    } else {
                        println!("Server: Event dispatch failed or no window focused on seat0 for event: {:?}", processed_event);
                    }
                }
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

    /// Sets the server's clipboard data.
    pub fn set_clipboard_data(&mut self, data: String) {
        self.clipboard.set_data(data);
        println!("Server: Clipboard data set.");
    }

    /// Retrieves the server's clipboard data.
    pub fn get_clipboard_data(&self) -> Option<String> {
        let data = self.clipboard.get_data();
        println!("Server: Clipboard data retrieved.");
        data
    }

    /// Clears the server's clipboard data.
    pub fn clear_clipboard_data(&mut self) {
        self.clipboard.clear_data();
        println!("Server: Clipboard data cleared.");
    }
}

impl Default for Server { fn default() -> Self { Self::new() } } // Ensure this is present

#[cfg(test)]
mod tests {
    use super::*;
    use crate::client::{ClientRequest, ServerEvent};
    use crate::input::event::{InputEvent, KeyState, Modifiers};

    // Redefine key codes for testing purposes if not accessible
    const KEY_C: u32 = 67;
    const KEY_V: u32 = 86;

    fn create_server_with_client() -> (Server, u32) {
        let mut server = Server::new();
        let client_id = server.add_client();
        (server, client_id)
    }

    #[test]
    fn test_server_handle_copy_text_request() {
        let (mut server, client_id) = create_server_with_client();
        let text_to_copy = "Hello from client".to_string();

        let request = ClientRequest::CopyText {
            client_id,
            text: text_to_copy.clone(),
        };
        let response = server.process_client_request(request);

        assert_eq!(response, Some(ServerEvent::TextCopied { client_id }));
        assert_eq!(server.get_clipboard_data(), Some(text_to_copy));
    }

    #[test]
    fn test_server_handle_paste_text_request() {
        let (mut server, client_id) = create_server_with_client();
        let clipboard_content = "Existing clipboard data".to_string();
        server.set_clipboard_data(clipboard_content.clone());

        let request = ClientRequest::PasteTextRequest { client_id };
        let response = server.process_client_request(request);

        assert_eq!(
            response,
            Some(ServerEvent::PasteTextResponse {
                client_id,
                text: Some(clipboard_content)
            })
        );
    }

    #[test]
    fn test_server_handle_paste_text_request_empty_clipboard() {
        let (mut server, client_id) = create_server_with_client();
        server.clear_clipboard_data(); // Ensure clipboard is empty

        let request = ClientRequest::PasteTextRequest { client_id };
        let response = server.process_client_request(request);

        assert_eq!(
            response,
            Some(ServerEvent::PasteTextResponse {
                client_id,
                text: None
            })
        );
    }

    #[test]
    fn test_server_handle_clipboard_request_invalid_client() {
        let mut server = Server::new();
        let invalid_client_id = 999; // A client ID that has not been added

        // Test CopyText with invalid client
        let copy_request = ClientRequest::CopyText {
            client_id: invalid_client_id,
            text: "test".to_string(),
        };
        let copy_response = server.process_client_request(copy_request);
        assert_eq!(copy_response, None);

        // Test PasteTextRequest with invalid client
        let paste_request = ClientRequest::PasteTextRequest {
            client_id: invalid_client_id,
        };
        let paste_response = server.process_client_request(paste_request);
        assert_eq!(paste_response, None);
    }

    #[test]
    fn test_server_handle_copy_shortcut() {
        let mut server = Server::new();
        let ctrl_c_event = InputEvent::Keyboard {
            key_code: KEY_C,
            state: KeyState::Pressed,
            modifiers: Modifiers { ctrl: true, shift: false, alt: false, logo: false },
        };

        server.run_loop_iteration(vec![ctrl_c_event]);

        assert_eq!(
            server.get_clipboard_data(),
            Some("Simulated copied text from active window".to_string())
        );
    }

    #[test]
    fn test_server_handle_paste_shortcut() {
        let mut server = Server::new();
        let initial_clipboard_data = "Test paste data".to_string();
        server.set_clipboard_data(initial_clipboard_data.clone());

        let ctrl_v_event = InputEvent::Keyboard {
            key_code: KEY_V,
            state: KeyState::Pressed,
            modifiers: Modifiers { ctrl: true, shift: false, alt: false, logo: false },
        };

        // Note: This test primarily checks that the clipboard state remains unchanged
        // and that the server processes the event (indicated by println! in implementation).
        // A more robust test would capture stdout or use a more testable event handling mechanism.
        server.run_loop_iteration(vec![ctrl_v_event]);

        // Paste operation should not clear the clipboard
        assert_eq!(server.get_clipboard_data(), Some(initial_clipboard_data));
        
        // Further checks could involve capturing stdout if the test environment supports it
        // and looking for "Server: Detected PasteShortcut, data: 'Test paste data'..."
    }
}
