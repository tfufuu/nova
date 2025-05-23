**Dokument-ID:** SYSARCH-003
**Titel:** NovaDE Systemarchitektur - Performance-Benchmarks und Ressourcen-Constraints

**Inhalt:**

**1. Einleitung**
    1.1. Zweck: Dieses Dokument definiert Performance-Benchmarks und Ressourcen-Constraints für das Nova Desktop Environment (NovaDE) über verschiedene Hardware-Profile hinweg. Ziel ist es, eine reaktionsschnelle und effiziente Benutzererfahrung sicherzustellen.
    1.2. Geltungsbereich: Umfasst Boot-Zeiten, UI-Reaktionsfähigkeit, Speichernutzung, CPU-Last und Grafik-Performance.

**2. Hardware-Profile Definition**
    2.1. **Profil 1: Low-End / Minimal-Spezifikation**
        2.1.1. CPU: Dual-Core @ 1.5 GHz (z.B. Intel Celeron N-Serie, AMD Athlon Silver)
        2.1.2. RAM: 4 GB DDR3/DDR4
        2.1.3. Speicher: HDD (5400 RPM) oder eMMC
        2.1.4. Grafik: Integrierte Grafik (z.B. Intel HD Graphics Gen 7/8, AMD Radeon R3/R4) mit grundlegender OpenGL 3.3 Unterstützung.
        2.1.5. Auflösung: 1366x768
    2.2. **Profil 2: Mid-Range / Empfohlene Spezifikation**
        2.2.1. CPU: Quad-Core @ 2.5 GHz (z.B. Intel Core i5 Gen 8+, AMD Ryzen 5 Serie)
        2.2.2. RAM: 8 GB DDR4
        2.2.3. Speicher: SATA SSD
        2.2.4. Grafik: Integrierte Grafik (z.B. Intel Iris Xe, AMD Radeon Vega 8/RX Vega 10) oder dedizierte Einsteiger-GPU mit guter OpenGL 3.3+ und Vulkan 1.1+ Unterstützung.
        2.2.5. Auflösung: 1920x1080
    2.3. **Profil 3: High-End / Enthusiast-Spezifikation**
        2.3.1. CPU: Hexa/Octa-Core @ 3.5 GHz+ (z.B. Intel Core i7/i9 Gen 10+, AMD Ryzen 7/9 Serie)
        2.3.2. RAM: 16 GB+ DDR4/DDR5
        2.3.3. Speicher: NVMe SSD
        2.3.4. Grafik: Dedizierte Mid-Range bis High-End GPU (z.B. NVIDIA GeForce RTX 3060+, AMD Radeon RX 6700+) mit exzellenter OpenGL und Vulkan Unterstützung.
        2.3.5. Auflösung: 2560x1440 oder 3840x2160

