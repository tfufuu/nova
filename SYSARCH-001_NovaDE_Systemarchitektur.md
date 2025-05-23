**Dokument-ID:** SYSARCH-001
**Titel:** NovaDE Systemarchitektur - Schichtenmodell

**Inhalt:**

**1. Einleitung**
    1.1. Zweck: Dieses Dokument definiert die hierarchische Schichtenarchitektur des Nova Desktop Environment (NovaDE).
    1.2. Geltungsbereich: Die Spezifikation umfasst alle Kernkomponenten und deren Interaktionen von der Hardware-Abstraktionsebene bis zur grafischen Benutzeroberfläche.

**2. Architekturprinzipien**
    2.1. **Modularität:** Jede Schicht ist in unabhängige Module unterteilt.
    2.2. **Lose Kopplung:** Schnittstellen zwischen den Schichten sind klar definiert, um Abhängigkeiten zu minimieren.
    2.3. **Wiederverwendbarkeit:** Komponenten sollen, wo möglich, wiederverwendbar sein.
    2.4. **Sicherheit:** Sicherheitsaspekte sind in jeder Schicht integriert.

**3. Schichtenarchitektur Übersicht**

```ascii
+-----------------------------------------------------+
| Schicht 4: User Interface Shell - NovaShell         |
| (nova-panel, nova-appmenu, nova-desktop, etc.)      |
+-----------------------------------------------------+
      ^                                 |
      | (Wayland, D-Bus, libnova-ui)    | (UI-Elemente, Desktop-Interaktion)
      v                                 |
+-----------------------------------------------------+
| Schicht 3: Application Framework & Libs - NovaKit   |
| (libnova-ui (GTK4), libnova-core, etc.)             |
+-----------------------------------------------------+
      ^                                 |
      | (D-Bus, Wayland, C APIs)        | (Anwendungs-APIs, UI-Toolkit)
      v                                 |
+-----------------------------------------------------+
| Schicht 2: Core Desktop Services - NovaCore         |
| (nova-wm, nova-session, nova-launcher, etc.)        |
+-----------------------------------------------------+
      ^                                 |
      | (Wayland, D-Bus, X11)           | (Fenster, Sitzungen, Benachrichtigungen)
      v                                 |
+-----------------------------------------------------+
| Schicht 1: Hardware Abstraction Layer - NovaHAL     |
| (nova-displayd, nova-inputd, nova-powerd, etc.)   |
+-----------------------------------------------------+
      ^                                 |
      | (D-Bus, Kernel-APIs)            | (Hardware-Abstraktion)
      v                                 |
+-----------------------------------------------------+
| Schicht 0: Kernel-Interface & Systemdienste         |
| (libsystemd, libudev, Kernel-Treiber-Interfaces)    |
+-----------------------------------------------------+
      ^                                 |
      | (Syscalls, Netlink, /dev)       | (Direkte Kernel-Interaktion)
      v                                 |
+-----------------------------------------------------+
|                  Linux Kernel                         |
+-----------------------------------------------------+
|                    Hardware                           |
+-----------------------------------------------------+
```

