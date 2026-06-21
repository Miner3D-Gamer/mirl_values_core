#[cfg_attr(feature = "mirl_derive", mirl_derive::derive_all(zerocopy = false))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
/// A color
pub enum Color {
    /// R G B support
    Normal(u8, u8, u8),
    /// R G B A support
    WithAlpha(u8, u8, u8, u8),
    /// Grayscale
    GrayScale(u8),
}
