//! The core lib of `mirl_values`, this only defines the raw data types

/// The core Simple and Container values
pub mod value;

/// Settings for the crate
pub mod settings;

// #[cfg(feature = "serde")]
// impl From<&serde_json::Value> for ValueType {
//     fn from(value: &serde_json::Value) -> Self {
//         match value {
//             serde_json::Value::None => Self::None,
//             serde_json::Value::Vec(_) => Self::Vec,
//             serde_json::Value::Bool(_) => Self::Bool,
//             serde_json::Value::Number(_) => Self::Number,
//             serde_json::Value::Object(_) => Self::Map,
//             serde_json::Value::String(_) => Self::String,
//         }
//     }
// }
