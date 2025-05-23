**0. Metadaten**
    *   **Dokument-ID:** `NOVA_DE_IMPL_UI_PANEL_001`
    *   **Bezieht sich auf:** `NOVA_DE_GLOBAL_ARCH_001`, `NOVA_DE_INTERFACES_001`, `NOVA_DE_IMPL_CORE_ERROR_001`, `NOVA_DE_IMPL_CORE_TYPES_GEOMETRY_001`
    *   **Version:** 1.0.0
    *   **Status:** In Entwicklung
    *   **Erstellt am:** 2024-07-17
    *   **Letzte Änderung:** 2024-07-17
    *   **Verantwortlich:** NovaGem KI Architekt

**1. Verzeichnis-/Modulname**
    *   `novade/ui/shell/panel` (Teil der NovaShell)

**2. Verantwortlichkeit**
    *   Dieses Modul implementiert eine Panel-Komponente für die NovaShell. Das Panel dient als primäre Interaktionsfläche für den Benutzer, um Anwendungen zu starten, laufende Anwendungen zu verwalten (Task-Manager), Systemstatusinformationen anzuzeigen (System-Tray, Uhrzeit, Benachrichtigungen) und auf Kernfunktionen des Desktops zuzugreifen. Es kann an verschiedenen Bildschirmkanten positioniert werden und ist konfigurierbar.