**3. Performance-Benchmarks und Ziele**

    3.1. **Boot-Zeit**
        3.1.1. Metrik: Zeit vom Bootloader (GRUB) bis zum voll funktionsfähigen NovaShell (Login-Bildschirm angezeigt und interaktiv).
        3.1.2. Werkzeug: `systemd-analyze`, `bootchart` (für detailliertere Analyse).
        3.1.3. Ziele:
            - Profil 1: < 45 Sekunden
            - Profil 2: < 25 Sekunden
            - Profil 3: < 15 Sekunden

    3.2. **UI-Reaktionsfähigkeit (NovaShell und Kernanwendungen)**
        3.2.1. Metriken:
            - Öffnen des Anwendungsmenüs (`nova-appmenu`): Zeit vom Klick/Tastendruck bis zur vollständigen Anzeige und Interaktivität.
            - Starten einer Kernanwendung (z.B. Dateimanager `nova-files`, Terminal `nova-terminal`): Zeit vom Klick bis Fenster sichtbar und interaktiv.
            - Fenster-Management (`nova-wm`): Verschieben, Größenänderung, Minimieren/Maximieren von Fenstern (wahrgenommene Flüssigkeit, Framerate).
            - Workspace-Wechsel: Zeit für den Übergang und Darstellung der Fenster auf dem neuen Workspace.
        3.2.2. Werkzeuge: Integrierte Tracing-Tools (z.B. `perf`, LTTng für Kernel/User-Space Events), Wayland-spezifische Debug-Tools (z.B. `weston-debug` falls anwendbar, oder interne `nova-wm` Tracing-Optionen), manuelle Beobachtung mit Hochgeschwindigkeitskamera für spezifische Interaktionen.
        3.2.3. Ziele (allgemein):
            - Anwendungsmenü öffnen: Profil 1: < 400ms; Profil 2: < 200ms; Profil 3: < 100ms.
            - Kernanwendung starten: Profil 1: < 1500ms; Profil 2: < 750ms; Profil 3: < 400ms.
            - Fensteroperationen: Flüssig und ohne sichtbares Ruckeln. Mindestens 30 FPS auf Profil 1, stabile 60 FPS auf Profil 2 & 3.
            - Workspace-Wechsel: Profil 1: < 300ms; Profil 2: < 150ms; Profil 3: < 75ms.

    3.3. **Speichernutzung (RAM)**
        3.3.1. Metriken:
            - Leerlauf (NovaShell geladen, ca. 1 Minute nach Login, keine zusätzlichen Benutzeranwendungen): RAM-Nutzung des Desktops (alle `nova-*` Prozesse und Systemdienste, die direkt von NovaDE beeinflusst werden).
            - Leichte Nutzung (NovaShell + `nova-files`, `nova-terminal`, `nova-texteditor` + Webbrowser mit 5 typischen Tabs): Gesamt-RAM-Nutzung des Systems.
        3.3.2. Werkzeug: `free -m`, `htop` (Summe der RSS-Werte für relevante Prozesse), `smem` (für genauere PSS-Analyse).
        3.3.3. Ziele (Leerlauf NovaDE-Komponenten):
            - Profil 1: < 750 MB
            - Profil 2: < 1 GB
            - Profil 3: < 1.2 GB
        3.3.4. Ziele (Leichte Nutzung, gesamtes System abzüglich Browser-Cache, falls messbar):
            - Profil 1 (4GB RAM System): Gesamtnutzung soll < 60% des System-RAMs betragen, um Swapping zu vermeiden. NovaDE-spezifischer Anteil < 1.5 GB.
            - Profil 2 (8GB RAM System): Gesamtnutzung soll < 40% des System-RAMs betragen. NovaDE-spezifischer Anteil < 2 GB.
            - Profil 3 (16GB+ RAM System): Gesamtnutzung soll < 30% des System-RAMs betragen. NovaDE-spezifischer Anteil < 2.5 GB.

    3.4. **CPU-Last**
        3.4.1. Metriken:
            - Leerlauf (NovaShell geladen, ca. 1 Minute nach Login, keine zusätzlichen Benutzeranwendungen, keine Mausbewegungen): Durchschnittliche CPU-Last über 5 Minuten.
            - Typische Desktop-Aktivitäten (Fenster verschieben und überlappen lassen, Scrollen in einem Textdokument in `nova-texteditor`, Öffnen/Schließen von Fenstern): Spitzen-CPU-Last auf einzelnen Kernen und Gesamtdurchschnitt.
        3.4.2. Werkzeug: `top`, `htop`, `vmstat`, `mpstat`.
        3.4.3. Ziele (Leerlauf):
            - Alle Profile: Durchschnittliche Gesamt-CPU-Last < 5% auf allen Kernen. Einzelne Kerne sollten idealerweise < 2% im Durchschnitt aufweisen.
        3.4.4. Ziele (Typische Aktivitäten, kurzfristige Spitzen):
            - Profil 1: < 70% auf einem Kern, durchschnittliche Gesamt-CPU-Last < 40%.
            - Profil 2: < 50% auf einem Kern, durchschnittliche Gesamt-CPU-Last < 25%.
            - Profil 3: < 30% auf einem Kern, durchschnittliche Gesamt-CPU-Last < 15%.

    3.5. **Grafik-Performance (Compositor `nova-wm`)**
        3.5.1. Metriken:
            - Vollbild-Videowiedergabe (1080p H.264, 30 FPS & 60 FPS): CPU-Last, Anzahl dropped frames, GPU-Auslastung.
            - Desktop-Animationen (Fenster öffnen/schließen-Effekte, Workspace-Wechsel-Animationen, Panel-Animationen): Wahrgenommene Framerate, Glätte, Auftreten von Tearing.
            - (Optional als Stresstest) 3D-Benchmark (z.B. `glxgears` - als Basistest für korrekte Einrichtung, Unigine Heaven/Superposition - anspruchsvoller): Erreichte FPS, Stabilität.
        3.5.2. Werkzeuge: `radeontop` (AMD), `intel_gpu_top` (Intel), `nvtop` oder `nvidia-smi` (NVIDIA), Wayland-spezifische Debug-Tools (z.B. `weston-debug`, interne `nova-wm` FPS-Counter/Debugging-Optionen), Video-Player interne Statistiken.
        3.5.3. Ziele:
            - Videowiedergabe (1080p H.264): Flüssig ohne sichtbare Frame-Drops (< 0.1% dropped frames). CPU-Last bei aktivierter Hardware-Dekodierung: Profil 1: < 50%; Profil 2: < 25%; Profil 3: < 15%. Ohne Hardware-Dekodierung (Software): Profil 1: < 80%; Profil 2: < 40%; Profil 3: < 20%.
            - Desktop-Animationen: Mindestens stabile 30 FPS auf Profil 1. Stabile 60 FPS auf Profil 2 & 3. Kein sichtbares Tearing.
            - 3D-Benchmark (Unigine Heaven, mittlere Einstellungen, passend zur Auflösung des Profils): Profil 1 (1366x768): >20 FPS; Profil 2 (1920x1080): >40 FPS; Profil 3 (1920x1080 oder 2560x1440): >60 FPS. (Diese Ziele sind sekundär und dienen eher der Verifizierung der GPU-Beschleunigung).

