//! Das `services` Modul enthält die Domänendienste, welche die Geschäftslogik
//! der NovaDE-Domäne implementieren. Sie verwenden Repository-Traits für den
//! Datenzugriff und operieren auf Domänenentitäten.

pub mod application_service;
pub mod workspace_service;
// Zukünftig: user_preference_service.rs

// Re-exportiere die Dienste für einfacheren Zugriff.
pub use application_service::ApplicationService;
pub use workspace_service::WorkspaceService;
// pub use user_preference_service::UserPreferenceService;
