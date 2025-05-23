//! Implementierung des `ApplicationRepository` Traits mittels `sled`.

use crate::persistence::DatabaseManager;
use crate::{SystemError, SystemResult};
use async_trait::async_trait;
use novade_core::types::NovaId;
use novade_core::{debug, error, info, warn}; // Logging
use novade_domain::entities::application::Application;
use novade_domain::repositories::ApplicationRepository;
use serde_json; // Für Serialisierung/Deserialisierung

const APP_TREE_NAME: &str = "applications";

/// Eine `sled`-basierte Implementierung des `ApplicationRepository`.
#[derive(Clone)] // Clone, da DatabaseManager Clone ist (wegen Arc<Db>)
pub struct SledApplicationRepository {
    db_manager: DatabaseManager,
}

impl SledApplicationRepository {
    /// Erstellt ein neues `SledApplicationRepository`.
    pub fn new(db_manager: DatabaseManager) -> Self {
        Self { db_manager }
    }

    // Interne Hilfsfunktion zum Öffnen des Trees
    fn open_tree(&self) -> SystemResult<sled::Tree> {
        self.db_manager.open_tree(APP_TREE_NAME)
    }
}

#[async_trait]
impl ApplicationRepository for SledApplicationRepository {
    async fn get_by_id(&self, id: &NovaId) -> SystemResult<Option<Application>> {
        let tree = self.open_tree()?;
        let id_bytes = id.to_string().into_bytes(); // NovaId wird zu String und dann zu Bytes

        match tree.get(&id_bytes)? {
            Some(ivec) => {
                let app: Application = serde_json::from_slice(&ivec).map_err(|e| {
                    error!("Deserialisierungsfehler für App ID {}: {}", id, e);
                    SystemError::PersistenceError {
                        store: APP_TREE_NAME.to_string(),
                        reason: format!("Fehler beim Deserialisieren der Anwendung: {}", e),
                    }
                })?;
                Ok(Some(app))
            }
            None => Ok(None),
        }
    }

    async fn get_all(&self) -> SystemResult<Vec<Application>> {
        let tree = self.open_tree()?;
        let mut apps = Vec::new();

        for item in tree.iter() {
            let (_id_bytes, ivec) = item?; // Fehler beim Iterieren abfangen
            let app: Application = serde_json::from_slice(&ivec).map_err(|e| {
                error!("Deserialisierungsfehler beim Laden aller Apps: {}", e);
                // Hier könnte man entscheiden, fehlerhafte Einträge zu überspringen oder abzubrechen
                SystemError::PersistenceError {
                    store: APP_TREE_NAME.to_string(),
                    reason: format!("Fehler beim Deserialisieren einer Anwendung in get_all: {}", e),
                }
            })?;
            apps.push(app);
        }
        Ok(apps)
    }

    async fn find_by_name(&self, search_term: &str) -> SystemResult<Vec<Application>> {
        let tree = self.open_tree()?;
        let mut apps = Vec::new();
        let lower_search_term = search_term.to_lowercase();

        for item in tree.iter() {
            let (_id_bytes, ivec) = item?;
            match serde_json::from_slice::<Application>(&ivec) {
                Ok(app) => {
                    if app.name.to_lowercase().contains(&lower_search_term) || 
                       app.display_name.as_ref().map_or(false, |dn| dn.to_lowercase().contains(&lower_search_term)) {
                        apps.push(app);
                    }
                }
                Err(e) => {
                    warn!("Konnte App bei find_by_name nicht deserialisieren, überspringe: {}", e);
                    // Fehler nicht propagieren, sondern nur loggen und weitermachen
                }
            }
        }
        Ok(apps)
    }

    async fn add(&self, application: &Application) -> SystemResult<()> {
        let tree = self.open_tree()?;
        let id_bytes = application.id.to_string().into_bytes();
        let app_bytes = serde_json::to_vec(application).map_err(|e| {
            SystemError::PersistenceError {
                store: APP_TREE_NAME.to_string(),
                reason: format!("Fehler beim Serialisieren der Anwendung: {}", e),
            }
        })?;

        // Optional: Prüfen, ob die ID bereits existiert, um Überschreiben zu verhindern
        // if tree.contains_key(&id_bytes)? {
        //     return Err(SystemError::PersistenceError {
        //         store: APP_TREE_NAME.to_string(),
        //         reason: format!("Anwendung mit ID {} existiert bereits.", application.id),
        //     });
        // }

        tree.insert(id_bytes, app_bytes)?;
        tree.flush_async().await?; // Wichtig für Persistenz
        info!("Anwendung '{}' ({}) hinzugefügt.", application.name, application.id);
        Ok(())
    }

