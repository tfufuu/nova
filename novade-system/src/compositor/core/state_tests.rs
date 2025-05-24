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
    // Let's adjust the expectation based on new() creating two outputs
    let mut state_for_ids = CompositorState::new(); // Fresh state
    assert_eq!(state_for_ids.outputs.len(), 2); // Confirm 2 outputs from new()
    // The internal counter next_output_id is private. We test its effect via the public method.
    // Initial state: output IDs 1 and 2 are used. So next_output_id() should start at 3.
    assert_eq!(state_for_ids.next_output_id(), 3); // First call after new()
    assert_eq!(state_for_ids.next_output_id(), 4); // Second call

    let mut state_for_win_ids = CompositorState::new(); // Fresh state for window IDs
    assert_eq!(state_for_win_ids.next_window_id(), 1);
    assert_eq!(state_for_win_ids.next_window_id(), 2);
}

#[test]
fn test_add_remove_output() {
    let mut state = CompositorState::new();
    // new() already adds 2 outputs. Clear them for a clean test of add/remove.
    state.outputs.clear();
    assert_eq!(state.outputs.len(), 0);
    // We cannot directly reset state.next_output_id as it's private.
    // The IDs will continue from where new() left off (e.g., 3, 4, ...). This is acceptable.
    // Or, for truly isolated ID testing, new() would need to not add outputs by default,
    // but that's a larger change. We'll work with continued IDs.

    let output_id1 = state.next_output_id(); // Will be 3 if new() created 2 outputs
    let output1 = Output::new(output_id1, "test_output1".to_string(), 1920, 1080, 0, 0, false);
    state.add_output(output1.clone());
    assert_eq!(state.outputs.len(), 1);
    assert_eq!(state.outputs[0].id, output_id1);

    let output_id2 = state.next_output_id(); // Will be 4
    let output2 = Output::new(output_id2, "test_output2".to_string(), 1280, 720, 0, 0, false);
    state.add_output(output2.clone());
    assert_eq!(state.outputs.len(), 2);


    assert!(state.remove_output(output_id1));
    assert_eq!(state.outputs.len(), 1);
    assert_eq!(state.outputs[0].id, output_id2); // Check remaining output

    assert!(state.remove_output(output_id2));
    assert_eq!(state.outputs.len(), 0);
    assert!(!state.remove_output(output_id1)); // Try removing again
    assert!(!state.remove_output(output_id2));
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
    state.outputs.clear(); // Clear default outputs for this specific test
    state.tile_windows(); // Should not panic and do nothing.
    assert!(state.windows.is_empty());
}

