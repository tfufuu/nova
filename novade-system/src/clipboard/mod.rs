// novade-system/src/clipboard/mod.rs

#[derive(Debug, Default)]
pub struct Clipboard {
    data: Option<String>,
}

impl Clipboard {
    /// Creates a new Clipboard instance.
    pub fn new() -> Self {
        Clipboard { data: None }
    }

    /// Sets the clipboard data.
    pub fn set_data(&mut self, data: String) {
        self.data = Some(data);
    }

    /// Retrieves the clipboard data.
    pub fn get_data(&self) -> Option<String> {
        self.data.clone()
    }

    /// Clears the clipboard data.
    pub fn clear_data(&mut self) {
        self.data = None;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_clipboard_new() {
        let clipboard = Clipboard::new();
        assert_eq!(clipboard.get_data(), None);
    }

    #[test]
    fn test_clipboard_set_get_data() {
        let mut clipboard = Clipboard::new();
        let test_data = "Hello, clipboard!".to_string();

        // Set data
        clipboard.set_data(test_data.clone());
        assert_eq!(clipboard.get_data(), Some(test_data.clone()));

        // Get data
        let retrieved_data = clipboard.get_data();
        assert_eq!(retrieved_data, Some(test_data));

        // Clear data and check
        clipboard.clear_data();
        assert_eq!(clipboard.get_data(), None);
    }

    #[test]
    fn test_clipboard_clear_data() {
        let mut clipboard = Clipboard::new();
        let test_data = "Some data".to_string();

        // Set some data first
        clipboard.set_data(test_data.clone());
        assert_eq!(clipboard.get_data(), Some(test_data));

        // Clear data
        clipboard.clear_data();
        assert_eq!(clipboard.get_data(), None);

        // Clear again when already empty
        clipboard.clear_data();
        assert_eq!(clipboard.get_data(), None);
    }
}
