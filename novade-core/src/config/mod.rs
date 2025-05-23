//! # Konfigurationsmanagement in `novade-core`
//!
//! Dieses Modul ist verantwortlich für das Laden, Parsen und Validieren von
//! Konfigurationen für die `novade-core` Schicht und potenziell für die gesamte Anwendung.
//!
//! ## Hauptkomponenten:
//!
//! - [`CoreConfig`]: Eine Struktur, die die Kernkonfigurationsparameter wie Log-Level,
//!   Standard-Lokalisierung und Version der Konfigurationsdatei enthält.
//! - [`DEFAULT_CORE_CONFIG_FILENAME`]: Der Standarddateiname ("core.toml"), der für die
//!   Kernkonfiguration verwendet wird.
//! - [`loader`]: Ein Untermodul, das die Funktionalität zum Laden von Konfigurationsdateien
//!   (aktuell TOML) bereitstellt.
//!
//! ## Verwendung:
//!
//! Die `CoreConfig` kann typischerweise durch Aufruf von [`CoreConfig::load_from_path()`]
//! geladen werden, wobei der Pfad zu einer TOML-Datei übergeben wird.
//! Für Tests oder Standardwerte kann [`CoreConfig::example()`] verwendet werden.
//!
//! ```rust,no_run
//! use novade_core::config::{CoreConfig, DEFAULT_CORE_CONFIG_FILENAME};
//! use novade_core::error::CoreResult;
//! use std::path::Path;
//!
//! fn load_app_config(config_dir: &Path) -> CoreResult<CoreConfig> {
//!     let config_file_path = config_dir.join(DEFAULT_CORE_CONFIG_FILENAME);
//!     CoreConfig::load_from_path(&config_file_path)
//! }
//!
//! // In main oder Initialisierungsfunktion:
//! // let config = load_app_config(Path::new("/etc/novade/")).expect("Konfiguration konnte nicht geladen werden");
//! // println!("Log-Level: {}", config.log_level);
//! ```

pub mod loader;

use crate::error::CoreResult;
use crate::types::Version;
use serde::Deserialize;
use std::path::{Path, PathBuf};

/// Der Standard-Dateiname für die Kernkonfigurationsdatei von NovaDE.
///
/// Dieser Dateiname (z.B. "core.toml") wird erwartet, wenn keine spezifische
/// Konfigurationsdatei angegeben wird, und typischerweise in Standard-Konfigurationsverzeichnissen
/// gesucht (siehe `novade_core::utils::get_app_config_dir`).
pub const DEFAULT_CORE_CONFIG_FILENAME: &str = "core.toml";

/// Definiert die Struktur für die Kernkonfigurationsparameter von NovaDE.
///
/// Diese Struktur wird aus einer Konfigurationsdatei (z.B. TOML) deserialisiert und
/// enthält grundlegende Einstellungen für die Anwendung.
#[derive(Deserialize, Debug, PartialEq)]
pub struct CoreConfig {
    /// Das zu verwendende globale Log-Level (z.B. "debug", "info", "warn", "error").
    ///
    /// Dieses Level kann durch die `RUST_LOG` Umgebungsvariable überschrieben werden,
    /// falls diese gesetzt ist. Siehe [`novade_core::logging::setup::initialize_logging()`].
    pub log_level: String,

    /// Die Standard-Lokalisierung für die Anwendung (z.B. "en-US", "de-DE").
    ///
    /// Wird verwendet, wenn keine spezifischere Lokalisierung verfügbar oder eingestellt ist.
    pub default_locale: String,

    /// Die Version der Konfigurationsdatei-Struktur selbst.
    ///
    /// Dies ermöglicht es, bei zukünftigen Änderungen an der Struktur der Konfigurationsdatei
    /// Migrationen oder Kompatibilitätsprüfungen durchzuführen.
    pub config_version: Version,

    /// Ein optionaler Pfad zu einem benutzerdefinierten Theme-Verzeichnis.
    ///
    /// Wenn gesetzt, kann die UI-Schicht versuchen, Themes von diesem Pfad zu laden.
    pub custom_theme_path: Option<PathBuf>,
}

impl CoreConfig {
    /// Lädt die Kernkonfiguration aus der Datei am angegebenen Pfad.
    ///
    /// Diese Methode delegiert an [`loader::load_config_from_file()`] und erwartet,
    /// dass die Datei im TOML-Format vorliegt.
    ///
    /// # Parameter
    /// * `path`: Der Pfad zur Konfigurationsdatei (z.B. `core.toml`).
    ///
    /// # Fehler
    /// Gibt `CoreError` zurück, wenn die Datei nicht gelesen werden kann (`ConfigLoadError`)
    /// oder wenn der Inhalt nicht als `CoreConfig` deserialisiert werden kann (`ConfigParseError`).
    pub fn load_from_path(path: &Path) -> CoreResult<Self> {
        loader::load_config_from_file(path)
    }

    /// Erstellt eine Beispiel-Konfiguration mit Standardwerten.
    ///
    /// Diese Funktion ist nützlich für Tests, Demonstrationen oder als Fallback,
    /// wenn keine Konfigurationsdatei gefunden wird und Standardverhalten gewünscht ist.
    ///
    /// # Rückgabe
    /// Eine `CoreConfig` Instanz mit vordefinierten Werten (z.B. log_level "info").
    pub fn example() -> Self {
        Self {
            log_level: "info".to_string(),
            default_locale: "en-US".to_string(),
            config_version: Version::new(1, 0, 0),
            custom_theme_path: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn test_load_core_config_successfully() {
        let mut temp_file = NamedTempFile::new().unwrap();
        let content = r#"
            log_level = "debug"
            default_locale = "de-DE"
            config_version = { major = 1, minor = 0, patch = 0 }
            custom_theme_path = "/usr/share/themes/MyTheme"
        "#;
        temp_file.write_all(content.as_bytes()).unwrap();

        let config = CoreConfig::load_from_path(temp_file.path()).unwrap();

        assert_eq!(config.log_level, "debug");
        assert_eq!(config.default_locale, "de-DE");
        assert_eq!(config.config_version, Version::new(1,0,0));
        assert_eq!(config.custom_theme_path, Some(PathBuf::from("/usr/share/themes/MyTheme")));
    }

    #[test]
    fn test_core_config_example() {
        let example_config = CoreConfig::example();
        assert_eq!(example_config.log_level, "info");
        assert_eq!(example_config.default_locale, "en-US");
        assert_eq!(example_config.config_version, Version::new(1,0,0));
        assert!(example_config.custom_theme_path.is_none());
    }
}
