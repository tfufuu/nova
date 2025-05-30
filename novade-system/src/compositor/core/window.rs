// src/compositor/core/window.rs
use crate::input::InputEvent;
// KeyState is used in process_event_queue, ensure crate::input::KeyState is used if not already.
use crate::input::KeyState;


/// Represents the different states a window can be in.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WindowState {
    /// The window floats freely, managed by the user or specific placement logic.
    Floating,
    /// The window is tiled according to a layout policy.
    Tiled,
    /// The window is minimized and not visible.
    Minimized,
    // Unmapped, // Considered, but is_mapped field is used instead for now.
}

/// Represents a window within the compositor.
#[derive(Debug, Clone, PartialEq)]
pub struct Window {
    /// Unique identifier for the window.
    pub id: u32,
    /// ID of the client that owns this window.
    pub client_id: u32,
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
    /// Whether the window is currently mapped (visible and interactable).
    ///
    /// A window becomes mapped after it has been created and its client
    /// requests it to be shown. Unmapped windows are not drawn or interactive.
    pub is_mapped: bool,
    /// Queue for pending input events for this window.
    pub event_queue: Vec<InputEvent>,
}

impl Window {
    /// Creates a new window associated with a client.
    ///
    /// Windows are initially unmapped and will have default non-zero dimensions
    /// if `width` or `height` are provided as zero.
    ///
    /// # Arguments
    /// * `id` - Unique ID for the new window.
    /// * `client_id` - ID of the client that owns this window.
    /// * `title` - Initial title for the window.
    /// * `width` - Requested initial width. If 0, defaults to 100.
    /// * `height` - Requested initial height. If 0, defaults to 100.
    /// * `x` - Initial x-coordinate.
    /// * `y` - Initial y-coordinate.
    pub fn new(
        id: u32,
        client_id: u32,
        title: String,
        width: u32,
        height: u32,
        x: i32,
        y: i32,
    ) -> Self {
        Self {
            id,
            client_id,
            title,
            width: if width == 0 { 100 } else { width },
            height: if height == 0 { 100 } else { height },
            x,
            y,
            state: WindowState::Floating,
            app_id: None,
            focused: false,
            is_mapped: false, // Initialized to false
            event_queue: Vec::new(),
        }
    }

    /// Queues an input event to be processed by this window.
    pub fn queue_event(&mut self, event: InputEvent) {
        self.event_queue.push(event);
    }

    /// Processes the pending input events for this window.
    pub fn process_event_queue(&mut self) {
        let events_to_process = std::mem::take(&mut self.event_queue);
        if !events_to_process.is_empty() {
            println!("Window [ID: {}, ClientID: {}]: Processing {} events from queue.", self.id, self.client_id, events_to_process.len());
        }
        for event in events_to_process {
            println!("Window [ID: {}, ClientID: {}]: Received event: {:?}", self.id, self.client_id, event);
            match event {
                InputEvent::Keyboard { key_code, state: KeyState::Pressed, modifiers } => { // Ensure KeyState is correctly pathed
                    if key_code == 88 && modifiers.ctrl {
                        println!("Window [ID: {}, ClientID: {}]: Action: Would close (Ctrl+X received).", self.id, self.client_id);
                    } else if key_code == 70 {
                         println!("Window [ID: {}, ClientID: {}]: Action: Would toggle fullscreen (F key received).", self.id, self.client_id);
                    }
                }
                _ => {}
            }
        }
    }

    /// Marks the window as mapped (visible and interactable).
    /// Typically called after the client has prepared the window content.
    pub fn map(&mut self) {
        if !self.is_mapped {
            self.is_mapped = true;
            println!("Window [ID: {}, ClientID: {}]: Mapped.", self.id, self.client_id);
        }
    }

    /// Marks the window as unmapped (hidden and not interactable).
    /// Unmapping also removes focus from the window.
    pub fn unmap(&mut self) {
        if self.is_mapped {
            self.is_mapped = false;
            self.focused = false; // Unmapping should also remove focus
            println!("Window [ID: {}, ClientID: {}]: Unmapped.", self.id, self.client_id);
        }
    }
}
