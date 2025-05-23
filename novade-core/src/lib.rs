//! novade-core: Grundlegende Datentypen, Dienstprogramme, Konfigurationsgrundlagen,
//! Infrastruktur für die Protokollierung sowie allgemeine Fehlerdefinitionen.

pub mod config;
pub mod error;
pub mod logging;
pub mod types;
pub mod utils;

/// Gibt eine Testnachricht aus, um die Funktionalität der Kernbibliothek zu demonstrieren.
pub fn print_core_message() {
    println!("Nachricht von novade-core: System initialisiert.");
}
