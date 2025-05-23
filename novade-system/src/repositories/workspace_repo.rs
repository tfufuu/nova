//! Implementierung des `WorkspaceRepository` Traits mittels `sled`.

use crate::persistence::DatabaseManager;
use crate::{SystemError, SystemResult}; // SystemError wird für From<sled::Error> benötigt
use async_trait::async_trait;
use novade_core::types::NovaId;
use novade_core::{debug, error, info, warn}; // Logging
use novade_domain::entities::workspace::Workspace;
use novade_domain::repositories::WorkspaceRepository;
use serde_json; // Für Serialisierung/Deserialisierung

const WORKSPACE_TREE_NAME: &str = "workspaces";

/// Eine `sled`-basierte Implementierung des `WorkspaceRepository`.
#[derive(Clone)]
pub struct SledWorkspaceRepository {
    db_manager: DatabaseManager,
}

impl SledWorkspaceRepository {
    /// Erstellt ein neues `SledWorkspaceRepository`.
    pub fn new(db_manager: DatabaseManager) -> Self {
        Self { db_manager }
    }

    fn open_tree(&self) -> SystemResult<sled::Tree> {
        self.db_manager.open_tree(WORKSPACE_TREE_NAME)
    }
}

#[async_trait]
impl WorkspaceRepository for SledWorkspaceRepository {
    async fn get_by_id(&self, id: &NovaId) -> SystemResult<Option<Workspace>> {
        let tree = self.open_tree()?;
        let id_bytes = id.to_string().into_bytes();

        match tree.get(&id_bytes)? {
            Some(ivec) => {
                let ws: Workspace = serde_json::from_slice(&ivec).map_err(|e| {
                    error!("Deserialisierungsfehler für Workspace ID {}: {}", id, e);
                    SystemError::PersistenceError {
                        store: WORKSPACE_TREE_NAME.to_string(),
                        reason: format!("Fehler beim Deserialisieren des Workspace: {}", e),
                    }
                })?;
                Ok(Some(ws))
            }
            None => Ok(None),
        }
    }
    
    async fn get_by_name(&self, name: &str) -> SystemResult<Option<Workspace>> {
        let tree = self.open_tree()?;
        // Da Sled ein Key-Value-Store ist, müssen wir iterieren, um nach Namen zu suchen,
        // es sei denn, wir führen einen sekundären Index (was Sled nicht direkt unterstützt).
        // Für eine kleine Anzahl von Workspaces ist Iteration okay.
        // Für viele Workspaces wäre eine andere DB oder ein Indexierungsmechanismus besser.
        for item in tree.iter() {
            let (_id_bytes, ivec) = item?;
             match serde_json::from_slice::<Workspace>(&ivec) {
                Ok(ws) => {
                    if ws.name == name {
                        return Ok(Some(ws));
                    }
                }
                Err(e) => {
                     warn!("Konnte Workspace bei get_by_name nicht deserialisieren, überspringe: {}", e);
                }
            }
        }
        Ok(None) // Nicht gefunden
    }

    async fn get_all(&self) -> SystemResult<Vec<Workspace>> {
        let tree = self.open_tree()?;
        let mut workspaces = Vec::new();

        for item in tree.iter() {
            let (_id_bytes, ivec) = item?;
            let ws: Workspace = serde_json::from_slice(&ivec).map_err(|e| {
                error!("Deserialisierungsfehler beim Laden aller Workspaces: {}", e);
                SystemError::PersistenceError {
                    store: WORKSPACE_TREE_NAME.to_string(),
                    reason: format!("Fehler beim Deserialisieren eines Workspace in get_all: {}", e),
                }
            })?;
            workspaces.push(ws);
        }
        Ok(workspaces)
    }

    async fn add(&self, workspace: &Workspace) -> SystemResult<()> {
        let tree = self.open_tree()?;
        let id_bytes = workspace.id.to_string().into_bytes();
        let ws_bytes = serde_json::to_vec(workspace).map_err(|e| {
            SystemError::PersistenceError {
                store: WORKSPACE_TREE_NAME.to_string(),
                reason: format!("Fehler beim Serialisieren des Workspace: {}", e),
            }
        })?;
        
        // Verhindere das Hinzufügen, wenn ein Workspace mit demselben Namen bereits existiert.
        // Dies ist eine Domänenregel, die hier im Repository durchgesetzt wird,
        // alternativ könnte dies im Domänendienst geprüft werden.
        if let Some(existing_ws_by_name) = self.get_by_name(&workspace.name).await? {
            if existing_ws_by_name.id != workspace.id { // Nur prüfen, wenn es nicht dasselbe Objekt ist
                 return Err(SystemError::PersistenceError {
                    store: WORKSPACE_TREE_NAME.to_string(),
                    reason: format!("Ein Workspace mit dem Namen '{}' existiert bereits.", workspace.name),
                });
            }
        }


        tree.insert(id_bytes, ws_bytes)?;
        tree.flush_async().await?;
        info!("Workspace '{}' ({}) hinzugefügt.", workspace.name, workspace.id);
        Ok(())
    }

