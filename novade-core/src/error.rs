//! # Fehlerbehandlung in `novade-core`
//!
//! Dieses Modul definiert die zentralen Fehler-Typen für die `novade-core` Schicht.
//! Das Haupt-Enum ist [`CoreError`], und [`CoreResult<T>`] ist ein praktischer Typalias
//! für `Result<T, CoreError>`.
//!
//! ## Verwendung
//!
//! Funktionen innerhalb von `novade-core` und Funktionen in anderen Crates, die Fehler
//! aus `novade-core` weitergeben können, sollten `CoreResult<T>` als Rückgabetyp verwenden.
//!
//! ```rust,no_run
//! use novade_core::error::{CoreResult, CoreError};
//! use std::path::Path;
//!
//! fn do_something_that_might_fail(path: &Path) -> CoreResult<()> {
//!     if !path.exists() {
//!         // Beispiel für die Erzeugung eines CoreError
//!         return Err(CoreError::InvalidPathError{ path: path.to_string_lossy().into_owned(), message: "Pfad existiert nicht.".to_string()});
//!     }
//!     // ... weitere Logik ...
//!     Ok(())
//! }
//! ```

use thiserror::Error;
use std::path::PathBuf;

/// Ein Alias für `Result<T, CoreError>`, der die Fehlerbehandlung in `novade-core` vereinfacht.
///
/// Anstelle von `Result<MyType, novade_core::error::CoreError>` kann einfach `CoreResult<MyType>`
/// geschrieben werden, vorausgesetzt, `CoreError` ist im aktuellen Gültigkeitsbereich.
pub type CoreResult<T> = Result<T, CoreError>;

/// Haupt-Enum für alle Fehler, die innerhalb der `novade-core` Schicht auftreten können.
///
/// Jede Variante repräsentiert eine spezifische Fehlerbedingung. Die `#[error(...)]` Attribute
/// von `thiserror` werden verwendet, um aussagekräftige Fehlermeldungen zu generieren.
#[derive(Debug, Error)]
pub enum CoreError {
    /// Fehler beim Laden einer Konfigurationsdatei vom Dateisystem.
    #[error("Konfiguration konnte nicht von Pfad '{path}' geladen werden: {source}")]
    ConfigLoadError {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },

    /// Fehler beim Parsen des Inhalts einer Konfigurationsdatei (z.B. ungültiges TOML).
    #[error("Konfigurationsdatei-Inhalt konnte nicht geparst werden (Format: {format}): {message}")]
    ConfigParseError { format: String, message: String },

    /// Fehler während der Initialisierung der Logging-Infrastruktur.
    #[error("Fehler bei der Initialisierung des Loggings: {0}")]
    LoggingInitError(String),

    /// Ein allgemeiner Ein-/Ausgabe-Fehler ist aufgetreten.
    ///
    /// Diese Variante kann durch `#[from]` automatisch aus `std::io::Error` konvertiert werden.
    #[error("Ein E/A-Fehler ist aufgetreten: {0}")]
    IoError(#[from] std::io::Error),

    /// Fehler bei der Serialisierung von Daten in ein bestimmtes Format (z.B. JSON, TOML).
    #[error("Fehler bei der Serialisierung von Daten (Format: {format}): {message}")]
    SerializationError { format: String, message: String },

    /// Fehler bei der Deserialisierung von Daten aus einem bestimmten Format.
    #[error("Fehler bei der Deserialisierung von Daten (Format: {format}): {message}")]
    DeserializationError { format: String, message: String },

    /// Ein angegebener Pfad ist ungültig, nicht auflösbar oder semantisch fehlerhaft.
    #[error("Ungültiger oder nicht auflösbarer Pfad angegeben: '{path}': {message}")]
    InvalidPathError { path: String, message: String },
    
    /// Fehler bei der Initialisierung einer spezifischen Kernkomponente.
    #[error("Fehler bei der Initialisierung einer Kernkomponente '{component}': {message}")]
    InitializationError { component: String, message: String },

    /// Ein unspezifischer oder nicht anderweitig kategorisierter Fehler innerhalb von `novade-core`.
    #[error("Ein unbekannter Kernfehler ist aufgetreten: {0}")]
    UnknownError(String),
}
