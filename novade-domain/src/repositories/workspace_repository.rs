//! # Workspace Repository Trait (`repositories::workspace_repository`)
//!
//! Definiert das Trait [`WorkspaceRepository`], das als Abstraktion für den
//! Datenzugriff auf [`Workspace`](crate::entities::Workspace) Entitäten dient.
//!
//! Diese Schnittstelle muss von einer konkreten Implementierung in der Systemschicht
//! (`novade-system`) erfüllt werden, um die Persistenz von Workspace-Daten
//! zu gewährleisten (z.B. Speichern in einer Konfigurationsdatei oder Datenbank).

use crate::entities::workspace::Workspace;
use crate::DomainResult; // Stellt sicher, dass Fehler als DomainError zurückgegeben werden
use async_trait::async_trait;
use novade_core::types::NovaId;

/// Ein Trait, das Operationen zum Speichern, Abrufen und Verwalten von
/// [`Workspace`](crate::entities::Workspace)-Entitäten abstrahiert.
///
/// Implementierungen dieses Traits sind für die Persistenzlogik von Workspaces zuständig.
/// Das Trait ist `async_trait`, um asynchrone Operationen zu unterstützen.
#[cfg_attr(test, mockall::automock)] // Hinzugefügt für Mocking in Tests
/// `Send + Sync` Bounds sind für die thread-sichere Nutzung erforderlich.
#[async_trait]
pub trait WorkspaceRepository: Send + Sync {
    /// Ruft einen spezifischen Workspace anhand seiner eindeutigen ID ab.
    ///
    /// # Parameter
    /// * `id`: Die [`NovaId`] des gesuchten Workspaces.
    ///
    /// # Rückgabe
    /// Ein `DomainResult`, das bei Erfolg `Some(Workspace)` enthält, wenn der Workspace
    /// gefunden wurde, oder `None`, andernfalls ein `DomainError`.
    async fn get_by_id(&self, id: &NovaId) -> DomainResult<Option<Workspace>>;

    /// Ruft einen spezifischen Workspace anhand seines Namens ab.
    ///
    /// Da Workspace-Namen potenziell eindeutig sein sollten (innerhalb eines Benutzerkontexts),
    /// ermöglicht diese Methode das Abrufen über den Namen.
    ///
    /// # Parameter
    /// * `name`: Der Name des gesuchten Workspaces.
    ///
    /// # Rückgabe
    /// Ein `DomainResult`, das bei Erfolg `Some(Workspace)` enthält, wenn ein Workspace
    /// mit diesem Namen gefunden wurde, oder `None`, andernfalls ein `DomainError`.
    async fn get_by_name(&self, name: &str) -> DomainResult<Option<Workspace>>;
    
    /// Ruft eine Liste aller im System bekannten Workspaces ab.
    ///
    /// # Rückgabe
    /// Ein `DomainResult`, das bei Erfolg einen Vektor von `Workspace`-Entitäten enthält.
    /// Der Vektor kann leer sein. Im Fehlerfall wird ein `DomainError` zurückgegeben.
    async fn get_all(&self) -> DomainResult<Vec<Workspace>>;

    /// Fügt einen neuen Workspace zum Repository hinzu.
    ///
    /// # Parameter
    /// * `workspace`: Eine Referenz auf den hinzuzufügenden `Workspace`.
    ///                Es wird erwartet, dass die `id` des Workspaces bereits gesetzt ist.
    ///
    /// # Rückgabe
    /// Ein `DomainResult<()>` das bei Erfolg `Ok(())` zurückgibt.
    /// Im Fehlerfall wird ein `DomainError` zurückgegeben (z.B. wenn ein Workspace
    /// mit derselben ID oder demselben Namen bereits existiert).
    async fn add(&self, workspace: &Workspace) -> DomainResult<()>;

    /// Aktualisiert einen bereits im Repository vorhandenen Workspace.
    ///
    /// Der zu aktualisierende Workspace wird typischerweise anhand seiner `id` identifiziert.
    ///
    /// # Parameter
    /// * `workspace`: Eine Referenz auf den `Workspace` mit den aktualisierten Daten.
    ///
    /// # Rückgabe
    /// Ein `DomainResult<()>` das bei Erfolg `Ok(())` zurückgibt.
    /// Im Fehlerfall wird ein `DomainError` zurückgegeben (z.B. wenn der Workspace
    /// nicht gefunden wird).
    async fn update(&self, workspace: &Workspace) -> DomainResult<()>;

    /// Entfernt einen Workspace anhand seiner eindeutigen ID aus dem Repository.
    ///
    /// # Parameter
    /// * `id`: Die [`NovaId`] des zu entfernenden Workspaces.
    ///
    /// # Rückgabe
    /// Ein `DomainResult<()>` das bei Erfolg `Ok(())` zurückgibt.
    /// Im Fehlerfall wird ein `DomainError` zurückgegeben (z.B. wenn kein Workspace
    /// mit dieser ID gefunden wird).
    async fn remove(&self, id: &NovaId) -> DomainResult<()>;
}