    async fn update(&self, workspace: &Workspace) -> SystemResult<()> {
        let tree = self.open_tree()?;
        let id_bytes = workspace.id.to_string().into_bytes();

        if !tree.contains_key(&id_bytes)? {
            return Err(SystemError::PersistenceError {
                store: WORKSPACE_TREE_NAME.to_string(),
                reason: format!("Workspace mit ID {} zum Aktualisieren nicht gefunden.", workspace.id),
            });
        }
        
        // Optional: Prüfen, ob der neue Name bereits von einem *anderen* Workspace verwendet wird.
        if let Some(existing_ws_by_name) = self.get_by_name(&workspace.name).await? {
            if existing_ws_by_name.id != workspace.id {
                 return Err(SystemError::PersistenceError {
                    store: WORKSPACE_TREE_NAME.to_string(),
                    reason: format!("Ein anderer Workspace mit dem Namen '{}' existiert bereits.", workspace.name),
                });
            }
        }

        let ws_bytes = serde_json::to_vec(workspace).map_err(|e| {
            SystemError::PersistenceError {
                store: WORKSPACE_TREE_NAME.to_string(),
                reason: format!("Fehler beim Serialisieren des Workspace für Update: {}", e),
            }
        })?;
        
        tree.insert(id_bytes, ws_bytes)?;
        tree.flush_async().await?;
        info!("Workspace '{}' ({}) aktualisiert.", workspace.name, workspace.id);
        Ok(())
    }

    async fn remove(&self, id: &NovaId) -> SystemResult<()> {
        let tree = self.open_tree()?;
        let id_bytes = id.to_string().into_bytes();

        match tree.remove(&id_bytes)? {
            Some(_) => {
                tree.flush_async().await?;
                info!("Workspace mit ID {} entfernt.", id);
                Ok(())
            }
            None => Err(SystemError::PersistenceError {
                store: WORKSPACE_TREE_NAME.to_string(),
                reason: format!("Workspace mit ID {} zum Entfernen nicht gefunden.", id),
            }),
        }
    }
}

// From<sled::Error> for SystemError ist bereits in application_repo.rs definiert.
// Wenn es dort global für die Crate zugänglich gemacht wird, ist es hier nicht erneut nötig.
// Falls nicht, müsste es hierher kopiert oder zentralisiert werden.
// Annahme: Es ist bereits vorhanden und anwendbar.

#[cfg(test)]
mod tests {
    use super::*;
    use crate::persistence::DatabaseManager;
    use tempfile::tempdir;
    use tokio;

    // Hilfsfunktion aus application_repo.rs Tests, angepasst
    // Diese Funktion muss im Testkontext zugänglich sein.
    // Eine Möglichkeit ist, sie in ein gemeinsames Test-Helper-Modul auszulagern
    // oder sie hier zu definieren, wenn sie spezifisch ist.
    // Um die Abhängigkeit von application_repo.rs tests zu vermeiden, definieren wir sie hier neu,
    // aber mit dem Hinweis, dass dies in einem echten Projekt besser organisiert werden sollte.
    fn temp_db_manager_for_ws_repo_tests() -> DatabaseManager {
        let temp_dir = tempdir().unwrap();
        let db_path = temp_dir.path().join("test_ws_repo_db");
        let db = sled::open(&db_path).expect("Test DB konnte nicht geöffnet werden.");
        
        // Diese Hilfsmethode muss auf DatabaseManager existieren oder hier bereitgestellt werden.
        // Wir fügen sie hier lokal für den Test hinzu, falls sie nicht öffentlich ist.
        struct TestDbManager(std::sync::Arc<sled::Db>);
        impl TestDbManager {
            fn db(&self) -> &std::sync::Arc<sled::Db> { &self.0 }
            fn open_tree(&self, name: &str) -> Result<sled::Tree, sled::Error> { self.0.open_tree(name) }
        }
        // Erstellen einer DatabaseManager Instanz.
        // Wenn DatabaseManager::new() verwendet wird, muss sichergestellt sein, dass es mit temp Pfaden umgehen kann
        // oder eine Testversion davon existiert.
        // Sicherste Variante für Tests ist oft, die DB direkt zu erstellen und zu wrappen.
        DatabaseManager::new_from_db_for_test(db) 
    }
    
    // Lokale Definition der Test-Hilfsmethode auf DatabaseManager, falls nicht schon vorhanden.
    // Diese sollte idealerweise in database.rs unter #[cfg(test)] als public Methode stehen.
    #[cfg(test)]
    impl DatabaseManager {
        fn new_from_db_for_test(db: sled::Db) -> Self {
            Self { db: std::sync::Arc::new(db) }
        }
    }

