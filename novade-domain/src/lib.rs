//! # NovaDE Domänenschicht (`novade-domain`)
//!
//! `novade-domain` definiert die Kernlogik und die domänenspezifischen Entitäten
//! des NovaDE Linux Desktop Environments. Diese Schicht ist verantwortlich für
//! die Geschäftsregeln und agiert unabhängig von UI- und Systemdetails.
//! Sie baut auf `novade-core` auf.
//!
//! ## Hauptkomponenten:
//!
//! - **Entitäten ([`entities`])**: Datenstrukturen, die Kernkonzepte wie Anwendungen
//!   ([`Application`]), Arbeitsbereiche ([`Workspace`]) und Benutzereinstellungen
//!   ([`UserPreferenceSetting`]) repräsentieren.
//! - **Repositories ([`repositories`])**: Traits, die Abstraktionen für den Datenzugriff
//!   auf Entitäten definieren (z.B. [`ApplicationRepository`]). Diese werden von
//!   der Systemschicht implementiert.
//! - **Dienste ([`services`])**: Implementieren die eigentliche Geschäftslogik und
//!   orchestrieren Operationen unter Verwendung von Entitäten und Repository-Abstraktionen
//!   (z.B. [`ApplicationService`], [`WorkspaceService`]).
//! - **Fehlerbehandlung ([`error`])**: Definiert domänenspezifische Fehler (`DomainError`)
//!   und ein `DomainResult<T>` für Operationen innerhalb dieser Schicht.
//!
//! ## Designprinzipien:
//!
//! - **Unabhängigkeit**: Keine Abhängigkeiten zu `novade-system` oder `novade-ui`.
//! - **Testbarkeit**: Geschäftslogik ist isoliert und kann durch Mocking der Repositories
//!   gut getestet werden.
//! - **Klare Schnittstellen**: Definiert klare Verträge für die Interaktion mit der Systemschicht
//!   (über Repository-Implementierungen) und der UI-Schicht (über die Domänendienste).
//!
//! ## Verwendung:
//!
//! Die System- und UI-Schichten verwenden `novade-domain`, um auf Geschäftslogik und -daten zuzugreifen.
//!
//! ```rust,no_run
//! use novade_domain::services::ApplicationService;
//! use novade_domain::repositories::ApplicationRepository; // Trait
//! use novade_domain::entities::Application;
//! use novade_domain::DomainResult;
//! use novade_core::types::NovaId;
//! use async_trait::async_trait;
//! use std::sync::Arc;
//!
//! // Beispiel für eine Mock-Implementierung eines Repositories (typischerweise in der Systemschicht oder in Tests)
//! struct MockAppRepo;
//!
//! #[async_trait]
//! impl ApplicationRepository for MockAppRepo {
//!     async fn get_by_id(&self, id: &NovaId) -> DomainResult<Option<Application>> {
//!         // ... Mock-Implementierung ...
//!         Ok(None)
//!     }
//!     async fn get_all(&self) -> DomainResult<Vec<Application>> { Ok(vec![]) }
//!     async fn find_by_name(&self, search_term: &str) -> DomainResult<Vec<Application>> { Ok(vec![]) }
//!     async fn add(&self, application: &Application) -> DomainResult<()> { Ok(()) }
//!     async fn update(&self, application: &Application) -> DomainResult<()> { Ok(()) }
//!     async fn remove(&self, id: &NovaId) -> DomainResult<()> { Ok(()) }
//! }
//!
//! #[tokio::main]
//! async fn main() -> DomainResult<()> {
//!     // Logging initialisieren (aus novade-core)
//!     let core_config = novade_core::CoreConfig::example();
//!     let _ = novade_core::initialize_logging(&core_config);
//!
//!     let app_repo = Arc::new(MockAppRepo);
//!     let app_service = ApplicationService::new(app_repo);
//!
//!     let apps = app_service.list_all_applications().await?;
//!     novade_core::info!("Gefundene Anwendungen: {}", apps.len());
//!
//!     Ok(())
//! }
//! ```

// Module werden öffentlich gemacht
pub mod entities;
pub mod error;
pub mod repositories;
pub mod services;

// Re-exportiere die wichtigsten Elemente für eine einfachere Nutzung.
pub use error::{DomainError, DomainResult};

// Re-Exporte aus entities (Beispiele, je nach Häufigkeit der Nutzung anpassen)
pub use entities::{
    Application, ApplicationType, PreferenceValue, UserPreferenceSetting, Workspace,
};

// Re-Exporte aus repositories (Traits sind wichtig für Implementierer)
pub use repositories::{
    ApplicationRepository, UserPreferenceRepository, WorkspaceRepository,
};

// Re-Exporte aus services (Dienste sind die Haupt-Einstiegspunkte für die Logik)
pub use services::{ApplicationService, WorkspaceService};


/// Gibt eine Testnachricht aus, um die Funktionalität der Domänenschicht zu demonstrieren.
///
/// Diese Funktion dient primär zu Test- und Demonstrationszwecken.
pub fn print_domain_message() {
    // Rufe eine Funktion aus novade-core auf, um die Abhängigkeit zu zeigen
    novade_core::print_core_message();

    // Verwende das Logging aus novade-core
    novade_core::info!(target: "novade_domain_lib", "Nachricht von novade-domain: Domänenlogik bereit.");
    println!("Nachricht von novade-domain: Domänenlogik bereit (via println).");
}

#[cfg(test)]
mod tests {
    use super::*; // Importiert alles aus dem lib-Modul, inkl. re-exportierter Typen

    #[test]
    fn test_print_domain_message_from_lib() {
        // Logging für den Test initialisieren
        let core_config = novade_core::CoreConfig::example();
        let _ = novade_core::initialize_logging(&core_config);

        print_domain_message();
    }

    #[test]
    fn test_domain_imports_are_accessible() {
        // Dieser Test prüft, ob die Re-Exporte funktionieren und Typen zugänglich sind.
        // Er ist mehr ein Compile-Zeit-Check.
        let _app_service: Option<ApplicationService> = None; // Nur Typverwendung
        let _id = novade_core::types::NovaId::new(); // Sicherstellen, dass Core-Typen auch gehen
        let _app = Application::new_desktop("test".to_string(), "/bin/test".to_string(), None);
        
        novade_core::info!("Domain-Import-Test: Typen sind zugänglich.");
        // Keine Assertions nötig, der Test besteht, wenn er kompiliert.
    }
}
