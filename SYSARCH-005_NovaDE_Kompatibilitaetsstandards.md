**Dokument-ID:** SYSARCH-005
**Titel:** NovaDE Systemarchitektur - Kompatibilitätsstandards

**Inhalt:**

**1. Einleitung**
    1.1. Zweck: Dieses Dokument definiert die Kompatibilitätsstandards für das Nova Desktop Environment (NovaDE) mit bestehenden Linux-Desktop-Anwendungen und -Technologien. Ziel ist es, eine möglichst nahtlose Integration und eine gute Benutzererfahrung zu gewährleisten.
    1.2. Geltungsbereich: Umfasst X11-Anwendungskompatibilität, Einhaltung von Freedesktop.org-Standards, Toolkit-Kompatibilität und Unterstützung für verbreitete Anwendungs-Paketformate.

**2. X11-Anwendungskompatibilität**
    2.1. **Mechanismus: XWayland**
        2.1.1. Beschreibung: NovaDE's Compositor (`nova-wm`) wird XWayland integrieren, um die Ausführung von X11-Anwendungen zu ermöglichen, die noch nicht nativ Wayland unterstützen.
        2.1.2. XWayland Version: 22.1+ (siehe SYSARCH-002).
        2.1.3. Integration: `nova-wm` ist verantwortlich für das Starten und Verwalten der XWayland-Instanz. X11-Fenster werden vom Compositor wie native Wayland-Fenster behandelt (gestapelt, verschoben, etc.).
    2.2. **Unterstützte X11-Funktionen:**
        2.2.1. Fenstermanagement: Grundlegende Fensteroperationen (Erstellen, Verschieben, Größe ändern, Schließen), Dekorationen (serverseitig durch `nova-wm` oder clientseitig).
        2.2.2. Input: Maus, Tastatur, grundlegende Touch-Ereignisse.
        2.2.3. Zwischenablage: Synchronisation zwischen X11- und Wayland-Zwischenablagen wird durch `nova-clipboardd` in Zusammenarbeit mit `nova-wm` sichergestellt.
        2.2.4. System-Tray/Status-Icons: Unterstützung für `XEmbed` oder das modernere StatusNotifierItem-Protokoll (via D-Bus) für X11-Anwendungen wird angestrebt. `nova-panel` wird entsprechende Schnittstellen bereitstellen.
        2.2.5. Drag-and-Drop: Unterstützung für Xdnd (X Drag and Drop Protocol) zwischen X11-Anwendungen und idealerweise auch zwischen X11- und Wayland-Anwendungen.
    2.3. **Nicht unterstützte/Eingeschränkte X11-Funktionen:**
        2.3.1. Globales Input-Grabbing (außer für definierte Hotkeys durch `nova-wm`): Aus Sicherheitsgründen in Wayland stark eingeschränkt.
        2.3.2. Direkter Zugriff auf den Root-Window: Nicht relevant in Wayland.
        2.3.3. Bestimmte X-Extensions, die keine Wayland-Entsprechung haben oder Sicherheitsrisiken darstellen.
    2.4. **Konfiguration:** Optionen zur Steuerung von XWayland-spezifischen Einstellungen (z.B. HiDPI-Skalierung für X11-Anwendungen) werden über `nova-settingsd` bereitgestellt.

