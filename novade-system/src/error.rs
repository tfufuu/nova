//! Definition systemspezifischer Fehler für die `novade-system` Schicht.

use novade_core::CoreError; // Importiere CoreError für das Wrapping
use thiserror::Error;

/// Ein Alias für `Result` mit `SystemError` als Fehlertyp.
pub type SystemResult<T> = Result<T, SystemError>;

/// Haupt-Enum für Fehler, die in der `novade-system` Schicht auftreten können.
#[derive(Debug, Error)]
pub enum SystemError {
    #[error("Aufruf an externen Dienst '{service}' Methode '{method}' fehlgeschlagen: {reason}")]
    ServiceCallFailed {
        service: String,
        method: String,
        reason: String,
    },

    #[error("Systemressource '{resource}' nicht verfügbar: {reason}")]
    ResourceUnavailable {
        resource: String,
        reason: String,
    },

    #[error("E/A-Fehler auf Systemebene: {0}")]
    IoError(#[from] CoreError), // Direkte Konvertierung von CoreError (welches std::io::Error wrappen kann)

    #[error("Fehler bei der Datenpersistenz im Store '{store}': {reason}")]
    PersistenceError {
        store: String,
        reason: String,
    },
    
    // Spezifischer PersistenceError für Sled, falls benötigt, oder generisch halten.
    // #[error("Sled DB Fehler: {0}")]
    // SledError(#[from] sled::Error),


    #[error("Ungültige Operation '{operation}' auf Systemebene: {reason}")]
    InvalidOperation {
        operation: String,
        reason: String,
    },

    #[error("Fehler im Prozessmanagement für Befehl '{command}': {reason}")]
    ProcessManagementError {
        command: String,
        reason: String,
    },

    #[error("Wayland-Fehler im Kontext '{context}': {reason}")]
    WaylandError {
        context: String,
        reason: String,
    },
    
    #[error("Ein unbekannter Systemfehler ist aufgetreten: {0}")]
    UnknownError(String),
}
