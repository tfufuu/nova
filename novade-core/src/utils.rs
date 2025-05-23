//! # Allgemeine Dienstprogramme (`utils`)
//!
//! Dieses Modul stellt eine Sammlung von allgemeinen Hilfsfunktionen bereit, die
//! in verschiedenen Teilen von `novade-core` und anderen NovaDE-Crates nützlich sein können.
//! Dazu gehören Funktionen zur Pfadmanipulation, zum Lesen von Dateien und zur Ermittlung
//! von Standard-Anwendungsverzeichnissen.
//!
//! ## Hauptfunktionen:
//!
//! - [`resolve_path()`]: Löst einen möglicherweise relativen Pfad relativ zu einem Basispfad auf
//!   und normalisiert ihn (entfernt `.` und `..`).
//! - [`read_file_to_string()`]: Liest den gesamten Inhalt einer Datei in einen String.
//! - [`get_app_config_dir()`]: Ermittelt das Standard-Konfigurationsverzeichnis für die Anwendung.
//! - [`get_app_data_dir()`]: Ermittelt das Standard-Datenverzeichnis für die Anwendung.
//! - [`get_app_cache_dir()`]: Ermittelt das Standard-Cache-Verzeichnis für die Anwendung.
//!
//! ## Fehlerbehandlung:
//!
//! Funktionen wie `resolve_path` und `read_file_to_string` geben `CoreResult` zurück und
//! verwenden spezifische `CoreError`-Varianten (z.B. `InvalidPathError`, `IoError`),
//! um Fehlerzustände zu signalisieren.
//!
//! ## Beispiel: Pfad auflösen und Datei lesen
//!
//! ```rust,no_run
//! use novade_core::utils::{resolve_path, read_file_to_string, get_app_config_dir};
//! use novade_core::error::CoreResult;
//! use std::path::{Path, PathBuf};
//!
//! fn read_my_config_setting(app_name: &str, config_file_name: &str, setting_name: &str) -> CoreResult<String> {
//!     // 1. Konfigurationsverzeichnis ermitteln
//!     let config_base_dir = get_app_config_dir(app_name)
//!         .ok_or_else(|| novade_core::CoreError::InitializationError {
//!             component: "config_path".to_string(),
//!             message: "Konnte kein Konfigurationsverzeichnis finden.".to_string(),
//!         })?;
//!
//!     // 2. Pfad zur Konfigurationsdatei auflösen
//!     let config_file_path = resolve_path(&config_base_dir, config_file_name)?;
//!
//!     // 3. Dateiinhalt lesen
//!     let content = read_file_to_string(&config_file_path)?;
//!
//!     // In einer echten Anwendung würde hier der Inhalt geparst (z.B. TOML, JSON)
//!     // Für dieses Beispiel suchen wir nur nach einer Zeile.
//!     content.lines()
//!         .find(|line| line.starts_with(setting_name))
//!         .map(|line| line.split('=').nth(1).unwrap_or("").trim().to_string())
//!         .ok_or_else(|| novade_core::CoreError::ConfigParseError{
//!             format: "text".to_string(),
//!             message: format!("Einstellung '{}' nicht gefunden.", setting_name)
//!         })
//! }
//!
//! // Beispielaufruf (würde in einer echten Anwendung Fehlerbehandlung benötigen)
//! // let my_setting = read_my_config_setting("my_app", "app.conf", "my_key").unwrap();
//! // println!("Wert der Einstellung: {}", my_setting);
//! ```

use crate::error::{CoreError, CoreResult};
use std::fs;
use std::path::{Component, Path, PathBuf}; // MAIN_SEPARATOR für Tests entfernt

