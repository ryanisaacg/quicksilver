/// The way the images should change when drawn at a scale
#[repr(u32)]
#[derive(Copy, Clone, Debug)]
pub enum ImageScaleStrategy {
    /// The image should attempt to preserve each pixel as accurately as possible
    Pixelate,
    /// The image should attempt to preserve the overall picture by blurring
    Blur
}

impl Default for ImageScaleStrategy {
    fn default() -> ImageScaleStrategy {
        ImageScaleStrategy::Pixelate
    }
}
