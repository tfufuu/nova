//! novade-system: Interaktion mit dem Betriebssystem, der Hardware und externen Diensten.

pub mod error;
// Re-exportiere SystemError und SystemResult für leichtere Erreichbarkeit
pub use error::{SystemError, SystemResult};

// Platzhalter für zukünftige Module
// pub mod persistence;
// pub mod repositories;
// pub mod process_manager;
// pub mod display_manager; // etc.

pub mod persistence;
// Optional: Re-exportiere DatabaseManager direkt unter novade_system::
// pub use persistence::DatabaseManager;

pub mod repositories;
// Für den Moment exportieren wir die konkrete Implementierung nicht direkt aus der Crate-Wurzel.
// Die Instanziierung erfolgt typischerweise innerhalb dieser Crate,
// und die Domänenschicht erhält eine Trait-Referenz.

pub mod process_manager;
// Optional: Re-exportiere ProcessManager Trait oder DefaultProcessManager direkt.
// pub use process_manager::ProcessManager;


/// Gibt eine Testnachricht aus, um die Funktionalität der Systemschicht zu demonstrieren.
pub fn print_system_message() {
    // Rufe eine Funktion aus novade-domain auf, um die Abhängigkeit zu zeigen
    novade_domain::print_domain_message(); 
    
    // Verwende das Logging aus novade-core
    novade_core::info!(target: "novade_system_lib", "Nachricht von novade-system: Systemdienste bereit.");
    println!("Nachricht von novade-system: Systemdienste bereit (via println).");
}

#[cfg(test)]
mod tests {
    use super::*; // Importiert print_system_message, SystemError, SystemResult
    use novade_core::CoreError; // Für den Test des Error-Wrappings

    #[test]
    fn test_print_system_message() {
        // Logging für den Test initialisieren
        let core_config = novade_core::CoreConfig::example();
        let _ = novade_core::initialize_logging(&core_config);
        
        print_system_message();
    }

    #[test]
    fn test_io_error_wrapping() {
        // Erzeuge einen Beispiel CoreError (der einen IO-Fehler wrappt)
        // In novade-core wurde CoreError::IoError so geändert, dass es einen String nimmt,
        // nachdem std::io::Error nicht Clone ist. Wir simulieren das hier.
        let core_io_error_string = "Datei nicht gefunden".to_string();
        let core_error = novade_core::CoreError::InitializationError { // Beispiel für einen CoreError
            component: "Test".to_string(),
            message: core_io_error_string.clone(),
        };
        

        // Konvertiere ihn in einen SystemError
        let system_error: SystemError = core_error.into();

        match system_error {
            SystemError::IoError(novade_core::CoreError::InitializationError{component, message}) => {
                assert_eq!(component, "Test");
                assert_eq!(message, core_io_error_string);
            }
            _ => panic!("Falscher Fehlertyp nach Konvertierung. Erwartet SystemError::IoError(CoreError::InitializationError)"),
        }
        novade_core::info!("SystemError wrapping (IoError from CoreError) test passed: {}", system_error);
    }
}
