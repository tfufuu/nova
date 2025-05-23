//! # Fehlerbehandlung in `novade-domain`
//!
//! Dieses Modul definiert die zentralen Fehler-Typen spezifisch für die `novade-domain` Schicht.
//! Das Haupt-Enum ist [`DomainError`], und [`DomainResult<T>`] ist ein praktischer Typalias
//! für `Result<T, DomainError>`.
//!
//! Domänenspezifische Fehler können auftreten, wenn Geschäftsregeln verletzt werden,
//! Entitäten nicht gefunden werden oder Operationen nicht zulässig sind.
//! Zusätzlich können Fehler aus der `novade-core` Schicht, insbesondere bei Repository-Operationen,
//! in `DomainError` gewrappt werden (siehe [`DomainError::RepositoryError`]).
//!
//! ## Verwendung
//!
//! Funktionen und Methoden innerhalb von `novade-domain` (insbesondere in den Diensten)
//! sollten `DomainResult<T>` als Rückgabetyp verwenden, um domänenspezifische Fehler
//! klar zu signalisieren.
//!
//! ```rust,no_run
//! use novade_domain::error::{DomainResult, DomainError};
//! use novade_domain::entities::Application; // Beispiel-Entität
//!
//! // Beispiel für eine Funktion, die einen DomainError zurückgeben könnte
//! fn validate_application_name(app: &Application) -> DomainResult<()> {
//!     if app.name.is_empty() {
//!         Err(DomainError::ValidationError {
//!             field: "application.name".to_string(),
//!             message: "Anwendungsname darf nicht leer sein.".to_string(),
//!         })
//!     } else {
//!         Ok(())
//!     }
//! }
//! ```

use novade_core::CoreError; // Importiere CoreError für das Wrapping
use thiserror::Error;

/// Ein Alias für `Result<T, DomainError>`, der die Fehlerbehandlung in `novade-domain` vereinfacht.
///
/// Anstelle von `Result<MyType, novade_domain::error::DomainError>` kann einfach `DomainResult<MyType>`
/// geschrieben werden, vorausgesetzt, `DomainError` ist im aktuellen Gültigkeitsbereich.
pub type DomainResult<T> = Result<T, DomainError>;

/// Haupt-Enum für alle Fehler, die innerhalb der `novade-domain` Schicht auftreten können.
///
/// Jede Variante repräsentiert eine spezifische domänenbezogene Fehlerbedingung.
/// Die `#[error(...)]` Attribute von `thiserror` werden verwendet, um aussagekräftige
/// Fehlermeldungen zu generieren.
#[derive(Debug, Error)]
pub enum DomainError {
    /// Wird zurückgegeben, wenn eine erwartete Entität nicht gefunden wurde.
    #[error("Entität '{entity_type}' mit ID '{entity_id}' nicht gefunden.")]
    EntityNotFound {
        /// Der Typ der Entität (z.B. "Application", "Workspace").
        entity_type: String,
        /// Die ID der nicht gefundenen Entität.
        entity_id: String, // Könnte auch NovaId sein, aber String ist flexibler für die Fehlermeldung.
    },

    /// Wird zurückgegeben, wenn eine Validierungsregel für ein Feld einer Entität verletzt wurde.
    #[error("Validierungsfehler für Feld '{field}': {message}")]
    ValidationError {
        /// Der Name des Feldes, das die Validierung nicht bestanden hat (z.B. "application.name").
        field: String,
        /// Eine Beschreibung des Validierungsfehlers.
        message: String,
    },

    /// Wird zurückgegeben, wenn eine angeforderte Operation unter den aktuellen Umständen nicht erlaubt ist.
    #[error("Operation '{operation}' nicht erlaubt: {reason}")]
    OperationNotPermitted {
        /// Die Bezeichnung der nicht erlaubten Operation (z.B. "delete_default_workspace").
        operation: String,
        /// Der Grund, warum die Operation nicht erlaubt ist.
        reason: String,
    },

    /// Kapselt einen Fehler, der aus der darunterliegenden `novade-core` Schicht stammt,
    /// oft im Kontext von Repository-Operationen (z.B. E/A-Fehler beim Dateizugriff).
    ///
    /// Diese Variante kann durch `#[from]` automatisch aus `novade_core::CoreError` konvertiert werden.
    #[error("Fehler in der Repository-Schicht oder Kernfunktionalität: {0}")]
    RepositoryError(#[from] CoreError),

    /// Ein spezifischer Fehler innerhalb eines Domänendienstes.
    #[error("Fehler im Domänendienst '{service_name}': {message}")]
    ServiceError {
        /// Der Name des Dienstes, in dem der Fehler aufgetreten ist.
        service_name: String,
        /// Eine Beschreibung des Dienstfehlers.
        message: String,
    },
    
    /// Ein unspezifischer oder nicht anderweitig kategorisierter Fehler innerhalb von `novade-domain`.
    #[error("Ein unbekannter Domänenfehler ist aufgetreten: {0}")]
    UnknownError(String),
}
