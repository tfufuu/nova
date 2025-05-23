**0. Metadaten**
    *   **Dokument-ID:** `NOVA_DE_INTERFACES_001`
    *   **Bezieht sich auf:** `NOVA_DE_GLOBAL_ARCH_001`
    *   **Version:** 1.0.0
    *   **Status:** In Entwicklung
    *   **Erstellt am:** 2024-07-17
    *   **Letzte Änderung:** 2024-07-17
    *   **Verantwortlich:** NovaGem KI Architekt

**1. Zweck und Geltungsbereich**
    1.1. **Zweck:** Dieses Dokument definiert die absoluten Kommunikationsverträge (Schnittstellen) zwischen den Hauptschichten des Nova Desktop Environment (NovaDE). Diese Verträge werden in Form von vollständigen Rust-Trait-Definitionen für direkte Funktionsaufrufe und Event-Enum-Definitionen für asynchrone Nachrichtenübermittlung über `tokio::sync::broadcast` Kanäle spezifiziert.
    1.2. **Geltungsbereich:** Umfasst die Schnittstellen zwischen den in `00_GLOBAL_ARCHITEKTUR_DEFINITION.md` definierten Schichten: NovaHAL (Schicht 1), NovaCore (Schicht 2), NovaKit (Schicht 3) und NovaShell (Schicht 4). Kernel-Interaktionen (Schicht 0) erfolgen über existierende Rust-Crates (`libc`, `udev-rs`, etc.) und werden hier nicht neu definiert, sondern als gegeben betrachtet.
    1.3. **Relevanz für KI:** Diese Spezifikationen sind entscheidend für die korrekte Modul- und Schichtenintegration. Die KI nutzt dies, um `use`-Statements zu generieren, Trait-Implementierungen sicherzustellen und die Event-Handler korrekt zu verdrahten.

**2. Allgemeine Konventionen für Schnittstellen**
    2.1. **Rust Traits:**
        *   Alle Methoden sind `async fn` wenn sie potenziell blockierende I/O-Operationen ausführen oder auf langlaufende Prozesse warten.
        *   Parameter und Rückgabetypen verwenden exakte Typen aus `GLOBAL_TYPES_DEFINITIONS.md` oder Standard-Rust-Typen.
        *   Fehler werden explizit mit `Result<T, E>` zurückgegeben, wobei `E` ein schicht- oder modulspezifischer Fehlertyp ist (definiert in den jeweiligen Modulplänen, oft basierend auf `thiserror`).
        *   Lifetimes werden explizit angegeben, wo notwendig.
        *   Traits sind `Send + Sync`, wo immer möglich, um Thread-übergreifende Nutzung zu erlauben.
    2.2. **Event-Bus (Broadcast Kanäle):**
        *   Events werden als Enums definiert, wobei jede Variante die spezifischen Event-Daten enthält.
        *   Event-Daten-Typen sind `Clone + Send + Sync + Debug`.
        *   Jeder Broadcast-Kanal hat einen klar definierten Zweck und eine begrenzte Anzahl von Event-Typen.
        *   Erzeuger (Producer) und Konsumenten (Consumer) jedes Events werden dokumentiert.
    2.3. **Namensgebung:**
        *   Traits: `[SchichtName]Service` oder `[Funktionalität]Provider`.
        *   Event-Enums: `[SchichtName]Event` oder `[Funktionalität]Notification`.
    2.4. **Fehler-Mapping:** Wo Fehler über Schichtgrenzen hinweg propagiert werden, wird das Mapping oder die Umwandlung der Fehlertypen dokumentiert oder es wird ein gemeinsamer Fehlertyp (`core::error::GlobalError`) verwendet.

