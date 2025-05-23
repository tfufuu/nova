**Dokument-ID:** SYSARCH-004
**Titel:** NovaDE Systemarchitektur - Sicherheitsarchitektur

**Inhalt:**

**1. Einleitung**
    1.1. Zweck: Dieses Dokument beschreibt die Sicherheitsarchitektur des Nova Desktop Environment (NovaDE). Es legt die grundlegenden Prinzipien, Mechanismen und Technologien fest, die zum Schutz des Systems und der Benutzerdaten eingesetzt werden.
    1.2. Geltungsbereich: Umfasst Authentifizierung, Autorisierung, Prozessisolation, sichere Interprozesskommunikation, Datensicherheit und Software-Updates.
    1.3. Bedrohungsmodell: Es wird von Standardbedrohungen für Desktop-Systeme ausgegangen (Malware, unautorisierter Zugriff, Datenlecks, Ausnutzung von Software-Schwachstellen).

**2. Sicherheitsprinzipien**
    2.1. **Least Privilege (Prinzip der geringsten Rechte):** Prozesse und Komponenten laufen mit den minimal notwendigen Rechten.
    2.2. **Defense in Depth (Mehrschichtige Verteidigung):** Mehrere Sicherheitsebenen schützen das System, sodass der Ausfall einer einzelnen Komponente nicht das gesamte System kompromittiert.
    2.3. **Secure by Default (Sicherheitsstandardkonfiguration):** Standardeinstellungen sind auf maximale Sicherheit ausgelegt, ohne die Benutzerfreundlichkeit unzumutbar einzuschränken.
    2.4. **Fail-Safe Defaults (Ausfallsicherheit):** Bei Fehlern gehen Komponenten in einen sicheren Zustand über.
    2.5. **Separation of Concerns (Trennung der Belange):** Sicherheitskritische Funktionen sind von anderen Funktionen isoliert.
    2.6. **Openness (Transparenz):** Verwendung von etablierten, offenen Standards und Protokollen, wo immer möglich. Quelloffenheit der NovaDE-Komponenten ermöglicht Audits.

**3. Authentifizierung**
    3.1. **Benutzer-Login:**
        3.1.1. Mechanismus: PAM (Pluggable Authentication Modules) wird als primäres Framework für die Benutzerauthentifizierung beim Login verwendet.
        3.1.2. Module: Konfigurierbare PAM-Stacks, die Module wie `pam_unix` (für lokale Passwortprüfung), `pam_systemd` (für Session-Management mit systemd-logind), und optional `pam_fprintd` (für Fingerabdruckleser) oder `pam_u2f` (für 2FA-Hardware-Token) unterstützen. `pam_pwquality.so` / `pam_cracklib.so` dienen zur Durchsetzung von Passwortkomplexitätsregeln.
        3.1.3. Login-Manager: `nova-session` interagiert mit PAM und `systemd-logind` zur Durchführung des Login-Prozesses und startet die NovaShell nach erfolgreicher Authentifizierung.
    3.2. **Bildschirmsperre:**
        3.2.1. Mechanismus: `nova-screensaver` (Teil der NovaShell) initiiert die Sperre. Die Authentifizierung erfolgt ebenfalls über PAM, typischerweise mit einem vereinfachten Stack.
    3.3. **Passwort-Management:**
        3.3.1. Speicherung: Lokale Passwörter werden mittels `SHA-512` (oder einem stärkeren, aktuellen Algorithmus wie `yescrypt`, konfiguriert in `/etc/login.defs` oder PAM) mit pro Benutzer einzigartigem Salt gehasht und in `/etc/shadow` gespeichert. Nur Root hat Lesezugriff.
        3.3.2. Richtlinien: Passwortrichtlinien (Komplexität, Ablauf, Historie) werden über PAM-Module (`pam_pwquality`, `pam_cracklib`) durchgesetzt.

