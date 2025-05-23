//! # NovaDE Kernschicht (`novade-core`)
//!
//! `novade-core` stellt die fundamentalen Bausteine für das NovaDE Linux Desktop Environment bereit.
//! Diese Schicht ist die unterste im Architekturmodell und hat keine Abhängigkeiten zu anderen
//! NovaDE-spezifischen Schichten (`novade-domain`, `novade-system`, `novade-ui`).
//!
//! ## Hauptverantwortlichkeiten:
//!
//! - **Grundlegende Datentypen**: Definition von gemeinsamen Typen wie `NovaId`, `Version`,
//!   `Timestamp` und `ResourceIdentifier`, die systemweit verwendet werden. Siehe [`types`].
//! - **Fehlerbehandlung**: Ein robustes Fehlermanagement durch das `CoreError` Enum und
//!   den `CoreResult<T>` Typalias. Siehe [`error`].
//! - **Konfigurationsmanagement**: Laden und Verwalten von Kernkonfigurationen
//!   (z.B. aus TOML-Dateien). Siehe [`config`].
//! - **Logging-Infrastruktur**: Initialisierung und Bereitstellung einer flexiblen Logging-Lösung
//!   basierend auf `tracing`. Siehe [`logging`].
//! - **Allgemeine Dienstprogramme**: Sammlung von Hilfsfunktionen für Pfadmanipulation,
//!   Dateizugriff und Ermittlung von Anwendungsverzeichnissen. Siehe [`utils`].
//!
//! ## Designprinzipien:
//!
//! - **Minimalismus**: Enthält nur wirklich grundlegende und schichtübergreifend benötigte Funktionalität.
//! - **Stabilität**: Änderungen an dieser Schicht sollten selten sein, da sie das Fundament bildet.
//! - **Portabilität**: Keine direkten Betriebssystem-spezifischen Aufrufe, die nicht abstrahiert sind
//!   (z.B. durch Crates wie `dirs`).
//!
//! ## Verwendung:
//!
//! Andere Crates im NovaDE-Workspace deklarieren `novade-core` als Abhängigkeit in ihrer `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! novade-core = { path = "../novade-core" }
//! ```
//!
//! Dann können die öffentlichen Module und Typen verwendet werden:
//!
//! ```rust,no_run
//! use novade_core::{CoreConfig, CoreResult, NovaId, initialize_logging, info, error};
//! use std::path::Path;
//!
//! fn main() -> CoreResult<()> {
//!     // Beispiel: Konfiguration laden (Pfad muss existieren und valide sein)
//!     // let config_path = Path::new("/etc/novade/core.toml");
//!     // let core_config = CoreConfig::load_from_path(config_path)?;
//!     
//!     // Dummy-Config für dieses Beispiel
//!     let core_config = CoreConfig::example(); 
//!
//!     // Logging initialisieren
//!     initialize_logging(&core_config)?;
//!
//!     info!("NovaDE-Kernkomponente startet mit ID: {}", NovaId::new());
//!
//!     // ... weitere Anwendungslogik ...
//!
//!     Ok(())
//! }
//! ```

// Module werden öffentlich gemacht
pub mod config;
pub mod error;
pub mod logging;
pub mod types;
pub mod utils;

// Re-exportiere die wichtigsten Elemente für eine einfachere Nutzung.
// Dies macht Typen wie `CoreConfig` direkt unter `novade_core::CoreConfig` verfügbar,
// anstatt `novade_core::config::CoreConfig`.
pub use config::{CoreConfig, DEFAULT_CORE_CONFIG_FILENAME};
pub use error::{CoreError, CoreResult};
pub use logging::setup::initialize_logging; // Spezifischer Pfad zur Initialisierungsfunktion
pub use logging::{debug, error, info, trace, warn, instrument, span, Level, Span}; // Logging-Makros
pub use types::{NovaId, ResourceIdentifier, Timestamp, Version};
// utils-Funktionen werden typischerweise spezifisch aufgerufen, z.B. novade_core::utils::resolve_path,
// daher werden sie hier nicht alle pauschal re-exportiert, es sei denn, es gibt sehr häufig genutzte.

/// Gibt eine Testnachricht aus, um die Funktionalität der Kernbibliothek zu demonstrieren.
///
/// Diese Funktion dient primär zu Test- und Demonstrationszwecken während der frühen
/// Entwicklungsphase.
///
/// # Beispiele
///
/// ```
/// novade_core::print_core_message();
/// ```
pub fn print_core_message() {
    // Verwendung des info! Makros aus der logging Infrastruktur
    info!(target: "novade_core_lib", "Nachricht von novade-core: System initialisiert und Logging funktioniert.");
    println!("Nachricht von novade-core: System initialisiert (via println).");
}

#[cfg(test)]
mod tests {
    use super::*; // Importiert alles aus dem lib-Modul, inkl. print_core_message und re-exportierter Typen

    #[test]
    fn test_print_core_message() {
        // Minimaler Test, um sicherzustellen, dass die Funktion aufgerufen werden kann.
        // Eine tatsächliche Überprüfung der Ausgabe wäre komplexer und hier nicht das Hauptziel.
        
        // Logging für den Test initialisieren (ignoriert Fehler, falls schon initialisiert)
        let test_config = CoreConfig::example();
        let _ = initialize_logging(&test_config);

        print_core_message(); // Ruft die Funktion auf
        // Hier könnte man z.B. prüfen, ob eine Log-Nachricht geschrieben wurde,
        // wenn man einen Test-Subscriber für `tracing` hätte.
    }

    #[test]
    fn test_core_imports_are_accessible() {
        let id = NovaId::new();
        info!("Test ID: {}", id);
        let version = Version::new(0,1,0);
        debug!("Test Version: {}", version);
        // Einfach nur prüfen, ob die re-exportierten Typen und Makros verwendet werden können.
        // Dieser Test ist mehr ein Compile-Zeit-Check.
    }
}
