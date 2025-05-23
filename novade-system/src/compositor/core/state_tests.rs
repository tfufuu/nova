// No `mod tests { ... }` wrapper needed here, file itself is the module.
use super::*; // Access CompositorState, Output, Window, etc. from core/mod.rs
// Or be more explicit:
// use super::state::CompositorState;
// use super::output::Output; // This is now explicitly imported below for clarity
// use super::window::Window;
// use super::seat::Seat; // If needed, but not directly used in tests.
// use super::display::Display; // If needed, but not directly used in tests.

// Added for dispatch_input_event tests
use crate::input::{InputEvent, KeyState, Modifiers as InputModifiers};

// Added for new window management tests
use crate::compositor::core::output::Output; // For testing tiling with an output
use crate::compositor::core::window::WindowState; // For asserting tiled state


#[test]
fn test_next_ids() {
    let mut state = CompositorState::new();
    assert_eq!(state.next_output_id(), 1);
    assert_eq!(state.next_output_id(), 2);
    assert_eq!(state.next_window_id(), 1);
    assert_eq!(state.next_window_id(), 2);
}

#[test]
fn test_add_remove_output() {
    let mut state = CompositorState::new();
    let output_id = state.next_output_id();
    let output = Output::new(output_id, "test_output".to_string(), 1920, 1080, 0, 0);
    state.add_output(output.clone());
    assert_eq!(state.outputs.len(), 1);
    assert_eq!(state.outputs[0].id, output_id);

    assert!(state.remove_output(output_id));
    assert_eq!(state.outputs.len(), 0);
    assert!(!state.remove_output(output_id));
}

#[test]
fn test_add_find_remove_window() {
    let mut state = CompositorState::new();
    let window_id = state.next_window_id();
    let window = Window::new(window_id, "Test Window".to_string(), 800, 600, 0, 0);
    state.add_window(window.clone());
    assert_eq!(state.windows.len(), 1);

    assert!(state.find_window(window_id).is_some());
    assert_eq!(state.find_window(window_id).unwrap().id, window_id);
    assert!(state.find_window(window_id + 100).is_none());

    if let Some(w_mut) = state.find_window_mut(window_id) {
        w_mut.title = "New Title".to_string();
    }
    assert_eq!(state.find_window(window_id).unwrap().title, "New Title");

    assert!(state.remove_window(window_id));
    assert_eq!(state.windows.len(), 0);
    assert!(!state.remove_window(window_id));
}

#[test]
fn test_focus_management() {
    let mut state = CompositorState::new();
    let window1_id = state.next_window_id();
    let window1 = Window::new(window1_id, "Window 1".to_string(), 800, 600, 0, 0);
    state.add_window(window1.clone());

    let window2_id = state.next_window_id();
    let window2 = Window::new(window2_id, "Window 2".to_string(), 1024, 768, 50, 50);
    state.add_window(window2.clone());

    assert!(state.set_focused_window_for_seat("seat0", Some(window1_id)));
    assert_eq!(state.seats[0].focused_window, Some(window1_id));
    assert!(state.find_window(window1_id).unwrap().focused);
    assert!(!state.find_window(window2_id).unwrap().focused);

    assert!(state.set_focused_window_for_seat("seat0", Some(window2_id)));
    assert_eq!(state.seats[0].focused_window, Some(window2_id));
    assert!(!state.find_window(window1_id).unwrap().focused);
    assert!(state.find_window(window2_id).unwrap().focused);

    assert!(state.set_focused_window_for_seat("seat0", None));
    assert_eq!(state.seats[0].focused_window, None);
    assert!(!state.find_window(window1_id).unwrap().focused);
    assert!(!state.find_window(window2_id).unwrap().focused);

    assert!(!state.set_focused_window_for_seat("non_existent_seat", Some(window1_id)));
}

// --- Tests for dispatch_input_event ---

// Helper to create default modifiers for tests
fn default_input_modifiers() -> InputModifiers {
    InputModifiers { shift: false, ctrl: false, alt: false, logo: false }
}