**3. Freedesktop.org Standards Konformität**
    NovaDE verpflichtet sich zur Einhaltung folgender (und weiterer relevanter) Freedesktop.org-Spezifikationen, um Interoperabilität und eine konsistente Benutzererfahrung zu gewährleisten:
    3.1. **XDG Base Directory Specification:** Für die Speicherung von Konfigurations-, Daten- und Cache-Dateien. Alle NovaDE-Komponenten und -Anwendungen müssen diese Spezifikation befolgen.
    3.2. **Desktop Entry Specification:** Für `.desktop`-Dateien zur Beschreibung von Anwendungen und deren Integration in `nova-launcher` und `nova-appmenu`.
    3.3. **Icon Theme Specification:** Für die Verwendung und das Auffinden von Icons. `nova-settingsd` verwaltet das aktuelle Icon-Thema.
    3.4. **MIME Applications Associations (mimeapps.list):** Für die Zuordnung von MIME-Typen zu Anwendungen. `libnova-core` wird Funktionen zur Handhabung bereitstellen, `nova-launcher` nutzt diese.
    3.5. **Desktop Notification Specification:** `nova-notificationd` implementiert diesen Standard für die Anzeige von Benachrichtigungen.
    3.6. **StatusNotifierItem Specification:** Für moderne System-Tray-Icons. `nova-panel` wird dies unterstützen.
    3.7. **Shared MIME-info Database:** Für die Definition von MIME-Typen.
    3.8. **Fontconfig:** Für die Schriftkonfiguration und -auswahl.
    3.9. **Recent File Storage Specification (zeitgeist oder ähnliches):** Optionale Integration für "Zuletzt verwendete Dateien".
    3.10. **xdg-utils:** NovaDE wird sicherstellen, dass `xdg-open` und andere `xdg-*` Werkzeuge korrekt innerhalb einer NovaDE-Sitzung funktionieren.

**4. UI-Toolkit-Kompatibilität**
    4.1. **GTK (GIMP Toolkit):**
        4.1.1. Primäres Toolkit: NovaDE's eigenes `libnova-ui` basiert auf GTK4 (siehe SYSARCH-001).
        4.1.2. GTK2/GTK3-Anwendungen: Werden nativ unter Wayland (mit GTK3+) oder via XWayland (GTK2) unterstützt. NovaDE wird sicherstellen, dass Theming und Einstellungen (via `nova-settingsd` und GSettings) so weit wie möglich auf GTK2/3-Anwendungen angewendet werden.
    4.2. **Qt (KDE Frameworks):**
        4.2.1. Qt5/Qt6-Anwendungen: Werden nativ unter Wayland unterstützt. NovaDE wird sicherstellen, dass Qt-Anwendungen das Plattformthema (via `QGtkTheme` oder einer spezifischen NovaDE-Plattformtheme-Implementierung für Qt) respektieren, um eine visuelle Integration zu erreichen. Einstellungen für Schriftarten, Icons etc. sollten übernommen werden.
    4.3. **Andere Toolkits (z.B. EFL, Tk, wxWidgets):**
        4.3.1. Unterstützung primär über XWayland, es sei denn, die Toolkits haben eigene, funktionierende Wayland-Backends.
        4.3.2. Grundlegende Fenstermanagement-Funktionen werden durch `nova-wm` bereitgestellt. Visuelle Integration kann variieren.

**5. Anwendungs-Paketformate**
    5.1. **Native Pakete:** Primäre Unterstützung für Anwendungen, die über das Paketmanagement der Distribution installiert werden (z.B. `.deb` für Debian/Ubuntu, `.rpm` für Fedora/openSUSE, Arch-Pakete für Arch/Manjaro). NovaDE wird sich nahtlos in die von der Distribution bereitgestellten Mechanismen integrieren.
    5.2. **Flatpak:**
        5.2.1. NovaDE wird Flatpak als primäres Format für containerisierte Anwendungen vollständig unterstützen. Dies beinhaltet:
            - Eine robuste `xdg-desktop-portal-nova` Implementierung (oder die Nutzung von `xdg-desktop-portal-gtk`, falls vollständig kompatibel und ausreichend) für den Zugriff auf Host-Ressourcen aus der Sandbox heraus (Dateiauswahl, Öffnen von URIs, Drucken etc.).
            - Vollständige Integration von Flatpak-Anwendungen in `nova-launcher` und `nova-appmenu`, einschließlich Icon-Anzeige und korrekter Ausführung.
            - Mechanismen zur Anwendung des NovaDE-Systemthemas auf Flatpak-Anwendungen (über GTK-Themen-Flatpaks und entsprechende Konfiguration).
            - `nova-settingsd` könnte Schnittstellen zur Verwaltung von Flatpak-Berechtigungen anbieten.
    5.3. **Snap:**
        5.3.1. Grundlegende Unterstützung für Snap-Anwendungen wird angestrebt. Die Integrationstiefe hängt von der Kompatibilität von Snap mit Wayland und `xdg-desktop-portal` ab.
        5.3.2. `nova-launcher` wird versuchen, Snap-Anwendungen zu erkennen und zu starten. Visuelle Integration und Portal-Nutzung können Einschränkungen unterliegen, bis die Snap-Wayland-Integration vollständiger ist.
    5.4. **AppImage:**
        5.4.1. AppImages, die ihre Abhängigkeiten bündeln, sollten auf NovaDE lauffähig sein.
        5.4.2. Für die Integration in `nova-launcher` und `nova-appmenu` (d.h. Anzeige im Menü) sind möglicherweise manuelle Erstellung von `.desktop`-Dateien durch den Benutzer oder die Verwendung von optionalen Werkzeugen wie `appimaged` (das `.desktop`-Dateien systemweit integriert) notwendig. NovaDE selbst wird keine Kernkomponente zur automatischen Integration von nicht-installierten AppImages bereitstellen, aber deren Ausführung nicht behindern.

