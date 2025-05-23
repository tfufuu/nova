**Dokument-ID:** SYSARCH-002
**Titel:** NovaDE Systemarchitektur - Technologie-Stack und Abhängigkeiten

**Inhalt:**

**1. Einleitung**
    1.1. Zweck: Dieses Dokument definiert den vollständigen Technologie-Stack für das Nova Desktop Environment (NovaDE), einschließlich spezifischer Softwarekomponenten, deren Versionen und Abhängigkeiten.
    1.2. Geltungsbereich: Die Spezifikation umfasst Kernsystembibliotheken, Programmiersprachen, Build-Systeme und Laufzeitumgebungen.
    1.3. Ziel-Distribution: Manjaro Linux (als primäre Referenz), mit dem Ziel der Portierbarkeit auf andere Arch-basierte und Debian-basierte Systeme.

**2. Programmiersprachen**
    2.1. **Primäre Systemsprache: C**
        2.1.1. Begründung: Performance, Systemnähe, breite Verfügbarkeit von Bibliotheken, GTK-Integration.
        2.1.2. Standard: C17 (ISO/IEC 9899:2018).
        2.1.3. Compiler: GCC (>= 11.x.x), Clang (>= 13.x.x).
    2.2. **Skriptsprache für Build-System/Tools: Python**
        2.2.1. Begründung: Weite Verbreitung, einfache Syntax, gute Bibliotheksunterstützung.
        2.2.2. Version: Python 3.9+.
    2.3. **Shell-Skripte: Bash**
        2.3.1. Begründung: Standard für Systemverwaltungsaufgaben.
        2.3.2. Standard: Bash 5.0+.

**3. Kernbibliotheken und Frameworks**
    3.1. **Windowing System & Compositing:**
        3.1.1. Protokoll: Wayland 1.20+
            - Begründung: Moderner Display-Server-Protokoll-Standard, sicherheitsorientiert und effizient.
        3.1.2. Compositor-Bibliothek: wlroots 0.15+ (für `nova-wm`)
            - Begründung: Modulare, leichtgewichtige und erweiterbare Bibliothek für die Entwicklung von Wayland-Compositoren. Bietet eine gute Basis für `nova-wm`.
        3.1.3. X11 Kompatibilität: XWayland 22.1+
            - Begründung: Gewährleistet die Lauffähigkeit von älteren X11-Anwendungen unter der Wayland-basierten NovaDE-Sitzung.
    3.2. **UI Toolkit (für NovaKit und NovaShell):**
        3.2.1. Bibliothek: GTK 4.8+
        3.2.2. Begründung: Moderne Features (GPU-Beschleunigung, verbesserte Layout-Manager), hervorragende Wayland-Unterstützung, aktive Entwicklungsgemeinschaft, C-basiert (passt gut zur primären Systemsprache C und erleichtert Bindings).
    3.3. **Systemdienste-Integration:**
        3.3.1. Bibliothek: libsystemd 250+
        3.3.2. Begründung: Standard-Schnittstelle für die Interaktion mit systemd, dem Init-System und Service-Manager der meisten modernen Linux-Distributionen, einschließlich Manjaro.
    3.4. **Hardware-Interaktion (NovaHAL):**
        3.4.1. Display: DRM/KMS (Kernel-API)
            - Begründung: Direkte und effiziente Methode zur Steuerung von Grafikkarten und Display-Modi, fundamental für Wayland-Compositoren.
        3.4.2. Input: libinput 1.19+
            - Begründung: Standardbibliothek für die Verarbeitung von Eingabegeräten unter Wayland, bietet einheitliche Behandlung verschiedener Hardware.
        3.4.3. Netzwerk: NetworkManager 1.30+ (primär), systemd-networkd 250+ (alternativ/fallback)
            - Begründung für NetworkManager: Weit verbreitet, benutzerfreundlich, bietet umfassende Funktionen für Desktop-Umgebungen. systemd-networkd als schlankere Alternative für spezifische Setups.
        3.4.4. Audio: ALSA lib 1.2.5+ (als Basis-API), PipeWire 0.3.40+ (als primärer Audio-Server)
            - Begründung für PipeWire: Vereinheitlicht Audio- und Video-Streaming unter Linux, bietet geringe Latenz, gute Kompatibilität mit PulseAudio-, JACK- und ALSA-Anwendungen, sowie verbesserte Sicherheit und Flatpak-Integration.
    3.5. **Interprozesskommunikation:**
        3.5.1. D-Bus: libdbus-1 1.12.20+ (oder GDBus aus GLib für einfachere Integration)
            - Begründung: Standard-IPC-Mechanismus auf dem Linux-Desktop, notwendig für die Kommunikation zwischen Desktop-Diensten und Anwendungen. GDBus (Teil von GLib) wird bevorzugt für neue C-Komponenten.
        3.5.2. GLib: 2.70+ (als Kern-Utility-Bibliothek)
            - Begründung: Fundamentale Bibliothek für GTK-basierte Entwicklung. Bietet Datenstrukturen, Hauptschleife, Threading-Abstraktionen, GObject-System für OOP in C, GIO für asynchrone E/A und Netzwerkoperationen, sowie GSettings für Konfigurationsmanagement.
    3.6. **Multimedia (für libnova-multimedia):**
        3.6.1. Framework: GStreamer 1.20+
        3.6.2. Begründung: Mächtiges und flexibles Multimedia-Framework, plattformübergreifend, unterstützt eine breite Palette von Formaten und Protokollen.
    3.7. **Sicherheits-Frameworks:**
        3.7.1. Authentifizierung: PAM (Pluggable Authentication Modules) - Standard Systemversion (z.B. Linux PAM 1.5.x)
            - Begründung: Standard-Framework für Authentifizierungsaufgaben unter Linux, ermöglicht flexible Konfiguration von Login-Prozessen.
        3.7.2. Policy Management: Polkit 0.120+
            - Begründung: Standard-Framework zur Verwaltung von Berechtigungen für Systemaktionen durch unprivilegierte Prozesse.
    3.8. **Schriftrendering:**
        3.8.1. FreeType 2.11+
            - Begründung: Hochwertige und portable Schrift-Rendering-Bibliothek.
        3.8.2. Fontconfig 2.13.94+
            - Begründung: Bibliothek zur Konfiguration und Anpassung des Zugriffs auf Schriften.
        3.8.3. HarfBuzz 4.0+ (für komplexe Textlayouts)
            - Begründung: Text-Shaping-Engine, die komplexe Schriftzeichen und Layouts (z.B. für Arabisch, Indisch) korrekt darstellt.
    3.9. **Grafik-Stack:**
        3.9.1. Mesa 22.0+ (für OpenGL und Vulkan Treiber)
            - Begründung: Open-Source-Implementierung der OpenGL-, Vulkan- und anderer Grafik-APIs.
        3.9.2. OpenGL Version: 3.3 Core Profile (Minimum für Compositor und Anwendungen)
            - Begründung: Weit unterstützter Standard, der ausreichende Funktionen für moderne Desktop-Effekte und Anwendungen bietet.
        3.9.3. Vulkan Version: 1.2 (optional, für High-Performance Grafik-Anwendungen)
            - Begründung: Moderne Low-Level-Grafik-API für anspruchsvolle 3D-Anwendungen und Spiele.

