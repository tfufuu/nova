**0. Metadaten**
    *   **Dokument-ID:** `NOVA_DE_IMPL_CORE_TYPES_GEOMETRY_001`
    *   **Bezieht sich auf:** `NOVA_DE_GLOBAL_ARCH_001`
    *   **Version:** 1.0.0
    *   **Status:** In Entwicklung
    *   **Erstellt am:** 2024-07-17
    *   **Letzte Änderung:** 2024-07-17
    *   **Verantwortlich:** NovaGem KI Architekt

**1. Verzeichnis-/Modulname**
    *   `novade/core/types/geometry`

**2. Verantwortlichkeit**
    *   Dieses Modul definiert grundlegende geometrische Datentypen, die im gesamten NovaDE-Projekt für Positionierung, Größenangaben und Bereichsdefinitionen verwendet werden. Diese Typen sollen einfach, effizient und leicht zu verwenden sein und grundlegende geometrische Operationen unterstützen. Sie dienen als Bausteine für UI-Layouts, Fenstermanagement und andere grafische Komponenten.

**3. Kern-Aufgaben (Tasks)**

    3.1. **Vollständige Rust-Typdefinitionen (Structs, Enums, Traits):**

        3.1.1. **`pub struct Point<T = i32>`**
            *   Sichtbarkeit: `pub`
            *   Generischer Parameter: `T` mit Default `i32`. `T` muss `Copy + Clone + Debug + Default + PartialEq + PartialOrd + std::ops::Add<Output = T> + std::ops::Sub<Output = T> + std::ops::Mul<Output = T> + std::ops::Div<Output = T>` implementieren.
            *   `#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]` (Eq und Hash nur wenn T Eq und Hash implementiert)
            *   `#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]` (mit optionalem "serde" feature)
            *   **Felder:**
                *   `pub x: T`
                *   `pub y: T`
            *   **Methoden (exemplarisch):**
                *   `pub fn new(x: T, y: T) -> Self`
                *   `pub fn zero() -> Self where T: num_traits::Zero` (benötigt `num_traits` Crate)
                *   `pub fn offset(&self, dx: T, dy: T) -> Self`
                *   `pub fn distance_to(&self, other: &Self) -> f64 where T: Into<f64> + num_traits::ToPrimitive`

        3.1.2. **`pub struct Size<T = u32>`**
            *   Sichtbarkeit: `pub`
            *   Generischer Parameter: `T` mit Default `u32`. `T` muss `Copy + Clone + Debug + Default + PartialEq + PartialOrd + std::ops::Add<Output = T> + std::ops::Sub<Output = T> + std::ops::Mul<Output = T> + std::ops::Div<Output = T> + num_traits::Zero` implementieren.
            *   `#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]`
            *   `#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]`
            *   **Felder:**
                *   `pub width: T`
                *   `pub height: T`
            *   **Methoden (exemplarisch):**
                *   `pub fn new(width: T, height: T) -> Self`
                *   `pub fn zero() -> Self`
                *   `pub fn area(&self) -> T`
                *   `pub fn is_empty(&self) -> bool` // Prüft ob width oder height zero ist

        3.1.3. **`pub struct Rect<P = i32, S = u32>`** (Position und Größe mit unterschiedlichen Typen)
            *   Sichtbarkeit: `pub`
            *   Generische Parameter: `P` für Positionstyp (Default `i32`), `S` für Größentyp (Default `u32`).
            *   `P`: `Copy + Clone + Debug + Default + PartialEq + PartialOrd + std::ops::Add<Output = P> + std::ops::Sub<Output = P> + Into<f64> + num_traits::ToPrimitive + std::cmp::Ord` (Ord für min/max in intersection)
            *   `S`: `Copy + Clone + Debug + Default + PartialEq + PartialOrd + std::ops::Add<Output = S> + std::ops::Sub<Output = S> + std::ops::Mul<Output = S> + std::ops::Div<Output = S> + num_traits::Zero + Into<P> + num_traits::ToPrimitive + TryFrom<P>` (TryFrom<P> für `intersection`, dessen Error Debug sein muss für `ok()?`)
            *   `#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]` (Eq und Hash wenn P und S dies implementieren)
            *   `#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]`
            *   **Felder:**
                *   `pub origin: Point<P>`
                *   `pub size: Size<S>`
            *   **Methoden (exemplarisch):**
                *   `pub fn new(x: P, y: P, width: S, height: S) -> Self`
                *   `pub fn from_point_size(origin: Point<P>, size: Size<S>) -> Self`
                *   `pub fn x(&self) -> P { self.origin.x }`
                *   `pub fn y(&self) -> P { self.origin.y }`
                *   `pub fn width(&self) -> S { self.size.width }`
                *   `pub fn height(&self) -> S { self.size.height }`
                *   `pub fn top_left(&self) -> Point<P> { self.origin }`
                *   `pub fn top_right(&self) -> Point<P> { Point::new(self.origin.x + self.size.width.into(), self.origin.y) }`
                *   `pub fn bottom_left(&self) -> Point<P> { Point::new(self.origin.x, self.origin.y + self.size.height.into()) }`
                *   `pub fn bottom_right(&self) -> Point<P> { Point::new(self.origin.x + self.size.width.into(), self.origin.y + self.size.height.into()) }`
                *   `pub fn center(&self) -> Point<f64> where P: Into<f64>, S: Into<f64>`
                *   `pub fn area(&self) -> S { self.size.area() }`
                *   `pub fn contains_point(&self, point: Point<P>) -> bool`
                *   `pub fn intersects(&self, other: &Rect<P, S>) -> bool`
                *   `pub fn intersection(&self, other: &Rect<P, S>) -> Option<Rect<P, S>> where <S as TryFrom<P>>::Error: std::fmt::Debug`
                *   `pub fn union(&self, other: &Rect<P, S>) -> Rect<P, S> where P: std::cmp::Ord, S: std::cmp::Ord` // Ord für min/max
                *   `pub fn inset(&self, d_width: S, d_height: S) -> Rect<P, S>` // Benötigt Sub für S, Add für P
                *   `pub fn outset(&self, d_width: S, d_height: S) -> Rect<P, S>` // Benötigt Add für S, Sub für P
                *   `pub fn with_position(&self, new_origin: Point<P>) -> Self { Rect::from_point_size(new_origin, self.size) }`
                *   `pub fn with_size(&self, new_size: Size<S>) -> Self { Rect::from_point_size(self.origin, new_size) }`

        3.1.4. **`pub struct Insets<T = u32>`** (Für Padding, Margin)
            *   Sichtbarkeit: `pub`
            *   Generischer Parameter: `T` mit Default `u32`. Constraints ähnlich `Size<T>`.
            *   `#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]`
            *   `#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]`
            *   **Felder:**
                *   `pub top: T`
                *   `pub right: T`
                *   `pub bottom: T`
                *   `pub left: T`
            *   **Methoden (exemplarisch):**
                *   `pub fn new(top: T, right: T, bottom: T, left: T) -> Self`
                *   `pub fn uniform(value: T) -> Self`
                *   `pub fn horizontal(&self) -> T { self.left + self.right }`
                *   `pub fn vertical(&self) -> T { self.top + self.bottom }`

    3.2. **Vollständige Rust-Funktionssignaturen:**
        *   Die meisten Funktionen sind als Methoden der oben definierten Structs implementiert.

    3.3. **Explizite Algorithmusbeschreibungen:**

        *   **`Rect<P,S>::contains_point(point: Point<P>) -> bool`**:
            1.  Prüfe, ob `point.x >= self.x()`.
            2.  Prüfe, ob `point.x < (self.x() + self.width().into())`.
            3.  Prüfe, ob `point.y >= self.y()`.
            4.  Prüfe, ob `point.y < (self.y() + self.height().into())`.
            5.  Return `true` wenn alle Bedingungen erfüllt sind, sonst `false`.

        *   **`Rect<P,S>::intersects(other: &Rect<P,S>) -> bool`**:
            1.  Prüfe, ob `self.x() < (other.x() + other.width().into())`.
            2.  Prüfe, ob `(self.x() + self.width().into()) > other.x()`.
            3.  Prüfe, ob `self.y() < (other.y() + other.height().into())`.
            4.  Prüfe, ob `(self.y() + self.height().into()) > other.y()`.
            5.  Return `true` wenn alle Bedingungen erfüllt sind, sonst `false`.

        *   **`Rect<P,S>::intersection(other: &Rect<P,S>) -> Option<Rect<P,S>> where <S as TryFrom<P>>::Error: std::fmt::Debug`**:
            1.  Wenn `!self.intersects(other)`, return `None`.
            2.  Berechne `intersect_x = std::cmp::max(self.x(), other.x())`.
            3.  Berechne `intersect_y = std::cmp::max(self.y(), other.y())`.
            4.  Berechne `self_br_x = self.x() + self.width().into()`.
            5.  Berechne `self_br_y = self.y() + self.height().into()`.
            6.  Berechne `other_br_x = other.x() + other.width().into()`.
            7.  Berechne `other_br_y = other.y() + other.height().into()`.
            8.  Berechne `intersect_br_x = std::cmp::min(self_br_x, other_br_x)`.
            9.  Berechne `intersect_br_y = std::cmp::min(self_br_y, other_br_y)`.
            10. Berechne `intersect_width_p_type = intersect_br_x - intersect_x`.
            11. Berechne `intersect_height_p_type = intersect_br_y - intersect_y`.
            12. Konvertiere `intersect_width_p_type` zu `S` mittels `S::try_from(intersect_width_p_type).ok()?`. Wenn dies `None` ergibt (weil z.B. `intersect_width_p_type` negativ ist und `S` unsigned, oder die Konvertierung fehlschlägt), return `None`. Sei das Ergebnis `intersect_width_s`.
            13. Konvertiere `intersect_height_p_type` zu `S` mittels `S::try_from(intersect_height_p_type).ok()?`. Wenn dies `None` ergibt, return `None`. Sei das Ergebnis `intersect_height_s`.
            14. Wenn `intersect_width_s == S::zero()` oder `intersect_height_s == S::zero()`, return `None`.
            15. Return `Some(Rect::new(intersect_x, intersect_y, intersect_width_s, intersect_height_s))`.

    3.4. **Erschöpfende Fehlerbehandlung:**
        *   Die Typen verwenden Generics `P` (Position) und `S` (Größe), wobei `S` typischerweise unsigned (`u32`) sein wird, um negative Größen zu verhindern.
        *   Die `intersection` Methode gibt `Option` zurück. Die Konvertierung zwischen `P` und `S` innerhalb von `intersection` muss sorgfältig behandelt werden (mit `S::try_from(P).ok()?`), um Fehler bei der Konvertierung (z.B. negativer Wert von `P` für ein unsigned `S`) abzufangen.
        *   `Size::is_empty()` prüft, ob `width` ODER `height` Null ist (`self.width == S::zero() || self.height == S::zero()`).

    3.5. **Speicher-/Ressourcenmanagement-Direktiven:**
        *   Alle Typen sind `Copy` und `Clone`. Keine dynamische Allokation.

