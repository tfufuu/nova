//! Das `persistence` Modul ist verantwortlich für die Datenpersistenz
//! der `novade-system` Schicht. Es abstrahiert die Interaktion mit
//! der zugrundeliegenden Datenbank (z.B. `sled`).

pub mod database;

// Re-exportiere wichtige Strukturen für einfacheren Zugriff.
pub use database::DatabaseManager;