**4. Build-System und Werkzeuge**
    4.1. **Build-System: Meson**
        4.1.1. Version: 0.60+
        4.1.2. Backend: Ninja 1.10+
        4.1.3. Begründung: Schnell, modern, benutzerfreundlich, speziell für C-Projekte entwickelt, exzellente Integration mit GTK und GLib, vereinfacht Abhängigkeitsmanagement und Cross-Kompilierung.
    4.2. **Versionskontrolle: Git**
        4.2.1. Version: 2.35+
        4.2.2. Begründung: De-facto-Standard für Versionskontrolle, verteilt, leistungsstark und flexibel.
    4.3. **Dokumentationsgenerator: Doxygen**
        4.3.1. Version: 1.9.1+ (für C-Code Dokumentation)
        4.3.2. Begründung: Weit verbreitetes Werkzeug zur Erstellung von Dokumentation aus kommentiertem Quellcode.
    4.4. **Code-Analyse und Formatierung:**
        4.4.1. Linter: `cppcheck` 2.7+ (für statische C-Code-Analyse)
            - Begründung: Findet potenzielle Fehler und stilistische Probleme im C-Code.
        4.4.2. Formatter: `clang-format` 13.0+
            - Begründung: Stellt einen einheitlichen Code-Stil im gesamten Projekt sicher.

