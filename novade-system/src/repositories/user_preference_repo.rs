//! Implementierung des `UserPreferenceRepository` Traits mittels `sled`.

use crate::persistence::DatabaseManager;
use crate::{SystemError, SystemResult}; // SystemError wird für From<sled::Error> benötigt
use async_trait::async_trait;
// use novade_core::types::NovaId; // Auskommentiert, da User-Bindung noch nicht implementiert
use novade_core::{debug, error, info, warn}; // Logging
use novade_domain::entities::user_preference::UserPreferenceSetting;
use novade_domain::repositories::UserPreferenceRepository;
use serde_json; // Für Serialisierung/Deserialisierung

const PREFERENCE_TREE_NAME: &str = "user_preferences";

/// Eine `sled`-basierte Implementierung des `UserPreferenceRepository`.
#[derive(Clone)]
pub struct SledUserPreferenceRepository {
    db_manager: DatabaseManager,
}

impl SledUserPreferenceRepository {
    /// Erstellt ein neues `SledUserPreferenceRepository`.
    pub fn new(db_manager: DatabaseManager) -> Self {
        Self { db_manager }
    }

    fn open_tree(&self) -> SystemResult<sled::Tree> {
        self.db_manager.open_tree(PREFERENCE_TREE_NAME)
    }
}

#[async_trait]
impl UserPreferenceRepository for SledUserPreferenceRepository {
    // async fn get_preference(&self, user_id: Option<&NovaId>, key: &str) -> SystemResult<Option<UserPreferenceSetting>> {
    async fn get_preference(&self, key: &str) -> SystemResult<Option<UserPreferenceSetting>> {
        // Aktuell ignorieren wir user_id und speichern Präferenzen global.
        // Eine Multi-User-Implementierung würde den Key präfixen oder separate Trees verwenden.
        let tree = self.open_tree()?;
        let key_bytes = key.as_bytes();

        match tree.get(key_bytes)? {
            Some(ivec) => {
                let setting: UserPreferenceSetting = serde_json::from_slice(&ivec).map_err(|e| {
                    error!("Deserialisierungsfehler für Präferenzschlüssel {}: {}", key, e);
                    SystemError::PersistenceError {
                        store: PREFERENCE_TREE_NAME.to_string(),
                        reason: format!("Fehler beim Deserialisieren der Präferenz: {}", e),
                    }
                })?;
                Ok(Some(setting))
            }
            None => Ok(None),
        }
    }
    
    // async fn get_all_preferences(&self, user_id: Option<&NovaId>) -> SystemResult<Vec<UserPreferenceSetting>> {
    async fn get_all_preferences(&self) -> SystemResult<Vec<UserPreferenceSetting>> {
        let tree = self.open_tree()?;
        let mut settings = Vec::new();

        for item in tree.iter() {
            let (_key_bytes, ivec) = item?;
            let setting: UserPreferenceSetting = serde_json::from_slice(&ivec).map_err(|e| {
                error!("Deserialisierungsfehler beim Laden aller Präferenzen: {}", e);
                SystemError::PersistenceError {
                    store: PREFERENCE_TREE_NAME.to_string(),
                    reason: format!("Fehler beim Deserialisieren einer Präferenz in get_all: {}", e),
                }
            })?;
            settings.push(setting);
        }
        Ok(settings)
    }

    // async fn set_preference(&self, user_id: Option<&NovaId>, setting: &UserPreferenceSetting) -> SystemResult<()> {
    async fn set_preference(&self, setting: &UserPreferenceSetting) -> SystemResult<()> {
        let tree = self.open_tree()?;
        let key_bytes = setting.key.as_bytes();
        let setting_bytes = serde_json::to_vec(setting).map_err(|e| {
            SystemError::PersistenceError {
                store: PREFERENCE_TREE_NAME.to_string(),
                reason: format!("Fehler beim Serialisieren der Präferenz: {}", e),
            }
        })?;
        
        tree.insert(key_bytes, setting_bytes)?;
        tree.flush_async().await?;
        info!("Präferenz '{}' gesetzt.", setting.key);
        Ok(())
    }
}

// From<sled::Error> for SystemError ist bereits in application_repo.rs oder workspace_repo.rs definiert
// und sollte global für die Crate gelten, wenn es nicht #[cfg(test)] spezifisch war.
// Annahme: Es ist bereits vorhanden und anwendbar.

#[cfg(test)]
mod tests {
    use super::*;
    use crate::persistence::DatabaseManager;
    use novade_domain::entities::user_preference::PreferenceValue;
    use tempfile::tempdir;
    use tokio;

