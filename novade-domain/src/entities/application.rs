//! # Application Entität (`entities::application`)
//!
//! Definiert die Kernentität [`Application`] zur Repräsentation einer Anwendung
//! im NovaDE-System sowie den zugehörigen Typ [`ApplicationType`].
//!
//! Eine `Application` kann eine Desktop-Anwendung, ein Kommandozeilen-Tool,
//! ein Hintergrunddienst oder eine Web-Anwendung sein. Die Struktur hält
//! Metadaten wie Name, Pfad zur ausführbaren Datei, Icon, Kategorien und Version.

use novade_core::types::{NovaId, Version};
use serde::{Deserialize, Serialize};

/// Repräsentiert den Typ oder die Kategorie einer Anwendung.
///
/// Dies hilft dem System zu verstehen, wie eine Anwendung behandelt oder dargestellt werden soll.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ApplicationType {
    /// Eine grafische Anwendung, typischerweise mit einer Desktop-Datei assoziiert.
    Desktop,
    /// Eine reine Kommandozeilenanwendung.
    Cli,
    /// Eine Web-Anwendung, die als eigenständige Entität im System repräsentiert wird.
    WebService,
    /// Ein Dienst, der im Hintergrund läuft und keine direkte Benutzeroberfläche hat.
    BackgroundService,
    /// Für andere, nicht spezifisch aufgeführte Anwendungstypen.
    /// Das Feld enthält eine genauere Beschreibung des Typs.
    Other(String),
}

/// Repräsentiert eine Anwendung, die im NovaDE-System bekannt ist und verwaltet werden kann.
///
/// Enthält alle notwendigen Informationen, um eine Anwendung zu identifizieren, darzustellen
/// und potenziell zu starten.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Application {
    /// Ein eindeutiger Identifikator für die Anwendung, generiert als [`NovaId`].
    pub id: NovaId,
    /// Der primäre, oft technische Name der Anwendung (z.B. "firefox", "org.gnome.TextEditor").
    pub name: String,
    /// Ein optionaler, benutzerfreundlicherer Anzeigename für die UI (z.B. "Firefox Web Browser").
    /// Wenn nicht gesetzt, kann `name` verwendet werden.
    pub display_name: Option<String>,
    /// Der vollständige Pfad zur ausführbaren Datei der Anwendung oder der auszuführende Befehl.
    pub executable_path: String,
    /// Optionale Liste von Standardargumenten, die beim Start der Anwendung übergeben werden sollen.
    pub arguments: Option<Vec<String>>,
    /// Optionales Arbeitsverzeichnis, in dem die Anwendung gestartet werden soll.
    pub working_directory: Option<String>,
    /// Name des Icons für die Anwendung, typischerweise gemäß der Freedesktop Icon Theme Specification
    /// (z.B. "firefox", "system-search"). Das System ist verantwortlich, das passende Icon-Theme zu finden.
    pub icon_name: Option<String>,
    /// Der Typ der Anwendung, definiert durch [`ApplicationType`].
    pub app_type: ApplicationType,
    /// Optionale Liste von Kategorien, denen die Anwendung zugeordnet ist (z.B. "Network", "Office", "Utility").
    /// Orientiert sich oft an den Kategorien der Freedesktop .desktop-Spezifikation.
    pub categories: Option<Vec<String>>,
    /// Optionale Liste von Schlüsselwörtern, die für die Suche nach der Anwendung verwendet werden können.
    pub keywords: Option<Vec<String>>,
    /// Eine kurze, optionale Beschreibung der Funktionalität der Anwendung.
    pub description: Option<String>,
    /// Die Version der Anwendung, falls bekannt, repräsentiert durch [`novade_core::types::Version`].
    pub version: Option<Version>,
}

impl Application {
    /// Erstellt eine neue `Application` vom Typ [`ApplicationType::Desktop`].
    ///
    /// Dies ist ein Hilfskonstruktor für einen häufigen Anwendungsfall.
    /// Die ID wird automatisch generiert. Viele Felder bleiben `None` und können später gesetzt werden.
    ///
    /// # Parameter
    /// * `name`: Der technische Name der Anwendung.
    /// * `executable_path`: Der Pfad zur ausführbaren Datei.
    /// * `icon_name`: Optional der Name des Icons.
    ///
    /// # Beispiele
    /// ```
    /// use novade_domain::entities::Application;
    ///
    /// let my_app = Application::new_desktop(
    ///     "my-editor".to_string(),
    ///     "/usr/bin/my-editor".to_string(),
    ///     Some("accessories-text-editor".to_string())
    /// );
    /// assert_eq!(my_app.name, "my-editor");
    /// assert_eq!(my_app.app_type, novade_domain::entities::ApplicationType::Desktop);
    /// ```
    pub fn new_desktop(
        name: String,
        executable_path: String,
        icon_name: Option<String>,
    ) -> Self {
        Self {
            id: NovaId::new(),
            name,
            display_name: None,
            executable_path,
            arguments: None,
            working_directory: None,
            icon_name,
            app_type: ApplicationType::Desktop,
            categories: None,
            keywords: None,
            description: None,
            version: None,
        }
    }

    // Weitere spezifische Konstruktoren oder Builder-Methoden könnten hier folgen,
    // z.B. `Application::new_cli(...)` oder ein `ApplicationBuilder`.
}
