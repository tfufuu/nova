//! # Konfigurations-Lader (`config::loader`)
//!
//! Dieses Untermodul von [`crate::config`] ist verantwortlich für die konkrete
//! Implementierung des Ladens von Konfigurationsdateien aus dem Dateisystem.
//!
//! ## Hauptfunktionalität:
//!
//! - [`load_config_from_file()`]: Eine generische Funktion, die eine Datei von einem
//!   gegebenen Pfad liest, ihren Inhalt als TOML interpretiert und versucht,
//!   diesen in einen beliebigen Typ `T` zu deserialisieren, der `serde::Deserialize` implementiert.
//!
//! ## Fehlerbehandlung:
//!
//! Die Ladefunktion gibt spezifische Fehler aus `CoreError` zurück, wie z.B.:
//! - `CoreError::ConfigLoadError`: Wenn die Datei nicht gelesen werden kann (z.B. nicht vorhanden, keine Berechtigungen).
//! - `CoreError::ConfigParseError`: Wenn der Inhalt der Datei kein valides TOML ist oder nicht zur Zielstruktur passt.
//!
//! ## Beispielhafte Verwendung (intern durch `CoreConfig`):
//!
//! ```rust,ignore
//! // Intern würde CoreConfig::load_from_path diese Funktion etwa so aufrufen:
//! use crate::config::CoreConfig; // Die Zielstruktur
//! use std::path::Path;
//!
//! let path_to_config = Path::new("/etc/novade/core.toml");
//! match load_config_from_file::<CoreConfig>(path_to_config) {
//!     Ok(config) => println!("Konfiguration geladen: {:?}", config),
//!     Err(e) => eprintln!("Fehler beim Laden der Konfiguration: {}", e),
//! }
//! ```
//! Normalerweise wird diese Funktion nicht direkt von außerhalb des `config`-Moduls aufgerufen,
//! sondern über Methoden wie [`crate::config::CoreConfig::load_from_path()`].

use crate::error::{CoreError, CoreResult};
use serde::Deserialize;
use std::fs;
use std::path::Path;

/// Lädt und deserialisiert eine Konfigurationsdatei vom angegebenen Pfad.
///
/// Diese Funktion liest den gesamten Inhalt der Datei am `path`, interpretiert ihn als
/// TOML-formatierten String und versucht dann, ihn in den Zieltyp `T` zu deserialisieren.
/// Der Typ `T` muss das `serde::Deserialize` Trait implementieren.
///
/// # Typparameter
/// * `T`: Der Typ, in den der Inhalt der Konfigurationsdatei deserialisiert werden soll.
///        Muss `serde::Deserialize` implementieren.
///
/// # Parameter
/// * `path`: Ein Referenz auf den `Path` der zu ladenden Konfigurationsdatei.
///
/// # Rückgabe
/// Gibt ein [`CoreResult<T>`] zurück:
/// - `Ok(T)`: Wenn das Laden und Deserialisieren erfolgreich war, enthält `T` die geparste Konfiguration.
/// - `Err(CoreError)`: Im Fehlerfall, z.B.:
///     - [`CoreError::ConfigLoadError`]: Wenn die Datei nicht gefunden wurde oder nicht gelesen werden konnte.
///       Der ursprüngliche `std::io::Error` wird als `source` mitgeführt.
///     - [`CoreError::ConfigParseError`]: Wenn der Dateiinhalt kein gültiges TOML war oder nicht
///       zur Struktur von `T` passte. Die Fehlermeldung des TOML-Parsers wird mitgeliefert.
pub fn load_config_from_file<T>(path: &Path) -> CoreResult<T>
where
    T: for<'de> Deserialize<'de>, // T muss für jede Lifetime 'de deserialisierbar sein.
{
    let content = fs::read_to_string(path).map_err(|err| CoreError::ConfigLoadError {
        path: path.to_path_buf(), // Klone den Pfad für die Fehlerstruktur.
        source: err,
    })?;

    toml::from_str(&content).map_err(|err| CoreError::ConfigParseError {
        format: "TOML".to_string(),
        message: err.to_string(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    // Eine einfache Teststruktur für die Deserialisierung.
    #[derive(Deserialize, PartialEq, Debug)]
    struct TestConfig {
        name: String,
        count: u32,
    }

    #[test]
    fn test_load_valid_config() {
        let mut temp_file = NamedTempFile::new().unwrap();
        let content = r#"
            name = "Test Name"
            count = 42
        "#;
        temp_file.write_all(content.as_bytes()).unwrap();

        let loaded_config: TestConfig = load_config_from_file(temp_file.path()).unwrap();
        assert_eq!(loaded_config, TestConfig { name: "Test Name".to_string(), count: 42 });
    }

    #[test]
    fn test_load_invalid_toml_format() {
        let mut temp_file = NamedTempFile::new().unwrap();
        let content = "this is not toml {"; // Ungültiges TOML
        temp_file.write_all(content.as_bytes()).unwrap();

        let result: CoreResult<TestConfig> = load_config_from_file(temp_file.path());
        // Die genaue Fehlermeldung kann von der toml-Crate-Version abhängen.
        // Wir prüfen allgemeiner auf "expected", da TOML-Parser oft melden, was sie erwartet haben.
        assert!(matches!(result, Err(CoreError::ConfigParseError { format, message })
            if format == "TOML" && message.contains("expected")));
    }

    #[test]
    fn test_load_non_existent_file() {
        let path = Path::new("hopefully_this_file_does_not_exist_for_real.toml");
        let result: CoreResult<TestConfig> = load_config_from_file(path);
        match result {
            Err(CoreError::ConfigLoadError { path: error_path, source }) => {
                assert_eq!(error_path, path.to_path_buf());
                assert_eq!(source.kind(), std::io::ErrorKind::NotFound);
            }
            _ => panic!("Expected ConfigLoadError for non-existent file, got {:?}", result),
        }
    }
    
    #[test]
    fn test_load_mismatched_structure() {
        let mut temp_file = NamedTempFile::new().unwrap();
        let content = r#"
            name = "Test Name"
            # 'count' ist hier ein String, aber TestConfig erwartet u32.
            count = "not_a_number_sadly" 
        "#;
        temp_file.write_all(content.as_bytes()).unwrap();

        let result: CoreResult<TestConfig> = load_config_from_file(temp_file.path());
         assert!(matches!(result, Err(CoreError::ConfigParseError { format, message })
            if format == "TOML" && message.contains("invalid type: string \"not_a_number_sadly\", expected u32")));
    }
}