    async fn update(&self, application: &Application) -> SystemResult<()> {
        let tree = self.open_tree()?;
        let id_bytes = application.id.to_string().into_bytes();

        if !tree.contains_key(&id_bytes)? {
            return Err(SystemError::PersistenceError { // Oder ein spezifischerer Fehler wie EntityNotFound
                store: APP_TREE_NAME.to_string(),
                reason: format!("Anwendung mit ID {} zum Aktualisieren nicht gefunden.", application.id),
            });
        }

        let app_bytes = serde_json::to_vec(application).map_err(|e| {
            SystemError::PersistenceError {
                store: APP_TREE_NAME.to_string(),
                reason: format!("Fehler beim Serialisieren der Anwendung für Update: {}", e),
            }
        })?;
        
        tree.insert(id_bytes, app_bytes)?;
        tree.flush_async().await?;
        info!("Anwendung '{}' ({}) aktualisiert.", application.name, application.id);
        Ok(())
    }

    async fn remove(&self, id: &NovaId) -> SystemResult<()> {
        let tree = self.open_tree()?;
        let id_bytes = id.to_string().into_bytes();

        match tree.remove(&id_bytes)? {
            Some(_) => {
                tree.flush_async().await?;
                info!("Anwendung mit ID {} entfernt.", id);
                Ok(())
            }
            None => Err(SystemError::PersistenceError { // Oder EntityNotFound
                store: APP_TREE_NAME.to_string(),
                reason: format!("Anwendung mit ID {} zum Entfernen nicht gefunden.", id),
            }),
        }
    }
}

