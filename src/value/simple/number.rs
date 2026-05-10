use std::{hint::unreachable_unchecked, str::FromStr};

// use mirl_extensions::*;

#[cfg_attr(feature = "strum", derive(strum::EnumIter))]
#[derive(Debug, Clone, PartialEq)]
/// A "dynamic" number
pub enum Number {
    /// Normal number
    Int(i128),
    /// Normal float
    Float(f64),
    /// Numbers bigger/Smaller than the i128 range
    BigInt(num_bigint::BigInt),
    /// Numbers outside the f64 range
    BigFloat(num_bigfloat::BigFloat),
}
impl FromStr for Number {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        i128::from_str(s).map_or_else(
            |_| {
                BigInt::from_str(s).map_or_else(
                    |_| {
                        f64::from_str(s).map_or_else(
                            |_| Err(()),
                            |x| {
                                if x.is_infinite() {
                                    BigFloat::from_str(s).map_or_else(
                                        |_| Ok(Self::Float(x)),
                                        |y| {
                                            if y.is_infinite() {
                                                Ok(Self::Float(x))
                                            } else {
                                                Ok(Self::BigFloat(y))
                                            }
                                        },
                                    )
                                } else {
                                    Ok(Self::Float(x))
                                }
                            },
                        )
                    },
                    |x| Ok(Self::BigInt(x)),
                )
            },
            |x| Ok(Self::Int(x)),
        )
    }
}

impl core::ops::Add for Number {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Self::Int(v1), Self::Int(v2)) => Self::Int(v1 + v2),
            (Self::BigInt(v1), Self::Int(v2)) => {
                Self::BigInt(v1 + num_bigint::BigInt::from(v2))
            }
            (Self::Int(v1), Self::BigInt(v2)) => {
                Self::BigInt(v2 + num_bigint::BigInt::from(v1))
            }
            (Self::BigInt(v1), Self::BigInt(v2)) => Self::BigInt(v1 + v2),
            (Self::Int(v1), Self::Float(v2)) => Self::Float((v1 as f64) + (v2)),
            (Self::Float(v1), Self::Int(v2)) => Self::Float((v2 as f64) + (v1)),
            (Self::Int(v1), Self::BigFloat(v2)) => {
                Self::BigFloat(num_bigfloat::BigFloat::from_i128(v1) + (v2))
            }
            (Self::BigFloat(v1), Self::Int(v2)) => {
                Self::BigFloat(num_bigfloat::BigFloat::from_i128(v2) + (v1))
            }
            (Self::Float(v1), Self::Float(v2)) => Self::Float(v1 + v2),
            (Self::Float(v1), Self::BigFloat(v2)) => {
                Self::BigFloat(num_bigfloat::BigFloat::from_f64(v1) + (v2))
            }
            (Self::Float(v1), Self::BigInt(v2)) => Self::BigFloat(
                num_bigfloat::BigFloat::from_f64(v1) + bigint_to_bigfloat(&v2),
            ),
            (Self::BigInt(v1), Self::Float(v2)) => Self::BigFloat(
                num_bigfloat::BigFloat::from_f64(v2) + bigint_to_bigfloat(&v1),
            ),
            (Self::BigInt(v1), Self::BigFloat(v2)) => {
                Self::BigFloat(v2 + bigint_to_bigfloat(&v1))
            }
            (Self::BigFloat(v1), Self::Float(v2)) => {
                Self::BigFloat(num_bigfloat::BigFloat::from_f64(v2) + (v1))
            }
            (Self::BigFloat(v1), Self::BigInt(v2)) => {
                Self::BigFloat(v1 + bigint_to_bigfloat(&v2))
            }
            (Self::BigFloat(v1), Self::BigFloat(v2)) => Self::BigFloat(v1 + v2),
        }
    }
}

use num_bigfloat::BigFloat;
use num_bigint::BigInt;
use num_traits::{Float, Signed, Zero};

