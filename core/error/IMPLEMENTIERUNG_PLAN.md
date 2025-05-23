**0. Metadaten**
    *   **Dokument-ID:** `NOVA_DE_IMPL_CORE_ERROR_001`
    *   **Bezieht sich auf:** `NOVA_DE_GLOBAL_ARCH_001`, `NOVA_DE_INTERFACES_001`
    *   **Version:** 1.0.0
    *   **Status:** In Entwicklung
    *   **Erstellt am:** 2024-07-17
    *   **Letzte Änderung:** 2024-07-17
    *   **Verantwortlich:** NovaGem KI Architekt

**1. Verzeichnis-/Modulname**
    *   `novade/core/error`

**2. Verantwortlichkeit**
    *   Dieses Modul definiert die grundlegenden Fehlerbehandlungstypen und -mechanismen, die im gesamten NovaDE-Projekt verwendet werden. Es stellt ein einheitliches Framework für die Fehlererstellung, -propagation und -behandlung bereit, um die Robustheit und Wartbarkeit des Systems zu gewährleisten. Es dient als Basis für spezifischere Fehlertypen in anderen Modulen und Schichten.

**3. Kern-Aufgaben (Tasks)**

    3.1. **Vollständige Rust-Typdefinitionen (Structs, Enums, Traits):**

        3.1.1. **`pub enum CoreError`**
            *   Sichtbarkeit: `pub`
            *   `#[derive(Debug, thiserror::Error)]`
            *   **Varianten:**
                *   `#[error("I/O Error: {0}")] Io(#[from] std::io::Error)`: Für Standard I/O Fehler.
                *   `#[error("Serialization Error ({format}): {source}")] Serialization { format: String, #[source] source: Box<dyn std::error::Error + Send + Sync + 'static> }`: Für Serialisierungs-/Deserialisierungsfehler (z.B. JSON, RON). `format` gibt das betroffene Format an.
                *   `#[error("Deserialization Error ({format}): {source}")] Deserialization { format: String, #[source] source: Box<dyn std::error::Error + Send + Sync + 'static> }`: Für Deserialisierungsfehler.
                *   `#[error("Configuration Error: {message}")] Configuration { message: String }`: Für allgemeine Konfigurationsfehler.
                *   `#[error("Initialization Error ({component}): {message}")] Initialization { component: String, message: String }`: Für Fehler während der Initialisierung von Komponenten.
                *   `#[error("Resource Not Found ({resource_type}): {resource_id}")] ResourceNotFound { resource_type: String, resource_id: String }`: Wenn eine erwartete Ressource nicht gefunden wurde.
                *   `#[error("Permission Denied: {action}")] PermissionDenied { action: String }`: Für Zugriffsverweigerungen.
                *   `#[error("Invalid Parameter ({parameter_name}): {reason}")] InvalidParameter { parameter_name: String, reason: String }`: Für ungültige Funktions- oder Methodenparameter.
                *   `#[error("Internal Error: {message}")] Internal { message: String }`: Für unerwartete interne Fehlerzustände.
                *   `#[error("Operation Timed Out: {operation}")] OperationTimedOut { operation: String }`: Wenn eine Operation die erwartete Zeit überschreitet.
                *   `#[error("D-Bus Error: {0}")] DBus(#[from] zbus::Error)`: Integration von zbus Fehlern.
                *   `#[error("Underlying Service Error ({service_name}): {source}")] UnderlyingService { service_name: String, #[source] source: Box<dyn std::error::Error + Send + Sync + 'static> }`: Für Fehler, die von einem tieferliegenden Dienst stammen und weitergereicht werden.

        3.1.2. **`pub trait ToNovaCoreError<T>`**
            *   Sichtbarkeit: `pub`
            *   Zweck: Ein Helfer-Trait, um externe Fehlertypen oder `Option` in `Result<T, CoreError>` zu konvertieren, oft mit Kontextinformationen.
            *   Methoden:
                *   `fn context_err(self, message: impl Into<String>) -> Result<T, CoreError>;`
                *   `fn context_res_not_found(self, resource_type: impl Into<String>, resource_id: impl Into<String>) -> Result<T, CoreError>;`
                *   `// Weitere kontextualisierende Konvertierungsmethoden bei Bedarf, z.B. für spezifische CoreError-Varianten`
                *   `fn context_invalid_param(self, parameter_name: impl Into<String>, reason: impl Into<String>) -> Result<T, CoreError>;`
                *   `fn context_config_err(self, message: impl Into<String>) -> Result<T, CoreError>;`

    3.2. **Vollständige Rust-Funktionssignaturen:**
        *   Keine direkten öffentlichen Funktionen in diesem Modul vorgesehen, da es primär Typen und Traits definiert.
        *   Implementierungen des Traits `ToNovaCoreError` für `Option<T>` und `Result<T, E: std::error::Error + Send + Sync + 'static>` werden bereitgestellt.

    3.3. **Explizite Algorithmusbeschreibungen:**
        *   **Fehlerkonvertierung mit `ToNovaCoreError` (Beispiel für `Option<V>` Implementierung):**
            1.  `impl<V> ToNovaCoreError<V> for Option<V>`
            2.  `fn context_err(self, message: impl Into<String>) -> Result<V, CoreError>`:
                *   Wenn `self` ist `Some(v)`, gib `Ok(v)` zurück.
                *   Wenn `self` ist `None`, gib `Err(CoreError::Internal { message: message.into() })` zurück.
            3.  `fn context_res_not_found(self, resource_type: impl Into<String>, resource_id: impl Into<String>) -> Result<V, CoreError>`:
                *   Wenn `self` ist `Some(v)`, gib `Ok(v)` zurück.
                *   Wenn `self` ist `None`, gib `Err(CoreError::ResourceNotFound { resource_type: resource_type.into(), resource_id: resource_id.into() })` zurück.
            4.  `fn context_invalid_param(self, parameter_name: impl Into<String>, reason: impl Into<String>) -> Result<V, CoreError>`:
                *   Wenn `self` ist `Some(v)`, gib `Ok(v)` zurück.
                *   Wenn `self` ist `None`, gib `Err(CoreError::InvalidParameter { parameter_name: parameter_name.into(), reason: reason.into() })` zurück (kann auch für `Result` implementiert werden, wo `None` einem Fehler entspricht).
            5.  `fn context_config_err(self, message: impl Into<String>) -> Result<V, CoreError>`:
                 *   Wenn `self` ist `Some(v)`, gib `Ok(v)` zurück.
                 *   Wenn `self` ist `None`, gib `Err(CoreError::Configuration { message: message.into() })` zurück.

        *   **Fehlerkonvertierung mit `ToNovaCoreError` (Beispiel für `Result<V, E: std::error::Error + Send + Sync + 'static>` Implementierung):**
            1.  `impl<V, E: std::error::Error + Send + Sync + 'static> ToNovaCoreError<V> for Result<V, E>`
            2.  `fn context_err(self, message: impl Into<String>) -> Result<V, CoreError>`:
                *   Wenn `self` ist `Ok(v)`, gib `Ok(v)` zurück.
                *   Wenn `self` ist `Err(e)`, gib `Err(CoreError::Internal { message: format!("{}: {}", message.into(), e) })` zurück oder eine spezifischere Variante, falls `e` gematcht werden kann oder die Variante `source` unterstützt. Ideal wäre, `e` als `source` zu verpacken, wenn die `CoreError` Variante dies zulässt. Für generische `Internal` könnte der Quellfehler in der Nachricht erwähnt werden.
                    `// Besser: Err(CoreError::UnderlyingService { service_name: "Unknown".to_string(), source: Box::new(e) }) wenn Kontext unklar`
                    `// oder eine neue Variante CoreError::Contextualized { message: String, source: Box<dyn Error...> }`
                    `// Für dieses Beispiel halten wir uns an die existierenden Varianten und betten in die Nachricht ein.`
                    `// Die Variante `Internal` hat kein `source` Feld. Man könnte eine neue Variante `ContextualizedError` hinzufügen oder `UnderlyingService` missbrauchen.`
                    `// Gemäß Definition ist `UnderlyingService` besser geeignet, wenn `e` als externer Fehler betrachtet wird.`
                    `// Vorerst wird `Internal` mit einer formatierten Nachricht verwendet, was eine Vereinfachung darstellt.`
                    `// Korrekter wäre es, eine CoreError-Variante zu wählen, die `source` unterstützt, oder eine zu schaffen.`
                    `// Für eine robustere Lösung wird empfohlen, dass kontextualisierte Fehler eine `source` behalten.`
                    `// Alternative für context_err bei Result<V,E>:`
                    `// match self { Ok(v) => Ok(v), Err(e) => Err(CoreError::UnderlyingService { service_name: message.into(), source: Box::new(e) }) }`
                    `// Dies passt besser zur Intention, den Originalfehler zu bewahren.`
            3.  Andere `context_*` Methoden folgen einem ähnlichen Muster: Bei `Ok(v)` wird `v` zurückgegeben, bei `Err(e)` wird ein passender `CoreError` erzeugt, wobei `e` idealerweise als `source` verpackt oder in die Nachricht integriert wird.

    3.4. **Erschöpfende Fehlerbehandlung:**
        *   Dieses Modul *definiert* die Fehler. Die Behandlung erfolgt in den Modulen, die `CoreError` verwenden.
        *   Bei der Erstellung von `CoreError` Instanzen:
            *   `Io`: Automatisch durch `#[from] std::io::Error`.
            *   `Serialization`: `format` (z.B. "JSON", "RON") und der `source`-Fehler (geboxt) müssen angegeben werden.
            *   `Deserialization`: `format` (z.B. "JSON", "RON") und der `source`-Fehler (geboxt) müssen angegeben werden.
            *   `Configuration`: Eine klare `message` ist erforderlich.
            *   `Initialization`: Der `component`-Name und eine klare `message` sind erforderlich.
            *   `ResourceNotFound`: `resource_type` und `resource_id` sind anzugeben.
            *   `PermissionDenied`: Die verweigerte `action` ist zu spezifizieren.
            *   `InvalidParameter`: `parameter_name` und `reason` für Ungültigkeit.
            *   `Internal`: Eine prägnante `message` des internen Problems.
            *   `OperationTimedOut`: Die `operation`, die Zeitüberschreitung hatte.
            *   `DBus`: Automatisch durch `#[from] zbus::Error`.
            *   `UnderlyingService`: Der `service_name` und der `source`-Fehler (geboxt) sind erforderlich.

    3.5. **Speicher-/Ressourcenmanagement-Direktiven:**
        *   `CoreError` ist `Send + Sync + 'static` durch die Verwendung von `Box<dyn std.error::Error + Send + Sync + 'static>` für Quellfehler und die Eigenschaften der anderen Felder.
        *   Die Verwendung von `Box` stellt sicher, dass die Größe von `CoreError` bekannt ist und Quellfehler verschiedener Typen gespeichert werden können, ohne `CoreError` generisch zu machen.

