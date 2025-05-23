//! # Grundlegende Datentypen in `novade-core`
//!
//! Dieses Modul definiert eine Reihe von grundlegenden Datentypen, die schichtübergreifend
//! im NovaDE-System verwendet werden. Dazu gehören Identifikatoren, Versionierung,
//! Zeitstempel und Ressourcenbezeichner.
//!
//! ## Wichtige Typen:
//!
//! - [`NovaId`]: Ein eindeutiger Identifikator (UUID v4) für Entitäten.
//! - [`Version`]: Repräsentiert eine semantische Version (Major, Minor, Patch).
//! - [`Timestamp`]: Ein Zeitstempel im UTC-Format.
//! - [`ResourceIdentifier`]: Ein Enum zur eindeutigen Identifizierung verschiedener
//!   Arten von Ressourcen (Dateien, Dienste, Komponenten etc.).
//!
//! Diese Typen sind oft mit `serde` für Serialisierung/Deserialisierung und `FromStr`
//! für das Parsen aus Strings ausgestattet.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::fmt;
use std::path::PathBuf;
use std::str::FromStr;
use uuid::Uuid;
use crate::error::CoreError; // CoreResult entfernt

/// Ein eindeutiger Identifikator für Entitäten im NovaDE-System.
///
/// Basiert intern auf einem UUID v4, um globale Eindeutigkeit zu gewährleisten.
/// `NovaId` implementiert `Default`, `Display`, `FromStr` sowie `Serialize` und `Deserialize`
/// für eine einfache Handhabung.
///
/// # Beispiele
/// ```
/// use novade_core::types::NovaId;
/// use std::str::FromStr;
///
/// // Eine neue, zufällige ID erstellen
/// let id1 = NovaId::new();
/// println!("Neue ID: {}", id1);
///
/// // Eine ID aus einem String parsen
/// let id_str = "f47ac10b-58cc-4372-a567-0e02b2c3d479";
/// let id2 = NovaId::from_str(id_str).unwrap();
/// assert_eq!(id1.to_string(), id_str); // Dieser Vergleich wird fehlschlagen, da id1 zufällig ist
/// assert_eq!(id2.to_string(), id_str);
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct NovaId(Uuid);

impl NovaId {
    /// Erstellt eine neue, zufällige `NovaId` (UUID v4).
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }

    /// Erstellt eine `NovaId` aus einem bestehenden `Uuid`.
    ///
    /// # Parameter
    /// * `uuid`: Das `Uuid`, das für diese `NovaId` verwendet werden soll.
    pub fn from_uuid(uuid: Uuid) -> Self {
        Self(uuid)
    }

    /// Gibt eine Referenz auf das zugrundeliegende `Uuid` zurück.
    pub fn as_uuid(&self) -> &Uuid {
        &self.0
    }
}

impl Default for NovaId {
    /// Erstellt eine neue, zufällige `NovaId` als Standardwert.
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for NovaId {
    /// Formatiert die `NovaId` als String (die Standard-UUID-Darstellung).
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromStr for NovaId {
    type Err = CoreError;
    /// Versucht, eine `NovaId` aus einem String zu parsen.
    ///
    /// Der String muss eine gültige UUID-Repräsentation sein.
    ///
    /// # Fehler
    /// Gibt `CoreError::DeserializationError` zurück, wenn der String keine gültige UUID ist.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Uuid::from_str(s)
            .map(NovaId)
            .map_err(|e| CoreError::DeserializationError {
                format: "NovaId".to_string(),
                message: format!("Ungültige UUID-Zeichenkette '{}': {}", s, e),
            })
    }
}

/// Repräsentiert eine semantische Version (Major, Minor, Patch).
///
/// Implementiert `PartialOrd` und `Ord` für Versionsvergleiche sowie `Display`, `FromStr`,
/// `Serialize` und `Deserialize`.
///
/// # Beispiele
/// ```
/// use novade_core::types::Version;
/// use std::str::FromStr;
///
/// let v1 = Version::new(1, 2, 3);
/// let v2 = Version::from_str("1.2.4").unwrap();
/// assert!(v2 > v1);
/// assert_eq!(v1.to_string(), "1.2.3");
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, PartialOrd, Ord)]
pub struct Version {
    /// Die Major-Komponente der Version. Inkompatible API-Änderungen.
    pub major: u16,
    /// Die Minor-Komponente der Version. Rückwärtskompatible neue Funktionalität.
    pub minor: u16,
    /// Die Patch-Komponente der Version. Rückwärtskompatible Fehlerbehebungen.
    pub patch: u16,
}

impl Version {
    /// Erstellt eine neue `Version`.
    pub fn new(major: u16, minor: u16, patch: u16) -> Self {
        Self { major, minor, patch }
    }
}

impl fmt::Display for Version {
    /// Formatiert die `Version` als String im Format "major.minor.patch".
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}.{}.{}", self.major, self.minor, self.patch)
    }
}

impl FromStr for Version {
    type Err = CoreError;
    /// Versucht, eine `Version` aus einem String im Format "major.minor.patch" zu parsen.
    ///
    /// # Fehler
    /// Gibt `CoreError::DeserializationError` zurück, wenn das Format ungültig ist
    /// oder die Komponenten nicht als Zahlen geparst werden können.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split('.').collect();
        if parts.len() != 3 {
            return Err(CoreError::DeserializationError{
                format: "Version".to_string(),
                message: format!("Ungültiges Versionsformat: '{}'. Erwartet 'major.minor.patch'.", s)
            });
        }
        let major = parts[0].parse::<u16>().map_err(|_| CoreError::DeserializationError{
            format: "Version".to_string(),
            message: format!("Ungültige Major-Version: '{}'", parts[0])
        })?;
        let minor = parts[1].parse::<u16>().map_err(|_| CoreError::DeserializationError{
            format: "Version".to_string(),
            message: format!("Ungültige Minor-Version: '{}'", parts[1])
        })?;
        let patch = parts[2].parse::<u16>().map_err(|_| CoreError::DeserializationError{
            format: "Version".to_string(),
            message: format!("Ungültige Patch-Version: '{}'", parts[2])
        })?;
        Ok(Version::new(major, minor, patch))
    }
}

