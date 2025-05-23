//! # Repository-Abstraktionen (`repositories`)
//!
//! Dieses Modul definiert Traits, die als Abstraktionsebene für den Datenzugriff
//! auf Domänenentitäten dienen. Diese Traits werden von der Systemschicht (`novade-system`)
//! implementiert, beispielsweise mittels Datenbanken, Konfigurationsdateien oder anderen
//! Persistenzmechanismen. Die Domänendienste ([`crate::services`]) verwenden diese Traits,
//! um auf Entitätsdaten zuzugreifen und diese zu manipulieren, ohne die Details der
//! konkreten Datenspeicherung kennen zu müssen.
//!
//! Dieses Design fördert die Entkopplung zwischen der Domänenlogik und der
//! Infrastruktur für die Datenhaltung, was die Testbarkeit und Wartbarkeit verbessert.
//!
//! ## Definierte Repository-Traits:
//!
//! - [`application_repository::ApplicationRepository`]: Für den Zugriff auf [`Application`](crate::entities::Application) Entitäten.
//! - [`user_preference_repository::UserPreferenceRepository`]: Für den Zugriff auf [`UserPreferenceSetting`](crate::entities::UserPreferenceSetting) Entitäten.
//! - [`workspace_repository::WorkspaceRepository`]: Für den Zugriff auf [`Workspace`](crate::entities::Workspace) Entitäten.
//!
//! Die Traits werden hier für einen einfacheren Zugriff re-exportiert.

pub mod application_repository;
pub mod user_preference_repository;
pub mod workspace_repository;

// Re-exportiere die Repository-Traits, um den Zugriff für Implementierer und Nutzer zu vereinfachen.
// Ermöglicht z.B. `use novade_domain::repositories::ApplicationRepository;`
pub use application_repository::ApplicationRepository;
pub use user_preference_repository::UserPreferenceRepository;
pub use workspace_repository::WorkspaceRepository;