**6. Barrierefreiheit (Accessibility - ATK/AT-SPI)**
    6.1. **ATK (Accessibility Toolkit):**
        6.1.1. Da `libnova-ui` auf GTK4 basiert, werden NovaDE-eigene Anwendungen und die NovaShell die ATK-Schnittstellen von GTK implementieren und exportieren. Dies ermöglicht Barrierefreiheitswerkzeugen den Zugriff auf Widget-Informationen.
    6.2. **AT-SPI (Assistive Technology Service Provider Interface):**
        6.2.1. NovaDE wird den `at-spi2-core` D-Bus-Dienst und die zugehörigen Bibliotheken (`at-spi2-atk`, `libatk-bridge`) integrieren. Dies stellt die Brücke zwischen Anwendungen (die ATK implementieren) und assistiven Technologien (wie Bildschirmlesern oder Lupen) her.
        6.2.2. Der Wayland-Compositor (`nova-wm`) muss so gestaltet sein, dass er die notwendigen Informationen für AT-SPI-Dienste bereitstellt oder deren Funktion nicht behindert. Dies kann die Weiterleitung bestimmter globaler Tastenkombinationen (nach expliziter Konfiguration) an AT-Dienste oder die Bereitstellung von Informationen über Fensterpositionen und -zustände umfassen.
        6.2.3. `nova-settingsd` wird eine Schnittstelle zur Aktivierung und Konfiguration von Barrierefreiheitsfunktionen bereitstellen (z.B. Aktivierung des Bildschirmlesers, Einstellung von Kontrastthemen, Schriftgrößen).
    6.3. **Unterstützte Assistive Technologien:**
        6.3.1. Ziel ist die Kompatibilität mit gängigen Open-Source-Assistiven-Technologien wie Orca (Bildschirmleser), KMag (Bildschirmlupe) und On-Screen-Tastaturen.

**7. Testen und Validierung**
    7.1. Es wird eine kuratierte Liste von populären und kritischen Linux-Anwendungen (sowohl X11 als auch Wayland-nativ, basierend auf GTK, Qt und ggf. anderen Toolkits) definiert. Diese Anwendungen werden regelmäßig auf NovaDE getestet, um Kompatibilitätsprobleme frühzeitig zu identifizieren.
        7.1.1. Beispiele: Firefox, Chrome/Chromium, LibreOffice, GIMP, Inkscape, Blender, VLC, OBS Studio, Steam (und ausgewählte Spiele), wichtige Entwicklerwerkzeuge (VS Code, IntelliJ IDEA-basierte IDEs).
    7.2. Community-Feedback wird aktiv gesammelt und zur Identifizierung und Priorisierung von Kompatibilitätsproblemen genutzt. Ein spezieller Bug-Tracker oder eine Kategorie für Kompatibilitätsprobleme wird eingerichtet.
    7.3. Automatisierte Tests werden, wo möglich, für grundlegende Kompatibilitätsaspekte (z.B. Starten von Anwendungen, Fensterinteraktionen) in Betracht gezogen.
    7.4. Regelmäßige Tests auf verschiedenen Hardware-Profilen (siehe SYSARCH-003) und mit unterschiedlichen Grafiktreibern (Mesa für Intel/AMD, NVIDIA proprietär) sind notwendig.
