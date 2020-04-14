use golem::GolemError;
use image::ImageError;

use std::error::Error;
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::io::Error as IOError;

/// An error generated by Quicksilver or one of its dependencies
// TODO: should this be #[non_exhaustive]
#[derive(Debug)]
pub enum QuicksilverError {
    /// Some image-parsing operation failed
    ImageError(ImageError),
    /// A file wasn't loaded correctly
    IOError(IOError),
    /// An error from within the graphics backend
    ///
    /// This is almost always a system or internal error, and probably means you should open a bug
    /// report
    GraphicsError(GolemError),
    /// An image was passed to a Surface with a non-exclusive reference
    SurfaceImageError,
    /// A surface operation was attempted with no image bound to the surface
    NoSurfaceImageBound,
    #[cfg(feature = "font")]
    FontError(FontError),
}

#[cfg(feature = "font")]
#[derive(Debug)]
pub enum FontError {
    /// A non-renderable glyph was passed to a font rendering function
    ///
    /// Generally this means the glyph was not included in the font, e.g. passing a non-ASCII
    /// character to an ASCII-only font
    NonRenderableGlyph(elefont::Glyph),
    /// The string passed to the font (at the given size) is too large to render
    ///
    /// Because glyphs are cached on the GPU, a given character or word may be too large to render.
    /// This is unlikely to be an issue for most applications, but if a character is rendered at a
    /// size larger than 2048x2048, this error may occur.
    StringTooLarge,
}

impl From<ImageError> for QuicksilverError {
    fn from(err: ImageError) -> QuicksilverError {
        QuicksilverError::ImageError(err)
    }
}

impl From<IOError> for QuicksilverError {
    fn from(err: IOError) -> QuicksilverError {
        QuicksilverError::IOError(err)
    }
}

impl From<GolemError> for QuicksilverError {
    fn from(err: GolemError) -> QuicksilverError {
        QuicksilverError::GraphicsError(err)
    }
}

#[cfg(feature = "font")]
impl From<FontError> for QuicksilverError {
    fn from(err: FontError) -> QuicksilverError {
        QuicksilverError::FontError(err)
    }
}

impl Display for QuicksilverError {
    fn fmt(&self, fmt: &mut Formatter) -> FmtResult {
        match self {
            QuicksilverError::ImageError(e) => e.fmt(fmt),
            QuicksilverError::IOError(e) => e.fmt(fmt),
            QuicksilverError::GraphicsError(e) => e.fmt(fmt),
            QuicksilverError::SurfaceImageError => write!(
                fmt,
                "An image was passed to Surface with non-exclusive reference"
            ),
            QuicksilverError::NoSurfaceImageBound => write!(
                fmt,
                "A surface operation was attempted with no image bound to the surface"
            ),
            QuicksilverError::FontError(FontError::NonRenderableGlyph(g)) => write!(
                fmt,
                "This glyph cannot be rendered: {:?}",
                g
            ),
            QuicksilverError::FontError(FontError::StringTooLarge) => write!(
                fmt,
                "A word or glyph passed to a font was too large to render."
            ),
        }
    }
}

impl Error for QuicksilverError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            QuicksilverError::ImageError(e) => Some(e),
            QuicksilverError::IOError(e) => Some(e),
            QuicksilverError::GraphicsError(e) => Some(e),
            _ => None,
        }
    }
}