impl Number {
    #[must_use]
    /// Attempt to narrow a `BigInt` back to `i128` after an operation.
    pub fn maybe_narrow_bigint(b: BigInt) -> Self {
        i128::try_from(&b).map_or(Self::BigInt(b), Self::Int)
    }
    #[must_use]
    /// Attempt to narrow a `BigFloat` back to `f64` after an operation.
    pub fn maybe_narrow_bigfloat(b: BigFloat) -> Self {
        let f = b.to_f64();
        if f.is_finite() {
            Self::Float(f)
        } else {
            Self::BigFloat(b)
        }
    }
}

// ── conversions ───────────────────────────────────────────────────────────────

impl From<i8> for Number {
    fn from(v: i8) -> Self {
        Self::Int(i128::from(v))
    }
}
impl From<i16> for Number {
    fn from(v: i16) -> Self {
        Self::Int(i128::from(v))
    }
}
impl From<i32> for Number {
    fn from(v: i32) -> Self {
        Self::Int(i128::from(v))
    }
}
impl From<i64> for Number {
    fn from(v: i64) -> Self {
        Self::Int(i128::from(v))
    }
}
impl From<i128> for Number {
    fn from(v: i128) -> Self {
        Self::Int(v)
    }
}
impl From<u8> for Number {
    fn from(v: u8) -> Self {
        Self::Int(i128::from(v))
    }
}
impl From<u16> for Number {
    fn from(v: u16) -> Self {
        Self::Int(i128::from(v))
    }
}
impl From<u32> for Number {
    fn from(v: u32) -> Self {
        Self::Int(i128::from(v))
    }
}
impl From<u64> for Number {
    fn from(v: u64) -> Self {
        Self::Int(i128::from(v))
    }
}
impl From<u128> for Number {
    fn from(v: u128) -> Self {
        if v <= i128::MAX as u128 {
            Self::Int(v as i128)
        } else {
            Self::BigInt(BigInt::from(v))
        }
    }
}
impl From<f32> for Number {
    fn from(v: f32) -> Self {
        Self::Float(f64::from(v))
    }
}
impl From<f64> for Number {
    fn from(v: f64) -> Self {
        Self::Float(v)
    }
}
impl From<BigInt> for Number {
    fn from(v: BigInt) -> Self {
        Self::maybe_narrow_bigint(v)
    }
}
impl From<BigFloat> for Number {
    fn from(v: BigFloat) -> Self {
        Self::maybe_narrow_bigfloat(v)
    }
}

impl core::ops::Sub for Number {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Self::Int(v1), Self::Int(v2)) => Self::Int(v1 - v2),
            (Self::BigInt(v1), Self::Int(v2)) => {
                Self::BigInt(v1 - BigInt::from(v2))
            }
            (Self::Int(v1), Self::BigInt(v2)) => {
                Self::BigInt(BigInt::from(v1) - v2)
            }
            (Self::BigInt(v1), Self::BigInt(v2)) => Self::BigInt(v1 - v2),
            (Self::Int(v1), Self::Float(v2)) => Self::Float((v1 as f64) - v2),
            (Self::Float(v1), Self::Int(v2)) => Self::Float(v1 - (v2 as f64)),
            (Self::Int(v1), Self::BigFloat(v2)) => {
                Self::BigFloat(BigFloat::from_i128(v1) - v2)
            }
            (Self::BigFloat(v1), Self::Int(v2)) => {
                Self::BigFloat(v1 - BigFloat::from_i128(v2))
            }
            (Self::Float(v1), Self::Float(v2)) => Self::Float(v1 - v2),
            (Self::Float(v1), Self::BigFloat(v2)) => {
                Self::BigFloat(BigFloat::from_f64(v1) - v2)
            }
            (Self::BigFloat(v1), Self::Float(v2)) => {
                Self::BigFloat(v1 - BigFloat::from_f64(v2))
            }
            (Self::Float(v1), Self::BigInt(v2)) => {
                Self::BigFloat(BigFloat::from_f64(v1) - bigint_to_bigfloat(&v2))
            }
            (Self::BigInt(v1), Self::Float(v2)) => {
                Self::BigFloat(bigint_to_bigfloat(&v1) - BigFloat::from_f64(v2))
            }
            (Self::BigInt(v1), Self::BigFloat(v2)) => {
                Self::BigFloat(bigint_to_bigfloat(&v1) - v2)
            }
            (Self::BigFloat(v1), Self::BigInt(v2)) => {
                Self::BigFloat(v1 - bigint_to_bigfloat(&v2))
            }
            (Self::BigFloat(v1), Self::BigFloat(v2)) => Self::BigFloat(v1 - v2),
        }
    }
}

