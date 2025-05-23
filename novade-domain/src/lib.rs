//! novade-domain: Zentrale Geschäftslogik und domänenspezifische Entitäten.

// Beispiel für ein Domänenmodul
// pub mod entities;
// pub mod services;
// pub mod repositories;

/// Gibt eine Testnachricht aus, um die Funktionalität der Domänenschicht zu demonstrieren.
pub fn print_domain_message() {
    novade_core::print_core_message(); // Beispiel für Aufruf aus novade-core
    println!("Nachricht von novade-domain: Domänenlogik bereit.");
}
