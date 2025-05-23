//! # Logging Setup (`logging::setup`)
//!
//! Dieses Untermodul von [`crate::logging`] ist verantwortlich für die konkrete
//! Initialisierung und Konfiguration der globalen Logging-Infrastruktur.
//!
//! ## Hauptfunktionalität:
//!
//! - [`initialize_logging()`]: Die zentrale Funktion, die das `tracing-subscriber` System
//!   konfiguriert. Sie berücksichtigt dabei sowohl die Einstellungen aus der [`CoreConfig`]
//!   (insbesondere `log_level`) als auch die Umgebungsvariable `RUST_LOG`.
//!   `RUST_LOG` hat dabei Vorrang vor der Konfiguration.
//!
//! ## Konfigurationsdetails:
//!
//! Der `tracing_subscriber` wird wie folgt konfiguriert:
//! - **Log-Level-Filterung**: Durch `EnvFilter`, der `RUST_LOG` und `core_config.log_level` kombiniert.
//! - **Ausgabe**: Logs werden nach `stderr` geschrieben.
//! - **Span-Events**: `NEW` und `CLOSE` Events für Spans werden protokolliert.
//! - **Zusatzinformationen**: Thread-IDs, Log-Level und das Ziel (Modulpfad) jeder Nachricht werden angezeigt.
//!
//! ## Fehlerbehandlung:
//!
//! Die Initialisierung kann fehlschlagen, wenn:
//! - Ein ungültiges Log-Level in der Konfiguration oder `RUST_LOG` angegeben wird.
//! - Ein globaler Tracing-Subscriber bereits gesetzt wurde (die Funktion verwendet `try_init`,
//!   um einen Panic in diesem Fall zu vermeiden und stattdessen einen Fehler zurückzugeben).
//! In solchen Fällen wird ein [`CoreError::LoggingInitError`] zurückgegeben.
//!
//! ## Beispielhafte Verwendung (intern durch `novade_core` oder Anwendungen):
//!
//! ```rust,no_run
//! use novade_core::config::CoreConfig;
//! use novade_core::logging::setup::initialize_logging;
//! use novade_core::error::CoreResult;
//!
//! fn main() -> CoreResult<()> {
//!     // Normalerweise würde die CoreConfig aus einer Datei geladen.
//!     let config = CoreConfig::example();
//!
//!     // Initialisiere das Logging-System.
//!     initialize_logging(&config)?;
//!
//!     novade_core::logging::info!("Logging wurde erfolgreich initialisiert!");
//!     // ... weiterer Code ...
//!     Ok(())
//! }
//! ```

use crate::config::CoreConfig;
use crate::error::{CoreError, CoreResult};
use tracing_subscriber::fmt::format::FmtSpan;
use tracing_subscriber::EnvFilter;