impl core::ops::Mul for Number {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Self::Int(v1), Self::Int(v2)) => Self::Int(v1 * v2),
            (Self::BigInt(v1), Self::Int(v2)) => {
                Self::BigInt(v1 * BigInt::from(v2))
            }
            (Self::Int(v1), Self::BigInt(v2)) => {
                Self::BigInt(v2 * BigInt::from(v1))
            }
            (Self::BigInt(v1), Self::BigInt(v2)) => Self::BigInt(v1 * v2),
            (Self::Int(v1), Self::Float(v2)) => Self::Float((v1 as f64) * v2),
            (Self::Float(v1), Self::Int(v2)) => Self::Float(v1 * (v2 as f64)),
            (Self::Int(v1), Self::BigFloat(v2)) => {
                Self::BigFloat(BigFloat::from_i128(v1) * v2)
            }
            (Self::BigFloat(v1), Self::Int(v2)) => {
                Self::BigFloat(v1 * BigFloat::from_i128(v2))
            }
            (Self::Float(v1), Self::Float(v2)) => Self::Float(v1 * v2),
            (Self::Float(v1), Self::BigFloat(v2)) => {
                Self::BigFloat(BigFloat::from_f64(v1) * v2)
            }
            (Self::BigFloat(v1), Self::Float(v2)) => {
                Self::BigFloat(v1 * BigFloat::from_f64(v2))
            }
            (Self::Float(v1), Self::BigInt(v2)) => {
                Self::BigFloat(BigFloat::from_f64(v1) * bigint_to_bigfloat(&v2))
            }
            (Self::BigInt(v1), Self::Float(v2)) => {
                Self::BigFloat(bigint_to_bigfloat(&v1) * BigFloat::from_f64(v2))
            }
            (Self::BigInt(v1), Self::BigFloat(v2)) => {
                Self::BigFloat(bigint_to_bigfloat(&v1) * v2)
            }
            (Self::BigFloat(v1), Self::BigInt(v2)) => {
                Self::BigFloat(v1 * bigint_to_bigfloat(&v2))
            }
            (Self::BigFloat(v1), Self::BigFloat(v2)) => Self::BigFloat(v1 * v2),
        }
    }
}

// Div
// Integer division always produces a Float to preserve the fractional part,
// unless the result is exact (remainder == 0).

