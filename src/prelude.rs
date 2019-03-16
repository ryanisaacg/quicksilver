//! A collection of imports for convenience

pub use crate::{
    Error, Future, Result, load_file,
    combinators::{join_all, result},
    geom::*,
    graphics::{Background::{self, *}, Color, Image, },
    input::{ButtonState, Key, MouseButton},
    lifecycle::{Asset, Event, Settings, State, Window, run}
};
#[cfg(feature = "fonts")]
pub use crate::graphics::{Font, FontStyle};
#[cfg(feature = "gamepads")]
pub use crate::input::{GamepadAxis, GamepadButton};
#[cfg(feature = "sounds")]
pub use crate::sound::Sound;

