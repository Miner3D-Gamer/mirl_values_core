
#[cfg_attr(feature = "mirl_derive", mirl_derive::derive_all)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
/// The type of an angle
pub enum AngleType {
    /// Full rotation 360
    Degrees,
    /// Full rotations pi*2
    Radians,
    /// Full rotation 400
    Grads,
    /// Full rotation 1.0
    Turns,
}

#[cfg_attr(feature = "mirl_derive", mirl_derive::derive_all(zerocopy = false))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
/// The length of 1d object
pub enum LengthType {
    /// Measure in nanometers
    Physical,
    /// Measure in pixels
    Digital,
    /// Relative unit
    Relative(RelativeLengthType),
}

#[cfg_attr(feature = "mirl_derive", mirl_derive::derive_all)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
/// Unit relative to context (css)
pub enum RelativeLengthType {
    /// em
    ElementFont,
    /// re
    RootElement,
    /// ch, the width of the 0 character
    WidthOf0,
    /// ex, the height of the lowercase x
    HeightOfX,
    /// lh
    LineHeight,
    /// rl
    RootLineHeight,
    /// vw
    ViewportWidth,
    /// vh
    ViewportHeight,
    /// vm, The smaller of the viewport width/height
    ViewportMin,
    /// vM, The bigger of the viewport width/height
    ViewportMax,
    /// vi
    InlineAxis,
    /// vb
    BlockAxis,
    /// %
    Percent,
}
