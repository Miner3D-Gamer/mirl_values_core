/// Time related structs
pub mod time;
// use mirl_extensions::TryFromPatch;
pub use time::*;
/// [Number](crate::value::number::Number)
pub mod number;
pub use number::Number;

/// Color related data
pub mod color;
pub use color::Color;

/// Units like length or angle
pub mod unit;
pub use unit::*;

use crate::value::ValueType;

/// A primitive "thing" — no containers
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SimpleValue {
    /// Null, None, Nothing. The third variant to bool
    None,
    /// True or False
    Bool(bool),
    /// A number
    Number(number::Number),
    /// A String
    String(String),
    /// [`Time`]
    Time(Time),
    /// [`Datetime`]
    DateTime(Datetime),
    /// A string without the quotes
    Literal(String),
    /// [`AngleType`]
    Angle(number::Number, AngleType),
    /// [`LengthType`]
    Length(number::Number, LengthType),
    /// [`Color`]
    Color(Color),
    /// A list of bytes and a string declaring what these bytes mean. If the string is empty
    Bytes(Vec<u8>, String),
}

impl SimpleValue {
    #[must_use]
    /// Gets the [`ValueType`] of the current item
    pub const fn get_value_type(&self) -> ValueType {
        match self {
            Self::None => ValueType::None,
            Self::Bool(_) => ValueType::Bool,
            Self::Number(_) => ValueType::Number,
            Self::String(_) => ValueType::String,
            Self::Time(_) => ValueType::Time,
            Self::DateTime(_) => ValueType::DateTime,
            Self::Literal(_) => ValueType::Literal,
            Self::Angle(_, _) => ValueType::Angle,
            Self::Length(_, _) => ValueType::Length,
            Self::Color(_) => ValueType::Color,
            Self::Bytes(_, _) => ValueType::Bytes,
        }
    }
}

// ── Ord / PartialOrd for SimpleValue ─────────────────────────────────────────

impl PartialOrd for SimpleValue {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for SimpleValue {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        const fn rank(v: &SimpleValue) -> u8 {
            match v {
                SimpleValue::None => 0,
                SimpleValue::Bool(_) => 1,
                SimpleValue::Number(_) => 2,
                SimpleValue::String(_) => 3,
                SimpleValue::Time(_) => 4,
                SimpleValue::DateTime(_) => 5,
                SimpleValue::Angle(_, _) => 6,
                SimpleValue::Literal(_) => 7,
                SimpleValue::Length(_, _) => 8,
                SimpleValue::Color(_) => 9,
                SimpleValue::Bytes(_, _) => 10,
            }
        }

        match rank(self).cmp(&rank(other)) {
            std::cmp::Ordering::Equal => {}
            ord => return ord,
        }

        match (self, other) {
            (Self::None, Self::None) => std::cmp::Ordering::Equal,
            (Self::Bool(a), Self::Bool(b)) => a.cmp(b),
            (Self::Number(a), Self::Number(b)) => a.cmp(b),
            (Self::String(a), Self::String(b)) => a.cmp(b),
            (Self::Time(a), Self::Time(b)) => a.cmp(b),
            (Self::DateTime(a), Self::DateTime(b)) => a.cmp(b),
            (Self::Angle(n1, a1), Self::Angle(n2, a2)) => match n1.cmp(n2) {
                std::cmp::Ordering::Equal => a1.cmp(a2),
                ord => ord,
            },
            (Self::Literal(l1), Self::Literal(l2)) => l1.cmp(l2),
            (Self::Length(val1, t1), Self::Length(val2, t2)) => {
                (val1, t1).cmp(&(val2, t2))
            }
            (Self::Color(c), Self::Color(c2)) => c.cmp(c2),
            (Self::Bytes(a, b), Self::Bytes(c, d)) => match a.cmp(c) {
                std::cmp::Ordering::Equal => b.cmp(d),
                ord => ord,
            },

            _ => unreachable!(),
        }
    }
}

// ── Hash for SimpleValue ──────────────────────────────────────────────────────

impl std::hash::Hash for SimpleValue {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        std::mem::discriminant(self).hash(state);

        match self {
            Self::None => {}
            Self::Bool(b) => b.hash(state),
            Self::Number(n) => n.hash(state),
            Self::String(s) => s.hash(state),
            Self::Time(t) => t.hash(state),
            Self::DateTime(dt) => dt.hash(state),
            Self::Angle(n, a) => {
                n.hash(state);
                a.hash(state);
            }
            Self::Literal(l) => l.hash(state),
            Self::Color(c) => c.hash(state),
            Self::Length(l, u) => {
                l.hash(state);
                u.hash(state);
            }
            Self::Bytes(v, t) => {
                v.hash(state);
                t.hash(state);
            }
        }
    }
}
