mod simple;

use std::fmt::Display;

pub use simple::*;

mod container;

pub use container::*;

mod conversion;

// ── ValueType ────────────────────────────────────────────────────────────────


#[cfg_attr(feature = "mirl_derive", mirl_derive::derive_all)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Empty value types that do not hold any data
pub enum ValueType {
    /// Refer to [`SimpleValue::None`]
    None,
    /// Refer to [`SimpleValue::Bool`]
    Bool,
    /// Refer to [`SimpleValue::Number`]
    Number,
    /// Refer to [`SimpleValue::String`]
    String,
    /// Refer to [`SimpleValue::Time`]
    Time,
    /// Refer to [`SimpleValue::DateTime`]
    DateTime,
    /// Refer to [`SimpleValue::Angle`]
    Angle,
    /// Refer to [`SimpleValue::Literal`]
    Literal,
    /// Refer to [`SimpleValue::Length`]
    Length,
    /// Refer to [`SimpleValue::Color`]
    Color,
    /// Refer to [`SimpleValue::Bytes`]
    Bytes,
    /// Refer to [`ContainerValue::Vec`]
    Vec,
    /// Refer to [`ContainerValue::Map`]
    Map,
    #[default]
    /// When no type could be determined
    ///
    /// When used in an error, it means the error originated outside the parser
    Invalid,
}
impl Display for ValueType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::None => "None",
            Self::Bool => "Bool",
            Self::Number => "Number",
            Self::String => "String",
            Self::Time => "Time",
            Self::DateTime => "Datetime",
            Self::Angle => "Angle",
            Self::Literal => "Literal",
            Self::Length => "Length",
            Self::Color => "Color",
            Self::Bytes => "Bytes",
            Self::Vec => "Vec",
            Self::Map => "Map",
            Self::Invalid => "Invalid",
        })
    }
}
