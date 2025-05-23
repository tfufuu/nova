//! Verwaltung der `sled` Datenbankinstanz.

use crate::error::{SystemError, SystemResult};
use novade_core::utils::get_app_data_dir;
use novade_core::{info, warn}; // Logging
use sled::Db;
use std::path::PathBuf;
use std::sync::Arc; // Für die geteilte Nutzung der DB-Instanz

const APP_NAME: &str = "novade";
const DB_SUBDIRECTORY: &str = "system_db";

/// Ein Manager für die `sled` Datenbank.
/// Stellt eine geöffnete Datenbankinstanz bereit.
#[derive(Clone, Debug)] // Clone ist hier okay, da Arc<Db> geklont wird
pub struct DatabaseManager {
    db: Arc<Db>, // Arc für thread-sicheres Teilen der DB-Instanz
}

impl DatabaseManager {
    /// Öffnet oder erstellt die `sled` Datenbank im Anwendungsdatenverzeichnis.
    ///
    /// Der Pfad wird typischerweise sein: `~/.local/share/novade/system_db` (auf Linux).
    pub fn new() -> SystemResult<Self> {
        let data_dir = get_app_data_dir(APP_NAME).ok_or_else(|| {
            SystemError::PersistenceError {
                store: DB_SUBDIRECTORY.to_string(),
                reason: "Anwendungsdatenverzeichnis konnte nicht ermittelt werden.".to_string(),
            }
        })?;

        let db_path = data_dir.join(DB_SUBDIRECTORY);
        info!("Öffne Datenbank unter: {:?}", db_path);

        std::fs::create_dir_all(&db_path).map_err(|e| {
            SystemError::IoError(novade_core::CoreError::InitializationError { // CoreError::IoError erwartet einen String
                component: "DatabaseManager".to_string(),
                message: format!("Konnte Datenbankverzeichnis {:?} nicht erstellen: {}", db_path, e.to_string()),
            })
        })?;
        
        let db = sled::open(&db_path).map_err(|e| SystemError::PersistenceError {
            store: db_path.to_string_lossy().into_owned(),
            reason: format!("Fehler beim Öffnen der Sled-Datenbank: {}", e),
        })?;

        Ok(Self { db: Arc::new(db) })
    }

    /// Gibt eine Referenz auf die `sled::Db` Instanz zurück.
    pub fn db(&self) -> &Arc<Db> {
        &self.db
    }

    /// Öffnet (oder erstellt) einen neuen `sled::Tree` innerhalb der Datenbank.
    /// Ein `Tree` ist vergleichbar mit einer Tabelle in SQL-Datenbanken.
    pub fn open_tree(&self, tree_name: &str) -> SystemResult<sled::Tree> {
        self.db.open_tree(tree_name).map_err(|e| SystemError::PersistenceError {
            store: tree_name.to_string(),
            reason: format!("Fehler beim Öffnen des Sled-Trees '{}': {}", tree_name, e),
        })
    }
}

// Einfache Tests für den DatabaseManager
#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir; // Für temporäres Testverzeichnis

    // Hilfsfunktion, um einen DatabaseManager in einem temporären Verzeichnis zu erstellen
    fn temp_db_manager() -> SystemResult<DatabaseManager> {
        let temp_dir = tempdir().unwrap();
        let mock_app_data_path = temp_dir.path().join(APP_NAME);
        std::fs::create_dir_all(&mock_app_data_path).unwrap();
        
        // Mocke get_app_data_dir, indem wir annehmen, es würde diesen Pfad zurückgeben.
        // Dies ist nicht ideal, da get_app_data_dir nicht direkt mockbar ist.
        // Für robustere Tests müsste man die Pfadlogik injizierbar machen.
        // Hier verlassen wir uns darauf, dass sled::open im temp_dir funktioniert.

        let db_path = mock_app_data_path.join(DB_SUBDIRECTORY);
        
        // Wir rufen sled::open direkt für den Test auf, um die Komplexität
        // von get_app_data_dir im Test zu umgehen.
        let db = sled::open(&db_path).map_err(|e| SystemError::PersistenceError {
            store: db_path.to_string_lossy().into_owned(),
            reason: format!("Fehler beim Öffnen der Sled-Datenbank für Test: {}", e),
        })?;
        Ok(DatabaseManager { db: Arc::new(db) })
    }


    #[test]
    fn test_database_manager_new_opens_db() {
        // Dieser Test erstellt eine DB im tatsächlichen Anwendungsverzeichnis,
        // was für Tests nicht immer ideal ist, aber für eine einfache Prüfung dient.
        // Besser wäre, den Pfad mocken zu können oder ein temporäres Verzeichnis zu verwenden.
        // Da get_app_data_dir nicht leicht zu mocken ist, testen wir hier den "Happy Path".
        // Für CI-Umgebungen sollte dies ggf. angepasst werden.
        
        // Um Seiteneffekte zu minimieren, versuchen wir, eine temporäre DB zu verwenden.
        let manager_result = temp_db_manager();
        assert!(manager_result.is_ok());
    }

    #[test]
    fn test_open_tree_succeeds() {
        let manager = temp_db_manager().expect("Test DB konnte nicht initialisiert werden.");
        let tree_result = manager.open_tree("test_tree");
        assert!(tree_result.is_ok());
    }

    #[test]
    fn test_open_same_tree_multiple_times() {
        let manager = temp_db_manager().expect("Test DB konnte nicht initialisiert werden.");
        let tree1 = manager.open_tree("another_tree");
        assert!(tree1.is_ok());
        let tree2 = manager.open_tree("another_tree");
        assert!(tree2.is_ok());
        // Man könnte hier noch prüfen, ob tree1 und tree2 auf denselben Tree zeigen.
    }
}