**4. Messmethodik**
    4.1. Alle Tests werden auf einer sauberen Installation der primären Ziel-Distribution (Manjaro Linux) durchgeführt, die entsprechend den Spezifikationen der jeweiligen Hardware-Profile (1, 2 und 3) konfiguriert ist.
    4.2. Vor jedem Benchmark-Durchlauf wird das System neu gestartet, um einen konsistenten Ausgangszustand sicherzustellen.
    4.3. Es werden keine anderen Benutzeranwendungen ausgeführt, außer denen, die explizit Teil des Benchmarks sind (z.B. für "Leichte Nutzung").
    4.4. Hintergrunddienste werden auf ein Minimum reduziert, das für den normalen Desktop-Betrieb von NovaDE notwendig ist. Unnötige Systemdienste und Drittanbieter-Agenten (z.B. Update-Checker, Indexierungsdienste außerhalb von NovaDE) werden deaktiviert.
    4.5. Jeder spezifische Benchmark-Test wird mindestens 5 Mal durchgeführt. Der Median der Ergebnisse wird verwendet, um die Auswirkungen von Ausreißern zu minimieren. Bei stark variierenden Ergebnissen werden die Ursachen untersucht.
    4.6. Für UI-Reaktionsfähigkeitstests (z.B. Öffnen des Anwendungsmenüs, Anwendungsstart) werden die genauen Schritte und Interaktionspunkte dokumentiert und versioniert, um Reproduzierbarkeit zu gewährleisten. Soweit möglich, werden Skripte zur Automatisierung dieser Tests eingesetzt.
    4.7. Die genauen Versionen aller Kernkomponenten von NovaDE (nova-wm, nova-shell, nova-core Bibliotheken etc.) und der verwendeten Systembibliotheken (GTK, wlroots, Kernel etc.) werden für jeden Benchmark-Report festgehalten.
    4.8. Bei der Messung der Speichernutzung wird zwischen RSS (Resident Set Size) und PSS (Proportional Set Size) unterschieden, wobei PSS für die Bewertung der tatsächlichen Speichernutzung von Prozessen mit gemeinsam genutzten Bibliotheken bevorzugt wird (z.B. mit `smem`).
    4.9. Änderungen an den Testverfahren oder Werkzeugen müssen dokumentiert und ihre Auswirkungen auf die Ergebnisse bewertet werden.