    #[tokio::test]
    async fn test_add_and_get_workspace() {
        let db_manager = temp_db_manager_for_ws_repo_tests();
        let repo = SledWorkspaceRepository::new(db_manager);

        let ws = Workspace::new("Test WS".to_string(), Some("output-1".to_string()));
        repo.add(&ws).await.unwrap();

        let retrieved_ws_opt = repo.get_by_id(&ws.id).await.unwrap();
        assert!(retrieved_ws_opt.is_some(), "Workspace sollte gefunden werden");
        let retrieved_ws = retrieved_ws_opt.unwrap();
        assert_eq!(ws.name, retrieved_ws.name); 
        assert_eq!(ws.id, retrieved_ws.id);
        assert_eq!(ws.primary_output_id, retrieved_ws.primary_output_id);
    }

    #[tokio::test]
    async fn test_get_workspace_by_name() {
        let db_manager = temp_db_manager_for_ws_repo_tests();
        let repo = SledWorkspaceRepository::new(db_manager);

        let ws1 = Workspace::new("Workplace Alpha".to_string(), None);
        let ws2 = Workspace::new("Gaming Room".to_string(), None);
        repo.add(&ws1).await.unwrap();
        repo.add(&ws2).await.unwrap();

        let retrieved_ws_opt = repo.get_by_name("Gaming Room").await.unwrap();
        assert!(retrieved_ws_opt.is_some(), "Workspace 'Gaming Room' sollte gefunden werden");
        let retrieved_ws = retrieved_ws_opt.unwrap();
        assert_eq!(ws2.id, retrieved_ws.id);
        
        let non_existent = repo.get_by_name("Non Existent WS").await.unwrap();
        assert!(non_existent.is_none(), "Nicht existierender Workspace sollte None ergeben");
    }
    
    #[tokio::test]
    async fn test_add_duplicate_name_workspace_fails() {
        let db_manager = temp_db_manager_for_ws_repo_tests();
        let repo = SledWorkspaceRepository::new(db_manager);
        let ws1 = Workspace::new("Unique Name".to_string(), None);
        repo.add(&ws1).await.unwrap();
        
        let ws2_same_name = Workspace::new("Unique Name".to_string(), Some("output-2".to_string()));
        let result = repo.add(&ws2_same_name).await;
        assert!(result.is_err(), "Hinzufügen eines Workspace mit doppeltem Namen sollte fehlschlagen");
        if let Err(SystemError::PersistenceError { reason, .. }) = result {
            assert!(reason.contains("existiert bereits"), "Fehlermeldung sollte auf existierenden Namen hinweisen. Grund: {}", reason);
        } else {
            panic!("Falscher Fehlertyp bei Duplikat-Namen-Test: {:?}", result);
        }
    }

    #[tokio::test]
    async fn test_update_workspace_name_collision() {
        let db_manager = temp_db_manager_for_ws_repo_tests();
        let repo = SledWorkspaceRepository::new(db_manager);
        let ws1 = Workspace::new("Name1".to_string(), None);
        let ws2 = Workspace::new("Name2".to_string(), None);
        repo.add(&ws1).await.unwrap();
        repo.add(&ws2).await.unwrap();

        let mut ws1_updated = ws1.clone();
        ws1_updated.name = "Name2".to_string(); // Versuche, ws1 so umzubenennen, dass es mit ws2 kollidiert
        
        let result = repo.update(&ws1_updated).await;
        assert!(result.is_err(), "Update, das zu Namenskollision führt, sollte fehlschlagen");
         if let Err(SystemError::PersistenceError { reason, .. }) = result {
            assert!(reason.contains("existiert bereits"), "Fehlermeldung sollte auf existierenden Namen hinweisen. Grund: {}", reason);
        } else {
            panic!("Falscher Fehlertyp bei Update-Namenskollisions-Test: {:?}", result);
        }
    }

    #[tokio::test]
    async fn test_get_all_workspaces() {
        let db_manager = temp_db_manager_for_ws_repo_tests();
        let repo = SledWorkspaceRepository::new(db_manager);

        let ws1 = Workspace::new("WS_Numero_Uno".to_string(), None);
        let ws2 = Workspace::new("WS_Numero_Dos".to_string(), Some("dp-1".to_string()));
        
        repo.add(&ws1).await.unwrap();
        repo.add(&ws2).await.unwrap();

        let all_ws = repo.get_all().await.unwrap();
        assert_eq!(all_ws.len(), 2, "Sollte zwei Workspaces finden");
        assert!(all_ws.iter().any(|ws| ws.id == ws1.id && ws.name == ws1.name));
        assert!(all_ws.iter().any(|ws| ws.id == ws2.id && ws.name == ws2.name));
    }

    #[tokio::test]
    async fn test_remove_workspace() {
        let db_manager = temp_db_manager_for_ws_repo_tests();
        let repo = SledWorkspaceRepository::new(db_manager);
        let ws = Workspace::new("DeleteMe".to_string(), None);

        repo.add(&ws).await.unwrap();
        assert!(repo.get_by_id(&ws.id).await.unwrap().is_some(), "Workspace sollte vor dem Löschen existieren");

        repo.remove(&ws.id).await.unwrap();
        assert!(repo.get_by_id(&ws.id).await.unwrap().is_none(), "Workspace sollte nach dem Löschen nicht mehr existieren");
    }
}