/// Löst einen möglicherweise relativen Pfad relativ zu einem gegebenen Basispfad auf und normalisiert ihn.
///
/// Die Normalisierung umfasst die Verarbeitung von `.` (aktuelles Verzeichnis) und `..` (Elternverzeichnis)
/// Pfadkomponenten. Diese Funktion prüft nicht, ob der resultierende Pfad tatsächlich im Dateisystem existiert.
///
/// # Parameter
/// * `base_path`: Der Basispfad, relativ zu dem `relative_path_str` aufgelöst wird.
///   Wenn `relative_path_str` bereits ein absoluter Pfad ist, wird `base_path` ignoriert.
/// * `relative_path_str`: Ein String-Slice, der den aufzulösenden Pfad darstellt. Kann absolut oder relativ sein.
///
/// # Rückgabe
/// Ein `CoreResult<PathBuf>`:
/// - `Ok(PathBuf)`: Der aufgelöste und normalisierte Pfad.
/// - `Err(CoreError::InvalidPathError)`: Wenn der Pfad als ungültig erachtet wird,
///   beispielsweise wenn versucht wird, mit `..` über das Wurzelverzeichnis eines absoluten Pfades hinauszugehen.
///
/// # Beispiele
/// ```
/// use novade_core::utils::resolve_path;
/// use std::path::{Path, PathBuf};
///
/// let base = Path::new("/usr/local");
/// assert_eq!(resolve_path(base, "bin/mytool").unwrap(), PathBuf::from("/usr/local/bin/mytool"));
/// assert_eq!(resolve_path(base, "../share/data").unwrap(), PathBuf::from("/usr/share/data"));
/// assert_eq!(resolve_path(base, "./lib").unwrap(), PathBuf::from("/usr/local/lib"));
///
/// // Absoluter Pfad als `relative_path_str` ignoriert `base_path`
/// assert_eq!(resolve_path(base, "/etc/config").unwrap(), PathBuf::from("/etc/config"));
///
/// // Komplexere Normalisierung
/// let complex_base = Path::new("/a/b/c");
/// assert_eq!(resolve_path(complex_base, "../../d/./../e/f").unwrap(), PathBuf::from("/a/e/f"));
///
/// // Auflösung zu "."
/// assert_eq!(resolve_path(Path::new("config"), "..").unwrap(), PathBuf::from("."));
///
/// // Fehlerfall: Versuch, über die Wurzel hinauszugehen
/// assert!(resolve_path(Path::new("/"), "..").is_err());
/// ```
pub fn resolve_path(base_path: &Path, relative_path_str: &str) -> CoreResult<PathBuf> {
    let relative_path = Path::new(relative_path_str);
    let mut combined_path = PathBuf::new();

    if relative_path.is_absolute() {
        combined_path.push(relative_path);
    } else {
        combined_path.push(base_path);
        combined_path.push(relative_path);
    }

    // Normalisiere den Pfad (Entferne ., ..)
    let mut components_vec = Vec::new();
    for component in combined_path.components() {
        match component {
            Component::CurDir => {} // '.' ignorieren
            Component::ParentDir => { // '..'
                // Wenn der letzte hinzugefügte Komponent ein normaler Name ist, entferne ihn.
                // Wenn es RootDir ist, kann '..' nicht angewendet werden (Fehler).
                // Wenn es ParentDir ist (z.B. bei relativen Pfaden wie "../../"), füge es hinzu.
                match components_vec.last() {
                    Some(Component::Normal(_)) => {
                        components_vec.pop();
                    }
                    Some(Component::RootDir) => {
                        return Err(CoreError::InvalidPathError {
                            path: combined_path.to_string_lossy().into_owned(),
                            message: "Pfad versucht, von der Wurzel aus zurückzugehen ('../').".to_string(),
                        });
                    }
                    Some(Component::ParentDir) | None => {
                        // Wenn leer oder letztes ist '..', dann füge '..' hinzu (für relative Pfade)
                        // Ausnahme: Wenn der ursprüngliche Pfad absolut war, ist dies ein Fehler.
                        if combined_path.is_absolute() {
                             return Err(CoreError::InvalidPathError {
                                path: combined_path.to_string_lossy().into_owned(),
                                message: "Pfad versucht, von der Wurzel aus zurückzugehen ('../') bei absolutem Pfad.".to_string(),
                            });
                        }
                        components_vec.push(component);
                    }
                    Some(Component::CurDir) => { /* Sollte nicht passieren, da CurDir ignoriert wird */ components_vec.push(component); }
                    Some(Component::Prefix(_)) => { /* Für Windows-Pfade, komplexer, hier vereinfacht */ components_vec.push(component); }

                }
            }
            c => { // RootDir, Normal, Prefix
                components_vec.push(c);
            }
        }
    }

    // Baue den finalen Pfad aus den verbleibenden Komponenten.
    // Wenn `components_vec` leer ist (z.B. `base="foo", rel=".."`), ist das Ergebnis `.`
    // Wenn `components_vec` nur `RootDir` enthält, ist das Ergebnis `/`
    let final_path: PathBuf = components_vec.iter().collect();
    if final_path.as_os_str().is_empty() && !combined_path.as_os_str().is_empty() {
        // Dies kann passieren, wenn der Pfad zu "." normalisiert wird, z.B. "a/.."
        Ok(PathBuf::from("."))
    } else {
        Ok(final_path)
    }
}