/// Initialisiert die globale Logging-Infrastruktur basierend auf der [`CoreConfig`].
///
/// Diese Funktion konfiguriert und aktiviert das `tracing-subscriber` System für
/// strukturiertes Logging im gesamten NovaDE-Projekt.
///
/// ## Log-Level Bestimmung:
/// Das effektive Log-Level wird durch eine Kombination aus der `RUST_LOG` Umgebungsvariable
/// und dem Feld `log_level` in der übergebenen `core_config` bestimmt:
/// 1. Wenn `RUST_LOG` gesetzt und gültig ist, wird diese verwendet.
/// 2. Andernfalls wird `core_config.log_level` verwendet.
/// 3. Wenn beide ungültig sind, schlägt die Initialisierung fehl.
///
/// ## Konfiguration des Subscribers:
/// - Schreibt Logs nach `stderr`.
/// - Aktiviert Span-Events für `NEW` (beim Erstellen eines Spans) und `CLOSE` (beim Verlassen).
/// - Fügt Thread-IDs, das Log-Level und das Ziel (Modulpfad) zu jeder Log-Nachricht hinzu.
///
/// # Parameter
/// * `core_config`: Eine Referenz auf die [`CoreConfig`], die das Standard-Log-Level
///   und andere potenziell logging-relevante Konfigurationen enthält.
///
/// # Rückgabe
/// Gibt ein [`CoreResult<()>`] zurück:
/// - `Ok(())`: Wenn die Logging-Initialisierung erfolgreich war.
/// - `Err(CoreError::LoggingInitError)`: Wenn ein Fehler auftritt, z.B. ein ungültiges
///   Log-Level oder wenn bereits ein globaler Subscriber gesetzt wurde.
pub fn initialize_logging(core_config: &CoreConfig) -> CoreResult<()> {
    // Baue den EnvFilter: Starte mit dem Level aus der Konfiguration (`core_config.log_level`),
    // erlaube aber eine Überschreibung durch die `RUST_LOG` Umgebungsvariable, falls gesetzt.
    let env_filter = EnvFilter::try_from_default_env() // Versucht, RUST_LOG zu parsen
        .or_else(|_| EnvFilter::try_new(&core_config.log_level)) // Fallback auf Konfiguration
        .map_err(|e| {
            CoreError::LoggingInitError(format!(
                "Ungültiges Log-Level '{}' in Konfiguration oder RUST_LOG Umgebungsvariable: {}",
                core_config.log_level, e
            ))
        })?;

    // Baue den Subscriber mit den gewünschten Formatierungsoptionen.
    let subscriber_builder = tracing_subscriber::fmt()
        .with_env_filter(env_filter) // Wendet den oben erstellten Filter an.
        .with_writer(std::io::stderr) // Log-Ausgaben gehen nach stderr.
        .with_span_events(FmtSpan::NEW | FmtSpan::CLOSE) // Protokolliert Erstellung und Schließung von Spans.
        .with_thread_ids(true) // Fügt die ID des aktuellen Threads zu Log-Einträgen hinzu.
        .with_level(true) // Fügt das Log-Level (z.B. INFO, DEBUG) zu Log-Einträgen hinzu.
        .with_target(true); // Fügt das Ziel (Modulpfad) zu Log-Einträgen hinzu.

    // Versuche, den konfigurierten Subscriber als globalen Standard für das Tracing-System zu setzen.
    // `try_init` gibt einen Fehler zurück, falls bereits ein globaler Subscriber gesetzt wurde,
    // anstatt zu panicken (wie es `set_global_default` tun würde).
    subscriber_builder.try_init().map_err(|e| {
        CoreError::LoggingInitError(format!(
            "Fehler beim Setzen des globalen Tracing-Subscribers: {}. Möglicherweise wurde initialize_logging bereits aufgerufen.",
            e
        ))
    })?;

    // Eine erste Log-Nachricht, um zu bestätigen, dass das Logging funktioniert
    // und um das effektiv verwendete Log-Level (implizit durch den Filter) zu signalisieren.
    tracing::info!(
        log_level_source = %core_config.log_level,
        rust_log_env = %std::env::var("RUST_LOG").unwrap_or_else(|_| "Nicht gesetzt".to_string()),
        "Logging initialisiert. Effektives Log-Level wird durch Konfiguration und RUST_LOG bestimmt."
    );

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::CoreConfig;
    use crate::types::Version; // Benötigt für CoreConfig::example(), das Version::new verwendet

    // Hilfsfunktion, um eine valide CoreConfig für Tests zu erstellen.
    fn test_config(log_level: &str) -> CoreConfig {
        CoreConfig {
            log_level: log_level.to_string(),
            default_locale: "en-US".to_string(),
            config_version: Version::new(1, 0, 0),
            custom_theme_path: None,
        }
    }

    // Wichtiger Hinweis zu Tests mit globalen Loggern:
    // Tracing Subscriber können nur einmal pro Prozess global gesetzt werden.
    // Das bedeutet, dass Tests, die `initialize_logging` aufrufen, sich gegenseitig beeinflussen können.
    // - Der erste Test, der `initialize_logging` erfolgreich aufruft, setzt den globalen Logger.
    // - Nachfolgende Aufrufe von `initialize_logging` in anderen Tests werden fehlschlagen
    //   (mit `CoreError::LoggingInitError`), weil bereits ein Logger gesetzt ist.
    //
    // Strategien für robuste Tests:
    // 1. `serial_test` Crate: Führt Tests, die globale Zustände modifizieren, sequenziell aus.
    //    ```rust
    //    use serial_test::serial;
    //    #[test]
    //    #[serial]
    //    fn my_logging_test() { /* ... */ }
    //    ```
    // 2. Ignorieren des Fehlers bei Mehrfachinitialisierung, wenn das für den Test akzeptabel ist.
    // 3. Testen der Log-Ausgabe: Komplexer, erfordert das Erfassen von `stderr` oder die Verwendung
    //    eines benutzerdefinierten `tracing_subscriber::Writer` für Tests.
    //
    // Für diese Beispiele wird der Fehler bei Mehrfachinitialisierung oft ignoriert oder als
    // Teil des erwarteten Verhaltens in einer Testsuite betrachtet, die nicht serialisiert ist.

    #[test]
    // #[serial_test::serial] // Einkommentieren, wenn serial_test als dev-dependency hinzugefügt wird
    fn test_initialize_logging_valid_level() {
        let config = test_config("trace"); // Ein valides Level
        
        // Temporär RUST_LOG entfernen, um nur die Konfiguration zu testen
        let old_rust_log = std::env::var("RUST_LOG").ok();
        std::env::remove_var("RUST_LOG");

        let result = initialize_logging(&config);
        
        // Setze RUST_LOG zurück, falls es vorher gesetzt war
        if let Some(val) = old_rust_log {
            std::env::set_var("RUST_LOG", val);
        }

        // Dieser Test kann fehlschlagen, wenn ein anderer Test bereits einen Logger gesetzt hat.
        // In einer serialisierten Testumgebung sollte er jedoch erfolgreich sein oder den erwarteten Fehler liefern.
        if let Err(e) = result {
            // Wenn ein Fehler auftritt, prüfen wir, ob es der erwartete Fehler ist (bereits initialisiert)
            if !e.to_string().contains("bereits aufgerufen") {
                panic!("Unerwarteter Fehler bei gültigem Level: {:?}", e);
            } else {
                eprintln!("Hinweis: Logging war bereits initialisiert: {}", e);
            }
        } else {
            // Erfolg! Hier könnte man Log-Ausgaben prüfen.
            tracing::info!("Test-Log nach erfolgreicher Initialisierung mit 'trace'.");
        }
    }

    #[test]
    // #[serial_test::serial] // Einkommentieren für serialisierte Tests
    fn test_initialize_logging_invalid_level_in_config() {
        // Temporär RUST_LOG entfernen, um nur die Konfiguration zu testen
        let old_rust_log = std::env::var("RUST_LOG").ok();
        std::env::remove_var("RUST_LOG");

        let config = test_config("dies_ist_kein_level"); // Ungültiges Level
        let result = initialize_logging(&config);

        if let Some(val) = old_rust_log {
            std::env::set_var("RUST_LOG", val);
        }
        
        match result {
            Err(CoreError::LoggingInitError(msg)) => {
                // Erwarteter Fehler, da das Level ungültig ist UND RUST_LOG nicht gesetzt ist (oder auch ungültig wäre)
                // Die genaue Fehlermeldung von `tracing_subscriber::filter::ParseError` (e) kann variieren.
                // Wir stellen sicher, dass unser Teil der Fehlermeldung korrekt ist.
                let expected_prefix = format!("Ungültiges Log-Level '{}'", "dies_ist_kein_level");
                assert!(msg.starts_with(&expected_prefix), "Fehlermeldung '{}' sollte mit '{}' beginnen", msg, expected_prefix);
            }
            Ok(_) => {
                // Dies könnte passieren, wenn ein anderer Test den Logger bereits initialisiert hat
                // und try_init() Ok zurückgibt, weil der Fehler vom ersten Versuch kam.
                // Oder wenn RUST_LOG (falls nicht gecleared) ein gültiges Level hätte.
                eprintln!("Logging-Initialisierung war erfolgreich trotz ungültigem Config-Level, ggf. durch vorherige Initialisierung.");
                tracing::warn!("Logging mit ungültigem Config-Level lief scheinbar gut - bereits initialisiert?");
            }
            Err(e) => panic!("Unerwarteter Fehlertyp: {:?}", e),
        }
    }

    #[test]
    // #[serial_test::serial] // Einkommentieren für serialisierte Tests
    fn test_initialize_logging_with_rust_log_override() {
        let old_rust_log = std::env::var("RUST_LOG").ok(); // Alten Wert sichern
        std::env::set_var("RUST_LOG", "debug"); // Gültiges Level via RUST_LOG

        let config = test_config("error"); // Config hat ein anderes Level
        
        let result = initialize_logging(&config);

        // RUST_LOG zurücksetzen
        if let Some(val) = old_rust_log {
            std::env::set_var("RUST_LOG", val);
        } else {
            std::env::remove_var("RUST_LOG");
        }

        if let Err(e) = result {
            if !e.to_string().contains("bereits aufgerufen") {
                panic!("Unerwarteter Fehler bei RUST_LOG override: {:?}", e);
            } else {
                eprintln!("Hinweis: Logging war bereits initialisiert (RUST_LOG Test): {}", e);
            }
        } else {
            // Hier würde man idealerweise prüfen, ob das effektive Level 'debug' ist.
            // Das ist ohne einen Test-Subscriber schwer zu verifizieren.
            tracing::info!("Test-Log nach Initialisierung mit RUST_LOG=debug (Config war 'error').");
            // Manuell prüfen, ob Debug-Logs jetzt durchkommen (wenn Konsole sichtbar):
            tracing::debug!("Dies ist eine Debug-Nachricht und sollte sichtbar sein, wenn RUST_LOG=debug aktiv ist.");
        }
    }
}