#[test]
fn test_tile_windows_one_window_default_screen() {
    let mut state = CompositorState::new();
    state.outputs.clear(); // Ensure tiling uses default screen, not default outputs
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
    state.outputs.clear(); // Clear defaults to add a specific one
    let output_id = state.next_output_id(); // ID will be 3
    state.add_output(Output::new(output_id, "Output-1".to_string(), 1600, 900, 0, 0, true)); // Make it primary

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

// --- Tests for multi-output logic ---

#[test]
fn test_compositor_state_new_initializes_outputs() {
    let state = CompositorState::new();
    assert_eq!(state.outputs.len(), 2, "Should initialize with two outputs by default");

    let primary_output = state.outputs.iter().find(|o| o.is_primary);
    assert!(primary_output.is_some(), "A primary output should exist");
    if let Some(po) = primary_output {
        assert_eq!(po.x, 0);
        assert_eq!(po.y, 0);
        assert_eq!(po.width, 1920);
        assert_eq!(po.height, 1080);
        assert_eq!(po.name, "Primary-1920x1080");
    }

    let secondary_output = state.outputs.iter().find(|o| !o.is_primary);
    assert!(secondary_output.is_some(), "A secondary output should exist");
    if let Some(so) = secondary_output {
        assert_eq!(so.x, 1920);
        assert_eq!(so.y, 0);
        assert_eq!(so.width, 1280);
        assert_eq!(so.height, 720);
        assert_eq!(so.name, "Secondary-1280x720");
    }
}

#[test]
fn test_tile_windows_on_primary_output() {
    let mut state = CompositorState::new(); // new() creates a primary and a secondary output
    let win1_id = state.next_window_id();
    state.add_window(Window::new(win1_id, "W1".to_string(), 10,10,0,0));
    let win2_id = state.next_window_id();
    state.add_window(Window::new(win2_id, "W2".to_string(), 10,10,0,0));

    state.tile_windows(); // Should tile on the primary output (1920x1080 at 0,0)

    let primary_output = state.outputs.iter().find(|o| o.is_primary).unwrap();
    let expected_width = primary_output.width / 2;

    let w1 = state.find_window(win1_id).unwrap();
    assert_eq!(w1.x, primary_output.x);
    assert_eq!(w1.y, primary_output.y);
    assert_eq!(w1.width, expected_width);
    assert_eq!(w1.height, primary_output.height);
    assert_eq!(w1.state, WindowState::Tiled);

    let w2 = state.find_window(win2_id).unwrap();
    assert_eq!(w2.x, primary_output.x + expected_width as i32);
    assert_eq!(w2.y, primary_output.y);
    assert_eq!(w2.width, expected_width);
    assert_eq!(w2.height, primary_output.height);
    assert_eq!(w2.state, WindowState::Tiled);
}

#[test]
fn test_tile_windows_on_first_output_if_no_primary() {
    let mut state = CompositorState::new();
    state.outputs.clear(); // Remove default outputs
    // Do not reset state.next_output_id here, let it continue from where new() left off.
    // This makes the test more robust to changes in new() regarding output ID generation.
    let out1_id = state.next_output_id(); // e.g., 3
    state.add_output(Output::new(out1_id, "OutputA-1000x600".to_string(), 1000, 600, 0, 0, false));
    let out2_id = state.next_output_id(); // e.g., 4
    state.add_output(Output::new(out2_id, "OutputB-800x500".to_string(), 800, 500, 1000, 0, false));


    let win1_id = state.next_window_id();
    state.add_window(Window::new(win1_id, "W1".to_string(), 10,10,0,0));

    state.tile_windows(); // Should tile on "OutputA" as it's the first one

    let first_output = &state.outputs[0]; // OutputA
    assert_eq!(first_output.name, "OutputA-1000x600");

    let w1 = state.find_window(win1_id).unwrap();
    assert_eq!(w1.x, first_output.x);
    assert_eq!(w1.y, first_output.y);
    assert_eq!(w1.width, first_output.width); // Only one window, takes full width
    assert_eq!(w1.height, first_output.height);
    assert_eq!(w1.state, WindowState::Tiled);
}

#[test]
fn test_tile_windows_with_no_outputs_uses_default_screen() {
    let mut state = CompositorState::new();
    state.outputs.clear(); // Ensure no outputs

    let win1_id = state.next_window_id();
    state.add_window(Window::new(win1_id, "W1".to_string(), 10,10,0,0));

    state.tile_windows(); // Should use default 1920x1080 at (0,0)

    let w1 = state.find_window(win1_id).unwrap();
    assert_eq!(w1.x, 0); // Default screen x
    assert_eq!(w1.y, 0); // Default screen y
    assert_eq!(w1.width, 1920); // Default screen width
    assert_eq!(w1.height, 1080); // Default screen height
    assert_eq!(w1.state, WindowState::Tiled);
}

#[test]
fn test_tile_windows_relative_to_output_origin() {
    let mut state = CompositorState::new();
    state.outputs.clear(); // Remove default outputs
    // Let next_output_id continue from where new() left off.
    let out_id = state.next_output_id(); // e.g., 3
    state.add_output(Output::new(out_id, "OffsetOutput-1024x768".to_string(), 1024, 768, 500, 300, true));

    let win1_id = state.next_window_id();
    state.add_window(Window::new(win1_id, "W1".to_string(), 10,10,0,0));
    let win2_id = state.next_window_id();
    state.add_window(Window::new(win2_id, "W2".to_string(), 10,10,0,0));

    state.tile_windows();

    let target_output = state.outputs.iter().find(|o| o.is_primary).unwrap();
    let expected_width = target_output.width / 2;

    let w1 = state.find_window(win1_id).unwrap();
    assert_eq!(w1.x, target_output.x); // X relative to output's X
    assert_eq!(w1.y, target_output.y); // Y relative to output's Y
    assert_eq!(w1.width, expected_width);
    assert_eq!(w1.height, target_output.height);

    let w2 = state.find_window(win2_id).unwrap();
    assert_eq!(w2.x, target_output.x + expected_width as i32); // X relative to output's X
    assert_eq!(w2.y, target_output.y);
}
