//! A 2D geometry module
//!
//! It contains basic shapes such as rectangles and circles, as well as vectors, lines, and a
//! universal Shape API. It also has matrix-backed Transforms for arbitrary constant-time 2D
//! transformations, such as rotating, scaling, or translating. 
//!
//! The Tilemap allows 2D storage of data in a world-like grid, and also moving objects at given
//! speeds around the map, which is highly useful for games like platformers.

mod vector;
mod line;
mod rect;
mod circ;
mod shape;
mod positioned;
mod tilemap;
mod transform;
mod util;
mod scalar;
pub use self::vector::Vector;
pub use self::line::Line;
pub use self::rect::Rectangle;
pub use self::circ::Circle;
pub use self::positioned::Positioned;
pub use self::shape::Shape;
pub use self::tilemap::{Tile, Tilemap};
pub use self::transform::Transform;
pub use self::util::{about_equal, lerp, lerp_angle};
pub use self::scalar::Scalar;