**3. Schnittstellen NovaHAL (Schicht 1) <-> NovaCore (Schicht 2)**

    3.1. **Traits (NovaHAL stellt bereit, NovaCore konsumiert):**

        3.1.1. **`trait DisplayManager: Send + Sync`** (Implementiert von `nova-displayd` in NovaHAL)
            ```rust
            use crate::core::types::geometry::{Resolution, MonitorId, Position}; // Annahme: Diese Typen sind in GLOBAL_TYPES_DEFINITIONS.md definiert
            use crate::core::error::DisplayError; // Beispielhafter Fehlertyp

            #[async_trait::async_trait]
            pub trait DisplayManager: Send + Sync {
                /// Listet alle verbundenen Monitore auf.
                async fn list_monitors(&self) -> Result<Vec<MonitorInfo>, DisplayError>;
                /// Setzt die Auflösung für einen bestimmten Monitor.
                async fn set_resolution(&self, monitor_id: MonitorId, resolution: Resolution) -> Result<(), DisplayError>;
                /// Setzt die Position eines Monitors (für Multi-Monitor-Setup).
                async fn set_monitor_position(&self, monitor_id: MonitorId, position: Position) -> Result<(), DisplayError>;
                /// Aktiviert/Deaktiviert einen Monitor.
                async fn set_monitor_enabled(&self, monitor_id: MonitorId, enabled: bool) -> Result<(), DisplayError>;
                // Weitere displaybezogene Methoden (Helligkeit, etc.)
            }

            #[derive(Clone, Debug)] // Typen müssen in GLOBAL_TYPES_DEFINITIONS.md definiert sein
            pub struct MonitorInfo {
                pub id: MonitorId,
                pub name: String,
                pub current_resolution: Resolution,
                pub available_resolutions: Vec<Resolution>,
                pub position: Position,
                pub is_primary: bool,
                pub is_enabled: bool,
            }
            ```

        3.1.2. **`trait InputDeviceManager: Send + Sync`** (Implementiert von `nova-inputd` in NovaHAL)
            ```rust
            use crate::core::types::input::{InputDeviceId, InputDeviceType, KeyboardLayout}; // Annahme: Diese Typen sind in GLOBAL_TYPES_DEFINITIONS.md definiert
            use crate::core::error::InputError; // Beispielhafter Fehlertyp

            #[async_trait::async_trait]
            pub trait InputDeviceManager: Send + Sync {
                /// Listet alle verbundenen Eingabegeräte auf.
                async fn list_input_devices(&self) -> Result<Vec<InputDeviceInfo>, InputError>;
                /// Setzt das Tastaturlayout.
                async fn set_keyboard_layout(&self, layout: KeyboardLayout) -> Result<(), InputError>;
                // Weitere inputbezogene Methoden (Mausempfindlichkeit, Touchpad-Einstellungen)
            }

            #[derive(Clone, Debug)] // Typen müssen in GLOBAL_TYPES_DEFINITIONS.md definiert sein
            pub struct InputDeviceInfo {
                pub id: InputDeviceId,
                pub name: String,
                pub device_type: InputDeviceType,
            }
            ```
        3.1.3. **`trait PowerManager: Send + Sync`** (Implementiert von `nova-powerd` in NovaHAL)
             ```rust
            use crate::core::error::PowerError; // Beispielhafter Fehlertyp
            use crate::core::types::power::{PowerState, BatteryInfo}; // Annahme: Diese Typen sind in GLOBAL_TYPES_DEFINITIONS.md definiert

            #[async_trait::async_trait]
            pub trait PowerManager: Send + Sync {
                async fn get_battery_info(&self) -> Result<Vec<BatteryInfo>, PowerError>;
                async fn suspend(&self) -> Result<(), PowerError>;
                async fn hibernate(&self) -> Result<(), PowerError>;
                async fn reboot(&self) -> Result<(), PowerError>;
                async fn shutdown(&self) -> Result<(), PowerError>;
            }
            ```

    3.2. **Events (NovaHAL sendet, NovaCore empfängt via `tokio::sync::broadcast`):**

        3.2.1. **`NovaHalEvent`** (Gesendet von verschiedenen NovaHAL-Diensten)
            ```rust
            use crate::core::types::input::{InputDeviceId, RawInputEvent}; // InputDeviceInfo wurde oben definiert
            use crate::core::types::display::DisplayConfigurationChanged; // Annahme: Diese Typen sind in GLOBAL_TYPES_DEFINITIONS.md definiert
            use crate::core::types::power::PowerStatusChanged;
            // InputDeviceInfo wurde bereits unter InputDeviceManager definiert
            use super::InputDeviceInfo;


            #[derive(Clone, Debug)]
            pub enum NovaHalEvent {
                InputDeviceAdded(InputDeviceInfo),
                InputDeviceRemoved(InputDeviceId),
                RawInputReceived(RawInputEvent), // Für den Compositor (NovaCore)
                DisplayConfigurationChanged(DisplayConfigurationChanged),
                PowerStatusChanged(PowerStatusChanged),
                // Weitere Hardware-Events
            }
            // Erzeuger: nova-inputd, nova-displayd, nova-powerd
            // Konsumenten: nova-wm (in NovaCore), nova-session (in NovaCore), nova-settingsd (in NovaCore)
            ```