**3. Kern-Aufgaben (Tasks):**

    3.1. **Vollständige Rust-Typdefinitionen (Structs, Enums, Traits):**

        3.1.1. **`pub struct NovaPanel`** (Hauptstruktur des Panels)
            *   Sichtbarkeit: `pub` (innerhalb des `panel` Moduls, ggf. als Teil einer größeren `NovaShell` Crate)
            *   `#[derive(gtk::CompositeTemplate, glib::Downgrade)]` (Verwendung von GTK4 Composite Templates)
            *   `#[template(file = "nova_panel.ui")]` (oder `string = "..."` für Inline-UI-Definition)
            *   **Felder (Widgets, intern):**
                *   `#[template_child] pub main_box: gtk::Box;` (Hauptcontainer des Panels)
                *   `#[template_child] pub app_launcher_button: libnova_ui::widgets::NovaButton;`
                *   `#[template_child] pub task_manager_container: gtk::Box;`
                *   `#[template_child] pub system_tray_container: gtk::Box;`
                *   `#[template_child] pub clock_widget: gtk::Label;`
                *   `#[template_child] pub notification_indicator: libnova_ui::widgets::NovaIcon;`
                *   `#[template_child] pub workspace_switcher: gtk::Box;` (optional)
                *   `layer_surface: gtk4_layer_shell::LayerSurface` // Initialisiert in `fn init_layer_shell`
            *   **Felder (Zustand, intern):**
                *   `config: std::sync::Arc<tokio::sync::RwLock<PanelConfig>>` // RwLock für Lese/Schreibzugriff
                *   `task_buttons: std::collections::HashMap<novade_interfaces_schichten_absolut::NovaCoreEvent_WindowId, libnova_ui::widgets::NovaToggleButton>` // WindowId muss Eq, Hash sein
                *   `system_tray_items: std::collections::HashMap<String, Box<dyn SystemTrayItem>>` (String ist D-Bus Service Name)
                *   `core_event_receiver: tokio::sync::broadcast::Receiver<novade_interfaces_schichten_absolut::NovaCoreEvent>`
                *   `settings_service: std::sync::Arc<dyn novade_interfaces_schichten_absolut::SettingsService + Send + Sync>`
                *   `window_manager_ctl: std::sync::Arc<dyn novade_interfaces_schichten_absolut::WindowManagerCtl + Send + Sync>`
                *   `active_window_id: std::sync::Arc<tokio::sync::Mutex<Option<novade_interfaces_schichten_absolut::NovaCoreEvent_WindowId>>>` // Geteilter Zustand

        3.1.2. **`pub struct PanelConfig`** (Konfiguration des Panels)
            *   Sichtbarkeit: `pub`
            *   `#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]`
            *   **Felder:**
                *   `position: PanelPosition` (Top, Bottom, Left, Right)
                *   `height: u32` (oder `width` für vertikale Panels)
                *   `autohide: bool`
                *   `widgets_left: Vec<PanelWidgetType>`
                *   `widgets_center: Vec<PanelWidgetType>`
                *   `widgets_right: Vec<PanelWidgetType>`
                *   `icon_size: u32`
                *   `clock_format: String` (z.B. "%H:%M")
                *   `show_workspace_switcher: bool`

        3.1.3. **`#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize, Default)] pub enum PanelPosition`**
            *   `#[default] Bottom`, `Top`, `Left`, `Right`

        3.1.4. **`#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)] pub enum PanelWidgetType`**
            *   `AppLauncher`, `TaskManager`, `WorkspaceSwitcher`, `SystemTray`, `Clock`, `NotificationIndicator`, `Spacer`

        3.1.5. **`pub trait SystemTrayItem: std::fmt::Debug + Send`**
            *   `fn get_widget(&self) -> gtk::Widget;` // Muss Widget zurückgeben, nicht &
            *   `fn on_activate(&self);`
            *   `fn on_secondary_activate(&self);`
            *   `fn id(&self) -> &str;`

    3.2. **Vollständige Rust-Funktionssignaturen (Auswahl für `NovaPanel`):**

        *   `pub fn new(core_event_receiver: tokio::sync::broadcast::Receiver<novade_interfaces_schichten_absolut::NovaCoreEvent>, settings_service: std::sync::Arc<dyn novade_interfaces_schichten_absolut::SettingsService + Send + Sync>, window_manager_ctl: std::sync::Arc<dyn novade_interfaces_schichten_absolut::WindowManagerCtl + Send + Sync>) -> Self`
        *   `pub fn init_layer_shell(&self, window: &impl glib::IsA<gtk::Window>)`
        *   `async fn load_config(self: std::sync::Arc<Self>)` // self: Arc<Self> für async Methoden in GTK Callbacks
        *   `fn apply_config(self: std::sync::Arc<Self>)`
        *   `fn setup_widgets(self: std::sync::Arc<Self>)`
        *   `fn connect_signals(self: std::sync::Arc<Self>)`
        *   `async fn listen_core_events(self: std::sync::Arc<Self>)`
        *   `fn update_task_manager(self: std::sync::Arc<Self>, event: &novade_interfaces_schichten_absolut::NovaCoreEvent_WindowLifeCycleEvent)`
        *   `fn add_task_button(self: std::sync::Arc<Self>, id: novade_interfaces_schichten_absolut::NovaCoreEvent_WindowId, title: Option<String>, app_id: Option<String>)`
        *   `fn remove_task_button(self: std::sync::Arc<Self>, id: &novade_interfaces_schichten_absolut::NovaCoreEvent_WindowId)`
        *   `fn set_task_button_active(self: std::sync::Arc<Self>, id: &novade_interfaces_schichten_absolut::NovaCoreEvent_WindowId, active: bool)`
        *   `fn update_clock(self: std::sync::Arc<Self>)`
        *   `fn update_notification_indicator(self: std::sync::Arc<Self>, count: u32, has_critical: bool)`
        *   `fn handle_app_launcher_clicked(self: std::sync::Arc<Self>)`
        *   `fn handle_task_button_clicked(self: std::sync::Arc<Self>, window_id: novade_interfaces_schichten_absolut::NovaCoreEvent_WindowId)`
        *   `async fn discover_system_tray_items(self: std::sync::Arc<Self>)`
        *   `fn add_system_tray_item(self: std::sync::Arc<Self>, item: Box<dyn SystemTrayItem>)`
        *   `fn remove_system_tray_item(self: std::sync::Arc<Self>, item_id: &str)`

    3.3. **Explizite Algorithmusbeschreibungen (Auswahl):**

        *   **Initialisierung (`NovaPanel::new`)**:
            1.  Erstelle die GTK-Widget-Instanz via `glib::Object::new()` (Composite Template wird initialisiert).
            2.  Initialisiere `layer_surface` (noch nicht mit Window verbunden).
            3.  Initialisiere `config` mit `Arc<RwLock<PanelConfig::default()>>`.
            4.  Initialisiere `active_window_id` mit `Arc<Mutex<None>>`.
            5.  Speichere `core_event_receiver`, `settings_service`, `window_manager_ctl`.
            6.  Rufe `glib::MainContext::spawn_local` für `Arc::clone(&self).load_config()`.
            7.  Rufe `glib::MainContext::spawn_local` für `Arc::clone(&self).listen_core_events()`.
            8.  Erstelle einen `glib::timeout_add_seconds_local` für `Arc::clone(&self).update_clock()`.
            9.  Rufe `glib::MainContext::spawn_local` für `Arc::clone(&self).discover_system_tray_items()`.
            10. Rufe `Arc::clone(&self).connect_signals()` auf.
            11. Gib `self` zurück.

        *   **`NovaPanel::init_layer_shell(&self, window: &impl glib::IsA<gtk::Window>)`**:
            1.  `self.layer_surface.init_for_window(window);`
            2.  `self.layer_surface.set_layer(gtk4_layer_shell::Layer::Top);`
            3.  `self.layer_surface.auto_exclusive_zone_enable();`
            4.  `// Weitere Layer-Shell Einstellungen (anchor, margin) basierend auf PanelConfig.position`
            5.  `self.apply_config()` (oder ein Teil davon, der die Layer-Shell betrifft) wird hier oder nach `load_config` aufgerufen.

        *   **`NovaPanel::apply_config(self: Arc<Self>)`**:
            1.  Lock `self.config` lesend.
            2.  Passe `self.layer_surface` an (anchor, margins, exclusive zone) gemäß `config.position` und `config.height`.
            3.  `main_box` Kinder leeren.
            4.  Iteriere durch `config.widgets_left`, `widgets_center`, `widgets_right` und erstelle/füge Widgets hinzu.
            5.  Setze Autohide-Verhalten: `self.layer_surface.set_auto_exclusive_zone(config.autohide);` (oder ähnliche API).
            6.  Aktualisiere Icon-Größen etc.

        *   **`NovaPanel::update_task_manager(self: Arc<Self>, event: &NovaCoreEvent_WindowLifeCycleEvent)`**:
            1.  `match event`:
                *   `WindowCreated { id, title, app_id, .. }`: Rufe `self.add_task_button(*id, title.clone(), app_id.clone())` auf.
                *   `WindowClosed { id, .. }`: Rufe `self.remove_task_button(id)` auf.
                *   `WindowFocused { id, .. }`:
                    *   Lock `self.active_window_id` schreibend.
                    *   Wenn `let Some(old_id) = active_window_id.take()`, rufe `self.set_task_button_active(&old_id, false)` auf.
                    *   Rufe `self.set_task_button_active(id, true)` auf.
                    *   `*active_window_id = Some(*id)`.
                *   `WindowTitleChanged { id, new_title, .. }`: Finde Button in `self.task_buttons`, aktualisiere Label/Tooltip.

        *   **`NovaPanel::discover_system_tray_items(self: Arc<Self>)`**:
            1.  Verwende `zbus` Async API.
            2.  Erstelle einen `zbus::proxy::Proxy` für `org.freedesktop.StatusNotifierWatcher` auf dem Session Bus.
            3.  Rufe `RegisterStatusNotifierItem` und `RegisterStatusNotifierHost` auf.
            4.  Abonniere `StatusNotifierItemRegistered` und `StatusNotifierItemUnregistered` Signale.
            5.  Bei `StatusNotifierItemRegistered(service_name)`:
                a.  Erstelle `DBusSystemTrayItem` (Proxy zum `service_name`).
                b.  Füge zu `self.system_tray_items` und `self.system_tray_container` hinzu.
            6.  Bei `StatusNotifierItemUnregistered(service_name_or_path)`: Entferne entsprechend.

    3.4. **Erschöpfende Fehlerbehandlung:**
        *   `load_config`: Bei Fehler (z.B. `SettingsService` nicht erreichbar, RON-Deserialisierungsfehler) wird geloggt und `PanelConfig::default()` verwendet.
        *   D-Bus Fehler in `discover_system_tray_items` oder bei `WindowManagerCtl` Aufrufen werden geloggt; betroffene UI-Teile zeigen ggf. keinen Inhalt oder reagieren nicht.
        *   `NovaCoreEvent` Empfangsfehler (z.B. wenn der Kanal geschlossen wird) werden geloggt und der `listen_core_events` Task beendet sich.
        *   Result-Typen (`CoreError` oder spezifischere UI-Fehler) werden von `async fn` Methoden zurückgegeben, wo sinnvoll.

    3.5. **Speicher-/Ressourcenmanagement-Direktiven:**
        *   `NovaPanel` implementiert `glib::Downgrade` und wird oft als `glib::WeakRef<NovaPanel>` in Callbacks übergeben, um Zyklen zu vermeiden. `glib::clone!` Makro verwenden.
        *   `gtk4_layer_shell::LayerSurface` wird von GTK verwaltet, sobald an ein Fenster gebunden.
        *   `tokio::sync::broadcast::Receiver::resubscribe()` wird für den `core_event_receiver` verwendet, um ihn in `Arc<Self>` zu klonen.
        *   Entfernen von Widgets aus Containern und aus `task_buttons`/`system_tray_items` HashMaps, wenn sie nicht mehr benötigt werden.
        *   D-Bus Proxies und Signal-Handler müssen bei Zerstörung des Panels korrekt abgemeldet werden.

