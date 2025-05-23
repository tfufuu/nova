//! Domänendienst für die Verwaltung von Anwendungen.

use crate::entities::application::{Application, ApplicationType};
use crate::repositories::application_repository::ApplicationRepository;
use crate::{DomainError, DomainResult};
use novade_core::types::NovaId;
use novade_core::info; // Logging
use std::sync::Arc;

pub struct ApplicationService {
    app_repository: Arc<dyn ApplicationRepository>,
}

impl ApplicationService {
    /// Erstellt einen neuen `ApplicationService`.
    pub fn new(app_repository: Arc<dyn ApplicationRepository>) -> Self {
        Self { app_repository }
    }

    /// Listet alle bekannten Anwendungen auf.
    pub async fn list_all_applications(&self) -> DomainResult<Vec<Application>> {
        info!("Auflistung aller Anwendungen angefordert.");
        self.app_repository.get_all().await
    }

    /// Sucht Anwendungen anhand eines Namens.
    pub async fn find_applications_by_name(&self, name_query: &str) -> DomainResult<Vec<Application>> {
        if name_query.trim().is_empty() {
            return Err(DomainError::ValidationError {
                field: "name_query".to_string(),
                message: "Suchbegriff darf nicht leer sein.".to_string(),
            });
        }
        info!(name_query, "Suche nach Anwendungen.");
        self.app_repository.find_by_name(name_query).await
    }
    
    /// Registriert eine neue Anwendung im System.
    pub async fn register_application(&self, app_data: Application) -> DomainResult<Application> {
        info!(app_name = %app_data.name, app_id = %app_data.id, "Registriere neue Anwendung.");
        // Hier könnten Validierungen stattfinden, z.B. ob der Pfad existiert (obwohl das eher Systemschicht wäre)
        // oder ob eine Anwendung mit gleichem Namen/Pfad schon existiert.
        if app_data.executable_path.trim().is_empty() {
             return Err(DomainError::ValidationError {
                field: "executable_path".to_string(),
                message: "Pfad zur ausführbaren Datei darf nicht leer sein.".to_string(),
            });
        }
        self.app_repository.add(&app_data).await?;
        Ok(app_data)
    }

    /// Ruft Details zu einer spezifischen Anwendung ab.
    pub async fn get_application_details(&self, app_id: &NovaId) -> DomainResult<Option<Application>> {
        info!(%app_id, "Details für Anwendung angefordert.");
        self.app_repository.get_by_id(app_id).await
    }

    // Weitere Methoden, z.B. für das Starten einer Anwendung (was hier eher das
    // "Vorbereiten zum Starten" bedeuten würde, der eigentliche Prozessstart
    // wäre in der Systemschicht).
    // pub async fn prepare_launch_application(&self, app_id: &NovaId) -> DomainResult<LaunchInfo> { ... }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::repositories::application_repository::MockApplicationRepository; // mockall generiert dies
    use novade_core::CoreError; // für RepositoryError wrapping
    use tokio; // für async tests

    #[tokio::test]
    async fn test_list_all_applications_success() {
        let mut mock_repo = MockApplicationRepository::new();
        let app1 = Application::new_desktop("App1".to_string(), "/bin/app1".to_string(), None);
        let app2 = Application::new_desktop("App2".to_string(), "/bin/app2".to_string(), None);
        let expected_apps = vec![app1.clone(), app2.clone()];

        mock_repo
            .expect_get_all()
            .times(1)
            .returning(move || Ok(expected_apps.clone()));

        let service = ApplicationService::new(Arc::new(mock_repo));
        let result = service.list_all_applications().await;

        assert!(result.is_ok());
        assert_eq!(result.unwrap().len(), 2);
    }

    #[tokio::test]
    async fn test_list_all_applications_error() {
        let mut mock_repo = MockApplicationRepository::new();
        let core_err = CoreError::UnknownError("Datenbank nicht erreichbar".to_string());
        
        mock_repo
            .expect_get_all()
            .times(1)
            .returning(move || Err(DomainError::RepositoryError(core_err.clone())));

        let service = ApplicationService::new(Arc::new(mock_repo));
        let result = service.list_all_applications().await;

        assert!(result.is_err());
        match result.unwrap_err() {
            DomainError::RepositoryError(CoreError::UnknownError(msg)) => {
                assert_eq!(msg, "Datenbank nicht erreichbar");
            }
            _ => panic!("Falscher Fehlertyp"),
        }
    }
    
    #[tokio::test]
    async fn test_register_application_empty_path() {
        let mock_repo = MockApplicationRepository::new(); // Wird nicht aufgerufen
        let service = ApplicationService::new(Arc::new(mock_repo));
        let app_data = Application {
            id: NovaId::new(),
            name: "Test App".to_string(),
            display_name: None,
            executable_path: " ".to_string(), // Leerer Pfad
            arguments: None,
            working_directory: None,
            icon_name: None,
            app_type: ApplicationType::Desktop,
            categories: None,
            keywords: None,
            description: None,
            version: None,
        };

        let result = service.register_application(app_data).await;
        assert!(matches!(result, Err(DomainError::ValidationError {field, ..}) if field == "executable_path"));
    }
}
