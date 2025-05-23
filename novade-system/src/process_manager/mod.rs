//! Das `process_manager` Modul stellt Funktionalitäten zum Starten und
//! Verwalten von externen Prozessen (Anwendungen) bereit.

pub mod default_process_manager;

// Re-exportiere das Trait und die Standardimplementierung.
pub use default_process_manager::{DefaultProcessManager, ProcessId, ProcessManager};
