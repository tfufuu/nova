//! Grundlegende, schichtübergreifend genutzte Datentypen.

// Beispiel für einen benutzerdefinierten ID-Typ
// #[derive(Debug, Clone, PartialEq, Eq, Hash)] // Ggf. serde::Serialize, serde::Deserialize
// pub struct NovaId(String);

// impl NovaId {
//     pub fn new(id: String) -> Self {
//         Self(id)
//     }
// }

// impl From<&str> for NovaId {
//     fn from(s: &str) -> Self {
//         Self(s.to_string())
//     }
// }

// impl std::fmt::Display for NovaId {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(f, "{}", self.0)
//     }
// }

// Weitere grundlegende Typen hier definieren
