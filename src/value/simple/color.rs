#[mirl_macros::derive_all(zerocopy = false)]
/// A color
pub enum Color {
    /// R G B support
    Normal(u8, u8, u8),
    /// R G B A support
    WithAlpha(u8, u8, u8, u8),
    /// Grayscale
    GrayScale(u8),
}
