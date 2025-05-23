//! # User Preference Entitäten (`entities::user_preference`)
//!
//! Definiert Entitäten zur Darstellung von Benutzereinstellungen im NovaDE-System,
//! insbesondere [`UserPreferenceSetting`] und den dazugehörigen Wert-Typ [`PreferenceValue`].
//!
//! Diese Strukturen ermöglichen es, verschiedene Arten von Einstellungen flexibel zu speichern
//! und zu verwalten, inklusive Metadaten wie Anzeigename, Beschreibung und ob ein Neustart
//! für die Aktivierung der Einstellung erforderlich ist.

// use novade_core::types::NovaId; // Import von NovaId für zukünftige Benutzerbindung - aktuell nicht verwendet
use serde::{Deserialize, Serialize};

/// Repräsentiert den tatsächlichen Wert einer Benutzereinstellung.
///
/// Dieses Enum ermöglicht es, verschiedene Datentypen für Einstellungen zu speichern.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum PreferenceValue {
    /// Ein Textwert.
    String(String),
    /// Ein ganzzahliger Wert.
    Integer(i64),
    /// Ein Fließkommawert.
    Float(f64),
    /// Ein boolescher Wert (wahr/falsch).
    Boolean(bool),
    /// Ein Farbwert, typischerweise als RGBA-String (z.B. "#RRGGBBAA" oder "rgba(r,g,b,a)").
    ColorRgba(String),
    /// Eine Liste von Textwerten.
    StringList(Vec<String>),
    // Zukünftige Erweiterungen könnten spezifischere Typen wie Keybinding, FontSetting etc. umfassen.
    // Enum(String, Vec<String>), // z.B. Enum("OptionA", vec!["OptionA", "OptionB"])
}

/// Repräsentiert eine einzelne, konfigurierbare Benutzereinstellung im System.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UserPreferenceSetting {
    /// Ein eindeutiger, maschinenlesbarer Schlüssel für die Einstellung.
    ///
    /// Konvention: `bereich.unterbereich.einstellung` (z.B. "theme.dark_mode_enabled", "keyboard.layout").
    pub key: String,
    /// Der aktuelle Wert der Einstellung, gespeichert als [`PreferenceValue`].
    pub value: PreferenceValue,
    /// Ein benutzerfreundlicher Name für die Einstellung, der in UIs angezeigt werden kann.
    pub display_name: String,
    /// Eine optionale, ausführlichere Beschreibung der Funktion dieser Einstellung.
    pub description: Option<String>,
    /// Gibt an, ob eine Änderung dieser Einstellung einen Neustart der Anwendung
    /// oder des gesamten Systems erfordert, um wirksam zu werden.
    pub requires_restart: bool,
    /// Eine optionale Gruppierungskategorie für die Einstellung,
    /// nützlich zur Organisation in Einstellungsdialogen (z.B. "Erscheinungsbild", "System", "Fensterverhalten").
    pub group: Option<String>,
    // Zukünftig könnte hier eine `user_id: Option<NovaId>` stehen, um Einstellungen
    // benutzerspezifisch zu machen oder systemweite Standardwerte zu kennzeichnen.
    // Für den Moment wird angenommen, dass Einstellungen global oder durch den Kontext
    // des Repositories benutzergebunden sind.
}

impl UserPreferenceSetting {
    /// Erstellt eine neue boolesche Benutzereinstellung.
    ///
    /// # Parameter
    /// * `key`: Der eindeutige Schlüssel für die Einstellung.
    /// * `display_name`: Der in der UI anzuzeigende Name.
    /// * `default_value`: Der initiale boolesche Wert der Einstellung.
    ///
    /// # Beispiele
    /// ```
    /// use novade_domain::entities::UserPreferenceSetting;
    ///
    /// let dark_mode_setting = UserPreferenceSetting::new_boolean(
    ///     "theme.dark_mode",
    ///     "Dunkler Modus",
    ///     false
    /// );
    /// assert_eq!(dark_mode_setting.key, "theme.dark_mode");
    /// match dark_mode_setting.value {
    ///     novade_domain::entities::PreferenceValue::Boolean(val) => assert!(!val),
    ///     _ => panic!("Falscher Werttyp"),
    /// }
    /// ```
    pub fn new_boolean(key: &str, display_name: &str, default_value: bool) -> Self {
        Self {
            key: key.to_string(),
            value: PreferenceValue::Boolean(default_value),
            display_name: display_name.to_string(),
            description: None,
            requires_restart: false,
            group: None,
        }
    }

    /// Erstellt eine neue String-Benutzereinstellung.
    pub fn new_string(key: &str, display_name: &str, default_value: String) -> Self {
        Self {
            key: key.to_string(),
            value: PreferenceValue::String(default_value),
            display_name: display_name.to_string(),
            description: None,
            requires_restart: false,
            group: None,
        }
    }
    // Weitere Konstruktoren für andere Typen (Integer, Float, etc.) können bei Bedarf hinzugefügt werden.
}
