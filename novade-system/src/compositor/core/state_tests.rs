// No `mod tests { ... }` wrapper needed here, file itself is the module.
use super::*; // Access CompositorState, Output, Window, etc. from core/mod.rs
// Or be more explicit:
// use super::state::CompositorState;
// use super::output::Output;
// use super::window::Window;
// use super::seat::Seat; // If needed, but not directly used in tests.
// use super::display::Display; // If needed, but not directly used in tests.

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