#[test]
fn test_dispatch_event_to_focused_window() {
    let mut state = CompositorState::new(); // Assumes new() creates "seat0"
    let window_id = state.next_window_id();
    let window = Window::new(window_id, "Focused Window".to_string(), 800, 600, 0, 0);
    state.add_window(window);
    state.set_focused_window_for_seat("seat0", Some(window_id));

    let event = InputEvent::Keyboard {
        key_code: 65, // 'A'
        state: KeyState::Pressed,
        modifiers: default_input_modifiers(),
    };

    assert!(state.dispatch_input_event(&event, "seat0"), "Event should be dispatched");

    let focused_window = state.find_window(window_id).expect("Window should exist");
    assert_eq!(focused_window.event_queue.len(), 1, "Event queue should have one event");
    if let InputEvent::Keyboard { key_code, .. } = focused_window.event_queue[0] {
        assert_eq!(key_code, 65);
    } else {
        panic!("Incorrect event type in queue");
    }
}

#[test]
fn test_dispatch_event_no_focused_window() {
    let mut state = CompositorState::new(); // Assumes "seat0" exists, but no window focused
    state.set_focused_window_for_seat("seat0", None); // Explicitly ensure no focus

    let event = InputEvent::Keyboard {
        key_code: 65,
        state: KeyState::Pressed,
        modifiers: default_input_modifiers(),
    };

    assert!(!state.dispatch_input_event(&event, "seat0"), "Event should not be dispatched if no window is focused");
}

#[test]
fn test_dispatch_event_invalid_seat_name() {
    let mut state = CompositorState::new();
    let window_id = state.next_window_id();
    let window = Window::new(window_id, "Test Window".to_string(), 800, 600, 0, 0);
    state.add_window(window);
    state.set_focused_window_for_seat("seat0", Some(window_id)); // Focus on valid seat

    let event = InputEvent::Keyboard {
        key_code: 65,
        state: KeyState::Pressed,
        modifiers: default_input_modifiers(),
    };

    assert!(!state.dispatch_input_event(&event, "non_existent_seat"), "Event should not be dispatched to an invalid seat");
}

#[test]
fn test_dispatch_event_stale_focused_window_id() {
    let mut state = CompositorState::new(); // Assumes "seat0" exists
    let window_id_stale = state.next_window_id(); // Get an ID

    // Set focus to an ID that doesn't correspond to an added window
    state.set_focused_window_for_seat("seat0", Some(window_id_stale));

    let event = InputEvent::Keyboard {
        key_code: 65,
        state: KeyState::Pressed,
        modifiers: default_input_modifiers(),
    };

    assert!(!state.dispatch_input_event(&event, "seat0"), "Event should not be dispatched if focused window ID is stale");
}

#[test]
fn test_dispatch_event_no_seats_at_all() { // Edge case, though new() creates one
    let mut state = CompositorState::default(); // Create a state with no default seat for this test.
    state.seats.clear(); // Ensure no seats

    let event = InputEvent::Keyboard {
        key_code: 65,
        state: KeyState::Pressed,
        modifiers: default_input_modifiers(),
    };
    assert!(!state.dispatch_input_event(&event, "seat0"), "Event dispatch should fail if no seats exist");
}

// --- Tests for new window management operations ---

#[test]
fn test_tile_windows_no_windows() {
    let mut state = CompositorState::new();
    state.tile_windows(); // Should not panic and do nothing.
    assert!(state.windows.is_empty());
}

#[test]
fn test_tile_windows_one_window_default_screen() {
    let mut state = CompositorState::new();
    let window_id = state.next_window_id();
    state.add_window(Window::new(window_id, "Win1".to_string(), 600, 400, 0, 0));

    state.tile_windows();

    assert_eq!(state.windows.len(), 1);
    let win = &state.windows[0];
    assert_eq!(win.x, 0);
    assert_eq!(win.y, 0);
    assert_eq!(win.width, 1920); // Default screen width
    assert_eq!(win.height, 1080); // Default screen height
    assert_eq!(win.state, WindowState::Tiled);
}

