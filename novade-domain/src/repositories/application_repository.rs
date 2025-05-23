//! # Application Repository Trait (`repositories::application_repository`)
//!
//! Definiert das Trait [`ApplicationRepository`], das als Abstraktion für den
//! Datenzugriff auf [`Application`](crate::entities::Application) Entitäten dient.
//!
//! Dieses Trait muss von einer konkreten Implementierung in der Systemschicht
//! (`novade-system`) erfüllt werden, um die tatsächliche Speicherung und das Abrufen
//! von Anwendungsdaten zu handhaben (z.B. aus einer Datenbank, Konfigurationsdateien
//! oder einem Verzeichnis von `.desktop`-Dateien).

use crate::entities::application::Application;
use crate::DomainResult; // Stellt sicher, dass Fehler als DomainError zurückgegeben werden
use async_trait::async_trait;
use novade_core::types::NovaId;

/// Ein Trait, das Operationen zum Speichern, Abrufen und Verwalten von
/// [`Application`](crate::entities::Application)-Entitäten abstrahiert.
///
/// Implementierungen dieses Traits sind verantwortlich für die Persistenzlogik.
/// Das Trait ist als `async_trait` definiert, um asynchrone Operationen zu ermöglichen,
#[cfg_attr(test, mockall::automock)] // Hinzugefügt für Mocking in Tests
/// was typisch für I/O-gebundene Aufgaben wie Datenbankzugriffe ist.
/// Die `Send + Sync` Bounds sind notwendig, damit Implementierungen sicher über Threads
/// hinweg geteilt werden können (z.B. wenn sie in einem `Arc` gehalten werden).
#[async_trait]
pub trait ApplicationRepository: Send + Sync {
    /// Ruft eine spezifische Anwendung anhand ihrer eindeutigen ID ab.
    ///
    /// # Parameter
    /// * `id`: Die [`NovaId`] der gesuchten Anwendung.
    ///
    /// # Rückgabe
    /// Ein `DomainResult` das bei Erfolg `Some(Application)` enthält, wenn die Anwendung
    /// gefunden wurde, oder `None`, wenn keine Anwendung mit dieser ID existiert.
    /// Im Fehlerfall wird ein `DomainError` zurückgegeben.
    async fn get_by_id(&self, id: &NovaId) -> DomainResult<Option<Application>>;

    /// Ruft eine Liste aller im System bekannten Anwendungen ab.
    ///
    /// # Rückgabe
    /// Ein `DomainResult` das bei Erfolg einen Vektor von `Application`-Entitäten enthält.
    /// Der Vektor kann leer sein, wenn keine Anwendungen vorhanden sind.
    /// Im Fehlerfall wird ein `DomainError` zurückgegeben.
    async fn get_all(&self) -> DomainResult<Vec<Application>>;
    
    /// Sucht und ruft Anwendungen ab, deren Name (oder ggf. Anzeigename)
    /// einem gegebenen Suchbegriff entspricht.
    ///
    /// Die genaue Suchlogik (z.B. exakte Übereinstimmung, Teilübereinstimmung, Groß-/Kleinschreibung)
    /// ist der Implementierung überlassen.
    ///
    /// # Parameter
    /// * `search_term`: Der Begriff, nach dem im Anwendungsnamen gesucht werden soll.
    ///
    /// # Rückgabe
    /// Ein `DomainResult` das bei Erfolg einen Vektor von passenden `Application`-Entitäten enthält.
    /// Im Fehlerfall wird ein `DomainError` zurückgegeben.
    async fn find_by_name(&self, search_term: &str) -> DomainResult<Vec<Application>>;

    /// Fügt eine neue Anwendung zum Repository hinzu.
    ///
    /// # Parameter
    /// * `application`: Eine Referenz auf die hinzuzufügende `Application`-Entität.
    ///                  Es wird erwartet, dass die `id` der Anwendung bereits gesetzt ist.
    ///
    /// # Rückgabe
    /// Ein `DomainResult<()>` das bei Erfolg `Ok(())` zurückgibt.
    /// Im Fehlerfall wird ein `DomainError` zurückgegeben (z.B. wenn eine Anwendung
    /// mit derselben ID bereits existiert oder ein Speicherfehler auftritt).
    async fn add(&self, application: &Application) -> DomainResult<()>;

    /// Aktualisiert eine bereits im Repository vorhandene Anwendung.
    ///
    /// Die zu aktualisierende Anwendung wird typischerweise anhand ihrer `id` identifiziert.
    ///
    /// # Parameter
    /// * `application`: Eine Referenz auf die `Application`-Entität mit den aktualisierten Daten.
    ///
    /// # Rückgabe
    /// Ein `DomainResult<()>` das bei Erfolg `Ok(())` zurückgibt.
    /// Im Fehlerfall wird ein `DomainError` zurückgegeben (z.B. wenn die zu aktualisierende
    /// Anwendung nicht gefunden wird).
    async fn update(&self, application: &Application) -> DomainResult<()>;

    /// Entfernt eine Anwendung anhand ihrer eindeutigen ID aus dem Repository.
    ///
    /// # Parameter
    /// * `id`: Die [`NovaId`] der zu entfernenden Anwendung.
    ///
    /// # Rückgabe
    /// Ein `DomainResult<()>` das bei Erfolg `Ok(())` zurückgibt.
    /// Im Fehlerfall wird ein `DomainError` zurückgegeben (z.B. wenn keine Anwendung
    /// mit dieser ID zum Entfernen gefunden wird).
    async fn remove(&self, id: &NovaId) -> DomainResult<()>;
}