**4. Schnittstellen NovaCore (Schicht 2) <-> NovaKit (Schicht 3) / NovaShell (Schicht 4)**

    4.1. **Traits (NovaCore stellt bereit, NovaKit/NovaShell konsumiert):**

        4.1.1. **`trait WindowManagerCtl: Send + Sync`** (Implementiert von `nova-wm` in NovaCore)
            ```rust
            use crate::core::types::window::{WindowId, WindowRect, WindowProperties}; // Annahme: Diese Typen sind in GLOBAL_TYPES_DEFINITIONS.md definiert
            use crate::core::error::WindowManagerError; // Beispielhafter Fehlertyp

            #[async_trait::async_trait]
            pub trait WindowManagerCtl: Send + Sync {
                /// Listet alle verwalteten Fenster auf.
                async fn list_windows(&self) -> Result<Vec<WindowId>, WindowManagerError>;
                /// Holt Eigenschaften eines Fensters.
                async fn get_window_properties(&self, id: WindowId) -> Result<WindowProperties, WindowManagerError>;
                /// Bringt ein Fenster in den Vordergrund.
                async fn focus_window(&self, id: WindowId) -> Result<(), WindowManagerError>;
                /// Schließt ein Fenster.
                async fn close_window(&self, id: WindowId) -> Result<(), WindowManagerError>;
                // Weitere Fenstermanagement-Aktionen (Minimieren, Maximieren, Verschieben etc.)
            }
            ```

        4.1.2. **`trait SessionManagerCtl: Send + Sync`** (Implementiert von `nova-session` in NovaCore)
            ```rust
            use crate::core::types::session::{SessionInfo, UserInfo}; // Annahme: Diese Typen sind in GLOBAL_TYPES_DEFINITIONS.md definiert
            use crate::core::error::SessionError; // Beispielhafter Fehlertyp

            #[async_trait::async_trait]
            pub trait SessionManagerCtl: Send + Sync {
                async fn get_current_session_info(&self) -> Result<SessionInfo, SessionError>;
                async fn get_current_user_info(&self) -> Result<UserInfo, SessionError>;
                async fn lock_session(&self) -> Result<(), SessionError>;
                async fn logout(&self) -> Result<(), SessionError>;
                // ... weitere sessionbezogene Methoden
            }
            ```

        4.1.3. **`trait SettingsService: Send + Sync`** (Implementiert von `nova-settingsd` in NovaCore)
            ```rust
            use serde::{Serialize, de::DeserializeOwned};
            use crate::core::error::SettingsError; // Beispielhafter Fehlertyp
            use tokio::sync::broadcast; // Expliziter Import für Klarheit

            #[derive(Clone, Debug, Serialize)] // Event-Typ für Einstellungsänderungen
            pub struct SettingChangedEvent {
                pub schema: String,
                pub key: String,
                // pub new_value: serde_json::Value, // Das neue Wert könnte hier als JSON-Wert übertragen werden
                                                  // In der Aufgabe nicht explizit gefordert, daher auskommentiert, aber eine wichtige Überlegung.
            }

            #[async_trait::async_trait]
            pub trait SettingsService: Send + Sync {
                /// Holt einen Konfigurationswert.
                async fn get_setting<T: DeserializeOwned + Send>(&self, schema: &str, key: &str) -> Result<T, SettingsError>;
                /// Setzt einen Konfigurationswert.
                async fn set_setting<T: Serialize + Send + Sync>(&self, schema: &str, key: &str, value: T) -> Result<(), SettingsError>;
                /// Abonniert Änderungen an einem Konfigurationswert.
                /// Gibt einen Broadcast-Receiver zurück, der bei Änderungen benachrichtigt wird.
                async fn subscribe_to_setting_changes(&self, schema: &str, key: &str) -> Result<broadcast::Receiver<SettingChangedEvent>, SettingsError>;
            }
            ```

    4.2. **Events (NovaCore sendet, NovaKit/NovaShell empfängt):**

        4.2.1. **`NovaCoreEvent`** (Gesendet von verschiedenen NovaCore-Diensten)
            ```rust
            use crate::core::types::window::{WindowId, WindowLifeCycleEvent}; // Annahme: Diese Typen sind in GLOBAL_TYPES_DEFINITIONS.md definiert
            use crate::core::types::application::ApplicationLaunchedEvent;
            use crate::core::types::notification::NotificationReceivedEvent; // Für UI-Benachrichtigungsanzeige

            #[derive(Clone, Debug)]
            pub enum NovaCoreEvent {
                WindowLifeCycle(WindowLifeCycleEvent), // z.B. WindowCreated, WindowClosed, WindowFocused
                ApplicationLaunched(ApplicationLaunchedEvent),
                NotificationReceived(NotificationReceivedEvent), // Von nova-notificationd an die Shell
                SessionLocked,
                SessionUnlocked,
                // Weitere Core-Events
            }
            // Erzeuger: nova-wm, nova-session, nova-launcher, nova-notificationd
            // Konsumenten: NovaShell-Komponenten (Panel, TaskManager etc.), libnova-ui (für Theming-Änderungen etc.)
            ```

