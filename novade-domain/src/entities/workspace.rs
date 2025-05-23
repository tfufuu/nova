//! # Workspace Entität (`entities::workspace`)
//!
//! Definiert die Kernentität [`Workspace`] zur Repräsentation eines Arbeitsbereichs
//! oder virtuellen Desktops innerhalb von NovaDE.
//!
//! Ein `Workspace` ist eine logische Gruppierung von Anwendungsfenstern und deren
//! Anordnung, typischerweise assoziiert mit einem oder mehreren Bildschirmen.
//! Er ermöglicht es Benutzern, ihre Arbeitsumgebung für verschiedene Aufgaben
//! oder Kontexte zu organisieren.

use novade_core::types::NovaId;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Repräsentiert einen Arbeitsbereich (Workspace) in NovaDE.
///
/// Ein Workspace kann als ein virtueller Desktop betrachtet werden, der eine bestimmte
/// Menge von Fenstern, deren Layout und zugehörige Einstellungen enthält.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Workspace {
    /// Ein eindeutiger Identifikator für den Workspace, generiert als [`NovaId`].
    pub id: NovaId,
    /// Ein benutzerdefinierter Name für den Workspace (z.B. "Arbeit", "Freizeit", "Entwicklung").
    pub name: String,
    /// Eine Beschreibung oder Konfiguration des Layouts für diesen Workspace.
    ///
    /// Dies könnte ein JSON-String sein, der Fensterpositionen und -größen beschreibt,
    /// oder ein Verweis auf ein vordefiniertes Layout-Template (z.B. "Kacheln", "Überlappend").
    /// Die genaue Interpretation liegt bei der UI- oder Systemschicht.
    pub layout_configuration: String,
    /// Optionale ID des primären Outputs (Bildschirms), mit dem dieser Workspace
    /// hauptsächlich verbunden ist. Nützlich in Multi-Monitor-Setups.
    pub primary_output_id: Option<String>, // Z.B. Name oder ID des Monitors
    /// Zusätzliche Metadaten oder benutzerspezifische Einstellungen für den Workspace.
    ///
    /// Kann verwendet werden, um beliebige Schlüssel-Wert-Paare zu speichern,
    /// wie z.B. Hintergrundbild, spezifische Panel-Einstellungen etc.
    pub metadata: HashMap<String, String>,
}

impl Workspace {
    /// Erstellt einen neuen `Workspace` mit einem gegebenen Namen und optionaler ID des primären Outputs.
    ///
    /// Die ID des Workspaces wird automatisch generiert. Die `layout_configuration` wird
    /// standardmäßig auf "default" gesetzt.
    ///
    /// # Parameter
    /// * `name`: Der Name für den neuen Workspace.
    /// * `primary_output_id`: Optionale ID des Bildschirms, dem dieser Workspace zugeordnet werden soll.
    ///
    /// # Beispiele
    /// ```
    /// use novade_domain::entities::Workspace;
    ///
    /// let ws1 = Workspace::new("Coding Space".to_string(), Some("HDMI-1".to_string()));
    /// assert_eq!(ws1.name, "Coding Space");
    /// assert_eq!(ws1.primary_output_id.as_deref(), Some("HDMI-1"));
    /// assert_eq!(ws1.layout_configuration, "default");
    ///
    /// let ws2 = Workspace::new("General".to_string(), None);
    /// assert_eq!(ws2.name, "General");
    /// assert!(ws2.primary_output_id.is_none());
    /// ```
    pub fn new(name: String, primary_output_id: Option<String>) -> Self {
        Self {
            id: NovaId::new(),
            name,
            layout_configuration: "default".to_string(), // Ein einfacher Standardwert
            primary_output_id,
            metadata: HashMap::new(),
        }
    }
}