impl core::ops::Div for Number {
    type Output = Self;
    fn div(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Self::Int(v1), Self::Int(v2)) => {
                if v2 != 0 && v1 % v2 == 0 {
                    Self::Int(v1 / v2)
                } else {
                    Self::Float((v1 as f64) / (v2 as f64))
                }
            }
            (Self::BigInt(v1), Self::Int(v2)) => {
                let bv2 = BigInt::from(v2);
                if !bv2.is_zero() && (&v1 % &bv2).is_zero() {
                    Self::maybe_narrow_bigint(v1 / bv2)
                } else {
                    Self::BigFloat(
                        bigint_to_bigfloat(&v1) / BigFloat::from_i128(v2),
                    )
                }
            }
            (Self::Int(v1), Self::BigInt(v2)) => {
                let bv1 = BigInt::from(v1);
                if !v2.is_zero() && (&bv1 % &v2).is_zero() {
                    Self::maybe_narrow_bigint(bv1 / v2)
                } else {
                    Self::BigFloat(
                        BigFloat::from_i128(v1) / bigint_to_bigfloat(&v2),
                    )
                }
            }
            (Self::BigInt(v1), Self::BigInt(v2)) => {
                if !v2.is_zero() && (&v1 % &v2).is_zero() {
                    Self::maybe_narrow_bigint(v1 / v2)
                } else {
                    Self::BigFloat(
                        bigint_to_bigfloat(&v1) / bigint_to_bigfloat(&v2),
                    )
                }
            }
            (Self::Int(v1), Self::Float(v2)) => Self::Float((v1 as f64) / v2),
            (Self::Float(v1), Self::Int(v2)) => Self::Float(v1 / (v2 as f64)),
            (Self::Int(v1), Self::BigFloat(v2)) => {
                Self::BigFloat(BigFloat::from_i128(v1) / v2)
            }
            (Self::BigFloat(v1), Self::Int(v2)) => {
                Self::BigFloat(v1 / BigFloat::from_i128(v2))
            }
            (Self::Float(v1), Self::Float(v2)) => Self::Float(v1 / v2),
            (Self::Float(v1), Self::BigFloat(v2)) => {
                Self::BigFloat(BigFloat::from_f64(v1) / v2)
            }
            (Self::BigFloat(v1), Self::Float(v2)) => {
                Self::BigFloat(v1 / BigFloat::from_f64(v2))
            }
            (Self::Float(v1), Self::BigInt(v2)) => {
                Self::BigFloat(BigFloat::from_f64(v1) / bigint_to_bigfloat(&v2))
            }
            (Self::BigInt(v1), Self::Float(v2)) => {
                Self::BigFloat(bigint_to_bigfloat(&v1) / BigFloat::from_f64(v2))
            }
            (Self::BigInt(v1), Self::BigFloat(v2)) => {
                Self::BigFloat(bigint_to_bigfloat(&v1) / v2)
            }
            (Self::BigFloat(v1), Self::BigInt(v2)) => {
                Self::BigFloat(v1 / bigint_to_bigfloat(&v2))
            }
            (Self::BigFloat(v1), Self::BigFloat(v2)) => Self::BigFloat(v1 / v2),
        }
    }
}

impl core::ops::Rem for Number {
    type Output = Self;
    fn rem(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Self::Int(v1), Self::Int(v2)) => Self::Int(v1 % v2),
            (Self::BigInt(v1), Self::Int(v2)) => {
                Self::maybe_narrow_bigint(v1 % BigInt::from(v2))
            }
            (Self::Int(v1), Self::BigInt(v2)) => {
                Self::maybe_narrow_bigint(BigInt::from(v1) % v2)
            }
            (Self::BigInt(v1), Self::BigInt(v2)) => {
                Self::maybe_narrow_bigint(v1 % v2)
            }
            (Self::Int(v1), Self::Float(v2)) => Self::Float((v1 as f64) % v2),
            (Self::Float(v1), Self::Int(v2)) => Self::Float(v1 % (v2 as f64)),
            (Self::Int(v1), Self::BigFloat(v2)) => {
                Self::maybe_narrow_bigfloat(BigFloat::from_i128(v1) % v2)
            }
            (Self::BigFloat(v1), Self::Int(v2)) => {
                Self::maybe_narrow_bigfloat(v1 % BigFloat::from_i128(v2))
            }
            (Self::Float(v1), Self::Float(v2)) => Self::Float(v1 % v2),
            (Self::Float(v1), Self::BigFloat(v2)) => {
                Self::maybe_narrow_bigfloat(BigFloat::from_f64(v1) % v2)
            }
            (Self::BigFloat(v1), Self::Float(v2)) => {
                Self::maybe_narrow_bigfloat(v1 % BigFloat::from_f64(v2))
            }
            (Self::Float(v1), Self::BigInt(v2)) => Self::maybe_narrow_bigfloat(
                BigFloat::from_f64(v1) % bigint_to_bigfloat(&v2),
            ),
            (Self::BigInt(v1), Self::Float(v2)) => Self::maybe_narrow_bigfloat(
                bigint_to_bigfloat(&v1) % BigFloat::from_f64(v2),
            ),
            (Self::BigInt(v1), Self::BigFloat(v2)) => {
                Self::maybe_narrow_bigfloat(bigint_to_bigfloat(&v1) % v2)
            }
            (Self::BigFloat(v1), Self::BigInt(v2)) => {
                Self::maybe_narrow_bigfloat(v1 % bigint_to_bigfloat(&v2))
            }
            (Self::BigFloat(v1), Self::BigFloat(v2)) => {
                Self::maybe_narrow_bigfloat(v1 % v2)
            }
        }
    }
}

