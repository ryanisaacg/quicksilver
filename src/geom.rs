//! A 2D geometry module
//!
//! It contains basic shapes such as rectangles and circles, as well as vectors, lines, and a
//! universal Shape API. It also has matrix-backed Transforms for arbitrary constant-time 2D
//! transformations, such as rotating, scaling, or translating. 
//!
//! The Tilemap allows 2D storage of data in a world-like grid, and also moving objects at given
//! speeds around the map, which is highly useful for games like platformers.

mod vector;
mod rectangle;
mod circle;
mod objects;
mod shape;
mod transform;
mod util;
mod scalar;
pub use self::{
    vector::Vector,
    rectangle::Rectangle,
    circle::Circle,
    objects::{Line, Triangle},
    shape::Shape,
    transform::Transform,
    util::{about_equal, lerp, lerp_angle},
    scalar::Scalar
};