**4. Autorisierung und Rechteverwaltung**
    4.1. **Administrative Rechte (Root-Zugriff):**
        4.1.1. Mechanismus: `polkit` wird als primärer Mechanismus für die Autorisierung von privilegierten Aktionen durch unprivilegierte Desktop-Anwendungen und -Dienste verwendet.
        4.1.2. `pkexec`: Wird für das Ausführen von Befehlen als anderer Benutzer (typischerweise root) nach erfolgreicher Autorisierung durch polkit verwendet. `sudo` bleibt als CLI-Werkzeug verfügbar.
        4.1.3. Richtlinien: Polkit-Richtliniendateien (`.policy`) definieren, welche Aktionen von welchen Benutzern (oder Gruppen) unter welchen Bedingungen (z.B. aktive lokale Sitzung) ausgeführt werden dürfen. NovaDE-spezifische Dienste (`nova-powerd`, `nova-netd`) liefern eigene polkit-Richtlinien für ihre privilegierten Operationen.
    4.2. **Dateisystemberechtigungen:**
        4.2.1. Standard-POSIX-Berechtigungen (Benutzer, Gruppe, Andere) bilden die Grundlage.
        4.2.2. Access Control Lists (ACLs): Können optional für feinere granulare Kontrolle verwendet werden (Unterstützung durch Kernel und `nova-settingsd` für UI).
    4.3. **Gerätezugriff:**
        4.3.1. `udev` verwaltet Gerätedateien und deren Berechtigungen.
        4.3.2. Technologien:
            - `udev` verwaltet Gerätedateien und deren Berechtigungen.
            - `systemd-logind` verwaltet den Zugriff auf Geräte basierend auf der aktiven Benutzersitzung (z.B. direkter Zugriff auf Soundkarten, Grafikkarten). Polkit kann für administrative Aktionen an Geräten verwendet werden.

**5. Prozessisolation und Sandboxing**
    5.1. **Systemdienste:** Jeder NovaDE-Systemdienst (`nova-*d`) läuft als eigener, unprivilegierter Benutzer (wo möglich) und nutzt systemd-Unit-Direktiven zur Härtung (z.B. `ProtectSystem=strict`, `PrivateTmp=yes`, `CapabilityBoundingSet=~CAP_SYS_ADMIN`).
        5.1.1. Prinzip: Jeder NovaDE-Systemdienst (gekennzeichnet durch das Suffix `-d`, z.B. `nova-powerd`, `nova-settingsd`) läuft als eigener, dedizierter und unprivilegierter Benutzer, wo immer dies technisch möglich ist.
        5.1.2. Technologien: systemd-Unit-Dateien werden verwendet, um die Härtung dieser Dienste zu maximieren. Dies beinhaltet Direktiven wie:
            - `User=` und `Group=`: Startet den Dienst unter einem spezifischen, nicht-privilegierten Benutzer/Gruppe.
            - `ProtectSystem=strict`: Macht das Dateisystem unter `/usr`, `/boot` und `/etc` schreibgeschützt.
            - `ProtectHome=read-only` oder `ProtectHome=yes`: Macht Benutzer-Home-Verzeichnisse schreibgeschützt oder unzugänglich.
            - `PrivateTmp=yes`: Stellt einen privaten temporären Ordner für den Dienst bereit.
            - `PrivateDevices=yes`: Verhindert den Zugriff auf physische Geräte.
            - `CapabilityBoundingSet=~CAP_SYS_ADMIN ...`: Entfernt nicht benötigte Linux Capabilities, insbesondere administrative wie `CAP_SYS_ADMIN`.
            - `NoNewPrivileges=yes`: Verhindert, dass der Dienst oder seine Kindprozesse ihre Privilegien erhöhen können (z.B. via `setuid` Binaries).
            - `SystemCallFilter=~@clock @debug ...`: Beschränkt die erlaubten Systemaufrufe auf ein Minimum.
            - `AppArmorProfile=` oder `SELinuxContext=`: Integration mit Linux Security Modules (LSMs) für Mandatory Access Control (MAC), falls auf dem System konfiguriert.
    5.2. **Anwendungs-Sandboxing (Zukunftsperspektive/Integration):**
        5.2.1. Ziel: Unterstützung für containerisierte Anwendungsformate wie Flatpak und Snap.
        5.2.2. `xdg-desktop-portal`: NovaDE wird eine Implementierung von `xdg-desktop-portal` bereitstellen oder eine bestehende nutzen, um Sandboxed-Anwendungen kontrollierten Zugriff auf Ressourcen außerhalb ihrer Sandbox zu ermöglichen (z.B. Datei öffnen Dialog, Benachrichtigungen).
        5.2.3. `nova-launcher` wird .desktop-Dateien von Flatpak/Snap-Anwendungen korrekt interpretieren und starten.
    5.3. **Wayland-Architektur als Sicherheitsmerkmal:** Wayland bietet inhärent bessere Isolation zwischen Anwendungen als X11, da Anwendungen nicht den Input oder Output anderer Anwendungen direkt abfangen können. Der Compositor (`nova-wm`) ist hier die zentrale Kontrollinstanz.
        5.3.1. Prinzip: Die Wayland-Architektur bietet von Grund auf eine verbesserte Isolation zwischen Anwendungen im Vergleich zu X11.
        5.3.2. Details:
            - Kein direkter Zugriff auf Eingabeereignisse: Anwendungen können nicht global Tastatur- oder Mausereignisse anderer Anwendungen abfangen (Keylogging, Mouse-Spoofing). Der Compositor (`nova-wm`) ist die einzige Komponente, die rohe Eingabeereignisse empfängt und sie an die fokussierte Anwendung weiterleitet.
            - Kein direkter Zugriff auf Fensterinhalte: Anwendungen können nicht den Inhalt der Fenster anderer Anwendungen auslesen oder verändern (Screen Scraping, Injection von Inhalten). Der Compositor ist verantwortlich für das Rendern aller Fenster.
            - Definierte Protokolle: Die Kommunikation zwischen einer Anwendung (Client) und dem Compositor (`nova-wm`) erfolgt über wohldefinierte Wayland-Protokolle. Der Compositor validiert die Anfragen und kann Zugriffe beschränken.
            - Keine impliziten Privilegien: Im Gegensatz zu X11, wo ein Client oft weitreichende Kontrolle über den X-Server hat, sind die Rechte eines Wayland-Clients stark eingeschränkt.

