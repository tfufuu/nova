//! Das `repositories` Modul enthält die konkreten Implementierungen der
//! Repository-Traits aus `novade-domain` unter Verwendung systemspezifischer
//! Persistenzmechanismen (z.B. `sled`).

pub mod application_repo;
pub mod user_preference_repo; // Hinzufügen
pub mod workspace_repo;

// Re-exportiere die konkreten Repository-Implementierungen.
pub use application_repo::SledApplicationRepository;
pub use user_preference_repo::SledUserPreferenceRepository; // Hinzufügen
pub use workspace_repo::SledWorkspaceRepository;
