**0. Metadaten**
    *   **Dokument-ID:** `NOVA_DE_IMPL_SYS_COMPOSITOR_001`
    *   **Bezieht sich auf:** `NOVA_DE_GLOBAL_ARCH_001`, `NOVA_DE_INTERFACES_001`, `NOVA_DE_IMPL_CORE_ERROR_001`, `NOVA_DE_IMPL_CORE_TYPES_GEOMETRY_001`
    *   **Version:** 1.0.0
    *   **Status:** In Entwicklung
    *   **Erstellt am:** 2024-07-17
    *   **Letzte Änderung:** 2024-07-17
    *   **Verantwortlich:** NovaGem KI Architekt

**1. Verzeichnis-/Modulname**
    *   `novade/system/compositor` (Haupt-Crate für `nova-wm`)

**2. Verantwortlichkeit**
    *   Dieses Modul implementiert den `nova-wm`, den zentralen Wayland-Compositor und Fenstermanager für NovaDE. Er ist verantwortlich für die Darstellung und Verwaltung von Anwendungsfenstern, die Weiterleitung von Eingabeereignissen, das Compositing der Desktop-Oberfläche, die Verwaltung von Ausgabegeräten (Monitoren) und die Implementierung von Wayland-Protokollen. Er interagiert eng mit dem `NovaHAL` für Hardware-Zugriff und der `NovaShell` für die Darstellung von Desktop-Elementen.