/// Liest den gesamten Inhalt einer Datei UTF-8 kodiert in einen String.
///
/// Diese Funktion ist ein einfacher Wrapper um `std::fs::read_to_string`.
///
/// # Parameter
/// * `path`: Der Pfad zur Datei, die gelesen werden soll.
///
/// # Rückgabe
/// Ein `CoreResult<String>`:
/// - `Ok(String)`: Der Inhalt der Datei als String.
/// - `Err(CoreError::IoError)`: Wenn ein Fehler beim Lesen der Datei auftritt
///   (z.B. Datei nicht gefunden, keine Berechtigungen, kein valides UTF-8).
///
/// # Beispiele
/// ```no_run
/// use novade_core::utils::read_file_to_string;
/// use std::path::Path;
///
/// match read_file_to_string(Path::new("meine_datei.txt")) {
///     Ok(inhalt) => println!("Dateiinhalt: {}", inhalt),
///     Err(e) => eprintln!("Fehler beim Lesen der Datei: {}", e),
/// }
/// ```
pub fn read_file_to_string(path: &Path) -> CoreResult<String> {
    fs::read_to_string(path).map_err(|err| CoreError::IoError(err.to_string()))
}

/// Ermittelt das Standard-Konfigurationsverzeichnis für die Anwendung gemäß den Konventionen des Betriebssystems.
///
/// Basiert auf dem `dirs` Crate.
/// - **Linux**: `$XDG_CONFIG_HOME/{app_name}` (typischerweise `~/.config/{app_name}`)
/// - **macOS**: `~/Library/Application Support/{app_name}`
/// - **Windows**: `%APPDATA%\{app_name}\config` (z.B. `C:\Users\Benutzer\AppData\Roaming\{app_name}\config`)
///
/// # Parameter
/// * `app_name`: Der Name der Anwendung. Dieser wird als letztes Verzeichniselement an den
///   Basispfad des Konfigurationsverzeichnisses angehängt.
///
/// # Rückgabe
/// `Some(PathBuf)` mit dem Pfad zum anwendungsspezifischen Konfigurationsverzeichnis,
/// oder `None`, wenn das Basis-Konfigurationsverzeichnis nicht ermittelt werden konnte
/// (z.B. in sehr eingeschränkten Umgebungen).
///
/// # Beispiele
/// ```
/// use novade_core::utils::get_app_config_dir;
///
/// if let Some(config_dir) = get_app_config_dir("MeineTolleApp") {
///     println!("Konfigurationsverzeichnis: {}", config_dir.display());
///     // Erwartete Ausgabe (Beispiel Linux): /home/benutzer/.config/MeineTolleApp
/// }
/// ```
pub fn get_app_config_dir(app_name: &str) -> Option<PathBuf> {
    dirs::config_dir().map(|path| path.join(app_name))
}

/// Ermittelt das Standard-Datenverzeichnis für die Anwendung gemäß den Konventionen des Betriebssystems.
///
/// Basiert auf dem `dirs` Crate.
/// - **Linux**: `$XDG_DATA_HOME/{app_name}` (typischerweise `~/.local/share/{app_name}`)
/// - **macOS**: `~/Library/Application Support/{app_name}` (oft dasselbe wie config auf macOS für einfache Apps)
/// - **Windows**: `%APPDATA%\{app_name}\data` (z.B. `C:\Users\Benutzer\AppData\Roaming\{app_name}\data`)
///
/// # Parameter
/// * `app_name`: Der Name der Anwendung.
///
/// # Rückgabe
/// `Some(PathBuf)` mit dem Pfad zum anwendungsspezifischen Datenverzeichnis, oder `None`.
///
/// # Beispiele
/// ```
/// use novade_core::utils::get_app_data_dir;
///
/// if let Some(data_dir) = get_app_data_dir("MeineTolleApp") {
///     println!("Datenverzeichnis: {}", data_dir.display());
/// }
/// ```
pub fn get_app_data_dir(app_name: &str) -> Option<PathBuf> {
    dirs::data_dir().map(|path| path.join(app_name))
}