/*#[cfg(all(not(target_arch = "wasm32"), feature = "gamepads"))]
use gilrs;
#[cfg(not(target_arch = "wasm32"))]
use glutin;
use image;
#[cfg(all(not(target_arch = "wasm32"), feature = "rodio"))]
use rodio;
use crate::graphics::{AtlasError, ImageError};
#[cfg(feature = "rusttype")]
use rusttype::Error as FontError;
#[cfg(feature = "saving")]
use crate::saving::SaveError;
#[cfg(feature = "sounds")]
use crate::sound::SoundError;
use std::{fmt, error::Error, io::Error as IOError};

#[derive(Debug)]
/// An error generated by some Quicksilver subsystem
pub enum QuicksilverError {
    /// An error from an image atlas
    AtlasError(AtlasError),
    /// Creating or manipulating the OpenGL Context failed
    ContextError(String),
    /// An error from loading an image
    ImageError(ImageError),
    /// An error from loading a file
    IOError(IOError),
    /// An error when creating a gilrs context
    #[cfg(feature = "gamepads")]
    GilrsError(gilrs::Error),
    /// An error from loading a sound
    #[cfg(feature = "sounds")]
    SoundError(SoundError),
    /// A serialize or deserialize error
    #[cfg(feature = "saving")]
    SaveError(SaveError),
    /// There was an error loading a font file
    #[cfg(feature = "rusttype")]
    FontError(FontError),
}

impl fmt::Display for QuicksilverError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}

impl Error for QuicksilverError {
    fn description(&self) -> &str {
        match self {
            QuicksilverError::AtlasError(err) => err.description(),
            QuicksilverError::ContextError(string) => string.as_str(),
            QuicksilverError::ImageError(err) => err.description(),
            QuicksilverError::IOError(err) => err.description(),
            #[cfg(feature = "gamepads")]
            QuicksilverError::GilrsError(err) => err.description(),
            #[cfg(feature = "sounds")]
            QuicksilverError::SoundError(err) => err.description(),
            #[cfg(feature = "saving")]
            QuicksilverError::SaveError(err) => err.description(),
            #[cfg(feature = "rusttype")]
            QuicksilverError::FontError(err) => err.description(),
        }
    }

    fn cause(&self) -> Option<&dyn Error> {
        match self {
            QuicksilverError::AtlasError(err) => Some(err),
            QuicksilverError::ContextError(_) => None,
            QuicksilverError::ImageError(err) => Some(err),
            QuicksilverError::IOError(err) => Some(err),
            #[cfg(feature = "gamepads")]
            QuicksilverError::GilrsError(err) => Some(err),
            #[cfg(feature = "sounds")]
            QuicksilverError::SoundError(err) => Some(err),
            #[cfg(feature = "saving")]
            QuicksilverError::SaveError(err) => Some(err),
            #[cfg(feature = "rusttype")]
            QuicksilverError::FontError(err) => Some(err),
        }
    }
}

#[doc(hidden)]
#[cfg(feature = "sounds")]
impl From<SoundError> for QuicksilverError {
    fn from(err: SoundError) -> QuicksilverError {
        QuicksilverError::SoundError(err)
    }
}

#[doc(hidden)]
impl From<AtlasError> for QuicksilverError {
    fn from(err: AtlasError) -> QuicksilverError {
        QuicksilverError::AtlasError(err)
    }
}

#[cfg(feature = "saving")]
impl From<SaveError> for QuicksilverError {
    fn from(err: SaveError) -> QuicksilverError {
        QuicksilverError::SaveError(err)
    }
}

#[doc(hidden)]
impl From<image::ImageError> for QuicksilverError {
    fn from(img: image::ImageError) -> QuicksilverError {
        let image_error: ImageError = img.into();
        image_error.into()
    }
}

#[doc(hidden)]
#[cfg(all(feature = "sounds", not(target_arch = "wasm32")))]
impl From<rodio::decoder::DecoderError> for QuicksilverError {
    fn from(snd: rodio::decoder::DecoderError) -> QuicksilverError {
        let sound_error: SoundError = snd.into();
        sound_error.into()
    }
}

#[doc(hidden)]
#[cfg(feature = "rusttype")]
impl From<FontError> for QuicksilverError {
    fn from(fnt: FontError) -> QuicksilverError {
        QuicksilverError::FontError(fnt)
    }
}

#[cfg(not(target_arch = "wasm32"))]
const ROBUST_ERROR: &str = r#"Internal Quicksilver error: robustness not supported
Please file a bug report at https://github.com/ryanisaacg/quicksilver that includes:
- A minimum reproducing code snippet
- The error message above
"#;

#[cfg(not(target_arch = "wasm32"))]
fn creation_to_str(err: &glutin::CreationError) -> String {
    match err {
        glutin::CreationError::OsError(string) => string.to_owned(),
        glutin::CreationError::NotSupported(err) => (*err).to_owned(),
        glutin::CreationError::NoBackendAvailable(error) => error.to_string(),
        glutin::CreationError::RobustnessNotSupported => ROBUST_ERROR.to_owned(),
        glutin::CreationError::OpenGlVersionNotSupported => {
            "OpenGL version not supported".to_owned()
        }
        glutin::CreationError::NoAvailablePixelFormat => "No available pixel format".to_owned(),
        glutin::CreationError::PlatformSpecific(string) => string.to_owned(),
        glutin::CreationError::Window(error) => match error {
            glutin::WindowCreationError::OsError(string) => string.to_owned(),
            glutin::WindowCreationError::NotSupported => {
                "Window creation failed: not supported".to_owned()
            }
        },
        glutin::CreationError::CreationErrors(errors) => {
            format!("{:?}", errors)
        }
    }
}

#[doc(hidden)]
#[cfg(not(target_arch = "wasm32"))]
impl From<glutin::CreationError> for QuicksilverError {
    fn from(err: glutin::CreationError) -> QuicksilverError {
        QuicksilverError::ContextError(creation_to_str(&err))
    }
}

#[doc(hidden)]
#[cfg(not(target_arch = "wasm32"))]
impl From<glutin::ContextError> for QuicksilverError {
    fn from(err: glutin::ContextError) -> QuicksilverError {
        match err {
            glutin::ContextError::IoError(err) => QuicksilverError::IOError(err),
            glutin::ContextError::OsError(err) => QuicksilverError::ContextError(err),
            glutin::ContextError::ContextLost => {
                QuicksilverError::ContextError("Context lost".to_owned())
            }
        }
    }
}

#[doc(hidden)]
#[cfg(feature = "gamepads")]
impl From<gilrs::Error> for QuicksilverError {
    fn from(err: gilrs::Error) -> QuicksilverError {
        QuicksilverError::GilrsError(err)
    }
}*/
