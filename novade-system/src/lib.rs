//! novade-system: Interaktion mit dem Betriebssystem, der Hardware und externen Diensten.

// Beispiel für Systemmodule (entsprechend Implementierungsplan.md)
// pub mod display_manager_service;
// pub mod network_manager_service;
// pub mod audio_service;
// pub mod input_service;
// pub mod power_management_service;
// pub mod notification_service;
// pub mod storage_service;
// pub mod process_manager_service;

/// Gibt eine Testnachricht aus, um die Funktionalität der Systemschicht zu demonstrieren.
pub fn print_system_message() {
    novade_domain::print_domain_message(); // Beispiel für Aufruf aus novade-domain
    println!("Nachricht von novade-system: Systemdienste bereit.");
}