// Konvertiere sled::Error zu SystemError für interne Verwendung
impl From<sled::Error> for SystemError {
    fn from(e: sled::Error) -> Self {
        SystemError::PersistenceError {
            store: "sled_db".to_string(), // Allgemeiner Store-Name für Sled-Fehler
            reason: e.to_string(),
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::persistence::DatabaseManager; // Für temp_db_manager
    use novade_domain::entities::application::ApplicationType;
    use tempfile::tempdir;
    use tokio;

    // Hilfsfunktion aus database.rs tests, angepasst für dieses Modul
    fn temp_db_manager_for_app_repo() -> DatabaseManager {
        let temp_dir = tempdir().unwrap();
        let db_path = temp_dir.path().join("test_app_repo_db");
        let db = sled::open(&db_path).expect("Test DB konnte nicht geöffnet werden.");
        DatabaseManager::new_from_db_for_test(db) // Annahme: Es gibt einen Test-Konstruktor
    }
    
    // Mock-Konstruktor für DatabaseManager (vereinfacht)
    // In einer echten Anwendung wäre DatabaseManager::new() komplexer und würde Pfade verwenden.
    // Hier erstellen wir eine temporäre DB direkt.
    // Diese Implementierung muss im `database.rs` Modul existieren oder hier zugänglich gemacht werden.
    // Für diesen Test nehmen wir an, sie ist verfügbar.
    // Wenn nicht, füge dies zu `database.rs` hinzu:
    /*
    #[cfg(test)]
    impl DatabaseManager {
        pub fn new_from_db_for_test(db: sled::Db) -> Self {
            Self { db: std::sync::Arc::new(db) }
        }
    }
    */
    // Da wir `database.rs` nicht direkt ändern, fügen wir hier eine lokale Implementierung
    // für den Testkontext ein, falls die originale nicht #[cfg(test)] public ist.
    #[cfg(test)]
    impl DatabaseManager {
        fn new_from_db_for_test(db: sled::Db) -> Self {
            Self { db: std::sync::Arc::new(db) }
        }
    }


    #[tokio::test]
    async fn test_add_and_get_application() {
        let db_manager = temp_db_manager_for_app_repo();
        let repo = SledApplicationRepository::new(db_manager);

        let app = Application {
            id: NovaId::new(),
            name: "TestApp".to_string(),
            display_name: Some("My Test Application".to_string()),
            executable_path: "/usr/bin/testapp".to_string(),
            arguments: Some(vec!["--test".to_string()]),
            working_directory: Some("/tmp".to_string()),
            icon_name: Some("test-app-icon".to_string()),
            app_type: ApplicationType::Desktop,
            categories: Some(vec!["Utility".to_string()]),
            keywords: Some(vec!["test".to_string(), "example".to_string()]),
            description: Some("Eine Testanwendung".to_string()),
            version: Some(novade_core::types::Version::new(1,0,0)),
        };

        repo.add(&app).await.unwrap();
        let retrieved_app = repo.get_by_id(&app.id).await.unwrap().unwrap();
        assert_eq!(app, retrieved_app);
    }

    #[tokio::test]
    async fn test_get_non_existent_application() {
        let db_manager = temp_db_manager_for_app_repo();
        let repo = SledApplicationRepository::new(db_manager);
        let non_existent_id = NovaId::new();
        let result = repo.get_by_id(&non_existent_id).await.unwrap();
        assert!(result.is_none());
    }

    #[tokio::test]
    async fn test_get_all_applications() {
        let db_manager = temp_db_manager_for_app_repo();
        let repo = SledApplicationRepository::new(db_manager);

        let app1 = Application::new_desktop("App1".to_string(), "/bin/app1".to_string(), None);
        let app2 = Application::new_desktop("App2".to_string(), "/bin/app2".to_string(), None);

        repo.add(&app1).await.unwrap();
        repo.add(&app2).await.unwrap();

        let all_apps = repo.get_all().await.unwrap();
        assert_eq!(all_apps.len(), 2);
        // Die Reihenfolge ist nicht garantiert bei sled, daher prüfen wir auf Existenz
        assert!(all_apps.contains(&app1));
        assert!(all_apps.contains(&app2));
    }
    
    #[tokio::test]
    async fn test_find_by_name() {
        let db_manager = temp_db_manager_for_app_repo();
        let repo = SledApplicationRepository::new(db_manager);

        let app1 = Application::new_desktop("Firefox".to_string(), "/bin/firefox".to_string(), None);
        let app2 = Application::new_desktop("Thunderbird".to_string(), "/bin/thunderbird".to_string(), None);
        let app3 = Application::new_desktop("Firewall Config".to_string(), "/bin/firewall".to_string(), None);


        repo.add(&app1).await.unwrap();
        repo.add(&app2).await.unwrap();
        repo.add(&app3).await.unwrap();

        let found_apps = repo.find_by_name("fire").await.unwrap();
        assert_eq!(found_apps.len(), 2); // Firefox, Firewall Config
        assert!(found_apps.iter().any(|a| a.name == "Firefox"));
        assert!(found_apps.iter().any(|a| a.name == "Firewall Config"));

        let found_apps_case = repo.find_by_name("FiRe").await.unwrap();
         assert_eq!(found_apps_case.len(), 2);
    }


    #[tokio::test]
    async fn test_update_application() {
        let db_manager = temp_db_manager_for_app_repo();
        let repo = SledApplicationRepository::new(db_manager);
        let mut app = Application::new_desktop("MyApp".to_string(), "/bin/myapp".to_string(), None);
        
        repo.add(&app).await.unwrap();
        
        app.name = "MyUpdatedApp".to_string();
        app.icon_name = Some("new-icon".to_string());
        repo.update(&app).await.unwrap();

        let retrieved_app = repo.get_by_id(&app.id).await.unwrap().unwrap();
        assert_eq!(retrieved_app.name, "MyUpdatedApp");
        assert_eq!(retrieved_app.icon_name.unwrap(), "new-icon");
    }

    #[tokio::test]
    async fn test_remove_application() {
        let db_manager = temp_db_manager_for_app_repo();
        let repo = SledApplicationRepository::new(db_manager);
        let app = Application::new_desktop("ToBeRemoved".to_string(), "/bin/toberemoved".to_string(), None);

        repo.add(&app).await.unwrap();
        assert!(repo.get_by_id(&app.id).await.unwrap().is_some());

        repo.remove(&app.id).await.unwrap();
        assert!(repo.get_by_id(&app.id).await.unwrap().is_none());
    }
}