**6. Sichere Interprozesskommunikation (IPC)**
    6.1. **D-Bus:**
        6.1.1. Nachrichten-Bus-Sicherheit: D-Bus-Daemon erzwingt Richtlinien (definiert in XML-Konfigurationsdateien), welche Peers welche Methoden auf welchen Interfaces aufrufen dürfen.
        6.1.2. System Bus vs. Session Bus: Klare Trennung. System Bus für systemweite Dienste, Session Bus für benutzerspezifische Anwendungen und Dienste.
    6.2. **Wayland-Protokolle:** Kommunikation zwischen Clients und `nova-wm` erfolgt über Unix Domain Sockets. Der Compositor validiert Client-Anfragen.

**7. Datensicherheit**
    7.1. **Home-Verzeichnis-Verschlüsselung (Optional):**
        7.1.1. Empfehlung und optionale Unterstützung für LUKS-basierte Vollverschlüsselung der Partition oder eCryptfs/fscrypt für Home-Verzeichnis-Verschlüsselung. Integration in den Installer und `nova-session`.
    7.2. **Temporäre Dateien:** Verwendung von `PrivateTmp=yes` für systemd-Dienste. Anwendungen sollten temporäre Dateien sicher erstellen (z.B. `mkstemp`).
    7.3. **Zwischenablage:** `nova-clipboardd` sollte Mechanismen zur Vermeidung von exzessivem Auslesen der Zwischenablage durch Hintergrundanwendungen prüfen (ggf. Benachrichtigung des Benutzers).
    7.4. **Schlüsselverwaltung:** Verwendung von `gnome-keyring` oder einem äquivalenten, etablierten Dienst zur sicheren Speicherung von Passwörtern und Geheimnissen für Anwendungen. Integration mit `libsecret`.

**8. Software-Updates und Integrität**
    8.1. **Paketmanagement:** Verlässt sich auf das Paketmanagementsystem der Distribution (z.B. pacman mit Signaturprüfung für Manjaro).
    8.2. **NovaDE-Komponenten-Updates:** Werden über das Distributions-Paketmanagement ausgeliefert.
    8.3. **Firmware-Updates:** Integration mit `fwupd` für sichere Firmware-Updates (LVFS). `nova-settingsd` könnte eine UI dafür bereitstellen.

**9. Sicherheitsaudits und Entwicklungspraktiken**
    9.1. Regelmäßige Code-Reviews mit Fokus auf Sicherheit.
    9.2. Einsatz von statischen Analysewerkzeugen (z.B. cppcheck, SonarQube) und dynamischen Analysewerkzeugen (Valgrind, Sanitizer: ASan, UBSan, TSan).
    9.3. Zeitnahe Reaktion auf gemeldete Sicherheitsschwachstellen.
    9.4. Vermeidung von unsicheren Funktionen (z.B. `strcpy`, `sprintf` -> `strncpy`, `snprintf`).
    9.5. **Dokumentation von Sicherheitsaspekten:** Sicherheitsrelevante Designentscheidungen und Mechanismen werden in der Entwicklerdokumentation festgehalten.

**6. Sichere Interprozesskommunikation (IPC)**

**7. Datensicherheit**

**8. Software-Updates und Integrität**

**9. Sicherheitsaudits und Entwicklungspraktiken**