**5. Schnittstellen NovaKit (Schicht 3) <-> NovaShell (Schicht 4)**
    *   Diese Interaktion ist primär durch die direkte Nutzung der `libnova-ui` (GTK4-rs basierte Widgets und Utilities) durch die NovaShell-Komponenten geprägt.
    *   Es gibt typischerweise weniger explizite Trait-basierte Dienste oder Event-Busse *zwischen* NovaKit als Bibliothek und NovaShell als direkter Nutzer, sondern eher innerhalb von NovaKit definierte APIs und innerhalb der NovaShell verwendete UI-Event-Handler (GTK-Signale).

    5.1. **Traits (NovaKit stellt bereit, NovaShell konsumiert - eher als Bibliotheks-API):**
        *   Die öffentlichen Strukturen und Methoden von `libnova-ui` (z.B. `NovaButton`, `NovaWindow`) und `libnova-core`.
        *   Beispiel: Ein `ThemeManager` Trait innerhalb von `libnova-ui`, den die Shell nutzen kann, um Theme-Änderungen zu abonnieren, falls dies nicht direkt über GSettings-Änderungen (von `nova-settingsd`) gehandhabt wird.

    5.2. **Events (Von NovaKit an NovaShell oder umgekehrt):**
        *   Primär GTK-Signale (z.B. `button_clicked`, `window_close_request`).
        *   Wenn NovaKit eigene asynchrone Operationen durchführt (z.B. Laden von Online-Account-Daten in `libnova-accounts`), könnte es `NovaKitEvent` über einen Broadcast-Kanal senden.
            ```rust
            // Annahme: Diese Typen sind in GLOBAL_TYPES_DEFINITIONS.md definiert
            use crate::core::types::account::{AccountId, AccountStatus}; 

            #[derive(Clone, Debug)]
            pub enum NovaKitEvent {
                OnlineAccountStatusChanged(AccountId, AccountStatus),
                // Weitere bibliotheksinterne Events, die für die UI relevant sind
            }
            ```

**6. Fehler-Mapping zwischen Schichten**
    *   **NovaHAL -> NovaCore:** `DisplayError`, `InputError`, `PowerError` (aus NovaHAL) werden von NovaCore-Komponenten entweder direkt behandelt oder in einen spezifischeren NovaCore-Fehlertyp (z.B. `NovaWmError`, `NovaSessionError`) umgewandelt, falls sie weiter propagiert werden. Oft wird `anyhow::Error` mit Kontext in den oberen Schichten genutzt.
    *   **NovaCore -> NovaShell/NovaKit:** Ähnlich, Fehler aus `WindowManagerError`, `SessionError`, `SettingsError` werden von der UI-Schicht behandelt (z.B. Anzeige einer Fehlermeldung) oder geloggt.
    *   Es wird empfohlen, dass jede Schicht ihre Fehlerdomäne klar definiert. Übergreifende Fehler, die bis zur UI durchschlagen, könnten einen gemeinsamen `UserVisibleError` Trait implementieren.