**4. Spezifische Artefakte/Dateien:**
    *   **`novade/core/types/geometry/src/lib.rs`**: Hauptdatei. Enthält Definitionen und `pub use` für Submodule.
    *   **`novade/core/types/geometry/src/point.rs`**: Definition für `Point<T>`.
    *   **`novade/core/types/geometry/src/size.rs`**: Definition für `Size<T>`.
    *   **`novade/core/types/geometry/src/rect.rs`**: Definition für `Rect<P, S>`.
    *   **`novade/core/types/geometry/src/insets.rs`**: Definition für `Insets<T>`.
    *   **`novade/core/types/geometry/src/tests.rs`** (oder Tests in jedem Modul): Unit-Tests.

**5. Abhängigkeiten:**
    *   **Intern:** Keine.
    *   **Extern:**
        *   `std`
        *   `serde = { version = "=1.0.188", features = ["derive"], optional = true }`
        *   `num-traits = { version = "=0.2.17", optional = true }`

**6. Kommunikationsmuster:**
    *   Passive Datenstrukturen.

**7. Erwartete Ergebnisse/Outputs:**
    *   Robuste, effiziente, generische geometrische Grundtypen.
    *   Gut getestete Implementierungen.
    *   Optionale Serialisierungsfähigkeit.

Stelle sicher, dass die Generics und Trait-Bounds präzise sind. Die Unterscheidung P (Position) und S (Größe) für Rect ist eine wichtige Präzisierung. Die Algorithmen sind entsprechend anzupassen.
Das Datum soll dynamisch eingefügt werden.
Die Trait-Bound für `S` in `Rect` wurde um `TryFrom<P>` erweitert, und die `intersection` Methode wurde entsprechend angepasst, um `ok()?` für die Fehlerbehandlung bei der Konvertierung zu verwenden. Die `where`-Klausel für `intersection` wurde hinzugefügt, um den `Debug`-Trait-Bound für den Fehler von `TryFrom` zu spezifizieren. `Size::is_empty()` wurde präzisiert. `Ord` wurde zu `P` hinzugefügt, wo min/max verwendet wird.