/// Ein Zeitstempel im UTC-Format, basierend auf `chrono::DateTime<Utc>`.
///
/// Implementiert `Default` (setzt auf `Utc::now()`), `Display` (RFC3339-Format), `FromStr`,
/// `Serialize` und `Deserialize`.
///
/// # Beispiele
/// ```
/// use novade_core::types::Timestamp;
/// use std::str::FromStr;
///
/// let now = Timestamp::now();
/// println!("Aktueller Zeitstempel: {}", now);
///
/// let rfc_str = "2023-10-26T07:30:00Z";
/// let ts_from_str = Timestamp::from_str(rfc_str).unwrap();
/// assert_eq!(ts_from_str.to_string(), rfc_str);
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, PartialOrd, Ord)]
pub struct Timestamp(DateTime<Utc>);

impl Timestamp {
    /// Erstellt einen neuen `Timestamp` mit der aktuellen UTC-Zeit.
    pub fn now() -> Self {
        Self(Utc::now())
    }

    /// Erstellt einen `Timestamp` aus einem gegebenen `chrono::DateTime<Utc>`.
    pub fn from_datetime(dt: DateTime<Utc>) -> Self {
        Self(dt)
    }

    /// Gibt eine Referenz auf das zugrundeliegende `chrono::DateTime<Utc>` zurück.
    pub fn as_datetime(&self) -> &DateTime<Utc> {
        &self.0
    }
}

impl Default for Timestamp {
    /// Erstellt einen neuen `Timestamp` mit der aktuellen UTC-Zeit als Standardwert.
    fn default() -> Self {
        Self::now()
    }
}

impl fmt::Display for Timestamp {
    /// Formatiert den `Timestamp` als String im RFC3339-Format.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0.to_rfc3339())
    }
}

impl FromStr for Timestamp {
    type Err = CoreError;
    /// Versucht, einen `Timestamp` aus einem String im RFC3339-Format zu parsen.
    ///
    /// # Fehler
    /// Gibt `CoreError::DeserializationError` zurück, wenn der String kein gültiges
    /// RFC3339-Datum darstellt.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        DateTime::parse_from_rfc3339(s)
            .map(|dt| Timestamp(dt.with_timezone(&Utc)))
            .map_err(|e| CoreError::DeserializationError {
                format: "Timestamp".to_string(),
                message: format!("Ungültiges RFC3339 Timestamp-Format '{}': {}", s, e),
            })
    }
}


/// Identifiziert eine Ressource innerhalb des NovaDE-Systems.
///
/// Dieses Enum dient dazu, verschiedene Arten von Ressourcen (wie Dateien, Dienste,
/// interne Komponenten oder URLs) eindeutig zu bezeichnen und zu referenzieren.
/// Es implementiert `Display` für eine menschenlesbare Darstellung.
///
/// # Varianten
/// - `File(PathBuf)`: Eine Datei im Dateisystem.
/// - `Directory(PathBuf)`: Ein Verzeichnis im Dateisystem.
/// - `Service(String)`: Ein Systemdienst, z.B. ein D-Bus-Servicename.
/// - `Component(String)`: Ein internes Softwaremodul oder eine Komponente von NovaDE.
/// - `Url(String)`: Eine URL, z.B. für Web-Ressourcen.
/// - `Other { r#type: String, identifier: String }`: Für andere, nicht spezifisch
///   aufgeführte Ressourcentypen. `r#type` beschreibt die Art (z.B. "ipc_channel")
///   und `identifier` den spezifischen Bezeichner.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ResourceIdentifier {
    /// Eine Datei im Dateisystem.
    File(PathBuf),
    /// Ein Verzeichnis im Dateisystem.
    Directory(PathBuf),
    /// Ein Systemdienst (z.B. D-Bus Service Name).
    Service(String),
    /// Eine interne Softwarekomponente oder ein Modul.
    Component(String),
    /// Eine Uniform Resource Locator (URL).
    Url(String),
    /// Ein anderer, nicht spezifisch typisierter Ressourcenbezeichner.
    Other {
        /// Die Art der Ressource (z.B. "ipc_channel", "hardware_device").
        r#type: String,
        /// Der eindeutige Bezeichner für diese Art von Ressource.
        identifier: String,
    },
}

impl fmt::Display for ResourceIdentifier {
    /// Formatiert den `ResourceIdentifier` als String.
    ///
    /// # Beispiele
    /// ```
    /// use novade_core::types::ResourceIdentifier;
    /// use std::path::PathBuf;
    ///
    /// assert_eq!(ResourceIdentifier::File(PathBuf::from("/tmp/file.txt")).to_string(), "file:///tmp/file.txt");
    /// assert_eq!(ResourceIdentifier::Service("org.novade.ExampleService".to_string()).to_string(), "service:org.novade.ExampleService");
    /// ```
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ResourceIdentifier::File(path) => write!(f, "file://{}", path.display()),
            ResourceIdentifier::Directory(path) => write!(f, "dir://{}", path.display()),
            ResourceIdentifier::Service(name) => write!(f, "service:{}", name),
            ResourceIdentifier::Component(name) => write!(f, "component:{}", name),
            ResourceIdentifier::Url(url) => write!(f, "url:{}", url),
            ResourceIdentifier::Other { r#type, identifier } => write!(f, "{}:{}", r#type, identifier),
        }
    }
}