**3. Kern-Aufgaben (Tasks):**

    3.1. **Vollständige Rust-Typdefinitionen (Structs, Enums, Traits):**

        3.1.1. **`pub struct NovaWm`** (Hauptstruktur des Compositors)
            *   Sichtbarkeit: `pub` (innerhalb des `compositor` Crate, öffentliche API über D-Bus/Wayland)
            *   `#[derive(Debug)]` (ggf. manuell, da `wlroots` Typen oft nicht Debug implementieren)
            *   **Felder (Auswahl, wlroots-basiert):**
                *   `backend: wlroots::Backend` (z.B. `LibinputBackend`, `SessionBackend`, `MultiBackend`)
                *   `renderer: wlroots::Renderer` (GLES2 Renderer)
                *   `allocator: wlroots::Allocator` (GBM oder anderer Buffer Allocator)
                *   `compositor: wlroots::Compositor` (Haupt-Compositor-Objekt)
                *   `display: wlroots::Display` (Wayland Display)
                *   `event_loop: wlroots::EventLoop` (Haupt-Event-Loop)
                *   `socket_name: String` (Name des Wayland Sockets, z.B. "wayland-1")
                *   `outputs: Vec<OutputManager>` (Verwaltung der Monitore)
                *   `inputs: InputManager` (Verwaltung der Eingabegeräte)
                *   `surfaces: std::collections::HashMap<wlroots::SurfaceHandle, SurfaceData>` (Verwaltung von Wayland Surfaces)
                *   `seats: std::collections::HashMap<String, SeatManager>` (Verwaltung von Seats, typischerweise "seat0")
                *   `xdg_shell_manager: wlroots::XdgShellManager` (für xdg-shell Clients)
                *   `xdg_decoration_manager: wlroots::XdgDecorationManagerV1` (für serverseitige Dekorationen)
                *   `layer_shell_manager: wlroots::LayerShellManagerV1` (für Shell-Komponenten wie Panels)
                *   `output_layout: wlroots::OutputLayout` (Anordnung der Monitore)
                *   `cursor: wlroots::Cursor` (Hardware-Cursor)
                *   `xwayland_server: Option<wlroots::XWaylandServer>` (für X11 Kompatibilität)
                *   `active_window: Option<WindowIdentifier>` (Identifiziert das gerade aktive Fenster)
                *   `windows: Vec<ManagedWindow>` (Liste der Top-Level Fenster)
                *   `dnd_manager: DragAndDropManager`
                *   `clipboard_manager: ClipboardManager`
                *   `screencopy_manager: ScreencopyManager` (für Screenshots)
                *   `config: std::sync::Arc<NovaWmConfig>` (Laufzeitkonfiguration)
                *   `core_services_bus_conn: Option<zbus::Connection>` // Für D-Bus Service
                *   `hal_event_receiver: tokio::sync::broadcast::Receiver<novade_interfaces_schichten_absolut::NovaHalEvent>`
                *   `core_event_sender: tokio::sync::broadcast::Sender<novade_interfaces_schichten_absolut::NovaCoreEvent>`

        3.1.2. **`pub struct OutputManager`**
            *   Felder: `handle: wlroots::OutputHandle`, `name: String`, `current_mode: Option<wlroots::OutputMode>`, `transform: wlroots::Transform`, `scale: f64`, `global: Option<wlroots::Global>`, `damage_tracker: wlroots::OutputDamage`
            *   Verwaltet einen einzelnen Monitor, dessen Modi, Transformation, Skalierung und Schadenverfolgung.

        3.1.3. **`pub struct InputManager`**
            *   Felder: `libinput_context: Option<wlroots::Libinput>` (oder äquivalent), `keyboard_devices: Vec<wlroots::KeyboardHandle>`, `pointer_devices: Vec<wlroots::PointerHandle>`, `touch_devices: Vec<wlroots::TouchHandle>`
            *   Verwaltet alle Eingabegeräte.

        3.1.4. **`pub struct SeatManager`**
            *   Felder: `seat: wlroots::Seat`, `keyboard_state: Option<KeyboardFocusState>`, `pointer_state: PointerFocusState`, `capabilities: wlroots::Capability`, `last_click_serial: u32`
            *   Verwaltet den Fokus von Tastatur und Maus, Drag&Drop-Operationen etc.

        3.1.5. **`pub struct ManagedWindow`** (Repräsentiert ein Top-Level Fenster, z.B. XDG Shell Surface)
            *   Felder: `id: WindowIdentifier`, `surface_handle: wlroots::SurfaceHandle`, `title: Option<String>`, `app_id: Option<String>`, `geometry: novade_core_types_geometry::Rect<i32, u32>`, `state: WindowState` (Maximized, Minimized, Tiled, Floating), `decoration_mode: Option<wlroots:: décoration::Mode>`, `parent: Option<WindowIdentifier>`, `is_popup: bool`.

        3.1.6. **`#[derive(Clone, Debug, PartialEq, Eq, Hash)] pub enum WindowIdentifier`**
            *   `Xdg(wlroots::XdgSurfaceHandle)`
            *   `Xwayland(wlroots::XWaylandSurfaceHandle)`
            // LayerShells werden typischerweise nicht als "ManagedWindow" mit ID geführt, sondern sind Teil der Shell.

        3.1.7. **`#[derive(Clone, Debug, serde::Deserialize)] pub struct NovaWmConfig`** (Laufzeitkonfiguration)
            *   Felder: `enable_xwayland: bool`, `default_keyboard_layout: String`, `tap_to_click: bool`, `focus_follows_mouse: bool`, `window_decoration_mode: String` ("client" oder "server").

        3.1.8. **`#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)] pub enum WindowState`**
            *   `#[default] Floating`, `Maximized`, `Minimized`, `TiledLeft`, `TiledRight`, `Fullscreen`

        3.1.9. **`pub struct SurfaceData`** // Daten assoziiert mit jedem wlroots::Surface
            *   Felder: `parent_window_id: Option<WindowIdentifier>`, `role: Option<String>` // z.B. "xdg_toplevel", "xdg_popup", "cursor"

        3.1.10. **`pub struct KeyboardFocusState`**
            *   Felder: `focused_surface: Option<wlroots::SurfaceHandle>`, `keyboard_handle: wlroots::KeyboardHandle`

        3.1.11. **`pub struct PointerFocusState`**
             *   Felder: `focused_surface: Option<wlroots::SurfaceHandle>`, `relative_pos: novade_core_types_geometry::Point<f64>`

        3.1.12. **`pub struct DragAndDropManager`** // Platzhalter für DND Logik
        3.1.13. **`pub struct ClipboardManager`** // Platzhalter für Clipboard Logik
        3.1.14. **`pub struct ScreencopyManager`** // Platzhalter für Screencopy Logik

        3.1.15. **Traits, die `NovaWm` (oder dessen innere Logik-Handler) implementiert:**
            *   `wlroots::CompositorHandler`
            *   `wlroots::BackendHandler`
            *   `wlroots::OutputHandler`
            *   `wlroots::InputHandler` (oder spezifischer: `KeyboardHandler`, `PointerHandler`, `TouchHandler`)
            *   `wlroots::XdgShellHandler`
            *   `wlroots::XdgDecorationHandlerV1`
            *   `wlroots::LayerShellHandlerV1`
            *   `wlroots::SeatHandler`
            *   `novade_interfaces_schichten_absolut::WindowManagerCtl` (aus `01_INTERFACES_SCHICHTEN_ABSOLUT.md`)

    3.2. **Vollständige Rust-Funktionssignaturen (Auswahl für `NovaWm`):**

        *   `pub async fn new(config_path: Option<std::path::PathBuf>, hal_event_receiver: tokio::sync::broadcast::Receiver<novade_interfaces_schichten_absolut::NovaHalEvent>, core_event_sender: tokio::sync::broadcast::Sender<novade_interfaces_schichten_absolut::NovaCoreEvent>) -> Result<Box<Self>, novade_core_error::CoreError>` (Return Box<Self> da wlroots Handler oft `Box<dyn Handler>` erwarten)
        *   `pub fn run(self: Box<Self>) -> Result<(), novade_core_error::CoreError>` (Startet den Haupt-Event-Loop)
        *   `fn setup_backend(&mut self)`
        *   `fn setup_renderer(&mut self)`
        *   `fn setup_compositor_protocols(&mut self)`
        *   `fn setup_input_devices(&mut self)`
        *   `fn setup_outputs(&mut self)`
        *   `fn setup_xwayland(&mut self)`
        *   `async fn process_hal_events(mut self: std::sync::Arc<tokio::sync::Mutex<Self>>)` // Beispiel für Task, der Zugriff auf Self braucht
        *   `fn render_output(&mut self, output_manager: &OutputManager, renderer: &mut wlroots::Renderer, frame: &mut wlroots::OutputFrame, when: timespec)`
        *   `fn handle_new_surface(&mut self, surface_handle: wlroots::SurfaceHandle)`
        *   `fn handle_new_xdg_surface(&mut self, xdg_surface_handle: wlroots::XdgSurfaceHandle)`
        *   `fn handle_new_layer_surface(&mut self, layer_surface_handle: wlroots::LayerSurfaceV1Handle)`
        *   `fn focus_surface(&mut self, surface_handle: Option<wlroots::SurfaceHandle>, seat_handle: wlroots::SeatHandle)`
        *   `fn get_surface_at(&self, point: novade_core_types_geometry::Point<f64>, visible_only: bool) -> Option<(wlroots::SurfaceHandle, novade_core_types_geometry::Point<i32>)>`
        *   `fn send_core_event(&self, event: novade_interfaces_schichten_absolut::NovaCoreEvent)`

    3.3. **Explizite Algorithmusbeschreibungen (Auswahl):**

        *   **Initialisierung (`NovaWm::new`)**:
            1.  Lade Konfiguration (`NovaWmConfig`).
            2.  Erstelle `wlroots::Display` und `wlroots::EventLoop`.
            3.  Erstelle `wlroots::Backend` (MultiBackend: DRM, Libinput, Session).
            4.  Erstelle `wlroots::Renderer` und `wlroots::Allocator`.
            5.  Erstelle `wlroots::Compositor`.
            6.  Initialisiere `wlroots::OutputLayout`.
            7.  Initialisiere `wlroots::Cursor` und verbinde mit OutputLayout.
            8.  Registriere `wlroots`-Handler Implementierungen (`Box::new(self)` oder `Rc<RefCell<Self>>` für Handler).
            9.  Registriere Wayland Globals.
            10. Starte XWayland Server falls konfiguriert.
            11. Initialisiere `InputManager` und `OutputManager` Listen.
            12. Starte D-Bus Service für `WindowManagerCtl` Trait in einem separaten Tokio-Task.
            13. Speichere `hal_event_receiver` und `core_event_sender`.
            14. Starte Tokio-Task für `process_hal_events`.
            15. Return `Ok(Box::new(Self))`.

        *   **Rendering eines Outputs (`NovaWm::render_output`)**:
            1.  Hole aktuellen Output Schaden.
            2.  Beginne Rendering.
            3.  Lösche Bildschirm / zeichne Hintergrund.
            4.  Iteriere durch Layer-Shell Surfaces (Hintergrund, unten) und rendere sie.
            5.  Iteriere durch XDG-Shell Surfaces (Fenster) in Stapelreihenfolge:
                a.  Hole Textur.
                b.  Wende Transformationen an.
                c.  Rendere Textur.
                d.  Rendere serverseitige Dekorationen falls aktiv.
            6.  Iteriere durch Layer-Shell Surfaces (oben, Overlays) und rendere sie.
            7.  Rendere Hardware-Cursor.
            8.  Committe Frame und aktualisiere Schaden.

        *   **Input Event Handling (Pointer Motion)**:
            1.  Event von `LibinputBackend` -> `InputHandler::on_pointer_motion`.
            2.  Aktualisiere Cursor Position.
            3.  Bestimme Output und Surface unter Cursor (`get_surface_at`).
            4.  Wenn Surface gefunden:
                a.  Setze Pointer-Fokus des Seats.
                b.  Sende Motion-Event an Client.
                c.  Behandle DND falls aktiv.
            5.  Wenn kein Surface, aber Fenster wird gezogen: bewege Fenster.
            6.  Rendere Cursor.

        *   **Verarbeitung von `NovaHalEvent::InputDeviceAdded` in `process_hal_events`**:
            1.  Empfange Event.
            2.  Extrahiere `InputDeviceInfo`.
            3.  Über einen Kanal oder direkten Aufruf (wenn `NovaWm` als `Arc<Mutex<Self>>` verfügbar ist und der Event-Loop es zulässt) die `NovaWm` Instanz benachrichtigen, das Gerät im `wlroots::Backend` zu registrieren.
            4.  `wlroots` löst dann die entsprechenden Handler-Callbacks aus.
            5.  In diesen Callbacks: Erstelle und registriere `wlroots::Keyboard/Pointer/TouchHandle`.
            6.  Konfiguriere Gerät (Layout, etc.).
            7.  Aktualisiere Seat Capabilities.

    3.4. **Erschöpfende Fehlerbehandlung:**
        *   `wlroots` Fehler werden geloggt; kritische Fehler (DRM) können zum Exit führen.
        *   Verwendung von `novade_core_error::CoreError`.
        *   D-Bus Methoden geben `zbus::fdo::Error` oder `CoreError` zurück.
        *   HAL Event Verarbeitungsfehler werden geloggt.
        *   Wayland Protokollfehler führen zum Client-Disconnect.

    3.5. **Speicher-/Ressourcenmanagement-Direktiven:**
        *   Korrekte Verwaltung von `wlroots` Handles (Rust-Bindings via `Drop`).
        *   Buffer Management durch `wlroots`.
        *   `std::sync::Arc` und `tokio::sync::Mutex` für geteilte Zustände zwischen wlroots-Callbacks und Tokio-Tasks.
        *   Minimierung von Klonen in Rendering/Input Pfaden.