    // Hilfsfunktion, idealerweise zentralisiert.
    // Hier wird angenommen, dass eine Test-Konstruktionsmethode auf DatabaseManager existiert.
    // Diese Methode muss in `database.rs` unter `#[cfg(test)]` definiert sein.
    // z.B. impl DatabaseManager { #[cfg(test)] pub fn new_for_test(db: sled::Db) -> Self { Self { db: Arc::new(db) }} }
    fn temp_db_manager_for_pref_repo() -> DatabaseManager {
        let temp_dir = tempdir().unwrap();
        let db_path = temp_dir.path().join("test_pref_repo_db");
        let db = sled::open(&db_path).expect("Test DB konnte nicht geöffnet werden.");
        DatabaseManager::new_from_db_for_test(db) // Verwendung der bereits existierenden Test-Hilfsmethode
    }
    
    // Falls die `new_from_db_for_test` nicht von den anderen Repo-Tests übernommen wurde, hier eine lokale Kopie
    // (idealerweise in database.rs zentralisieren)
    #[cfg(test)]
    impl DatabaseManager {
         #[allow(dead_code)] // Kann als unbenutzt markiert werden, wenn andere Tests sie schon definieren
        fn new_from_db_for_test(db: sled::Db) -> Self {
            Self { db: std::sync::Arc::new(db) }
        }
    }


    #[tokio::test]
    async fn test_set_and_get_preference() {
        let db_manager = temp_db_manager_for_pref_repo();
        let repo = SledUserPreferenceRepository::new(db_manager);

        let setting = UserPreferenceSetting {
            key: "theme.dark_mode".to_string(),
            value: PreferenceValue::Boolean(true),
            display_name: "Dark Mode".to_string(),
            description: Some("Enables dark mode for the UI.".to_string()),
            requires_restart: false,
            group: Some("Appearance".to_string()),
        };

        repo.set_preference(&setting).await.unwrap();
        let retrieved_setting = repo.get_preference("theme.dark_mode").await.unwrap().unwrap();
        assert_eq!(setting, retrieved_setting);
    }

    #[tokio::test]
    async fn test_get_non_existent_preference() {
        let db_manager = temp_db_manager_for_pref_repo();
        let repo = SledUserPreferenceRepository::new(db_manager);
        let result = repo.get_preference("non.existent.key").await.unwrap();
        assert!(result.is_none());
    }

    #[tokio::test]
    async fn test_get_all_preferences() {
        let db_manager = temp_db_manager_for_pref_repo();
        let repo = SledUserPreferenceRepository::new(db_manager);

        let setting1 = UserPreferenceSetting {
            key: "general.show_tips".to_string(),
            value: PreferenceValue::Boolean(false),
            display_name: "Show Tips".to_string(),
            description: None, requires_restart: false, group: None,
        };
        let setting2 = UserPreferenceSetting {
            key: "window.border_size".to_string(),
            value: PreferenceValue::Integer(2),
            display_name: "Window Border Size".to_string(),
            description: None, requires_restart: true, group: Some("Appearance".to_string()),
        };

        repo.set_preference(&setting1).await.unwrap();
        repo.set_preference(&setting2).await.unwrap();

        let all_settings = repo.get_all_preferences().await.unwrap();
        assert_eq!(all_settings.len(), 2);
        assert!(all_settings.contains(&setting1));
        assert!(all_settings.contains(&setting2));
    }
    
    #[tokio::test]
    async fn test_update_preference_by_setting_same_key() {
        let db_manager = temp_db_manager_for_pref_repo();
        let repo = SledUserPreferenceRepository::new(db_manager);

        let initial_setting = UserPreferenceSetting {
            key: "sound.volume".to_string(),
            value: PreferenceValue::Integer(80),
            display_name: "Master Volume".to_string(),
            description: None, requires_restart: false, group: Some("Sound".to_string()),
        };
        repo.set_preference(&initial_setting).await.unwrap();

        let updated_setting = UserPreferenceSetting {
            key: "sound.volume".to_string(), // Gleicher Schlüssel
            value: PreferenceValue::Integer(65), // Neuer Wert
            display_name: "Master Volume".to_string(), // Display Name könnte auch ändern
            description: Some("System master volume level".to_string()), 
            requires_restart: false, 
            group: Some("Sound".to_string()),
        };
        repo.set_preference(&updated_setting).await.unwrap();
        
        let retrieved_setting = repo.get_preference("sound.volume").await.unwrap().unwrap();
        assert_eq!(retrieved_setting.value, PreferenceValue::Integer(65));
        assert_eq!(retrieved_setting.description.unwrap(), "System master volume level");
    }
}
