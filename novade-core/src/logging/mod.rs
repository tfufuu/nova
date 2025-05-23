//! # Logging-Infrastruktur in `novade-core`
//!
//! Dieses Modul stellt die zentrale Logging-Funktionalität für NovaDE bereit,
//! basierend auf dem `tracing` Crate. Es ist verantwortlich für die Initialisierung
//! des Logging-Subsystems und das Re-Exportieren der gebräuchlichsten Logging-Makros.
//!
//! ## Hauptkomponenten:
//!
//! - [`setup`]: Ein Untermodul, das die Funktion [`setup::initialize_logging()`]
//!   bereitstellt, um das globale Logging-System zu konfigurieren und zu starten.
//! - **Re-exportierte Makros**: Die Standard-Logging-Makros von `tracing`
//!   (`info!`, `warn!`, `error!`, `debug!`, `trace!`) sowie `span!`, `instrument`, `Level` und `Span`
//!   werden direkt unter `novade_core::logging` (oder `novade_core::*` bei entsprechendem `pub use`
//!   in `lib.rs`) verfügbar gemacht, um die Verwendung in der gesamten Anwendung zu vereinfachen.
//!
//! ## Verwendung:
//!
//! Zuerst muss das Logging-System initialisiert werden, typischerweise früh im Programmablauf:
//!
//! ```rust,no_run
//! use novade_core::config::CoreConfig;
//! use novade_core::logging::setup::initialize_logging; // Direkter Pfad zur Funktion
//! // Alternativ, wenn in lib.rs re-exportiert: use novade_core::initialize_logging;
//!
//! fn main() -> novade_core::error::CoreResult<()> {
//!     let core_config = CoreConfig::example(); // Beispielkonfiguration
//!     initialize_logging(&core_config)?;
//!     // ... Rest der Anwendung ...
//!     Ok(())
//! }
//! ```
//!
//! Nach der Initialisierung können die Logging-Makros verwendet werden:
//!
//! ```rust
//! use novade_core::logging::{info, warn, error, debug, trace, span, Level, instrument};
//! // Alternativ, wenn in lib.rs re-exportiert: use novade_core::{info, warn, ...};
//!
//! #[instrument]
//! pub fn process_data(data: &str) {
//!     info!(input = %data, "Verarbeite Daten");
//!
//!     let computation_span = span!(Level::DEBUG, "schwere_berechnung");
//!     let _enter = computation_span.enter(); // Span betreten
//!
//!     if data.is_empty() {
//!         warn!("Eingabedaten sind leer.");
//!     }
//!     debug!("Zwischenschritt: Datenlänge ist {}", data.len());
//!
//!     if data == "fehler" {
//!         error!(error_code = 42, "Ein simulierter Fehler ist aufgetreten!");
//!     }
//!     trace!("Detailinformation: Erster Buchstabe ist {:?}", data.chars().next());
//!
//!     // Span wird beim Verlassen von _enter automatisch geschlossen
//! }
//!
//! // Um die Logs zu sehen, muss das Logging initialisiert sein (siehe oben)
//! // und das Log-Level entsprechend konfiguriert sein (z.B. über CoreConfig oder RUST_LOG).
//! // process_data("test");
//! ```

pub mod setup;

// Re-Exportiere die wichtigsten Tracing-Makros und Typen für eine einfache Nutzung
// in anderen Teilen des `novade-core` Crates und von abhängigen Crates.
pub use tracing::{debug, error, info, trace, warn, instrument, span, Level, Span};