**4. Spezifische Artefakte/Dateien (innerhalb `novade/system/compositor/src/`):**
    *   **`main.rs`**: Einstiegspunkt.
    *   **`compositor.rs`**: `NovaWm` Definition, Hauptlogik, `wlroots::CompositorHandler`.
    *   **`backend.rs`**: `wlroots::BackendHandler`.
    *   **`output.rs`**: `OutputManager`, `wlroots::OutputHandler`.
    *   **`input.rs`**: `InputManager`, `wlroots::InputHandler`.
    *   **`seat.rs`**: `SeatManager`, `wlroots::SeatHandler`.
    *   **`surface.rs`**: Verwaltung von `wlroots::Surface`, `SurfaceData`.
    *   **`shells/xdg.rs`**: `wlroots::XdgShellHandler`.
    *   **`shells/layer.rs`**: `wlroots::LayerShellHandlerV1`.
    *   **`shells/xwayland.rs`**: XWayland Server Logik.
    *   **`render/mod.rs`**: Rendering-Logik.
    *   **`manager/window.rs`**: `ManagedWindow`, Fenstermanagement.
    *   **`manager/drag_and_drop.rs`**: DND Logik.
    *   **`manager/clipboard.rs`**: Clipboard Logik.
    *   **`manager/screencopy.rs`**: Screenshot Logik.
    *   **`config.rs`**: `NovaWmConfig`.
    *   **`dbus.rs`**: Implementierung `WindowManagerCtl` D-Bus Service.
    *   **`event_handlers.rs`**: Handler für `NovaHalEvent`, Erzeugung `NovaCoreEvent`.
    *   **`utils.rs`**: Hilfsfunktionen.