**5. Abhängigkeitsmanagement**
    5.1. **Systemabhängigkeiten:**
        5.1.1. Methode: Verwaltung über das Paketmanagement der Ziel-Distribution (z.B. `pacman` für Manjaro/Arch, `apt` für Debian/Ubuntu).
        5.1.2. Begründung: Standardverfahren unter Linux, nutzt die Stabilität und Sicherheitsupdates der Distributionen.
    5.2. **Build-Abhängigkeiten:**
        5.2.1. Deklaration: Explizit in `meson.build`-Dateien über `dependency()` Funktion.
        5.2.2. Begründung: Ermöglicht Meson, das Vorhandensein und die korrekte Version von Build-Abhängigkeiten zu überprüfen und erleichtert die Fehlersuche.
    5.3. **Subprojekte (Bundling):**
        5.3.1. Methode: Meson-Subprojekte (`subproject()`) oder Git-Submodule.
        5.3.2. Anwendung: Sparsam verwenden, nur für kleine, spezifische Bibliotheken, die nicht allgemein über Distributionen verfügbar sind oder eine bestimmte Version erfordern, die mit der Systemversion konfligieren könnte.
        5.3.3. Begründung: Erhöht die Portabilität in einigen Fällen, kann aber das Build-System komplexer machen und zu größeren Build-Zeiten führen. Vorrang hat die Nutzung von Systembibliotheken.
    5.4. **Mindestversionen:** Alle oben genannten Bibliotheksversionen sind Mindestversionen. Neuere Versionen werden bevorzugt, solange die API-Kompatibilität gewährleistet ist.

**6. Laufzeitumgebung**
    6.1. **Linux Kernel:**
        6.1.1. Version: 5.15 LTS oder neuer.
        6.1.2. Begründung: Bietet notwendige Features für moderne Hardware-Unterstützung (DRM/KMS, libinput), Wayland und Sicherheitsfunktionen. LTS-Versionen werden für Stabilität bevorzugt.
    6.2. **systemd:**
        6.2.1. Version: 250 oder neuer.
        6.2.2. Begründung: Enge Integration mit Kernkomponenten von NovaDE (z.B. `nova-session`, `nova-powerd`), erfordert moderne systemd-Features.
    6.3. **Wayland Display Server:**
        6.3.1. Anforderung: Erforderlich. `nova-wm` fungiert als Wayland Compositor.
        6.3.2. Begründung: NovaDE ist nativ auf Wayland ausgelegt, um moderne Grafik- und Sicherheitsfeatures zu nutzen.
    6.4. **D-Bus Daemon:**
        6.4.1. Version: Standardversion, die mit `libdbus-1` 1.12.20+ kompatibel ist.
        6.4.2. Begründung: Essentiell für die Interprozesskommunikation zwischen NovaDE-Komponenten und Anwendungen.
    6.5. **Schriftarten:**
        6.5.1. Anforderung: Ein Basissatz von TrueType/OpenType-Schriftarten (z.B. Noto, DejaVu) muss installiert sein.
        6.5.2. Begründung: Notwendig für die korrekte Darstellung der Benutzeroberfläche und von Anwendungsfenstern.

**7. Kompatibilitätsziele**
    7.1. **Freedesktop.org Standards:**
        7.1.1. Ziel: Strikte Einhaltung relevanter Spezifikationen.
        7.1.2. Beispiele: XDG Base Directory Specification, Desktop Entry Specification, Icon Theme Specification, Status Notifier Item Specification, Freedesktop.org Notifications Specification.
        7.1.3. Begründung: Gewährleistet Interoperabilität mit anderen Desktop-Umgebungen und Anwendungen im Linux-Ökosystem.
    7.2. **Anwendungs-Binärkompatibilität (ABI) & API-Stabilität:**
        7.2.1. `libnova-core` und `libnova-ui` (NovaKit): Es wird angestrebt, eine stabile C-API und ABI nach der ersten Hauptversion (1.0) bereitzustellen, um die Entwicklung von Drittanbieter-Anwendungen zu erleichtern. Semantic Versioning wird verwendet.
        7.2.2. Interne Komponenten: Interne D-Bus-Schnittstellen und Bibliotheken können schneller iterieren, solange die externen APIs stabil bleiben.
        7.2.3. Begründung: Fördert die Entwicklung eines robusten Ökosystems um NovaDE.
    7.3. **X11-Anwendungskompatibilität:**
        7.3.1. Methode: Wird durch XWayland sichergestellt.
        7.3.2. Ziel: Nahtlose Ausführung der meisten bestehenden X11-Anwendungen.
        7.3.3. Begründung: Wichtig für die Akzeptanz durch Benutzer, die auf ältere Anwendungen angewiesen sind.
    7.4. **Distribution Portabilität:**
        7.4.1. Primärziel: Manjaro Linux und andere Arch-basierte Distributionen.
        7.4.2. Sekundärziel: Debian-basierte Distributionen (z.B. Ubuntu).
        7.4.3. Methode: Vermeidung von distributionsspezifischen Pfaden oder Konfigurationen wo möglich, Verwendung von Standard-Build-Tools (Meson) und Abhängigkeitsmanagement über das Distributionspaketmanagement.
        7.4.4. Begründung: Erhöht die Reichweite und Nutzerbasis von NovaDE.

**6. Laufzeitumgebung**

**7. Kompatibilitätsziele**
