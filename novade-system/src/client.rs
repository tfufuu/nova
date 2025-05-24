// src/client.rs

//! Defines structures related to clients and their communication with the server.

/// Represents a client connected to the server.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Client {
    /// Unique identifier for the client.
    pub id: u32,
}

impl Client {
    /// Creates a new client with the given ID.
    pub fn new(id: u32) -> Self {
        Self { id }
    }
}

/// Represents requests that a client can make to the server.
#[derive(Debug, Clone, PartialEq)]
pub enum ClientRequest {
    /// Request to create a new window.
    CreateWindow {
        /// The ID of the client making the request.
        client_id: u32,
        /// The initial title for the new window.
        title: String,
        /// The requested initial width of the new window.
        initial_width: u32,
        /// The requested initial height of the new window.
        initial_height: u32,
    },
    // Future requests:
    // CloseWindow { client_id: u32, window_id: u32 },
    // MapWindow { client_id: u32, window_id: u32 },
    // UnmapWindow { client_id: u32, window_id: u32 },
}

/// Represents events that the server can send to clients (or use internally for now).
#[derive(Debug, Clone, PartialEq)]
pub enum ServerEvent {
    /// Indicates that a new window was successfully created.
    WindowCreated {
        /// The ID of the newly created window.
        window_id: u32,
        /// The ID of the client that owns the window.
        client_id: u32,
        /// The initial geometry (x, y, width, height) of the created window.
        initial_geometry: (i32, i32, u32, u32),
    },
    // Future events:
    // WindowClosed { window_id: u32 },
    // WindowMapped { window_id: u32 },
    // WindowUnmapped { window_id: u32 },
}