**5. Ressourcen-Constraints (Entwicklungsrichtlinien)**
    5.1. **Allgemeine Ressourcenschonung:** Jede Komponente und jeder Prozess von NovaDE muss mit dem Ziel der minimalen Ressourcenbeanspruchung (CPU, RAM, GPU, I/O) entwickelt werden, insbesondere im Leerlauf.
    5.2. **Speichermanagement:**
        5.2.1. Memory-Leaks sind strikt zu vermeiden. Der Einsatz von Werkzeugen wie Valgrind (Memcheck Tool), AddressSanitizer (ASan) und LeakSanitizer (LSan) ist während der Entwicklungs- und Testphasen obligatorisch für C-basierten Code.
        5.2.2. Wiederverwendung von Speicher und Reduzierung von Speicherfragmentierung sind anzustreben.
        5.2.3. GObject-basierter Code muss korrekte Referenzzählung sicherstellen.
    5.3. **CPU-Nutzung:**
        5.3.1. Hintergrundprozesse und -dienste müssen im Ruhezustand (Idle) eine CPU-Nutzung nahe 0% aufweisen. Dies wird durch Minimierung von Timern, Vermeidung von Polling und effiziente Nutzung von Event-basierten Mechanismen (z.B. D-Bus Signale, Kernel Events) erreicht.
        5.3.2. Algorithmen und Datenstrukturen müssen hinsichtlich ihrer Performance-Charakteristika (Zeit- und Raumkomplexität) sorgfältig ausgewählt werden.
        5.3.3. Blockierende Operationen in Haupt-Threads von UI-nahen Prozessen sind zu vermeiden. Asynchrone Operationen (z.B. über GIO) sind zu bevorzugen.
    5.4. **Grafik-Ressourcen:**
        5.4.1. GPU-Beschleunigung (OpenGL/Vulkan über Mesa) soll für alle Rendering-Aufgaben im Compositor (`nova-wm`) und in `libnova-ui` (GTK4) genutzt werden.
        5.4.2. Die Anzahl der Render-Passes und die Komplexität von Shadern sind zu minimieren, ohne die gewünschte visuelle Qualität zu beeinträchtigen.
        5.4.3. Übermäßiges Neuzeichnen (Redraws) von UI-Elementen ist zu vermeiden. GTK4 bietet hierfür bereits gute Mechanismen.
    5.5. **I/O-Nutzung:**
        5.5.1. Lese- und Schreibvorgänge auf Speichermedien sind zu minimieren. Konfigurationen und Zustände sollten nur bei Bedarf gespeichert werden.
        5.5.2. Caching-Mechanismen sollen sinnvoll eingesetzt werden, um redundante Berechnungen oder Datenabrufe zu vermeiden.
    5.6. **Energieeffizienz:**
        5.6.1. Auf mobilen Geräten (Laptops) ist die Energieeffizienz ein wichtiger Aspekt. Dies korreliert stark mit der Minimierung der CPU-, GPU- und I/O-Aktivität.
        5.6.2. Nutzung von Power-Management Features des Kernels (z.B. CPU-Frequenzskalierung, Runtime Power Management für Geräte) soll nicht behindert werden.
    5.7. **Code-Optimierung:**
        5.7.1. Performance-kritische Codepfade sollen durch Profiling (z.B. mit `perf`, `gprof`) identifiziert und gezielt optimiert werden.
        5.7.2. Vorzeitige Optimierung ist zu vermeiden. Optimierungen sollten auf Basis von Messdaten und nachgewiesenen Engpässen erfolgen.
        5.7.3. Compiler-Optimierungen (z.B. `-O2` oder `-O3` mit Bedacht) sollen genutzt werden, deren Auswirkungen auf die Performance und Stabilität sind jedoch zu testen.
    5.8. **Abhängigkeiten:** Die Auswahl von externen Bibliotheken soll auch deren Performance-Implikationen und Ressourcenverbrauch berücksichtigen. Leichtgewichtige und effiziente Bibliotheken sind zu bevorzugen.