**4. Spezifische Artefakte/Dateien:**
    *   **`novade/core/error/src/lib.rs`**: Enthält die Definition von `CoreError`, `ToNovaCoreError` und die Implementierungen von `ToNovaCoreError` für `Option<T>` und `Result<T, E: std::error::Error + Send + Sync + 'static>`.
    *   **`novade/core/error/tests/tests.rs`** (oder inline Tests in `lib.rs`): Unit-Tests für die Erstellung der verschiedenen `CoreError`-Varianten und die Funktionalität der `ToNovaCoreError`-Implementierungen.

**5. Abhängigkeiten:**
    *   **Intern:** Keine.
    *   **Extern:**
        *   `std`
        *   `thiserror = "1.0"` (gemäß `00_GLOBAL_ARCHITEKTUR_DEFINITION.md`, exakte Version z.B. "1.0.50" oder neuer, falls kompatibel)
        *   `zbus = "3.14"` (gemäß `00_GLOBAL_ARCHITEKTUR_DEFINITION.md`, exakte Version z.B. "3.14.1" oder neuer, falls kompatibel)

**6. Kommunikationsmuster:**
    *   **Inbound:** `CoreError` wird von Funktionen/Methoden in anderen Modulen als Fehlertyp in `Result<T, CoreError>` zurückgegeben oder durch die Verwendung von `ToNovaCoreError` erzeugt.
    *   **Outbound:** Nicht anwendbar.
    *   **Synchronisation:** Nicht anwendbar.

**7. Erwartete Ergebnisse/Outputs:**
    *   Eine `core::error` Crate mit dem definierten `CoreError` Enum und dem `ToNovaCoreError` Trait inklusive Implementierungen.
    *   Einheitliche und informative Fehlerbehandlung im gesamten NovaDE-Projekt.
    *   Vereinfachte Konvertierung von `Option` und externen Fehlern in `CoreError`.
    *   Verbesserte Debugging-Möglichkeiten durch strukturierte Fehler und die Möglichkeit, Quellfehler zu verketten.
    *   Vorlagen für die Erstellung spezifischerer Fehlertypen in anderen Modulen, die ggf. `CoreError` wrappen oder davon inspiriert sind.
```