#[test]
fn test_tile_windows_multiple_windows_with_output() {
    let mut state = CompositorState::new();
    let output_id = state.next_output_id();
    // Custom output dimensions
    state.add_output(Output::new(output_id, "Output-1".to_string(), 1600, 900, 0, 0));

    let win1_id = state.next_window_id();
    state.add_window(Window::new(win1_id, "Win1".to_string(), 100, 100, 0, 0));
    let win2_id = state.next_window_id();
    state.add_window(Window::new(win2_id, "Win2".to_string(), 100, 100, 0, 0));

    state.tile_windows();

    assert_eq!(state.windows.len(), 2);
    let win1 = state.find_window(win1_id).unwrap();
    let win2 = state.find_window(win2_id).unwrap();

    assert_eq!(win1.width, 1600 / 2); // Tiled width based on output
    assert_eq!(win1.height, 900);    // Full height of output
    assert_eq!(win1.x, 0);
    assert_eq!(win1.y, 0);
    assert_eq!(win1.state, WindowState::Tiled);

    assert_eq!(win2.width, 1600 / 2);
    assert_eq!(win2.height, 900);
    assert_eq!(win2.x, (1600 / 2) as i32);
    assert_eq!(win2.y, 0);
    assert_eq!(win2.state, WindowState::Tiled);
}

#[test]
fn test_focus_next_window_no_windows() {
    let mut state = CompositorState::new(); // Assumes "seat0" from new()
    assert!(!state.focus_next_window("seat0"), "Should return false if no windows");
    // Re-fetch seat after potential modification
    let seat = state.seats.iter().find(|s| s.name == "seat0").unwrap();
    assert!(seat.focused_window.is_none());
}

#[test]
fn test_focus_next_window_one_window() {
    let mut state = CompositorState::new();
    let window_id = state.next_window_id();
    state.add_window(Window::new(window_id, "Win1".to_string(), 100, 100, 0, 0));

    assert!(state.focus_next_window("seat0"), "Focus should be set");
    let seat_after_first_focus = state.seats.iter().find(|s| s.name == "seat0").unwrap();
    assert_eq!(seat_after_first_focus.focused_window, Some(window_id));
    assert!(state.windows[0].focused);

    // Call again, focus should remain on the same window
    assert!(state.focus_next_window("seat0"));
    let seat_after_second_focus = state.seats.iter().find(|s| s.name == "seat0").unwrap();
    assert_eq!(seat_after_second_focus.focused_window, Some(window_id));
    assert!(state.windows[0].focused);
}

#[test]
fn test_focus_next_window_multiple_windows_cycling_and_wrapping() {
    let mut state = CompositorState::new();
    let win1_id = state.next_window_id();
    state.add_window(Window::new(win1_id, "Win1".to_string(), 100, 100, 0, 0));
    let win2_id = state.next_window_id();
    state.add_window(Window::new(win2_id, "Win2".to_string(), 100, 100, 0, 0));
    let win3_id = state.next_window_id();
    state.add_window(Window::new(win3_id, "Win3".to_string(), 100, 100, 0, 0));

    // Initial focus (should go to win1_id)
    assert!(state.focus_next_window("seat0"));
    let seat_after_focus1 = state.seats.iter().find(|s| s.name == "seat0").unwrap();
    assert_eq!(seat_after_focus1.focused_window, Some(win1_id));
    assert!(state.find_window(win1_id).unwrap().focused);
    assert!(!state.find_window(win2_id).unwrap().focused);
    assert!(!state.find_window(win3_id).unwrap().focused);

    // Focus next (should go to win2_id)
    assert!(state.focus_next_window("seat0"));
    let seat_after_focus2 = state.seats.iter().find(|s| s.name == "seat0").unwrap();
    assert_eq!(seat_after_focus2.focused_window, Some(win2_id));
    assert!(!state.find_window(win1_id).unwrap().focused);
    assert!(state.find_window(win2_id).unwrap().focused);
    assert!(!state.find_window(win3_id).unwrap().focused);

    // Focus next (should go to win3_id)
    assert!(state.focus_next_window("seat0"));
    let seat_after_focus3 = state.seats.iter().find(|s| s.name == "seat0").unwrap();
    assert_eq!(seat_after_focus3.focused_window, Some(win3_id));
    assert!(!state.find_window(win1_id).unwrap().focused);
    assert!(!state.find_window(win2_id).unwrap().focused);
    assert!(state.find_window(win3_id).unwrap().focused);

    // Focus next (should wrap to win1_id)
    assert!(state.focus_next_window("seat0"));
    let seat_after_focus4 = state.seats.iter().find(|s| s.name == "seat0").unwrap();
    assert_eq!(seat_after_focus4.focused_window, Some(win1_id));
    assert!(state.find_window(win1_id).unwrap().focused);
    assert!(!state.find_window(win2_id).unwrap().focused);
    assert!(!state.find_window(win3_id).unwrap().focused);
}