// ── Neg ───────────────────────────────────────────────────────────────────────

impl core::ops::Neg for Number {
    type Output = Self;
    fn neg(self) -> Self::Output {
        match self {
            Self::Int(v) => Self::Int(-v),
            Self::Float(v) => Self::Float(-v),
            Self::BigInt(v) => Self::BigInt(-v),
            Self::BigFloat(v) => Self::BigFloat(-v),
        }
    }
}

macro_rules! impl_assign_op {
    ($trait:ident, $method:ident, $op:tt) => {
        impl core::ops::$trait for Number {
            fn $method(&mut self, rhs: Self) {
                *self = core::mem::take(self) $op rhs;
            }
        }
    };
}

impl Default for Number {
    fn default() -> Self {
        Self::Int(0)
    }
}

impl_assign_op!(AddAssign, add_assign, +);
impl_assign_op!(SubAssign, sub_assign, -);
impl_assign_op!(MulAssign, mul_assign, *);
impl_assign_op!(DivAssign, div_assign, /);
impl_assign_op!(RemAssign, rem_assign, %);

// ── abs / pow / sqrt ─────────────────────────────────────────────────────────

impl Number {
    /// TODO: Put me into the trait
    #[must_use]
    pub fn abs(self) -> Self {
        match self {
            Self::Int(v) => Self::Int(v.abs()),
            Self::Float(v) => Self::Float(v.abs()),
            Self::BigInt(v) => Self::BigInt(v.abs()),
            Self::BigFloat(v) => Self::BigFloat(v.abs()),
        }
    }

    // /// Integer exponentiation. The exponent is always a plain `u32` because
    // /// raising to a BigInt power is (almost always) not what you want.
    // pub fn powi(self, exp: u32) -> Self {
    //     match self {
    //         Self::Int(v) => Self::BigInt(BigInt::from(v).pow(exp)),
    //         Self::Float(v) => Self::Float(v.powi(exp as i32)),
    //         Self::BigInt(v) => Self::BigInt(v.pow(exp)),
    //         Self::BigFloat(v) => Self::BigFloat(
    //             v.powi(exp as i32, num_bigfloat::RoundingMode::Up),
    //         ),
    //     }
    // }

    /// TODO: Put me into the trait
    /// General floating-point power.
    #[must_use]
    pub fn powf(self, exp: Self) -> Self {
        let base = match self {
            Self::Int(v) => BigFloat::from_i128(v),
            Self::Float(v) => BigFloat::from_f64(v),
            Self::BigInt(v) => bigint_to_bigfloat(&v),
            Self::BigFloat(v) => v,
        };
        let e = match exp {
            Self::Int(v) => BigFloat::from_i128(v),
            Self::Float(v) => BigFloat::from_f64(v),
            Self::BigInt(v) => bigint_to_bigfloat(&v),
            Self::BigFloat(v) => v,
        };
        Self::maybe_narrow_bigfloat(base.pow(&e))
    }

    /// TODO: Put me into the trait
    #[must_use]
    pub fn sqrt(self) -> Self {
        match self {
            Self::Int(v) => {
                Self::maybe_narrow_bigfloat(BigFloat::from_i128(v).sqrt())
            }
            Self::Float(v) => Self::Float(v.sqrt()),
            Self::BigInt(v) => {
                Self::maybe_narrow_bigfloat(bigint_to_bigfloat(&v).sqrt())
            }
            Self::BigFloat(v) => Self::maybe_narrow_bigfloat(v.sqrt()),
        }
    }

