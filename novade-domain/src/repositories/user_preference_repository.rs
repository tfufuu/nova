//! # User Preference Repository Trait (`repositories::user_preference_repository`)
//!
//! Definiert das Trait [`UserPreferenceRepository`], das als Abstraktion für den
//! Datenzugriff auf [`UserPreferenceSetting`](crate::entities::user_preference::UserPreferenceSetting) Entitäten dient.
//!
//! Diese Schnittstelle ermöglicht das Speichern und Abrufen von Benutzereinstellungen,
//! ohne dass die Domänendienste die Details der Persistenz kennen müssen.
//! Die konkrete Implementierung erfolgt in der Systemschicht (`novade-system`).
//!
//! **Hinweis zur Benutzerbindung**: Die aktuellen Methoden sind nicht explizit
//! benutzergebunden (d.h. sie nehmen keine `user_id` als Parameter). Es wird angenommen,
//! dass die Implementierung des Repositories den aktuellen Benutzerkontext kennt
//! oder dass die Einstellungen systemweit gelten. Zukünftige Erweiterungen könnten
//! eine explizite Benutzer-ID einführen.

use crate::entities::user_preference::UserPreferenceSetting;
use crate::DomainResult;
use async_trait::async_trait;
// use novade_core::types::NovaId; // Auskommentiert, da aktuell nicht für Benutzerbindung verwendet

/// Ein Trait, das Operationen zum Speichern und Abrufen von
/// [`UserPreferenceSetting`](crate::entities::user_preference::UserPreferenceSetting)-Entitäten abstrahiert.
///
/// Implementierungen dieses Traits sind für die Persistenzlogik von Benutzereinstellungen zuständig.
/// Das Trait ist `async_trait`, um asynchrone Operationen zu unterstützen.
/// `Send + Sync` Bounds sind für die thread-sichere Nutzung erforderlich.
#[async_trait]
pub trait UserPreferenceRepository: Send + Sync {
    /// Ruft eine spezifische Benutzereinstellung anhand ihres eindeutigen Schlüssels ab.
    ///
    /// # Parameter
    /// * `key`: Der eindeutige Schlüssel der gesuchten Einstellung (z.B. "theme.dark_mode").
    ///
    /// # Rückgabe
    /// Ein `DomainResult`, das bei Erfolg `Some(UserPreferenceSetting)` enthält, wenn die
    /// Einstellung gefunden wurde, oder `None`, andernfalls ein `DomainError`.
    async fn get_preference(&self, key: &str) -> DomainResult<Option<UserPreferenceSetting>>;
    
    /// Ruft eine Liste aller bekannten Benutzereinstellungen ab.
    ///
    /// Abhängig von der Implementierung können dies systemweite Standardeinstellungen
    /// oder benutzerspezifische Einstellungen sein, falls der Kontext bekannt ist.
    ///
    /// # Rückgabe
    /// Ein `DomainResult`, das bei Erfolg einen Vektor von `UserPreferenceSetting`-Entitäten enthält.
    /// Der Vektor kann leer sein. Im Fehlerfall wird ein `DomainError` zurückgegeben.
    async fn get_all_preferences(&self) -> DomainResult<Vec<UserPreferenceSetting>>;

    /// Speichert eine Benutzereinstellung (fügt hinzu oder aktualisiert sie).
    ///
    /// Wenn bereits eine Einstellung mit demselben Schlüssel existiert, wird diese
    /// typischerweise überschrieben.
    ///
    /// # Parameter
    /// * `setting`: Eine Referenz auf die zu speichernde `UserPreferenceSetting`.
    ///
    /// # Rückgabe
    /// Ein `DomainResult<()>` das bei Erfolg `Ok(())` zurückgibt.
    /// Im Fehlerfall wird ein `DomainError` zurückgegeben.
    async fn set_preference(&self, setting: &UserPreferenceSetting) -> DomainResult<()>;

    // Zukünftige mögliche Erweiterungen:
    // /// Entfernt eine Einstellung anhand ihres Schlüssels.
    // async fn remove_preference(&self, key: &str) -> DomainResult<()>;
    //
    // /// Setzt eine Einstellung auf ihren Standardwert zurück (falls definiert).
    // async fn reset_preference(&self, key: &str) -> DomainResult<()>;
    //
    // /// Setzt alle Einstellungen einer bestimmten Gruppe zurück.
    // async fn reset_preferences_in_group(&self, group_name: &str) -> DomainResult<()>;
}