#[test]
fn test_focus_next_window_invalid_seat() {
    let mut state = CompositorState::new();
    let window_id = state.next_window_id();
    state.add_window(Window::new(window_id, "Win1".to_string(), 100, 100, 0, 0));
    assert!(!state.focus_next_window("non_existent_seat"), "Should return false for invalid seat");
}

#[test]
fn test_focus_next_window_stale_focus_id() {
    let mut state = CompositorState::new();
    let win1_id = state.next_window_id();
    state.add_window(Window::new(win1_id, "Win1".to_string(), 100, 100, 0, 0));
    let win2_id = state.next_window_id(); // This window won't be added, making its ID stale if focused

    // Manually set a stale focused_window ID on the seat
    let seat_mut = state.seats.iter_mut().find(|s| s.name == "seat0").unwrap();
    seat_mut.focused_window = Some(win2_id); // win2_id does not exist in state.windows

    // focus_next_window should recover by focusing the first available window (win1_id)
    assert!(state.focus_next_window("seat0"));
    let seat_after_focus = state.seats.iter().find(|s| s.name == "seat0").unwrap();
    assert_eq!(seat_after_focus.focused_window, Some(win1_id));
    assert!(state.find_window(win1_id).unwrap().focused);
}

// --- Tests for resize_window and move_window ---

#[test]
fn test_resize_window_success() {
    let mut state = CompositorState::new();
    let window_id = state.next_window_id();
    state.add_window(Window::new(window_id, "Resize Me".to_string(), 100, 100, 0, 0));

    assert!(state.resize_window(window_id, 200, 150), "Resize should succeed");
    let window = state.find_window(window_id).unwrap();
    assert_eq!(window.width, 200);
    assert_eq!(window.height, 150);
}

#[test]
fn test_resize_window_zero_width() {
    let mut state = CompositorState::new();
    let window_id = state.next_window_id();
    state.add_window(Window::new(window_id, "No Zero Width".to_string(), 100, 100, 0, 0));

    assert!(!state.resize_window(window_id, 0, 150), "Resize with zero width should fail");
    let window = state.find_window(window_id).unwrap();
    assert_eq!(window.width, 100, "Width should not change on failed resize");
    assert_eq!(window.height, 100, "Height should not change on failed resize");
}

#[test]
fn test_resize_window_zero_height() {
    let mut state = CompositorState::new();
    let window_id = state.next_window_id();
    state.add_window(Window::new(window_id, "No Zero Height".to_string(), 100, 100, 0, 0));

    assert!(!state.resize_window(window_id, 200, 0), "Resize with zero height should fail");
    let window = state.find_window(window_id).unwrap();
    assert_eq!(window.width, 100, "Width should not change on failed resize");
    assert_eq!(window.height, 100, "Height should not change on failed resize");
}

#[test]
fn test_resize_window_non_existent() {
    let mut state = CompositorState::new();
    let non_existent_window_id = state.next_window_id(); // Get an ID but don't add the window

    assert!(!state.resize_window(non_existent_window_id, 200, 150), "Resize of non-existent window should fail");
}

#[test]
fn test_move_window_success() {
    let mut state = CompositorState::new();
    let window_id = state.next_window_id();
    state.add_window(Window::new(window_id, "Move Me".to_string(), 100, 100, 0, 0));

    assert!(state.move_window(window_id, 50, 75), "Move should succeed");
    let window = state.find_window(window_id).unwrap();
    assert_eq!(window.x, 50);
    assert_eq!(window.y, 75);
}

#[test]
fn test_move_window_non_existent() {
    let mut state = CompositorState::new();
    let non_existent_window_id = state.next_window_id(); // Get an ID but don't add the window

    assert!(!state.move_window(non_existent_window_id, 50, 75), "Move of non-existent window should fail");
}