    /// TODO: Put me into the trait
    #[must_use]
    /// Returns true for any zero value.
    pub fn is_zero(&self) -> bool {
        match self {
            Self::Int(v) => *v == 0,
            Self::Float(v) => *v == 0.0,
            Self::BigInt(v) => v.is_zero(),
            Self::BigFloat(v) => v.is_zero(),
        }
    }
    /// TODO: Put me into the trait
    #[must_use]
    /// Returns true when the value is an integer (no fractional part).
    pub fn is_integer(&self) -> bool {
        match self {
            Self::Int(_) | Self::BigInt(_) => true,
            Self::Float(v) => v.fract() == 0.0,
            Self::BigFloat(v) => v.frac().is_zero(),
        }
    }

    #[must_use]
    /// Floor division (always returns an integer type).
    pub fn floor_div(self, rhs: Self) -> Self {
        match (self, rhs) {
            (Self::Int(a), Self::Int(b)) => {
                // Rust's `/` truncates; replicate Python-style floor div.
                let d = a / b;
                let r = a % b;
                if (r != 0) && ((r < 0) != (b < 0)) {
                    Self::Int(d - 1)
                } else {
                    Self::Int(d)
                }
            }
            (a, b) => {
                // Fall back: compute via float then truncate.
                match a / b {
                    Self::Int(v) => Self::Int(v),
                    Self::Float(v) => Self::Int(v.floor() as i128),
                    Self::BigFloat(v) => Self::maybe_narrow_bigint(
                        BigInt::parse_bytes(
                            v.floor().to_string().as_bytes(),
                            10,
                        )
                        .unwrap_or_default(),
                    ),
                    other @ Self::BigInt(_) => other,
                }
            }
        }
    }
}

impl std::fmt::Display for Number {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&match self {
            Self::Int(val) => val.to_string(),
            Self::Float(val) => val.to_string(),
            Self::BigInt(val) => val.to_string(),
            Self::BigFloat(val) => val.to_string(),
        })
    }
}

impl std::hash::Hash for Number {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        match self {
            Self::Int(val) => val.hash(state),
            Self::BigInt(val) => val.hash(state),
            Self::Float(val) => val.to_bits().hash(state),
            Self::BigFloat(val) => val.to_string().hash(state),
        }
    }
}

fn bigint_to_bigfloat(n: &num_bigint::BigInt) -> num_bigfloat::BigFloat {
    // Safety: This should always be safe as float supports int
    unsafe {
        num_bigfloat::BigFloat::from_str(&n.to_string()).unwrap_unchecked()
    }
}

// TODO: Remove this potential UB and implement Ord manually for Value
impl Eq for Number {}
impl PartialOrd for Number {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Number {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self, other) {
            (Self::Int(v1), Self::Int(v2)) => v1.cmp(v2),
            (Self::BigInt(v1), Self::Int(v2)) => {
                v1.cmp(&num_bigint::BigInt::from(*v2))
            }
            (Self::Int(v1), Self::BigInt(v2)) => {
                num_bigint::BigInt::from(*v1).cmp(v2)
            }
            (Self::BigInt(v1), Self::BigInt(v2)) => v1.cmp(v2),
            (Self::Int(v1), Self::Float(v2)) => (*v1 as f64)
                .partial_cmp(v2)
                .unwrap_or(std::cmp::Ordering::Equal),
            (Self::Float(v1), Self::Int(v2)) => v1
                .partial_cmp(&(*v2 as f64))
                .unwrap_or(std::cmp::Ordering::Equal),
            (Self::Int(v1), Self::BigFloat(v2)) => {
                // Safety: cmp on [`BigFloat`](num_bigfloat::BigFloat) returns -1, 0, or 1 which are all in range of an i8
                num_bigfloat::BigFloat::from_i128(*v1)
                    .cmp(v2)
                    .map_or(std::cmp::Ordering::Equal, sign_to_ordering)
            }
            (Self::BigFloat(v1), Self::Int(v2)) => {
                // Safety: cmp on [`BigFloat`](num_bigfloat::BigFloat) returns -1, 0, or 1 which are all in range of an i8
                v1.cmp(&num_bigfloat::BigFloat::from_i128(*v2))
                    .map_or(std::cmp::Ordering::Equal, sign_to_ordering)
            }
            (Self::Float(v1), Self::Float(v2)) => {
                v1.partial_cmp(v2).unwrap_or(std::cmp::Ordering::Equal)
            }
            (Self::Float(v1), Self::BigFloat(v2)) => {
                num_bigfloat::BigFloat::from_f64(*v1)
                    .cmp(v2)
                    .map_or(std::cmp::Ordering::Equal, sign_to_ordering)
            }
            (Self::Float(v1), Self::BigInt(v2)) => {
                num_bigfloat::BigFloat::from_f64(*v1)
                    .cmp(&bigint_to_bigfloat(v2))
                    .map_or(std::cmp::Ordering::Equal, sign_to_ordering)
            }
            (Self::BigInt(v1), Self::Float(v2)) => bigint_to_bigfloat(v1)
                .cmp(&num_bigfloat::BigFloat::from_f64(*v2))
                .map_or(std::cmp::Ordering::Equal, sign_to_ordering),
            (Self::BigInt(v1), Self::BigFloat(v2)) => bigint_to_bigfloat(v1)
                .cmp(v2)
                .map_or(std::cmp::Ordering::Equal, sign_to_ordering),
            (Self::BigFloat(v1), Self::Float(v2)) => v1
                .cmp(&num_bigfloat::BigFloat::from_f64(*v2))
                .map_or(std::cmp::Ordering::Equal, sign_to_ordering),
            (Self::BigFloat(v1), Self::BigInt(v2)) => v1
                .cmp(&bigint_to_bigfloat(v2))
                .map_or(std::cmp::Ordering::Equal, sign_to_ordering),
            (Self::BigFloat(v1), Self::BigFloat(v2)) => {
                v1.cmp(v2).map_or(std::cmp::Ordering::Equal, sign_to_ordering)
            }
        }
    }
}

