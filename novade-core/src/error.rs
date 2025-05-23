//! Definition allgemeiner Fehlerdefinitionen und -typen für die Kernschicht.

// #[derive(Debug, thiserror::Error)] // Ggf. thiserror für bessere Fehlerbehandlung
// pub enum CoreError {
//     #[error("Konfiguration konnte nicht geladen werden: {0}")]
//     ConfigLoadError(String),
//
//     #[error("Fehler bei der Initialisierung des Loggings: {0}")]
//     LoggingInitError(String),
//
//     #[error("Ein E/A-Fehler ist aufgetreten: {0}")]
//     IoError(#[from] std::io::Error), // Beispiel für die Konvertierung von std::io::Error
//
//     #[error("Ein unbekannter Kernfehler ist aufgetreten.")]
//     UnknownError,
// }