/// Ermittelt das Standard-Cache-Verzeichnis für die Anwendung gemäß den Konventionen des Betriebssystems.
///
/// Basiert auf dem `dirs` Crate.
/// - **Linux**: `$XDG_CACHE_HOME/{app_name}` (typischerweise `~/.cache/{app_name}`)
/// - **macOS**: `~/Library/Caches/{app_name}`
/// - **Windows**: `%LOCALAPPDATA%\{app_name}\cache` (z.B. `C:\Users\Benutzer\AppData\Local\{app_name}\cache`)
///
/// # Parameter
/// * `app_name`: Der Name der Anwendung.
///
/// # Rückgabe
/// `Some(PathBuf)` mit dem Pfad zum anwendungsspezifischen Cache-Verzeichnis, oder `None`.
///
/// # Beispiele
/// ```
/// use novade_core::utils::get_app_cache_dir;
///
/// if let Some(cache_dir) = get_app_cache_dir("MeineTolleApp") {
///     println!("Cache-Verzeichnis: {}", cache_dir.display());
/// }
/// ```
pub fn get_app_cache_dir(app_name: &str) -> Option<PathBuf> {
    dirs::cache_dir().map(|path| path.join(app_name))
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;
    // tempdir kann für Tests nützlich sein, die Verzeichnisstrukturen erfordern.
    // use tempfile::tempdir;

    #[test]
    fn test_resolve_path_basic_absolute() {
        let base = Path::new("/usr/local");
        assert_eq!(resolve_path(base, "bin").unwrap(), PathBuf::from("/usr/local/bin"));
        assert_eq!(resolve_path(base, "../share").unwrap(), PathBuf::from("/usr/share"));
        assert_eq!(resolve_path(base, "./lib").unwrap(), PathBuf::from("/usr/local/lib"));
    }

    #[test]
    fn test_resolve_path_relative_path_is_absolute() {
        let base = Path::new("/usr/local");
        assert_eq!(resolve_path(base, "/etc/passwd").unwrap(), PathBuf::from("/etc/passwd"));
    }

    #[test]
    fn test_resolve_path_complex_dot_sequences() {
        let base = Path::new("/a/b/c");
        assert_eq!(resolve_path(base, "../../d/./../e").unwrap(), PathBuf::from("/a/e"));
    }

    #[test]
    fn test_resolve_path_to_root_and_beyond() {
        let base = Path::new("/a/b/c");
        assert_eq!(resolve_path(base, "../../..").unwrap(), PathBuf::from("/"));
        
        let err = resolve_path(base, "../../../..").unwrap_err();
        match err {
            CoreError::InvalidPathError { path: _, message } => {
                assert!(message.contains("von der Wurzel aus zurückzugehen"));
            }
            _ => panic!("Falscher Fehlertyp: {:?}", err),
        }
    }
    
    #[test]
    fn test_resolve_path_with_relative_base() {
        let base = Path::new("some/dir");
        let expected_file_txt = PathBuf::from(format!("some{}dir{}file.txt", MAIN_SEPARATOR, MAIN_SEPARATOR));
        assert_eq!(resolve_path(base, "file.txt").unwrap(), expected_file_txt);
        
        let expected_other_txt = PathBuf::from(format!("some{}other.txt", MAIN_SEPARATOR));
        assert_eq!(resolve_path(base, "../other.txt").unwrap(), expected_other_txt);

        // Fall: some/dir/../.. -> .
        assert_eq!(resolve_path(base, "../..").unwrap(), PathBuf::from("."));
        // Fall: some/dir/../../.. -> ..
        assert_eq!(resolve_path(base, "../../..").unwrap(), PathBuf::from(".."));
    }

    #[test]
    fn test_resolve_path_empty_relative_path_string() {
        let base = Path::new("/usr/local");
        assert_eq!(resolve_path(base, "").unwrap(), PathBuf::from("/usr/local"));
        
        let relative_base = Path::new("data");
        assert_eq!(resolve_path(relative_base, "").unwrap(), PathBuf::from("data"));
    }

    #[test]
    fn test_read_file_to_string_success() {
        let mut temp_file = NamedTempFile::new().unwrap();
        let content = "Hallo, NovaDE Kernschicht!\nMit Umlauten: äöüß.";
        temp_file.write_all(content.as_bytes()).unwrap();
        assert_eq!(read_file_to_string(temp_file.path()).unwrap(), content);
    }

    #[test]
    fn test_read_file_to_string_file_not_found() {
        let path = Path::new("hoffentlich_existiert_diese_datei_niemals.txt");
        let result = read_file_to_string(path);
        assert!(matches!(result, Err(CoreError::IoError(_))));
        if let Err(CoreError::IoError(io_err)) = result {
            assert_eq!(io_err.kind(), std::io::ErrorKind::NotFound);
        }
    }

    const TEST_APP_NAME_FOR_DIRS: &str = "NovaDE-UtilsTest";

    #[test]
    fn test_get_app_config_dir_structure() {
        // Dieser Test ist systemabhängig. Wir prüfen nur, ob ein Pfad zurückgegeben wird
        // und ob er den app_name enthält.
        if let Some(path) = get_app_config_dir(TEST_APP_NAME_FOR_DIRS) {
            assert!(path.ends_with(TEST_APP_NAME_FOR_DIRS), "Pfad {} sollte mit {} enden", path.display(), TEST_APP_NAME_FOR_DIRS);
            // Beispielhafte Überprüfung für Unix-ähnliche Systeme (außer macOS, da dort der Pfad anders ist)
            if cfg!(all(unix, not(target_os = "macos"))) {
                assert!(path.to_string_lossy().contains(".config"), "Unix-Pfad {} sollte .config enthalten", path.display());
            } else if cfg!(target_os = "macos") {
                 assert!(path.to_string_lossy().contains("Library/Application Support"), "macOS-Pfad {} sollte Library/Application Support enthalten", path.display());
            }
        } else {
            // In einigen CI-Umgebungen oder minimalen Systemen ist kein Home-Verzeichnis konfiguriert.
            eprintln!("get_app_config_dir hat None zurückgegeben. Dies kann in bestimmten Testumgebungen normal sein.");
        }
    }

    #[test]
    fn test_get_app_data_dir_structure() {
        if let Some(path) = get_app_data_dir(TEST_APP_NAME_FOR_DIRS) {
            assert!(path.ends_with(TEST_APP_NAME_FOR_DIRS));
            if cfg!(all(unix, not(target_os = "macos"))) {
                assert!(path.to_string_lossy().contains(".local/share"), "Unix-Datenpfad {} sollte .local/share enthalten", path.display());
            } else if cfg!(target_os = "macos") {
                 assert!(path.to_string_lossy().contains("Library/Application Support"), "macOS-Datenpfad {} sollte Library/Application Support enthalten", path.display());
            }
        } else {
            eprintln!("get_app_data_dir hat None zurückgegeben.");
        }
    }
    
    #[test]
    fn test_get_app_cache_dir_structure() {
        if let Some(path) = get_app_cache_dir(TEST_APP_NAME_FOR_DIRS) {
            assert!(path.ends_with(TEST_APP_NAME_FOR_DIRS));
            if cfg!(all(unix, not(target_os = "macos"))) {
                assert!(path.to_string_lossy().contains(".cache"), "Unix-Cachepfad {} sollte .cache enthalten", path.display());
            } else if cfg!(target_os = "macos") {
                 assert!(path.to_string_lossy().contains("Library/Caches"), "macOS-Cachepfad {} sollte Library/Caches enthalten", path.display());
            }
        } else {
            eprintln!("get_app_cache_dir hat None zurückgegeben.");
        }
    }
}
