//! novade-ui: Implementierung der Benutzeroberfläche auf Basis von GTK4 und gtk4-rs.

// Beispiel für UI-Module
// pub mod widgets;
// pub mod windows;
// pub mod application;

/// Gibt eine Testnachricht aus, um die Funktionalität der UI-Schicht zu demonstrieren.
pub fn print_ui_message() {
    novade_system::print_system_message(); // Beispiel für Aufruf aus novade-system
    println!("Nachricht von novade-ui: Benutzeroberfläche bereit.");
}