const fn sign_to_ordering(value: i16) -> core::cmp::Ordering {
    use core::cmp::Ordering;
    if value < 0 {
        Ordering::Less
    } else if value == 0 {
        Ordering::Equal
    } else if value > 0 {
        Ordering::Greater
    } else {
        unsafe { unreachable_unchecked() }
    }
}

// ── tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_int_int() {
        assert_eq!(Number::Int(3) + Number::Int(4), Number::Int(7));
    }

    #[test]
    fn sub_promotes_to_float() {
        // 3 - 4 is -1, stays Int
        assert_eq!(Number::Int(3) - Number::Int(4), Number::Int(-1));
    }

    #[test]
    fn div_exact_stays_int() {
        assert_eq!(Number::Int(9) / Number::Int(3), Number::Int(3));
    }

    #[test]
    fn div_inexact_promotes() {
        assert_eq!(Number::Int(7) / Number::Int(2), Number::Float(3.5));
    }

    #[test]
    fn neg() {
        assert_eq!(-Number::Int(5), Number::Int(-5));
        assert_eq!(-Number::Float(1.5), Number::Float(-1.5));
    }

    #[test]
    fn ordering() {
        assert!(Number::Int(1) < Number::Float(1.5));
        assert!(Number::Float(2.0) > Number::Int(1));
    }

    #[test]
    fn abs() {
        assert_eq!(Number::Int(-7).abs(), Number::Int(7));
        assert_eq!(Number::Float(-3.0).abs(), Number::Float(3.0));
    }

    #[test]
    fn sqrt_int() {
        assert_eq!(Number::Int(4).sqrt(), Number::Float(2.0));
    }

    #[test]
    fn assign_ops() {
        let mut n = Number::Int(10);
        n += Number::Int(5);
        assert_eq!(n, Number::Int(15));
        n -= Number::Int(3);
        assert_eq!(n, Number::Int(12));
        n *= Number::Int(2);
        assert_eq!(n, Number::Int(24));
        n /= Number::Int(4);
        assert_eq!(n, Number::Int(6));
        n %= Number::Int(4);
        assert_eq!(n, Number::Int(2));
    }

    #[test]
    fn floor_div() {
        // Python-style: -7 // 2 == -4
        assert_eq!(Number::Int(-7).floor_div(Number::Int(2)), Number::Int(-4));
        assert_eq!(Number::Int(7).floor_div(Number::Int(2)), Number::Int(3));
    }
}