**5. Abhängigkeiten:**
    *   **Intern:**
        *   `novade_core_error`
        *   `novade_core_types_geometry`
        *   `novade_interfaces_schichten_absolut` (für Event- und Trait-Definitionen)
    *   **Extern (Versionen gemäß `00_GLOBAL_ARCHITEKTUR_DEFINITION.md`):**
        *   `wlroots-rs = "=0.17"`
        *   `wlroots-sys` (entsprechende Version)
        *   `tokio = { version = "=1.35", features = ["full"] }`
        *   `async-trait = "=0.1.73"`
        *   `thiserror = "=1.0"` (z.B. 1.0.50)
        *   `zbus = "=3.14.1"`
        *   `tracing = "=0.1"`
        *   `tracing-subscriber = "=0.3"`
        *   `serde = { version = "=1.0", features = ["derive"], optional = true }`
        *   `ron = { version = "=0.8", optional = true }`
        *   `libc`
        *   `std`
        *   `wayland-server` (von wlroots-rs)
        *   `wayland-protocols` (von wlroots-rs)
        *   `xkbcommon-rs`

**6. Kommunikationsmuster:**
    *   **Inbound:** Wayland Nachrichten, wlroots Backend Events, `NovaHalEvent`s (Broadcast), D-Bus Aufrufe.
    *   **Outbound:** Wayland Nachrichten, `NovaCoreEvent`s (Broadcast).
    *   **Synchronisation:**
        *   `wlroots` Event-Loop ist single-threaded. Callbacks von `wlroots` werden in diesem Thread ausgeführt.
        *   Tokio Tasks für D-Bus und HAL Event Verarbeitung.
        *   Kommunikation zwischen Tokio Tasks und dem `wlroots` Thread muss sorgfältig gehandhabt werden:
            *   Für Daten, die von Tokio Tasks modifiziert und vom `wlroots` Thread gelesen werden: `std::sync::Arc<tokio::sync::Mutex<DataType>>`.
            *   Um Aktionen im `wlroots` Thread von einem Tokio Task auszulösen: `glib::MainContext::spawn_local` (wenn der `wlroots::EventLoop` mit GLib integriert ist) oder ein `std::sync::mpsc` Kanal, dessen Empfänger im `wlroots` Thread gepollt wird. `wlroots::EventLoop::channel()` kann hierfür verwendet werden.
            *   `NovaWm` selbst könnte als `Box<NovaWm>` im `wlroots::EventLoop` leben. Zugriff von Tokio-Tasks darauf wäre indirekt über Kanäle oder durch `Arc<Mutex<...>>` für geteilte Daten.

**7. Erwartete Ergebnisse/Outputs:**
    *   Funktionaler `nova-wm` Wayland-Compositor.
    *   Unterstützung für XDG Shell, Layer Shell, XWayland.
    *   Effizientes Rendering und Input-Handling.
    *   Bereitstellung der `WindowManagerCtl` Schnittstelle.
    *   Verarbeitung von Hardware-Events und Erzeugung von Core-Events.
