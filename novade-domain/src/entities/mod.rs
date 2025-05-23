//! # Domänenentitäten (`entities`)
//!
//! Dieses Modul enthält die Definitionen der Kern-Domänenentitäten von NovaDE.
//! Diese Entitäten repräsentieren die zentralen Konzepte und Datenstrukturen,
//! mit denen die Domänenlogik in den [`crate::services`] operiert.
//!
//! Jede Entität ist in ihrem eigenen Untermodul definiert:
//! - [`application`]: Definiert [`Application`] und [`ApplicationType`].
//! - [`user_preference`]: Definiert [`UserPreferenceSetting`] und [`PreferenceValue`].
//! - [`workspace`]: Definiert [`Workspace`].
//!
//! Die wichtigsten Entitäten werden hier für einen einfacheren Zugriff aus anderen Teilen
//! der `novade-domain` Crate oder von externen Crates re-exportiert.

pub mod application;
pub mod user_preference;
pub mod workspace;

// Re-exportiere die Kernentitäten für einen einfacheren Zugriff.
// Dies ermöglicht es, z.B. `novade_domain::entities::Application` anstelle von
// `novade_domain::entities::application::Application` zu verwenden, wenn dieses Modul importiert wird.
// Für den direkten Zugriff über `novade_domain::*` (wie in `lib.rs` konfiguriert) sind diese spezifischen
// Re-Exporte hier weniger kritisch, aber sie sind nützlich für eine klare Struktur innerhalb des `entities`-Moduls.
pub use application::{Application, ApplicationType};
pub use user_preference::{PreferenceValue, UserPreferenceSetting};
pub use workspace::Workspace;
