//! Domänendienst für die Verwaltung von Workspaces.

use crate::entities::workspace::Workspace;
use crate::repositories::workspace_repository::WorkspaceRepository;
use crate::{DomainError, DomainResult};
use novade_core::types::NovaId;
use novade_core::info; // Logging
use std::sync::Arc;

pub struct WorkspaceService {
    workspace_repository: Arc<dyn WorkspaceRepository>,
}

impl WorkspaceService {
    pub fn new(workspace_repository: Arc<dyn WorkspaceRepository>) -> Self {
        Self { workspace_repository }
    }

    pub async fn create_new_workspace(&self, name: String, primary_output_id: Option<String>) -> DomainResult<Workspace> {
        if name.trim().is_empty() {
            return Err(DomainError::ValidationError {
                field: "name".to_string(),
                message: "Workspace-Name darf nicht leer sein.".to_string(),
            });
        }
        // Prüfen, ob ein Workspace mit diesem Namen bereits existiert
        if self.workspace_repository.get_by_name(&name).await?.is_some() {
            return Err(DomainError::OperationNotPermitted {
                operation: "create_workspace".to_string(),
                reason: format!("Ein Workspace mit dem Namen '{}' existiert bereits.", name),
            });
        }

        let workspace = Workspace::new(name.clone(), primary_output_id);
        info!(workspace_id = %workspace.id, workspace_name = %workspace.name, "Erstelle neuen Workspace.");
        self.workspace_repository.add(&workspace).await?;
        Ok(workspace)
    }

    pub async fn list_all_workspaces(&self) -> DomainResult<Vec<Workspace>> {
        info!("Auflistung aller Workspaces angefordert.");
        self.workspace_repository.get_all().await
    }
    
    pub async fn get_workspace_details(&self, id: &NovaId) -> DomainResult<Option<Workspace>> {
        info!(workspace_id = %id, "Details für Workspace angefordert.");
        self.workspace_repository.get_by_id(id).await
    }

    // Weitere Methoden z.B. zum Wechseln, Schließen, Umbenennen von Workspaces
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::repositories::workspace_repository::MockWorkspaceRepository;
    use tokio;

    #[tokio::test]
    async fn test_create_new_workspace_success() {
        let mut mock_repo = MockWorkspaceRepository::new();
        let workspace_name = "Test Workspace".to_string();

        mock_repo.expect_get_by_name()
            .withf(move |name_param: &str| name_param == workspace_name)
            .times(1)
            .returning(|_| Ok(None)); // Kein Workspace mit dem Namen existiert

        mock_repo.expect_add()
            .times(1)
            .returning(|_ws| Ok(()));
        
        let service = WorkspaceService::new(Arc::new(mock_repo));
        let result = service.create_new_workspace("Test Workspace".to_string(), None).await;

        assert!(result.is_ok());
        let workspace = result.unwrap();
        assert_eq!(workspace.name, "Test Workspace");
    }

    #[tokio::test]
    async fn test_create_new_workspace_name_exists() {
        let mut mock_repo = MockWorkspaceRepository::new();
        let existing_workspace = Workspace::new("Existing".to_string(), None);
        
        mock_repo.expect_get_by_name()
            .withf(|name: &str| name == "Existing")
            .times(1)
            .returning(move |_| Ok(Some(existing_workspace.clone())));

        // add wird nicht erwartet
        mock_repo.expect_add().never();

        let service = WorkspaceService::new(Arc::new(mock_repo));
        let result = service.create_new_workspace("Existing".to_string(), None).await;

        assert!(result.is_err());
        match result.unwrap_err() {
            DomainError::OperationNotPermitted { reason, .. } => {
                assert!(reason.contains("existiert bereits"));
            }
            _ => panic!("Falscher Fehlertyp"),
        }
    }
}
