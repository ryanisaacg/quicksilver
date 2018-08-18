#[repr(u32)]
#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq)]
/// The way the colors are blended when drawing on top of other color
///
/// Blend modes only apply to RGB values
pub enum BlendMode {
    /// Add the color being drawn onto and the color being drawn
    ///
    /// Adding red and blue will produce purple for example
    Additive,
    /// Subtract the color being drawn onto and the color being drawn
    ///
    /// Subtracting red from purple will produce blue for example
    Subtractive,
    /// Take the minimum of each component of the color
    ///
    /// Purple and red will produce red, blue and red will produce black
    Minimum,
    /// Take the maximum of each component of the color
    ///
    /// Purple and red will produce purple, blue and red will produce purple
    Maximum
}