**4. Spezifische Artefakte/Dateien (innerhalb `novade/ui/shell/panel/src/`):**
    *   **`lib.rs`** oder **`mod.rs`**: Hauptdatei des Moduls, definiert `NovaPanel`.
    *   **`nova_panel.ui`**: XML-Datei für das GTK Composite Template.
    *   **`config.rs`**: Definition von `PanelConfig`, `PanelPosition`, `PanelWidgetType`.
    *   **`widgets/mod.rs`**: Submodul für spezifische Panel-Widgets (z.B. TaskButton, Clock).
        *   `widgets/task_button.rs`
        *   `widgets/clock_display.rs`
    *   **`system_tray/mod.rs`**: Logik für System-Tray-Integration.
        *   `system_tray/dbus_item.rs` (Implementierung von `SystemTrayItem` für D-Bus)
    *   **`event_handlers.rs`**: Implementierung der Logik zur Verarbeitung von `NovaCoreEvent`s.
    *   **`dispatch.rs`**: Enthält GTK-Signal-Handler und asynchrone Logik, die von `NovaPanel` aufgerufen wird (oft als `impl NovaPanel {}`).

**5. Abhängigkeiten:**
    *   **Intern:**
        *   `novade_core_error`
        *   `novade_core_types_geometry`
        *   `novade_interfaces_schichten_absolut`
        *   `libnova_ui`
    *   **Extern (Versionen gemäß `00_GLOBAL_ARCHITEKTUR_DEFINITION.md`):**
        *   `gtk4 = "=0.8"`
        *   `glib = "=0.18"`
        *   `gtk4-layer-shell = "=0.2"` (oder neueste kompatible Version, z.B. "0.2.1")
        *   `tokio = { version = "=1.35", features = ["full"] }`
        *   `async-trait = "=0.1.73"` // Wenn Traits mit async fn Methoden definiert werden, die von NovaPanel implementiert werden
        *   `thiserror = "=1.0"` // z.B. 1.0.50
        *   `serde = { version = "=1.0", features = ["derive"] }`
        *   `ron = "=0.8"`
        *   `zbus = "=3.14.1"`
        *   `tracing = "=0.1"`
        *   `chrono = { version = "=0.4", features = ["serde"], optional = true }`

**6. Kommunikationsmuster:**
    *   **Inbound:** `NovaCoreEvent`s (Broadcast), GTK-Signale, D-Bus-Antworten/Signale (Settings, WindowManager, StatusNotifierItem).
    *   **Outbound:** D-Bus-Aufrufe (`WindowManagerCtl`, `SettingsService`, `nova-launcher` D-Bus Service).
    *   **Synchronisation:** `glib::MainContext::spawn_local` für UI-Updates aus Tokio-Tasks. `Arc<Mutex/RwLock>` für geteilten Zustand. `glib::clone!` für sichere `self`-Referenzen in Callbacks.

**7. Erwartete Ergebnisse/Outputs:**
    *   Ein funktionales, konfigurierbares `NovaPanel` Widget.
    *   Integration von AppLauncher, Task-Manager, System-Tray, Uhr, Benachrichtigungsindikator.
    *   Korrekte Positionierung und Verhalten mittels `gtk4-layer-shell`.
    *   Reaktionsfähigkeit auf Systemereignisse und Benutzerinteraktionen.
```