**4. Detaillierte Schichtdefinitionen**

    4.1. **Schicht 0: Kernel-Interface & Systemdienste**
        4.1.1. **Verantwortlichkeiten:**
            - Direkte Interaktion mit dem Linux-Kernel (syscalls, netlink, udev).
            - Hardware-Ereignisbehandlung (via udev).
            - Low-Level Systemdienste (z.B. dbus-daemon, systemd-logind).
        4.1.2. **Kernkomponenten:**
            - `libsystemd`: Client-Bibliothek für systemd Interaktion.
            - `libudev`: Client-Bibliothek für udev Interaktion.
            - Kernel-spezifische Treiber-Schnittstellen (DRM/KMS, evdev).
        4.1.3. **Schnittstellen:**
            - POSIX System Calls.
            - Netlink Sockets.
            - `/dev` Gerätedateien.
            - D-Bus System Bus.

    4.2. **Schicht 1: Hardware Abstraction Layer (HAL) - NovaHAL**
        4.2.1. **Verantwortlichkeiten:**
            - Abstraktion der Hardware-spezifischen Details für höhere Schichten.
            - Bereitstellung einheitlicher APIs für Display-Management, Input-Geräte, Power-Management, Netzwerk-Konfiguration und Audio-Hardware.
        4.2.2. **Kernkomponenten:**
            - `nova-displayd`: Display-Konfiguration und -Management (Auflösung, Multi-Monitor, Helligkeit). Basiert auf DRM/KMS.
            - `nova-inputd`: Verarbeitung von Eingabegeräten (Tastatur, Maus, Touchpad, Touchscreen). Basiert auf libinput.
            - `nova-powerd`: Energieverwaltung (Batteriestatus, Suspend/Resume, CPU-Frequenz-Skalierung). Interagiert mit systemd-logind.
            - `nova-netd`: Netzwerkverbindungsmanagement (Ethernet, WLAN, WWAN). Nutzt NetworkManager oder systemd-networkd Backend.
            - `nova-audiod`: Audio-Hardware-Management und Mixer-Steuerung. Nutzt ALSA als Backend, stellt PulseAudio/PipeWire-kompatible Schnittstellen bereit.
        4.2.3. **Schnittstellen:**
            - D-Bus APIs (z.B. `org.novade.Display`, `org.novade.Input`).
            - Intern: Plugin-Schnittstellen für Hardware-spezifische Backends.

    4.3. **Schicht 2: Core Desktop Services - NovaCore**
        4.3.1. **Verantwortlichkeiten:**
            - Grundlegende Desktop-Dienste, die von Anwendungen und der UI-Shell benötigt werden.
            - Window-Management, Compositing, Session-Management, Anwendungsstart, Benachrichtigungen, Zwischenablage.
        4.3.2. **Kernkomponenten:**
            - `nova-wm`: Wayland Compositor und XWayland-Server. Verantwortlich für Fensterstapelung, Eingabe-Routing, Compositing-Effekte. Implementiert Wayland-Protokolle (wl_shell, xdg_shell etc.).
            - `nova-session`: Sitzungsverwaltung (Login, Logout, Sperrbildschirm). Interagiert mit systemd-logind und PAM.
            - `nova-launcher`: Anwendungsstarter und -Management. Liest .desktop-Dateien.
            - `nova-notificationd`: Dienst für Desktop-Benachrichtigungen (gemäß freedesktop.org Spezifikation).
            - `nova-clipboardd`: Zwischenablage-Management.
            - `nova-settingsd`: Zentraler Dienst für Desktop-Einstellungen (Theming, Schriftarten, Barrierefreiheit). Speichert Konfigurationen via GSettings/dconf.
        4.3.3. **Schnittstellen:**
            - Wayland Protokolle für Window-Management.
            - D-Bus APIs (z.B. `org.novade.Session`, `org.freedesktop.Notifications`).
            - X11 Protokoll (via XWayland für Kompatibilität).

    4.4. **Schicht 3: Application Framework & Libraries - NovaKit**
        4.4.1. **Verantwortlichkeiten:**
            - Bereitstellung von Bibliotheken und Werkzeugen für die Anwendungsentwicklung auf NovaDE.
            - UI-Toolkit, Datenzugriffsbibliotheken, Multimedia-Framework.
        4.4.2. **Kernkomponenten:**
            - `libnova-ui`: Primäres UI-Toolkit für NovaDE-Anwendungen (basierend auf GTK4 für engere GNOME-Ökosystem-Integration und C-basierte API für leichtere Bindings). Bietet NovaDE-spezifische Widgets und Theming-Integration.
            - `libnova-core`: Kernbibliotheken für allgemeine Aufgaben (Dateisystem-Utilities, MIME-Type-Handling, Internationalisierung).
            - `libnova-multimedia`: Multimedia-Framework für Audio/Video-Wiedergabe und -Aufnahme (potenziell GStreamer-basiert).
            - `libnova-accounts`: Bibliothek für Online-Account-Integration.
        4.4.3. **Schnittstellen:**
            - C APIs für alle Bibliotheken.
            - GObject Introspection für Sprach-Bindings.

    4.5. **Schicht 4: User Interface Shell - NovaShell**
        4.5.1. **Verantwortlichkeiten:**
            - Bereitstellung der primären grafischen Benutzeroberfläche des Desktops.
            - Desktop-Hintergrund, Panels/Docks, Anwendungsmenü, System-Tray, Task-Switcher, Lock-Screen-UI.
        4.5.2. **Kernkomponenten:**
            - `nova-panel`: Hauptpanel (oder mehrere Panels) für Anwendungsstarter, Taskleiste, Systemindikatoren.
            - `nova-appmenu`: Vollbild- oder Menü-basierter Anwendungs-Launcher.
            - `nova-desktop`: Verwaltung des Desktophintergrunds und ggf. Desktop-Icons.
            - `nova-screensaver`: Bildschirmschoner und Lock-Screen-Benutzeroberfläche.
            - `nova-osd`: On-Screen-Displays für Lautstärke, Helligkeit etc.
        4.5.3. **Schnittstellen:**
            - Interagiert mit `nova-wm` über Wayland-Protokolle (insbesondere private Protokolle für Shell-spezifische Funktionen).
            - Nutzt `libnova-ui` für die UI-Darstellung.
            - Interagiert mit `nova-settingsd` für Konfigurationen.

**5. Schichtübergreifende Aspekte**
    5.1. **Interprozesskommunikation (IPC):** Primär D-Bus für Dienste, Wayland-Protokolle für Grafik/Input.
    5.2. **Konfigurationsmanagement:** GSettings/dconf für Systemeinstellungen, anwendungsspezifische Konfigurationsdateien.
    5.3. **Theming:** Zentrales Theming-System, das GTK-Themen, Icon-Themen und Cursor-Themen unterstützt. `nova-settingsd` verwaltet die aktuellen Themen.
