// src/compositor/core/window.rs
use crate::input::{InputEvent, KeyState}; // Added KeyState for matching

/// Represents the different states a window can be in.
#[derive(Debug, Clone, PartialEq, Eq)] // Added Eq
pub enum WindowState {
    /// The window floats freely, managed by the user or specific placement logic.
    Floating,
    /// The window is tiled according to a layout policy.
    Tiled,
    /// The window is minimized and not visible.
    Minimized,
}

/// Represents a window within the compositor.
#[derive(Debug, Clone, PartialEq)]
pub struct Window {
    /// Unique identifier for the window.
    pub id: u32,
    /// Title of the window.
    pub title: String,
    /// Current width of the window in pixels.
    pub width: u32,
    /// Current height of the window in pixels.
    pub height: u32,
    /// X-coordinate of the window's top-left corner.
    pub x: i32,
    /// Y-coordinate of the window's top-left corner.
    pub y: i32,
    /// Current state of the window (e.g., floating, tiled).
    pub state: WindowState,
    /// Optional application identifier for the window.
    pub app_id: Option<String>,
    /// Indicates if the window currently has focus.
    pub focused: bool,
    /// Queue for pending input events for this window.
    pub event_queue: Vec<InputEvent>,
}

impl Window {
    /// Creates a new window.
    pub fn new(id: u32, title: String, width: u32, height: u32, x: i32, y: i32) -> Self {
        Self {
            id,
            title,
            width,
            height,
            x,
            y,
            state: WindowState::Floating,
            app_id: None,
            focused: false,
            event_queue: Vec::new(),
        }
    }

    /// Queues an input event to be processed by this window.
    ///
    /// # Arguments
    ///
    /// * `event` - The input event to queue.
    pub fn queue_event(&mut self, event: InputEvent) {
        self.event_queue.push(event);
    }

    /// Processes the pending input events for this window.
    ///
    /// This method iterates through the events in the window's queue,
    /// simulates handling them by printing information, and clears the queue.
    /// It includes a placeholder for specific key press actions.
    /// The queue is cleared after processing.
    pub fn process_event_queue(&mut self) {
        let events_to_process = std::mem::take(&mut self.event_queue);

        if !events_to_process.is_empty() {
            println!("Window [ID: {}]: Processing {} events from queue.", self.id, events_to_process.len());
        }

        for event in events_to_process {
            println!("Window [ID: {}]: Received event: {:?}", self.id, event);

            // Optional placeholder for specific event handling
            match event {
                InputEvent::Keyboard { key_code, state: KeyState::Pressed, modifiers } => {
                    // Example: 'X' key (e.g., key_code 88) with Ctrl to simulate close
                    if key_code == 88 && modifiers.ctrl { // Assuming 88 is 'X'
                        println!("Window [ID: {}]: Action: Would close (Ctrl+X received).", self.id);
                    }
                    // Example: 'F' key (e.g., key_code 70) to simulate fullscreen
                    else if key_code == 70 {
                         println!("Window [ID: {}]: Action: Would toggle fullscreen (F key received).", self.id);
                    }
                }
                _ => {
                    // Other event types are just printed for now
                }
            }
        }
    }
}
