//! A 2D geometry module
//!
//! It contains basic shapes such as rectangles and circles, as well as
//! vectors, lines, and a universal Shape API. It also has matrix-backed
//! Transforms for arbitrary constant-time 2D transformations, such as
//! rotating, scaling, or translating.
//!
//! The Tilemap allows 2D storage of data in a world-like grid, and also moving
//! objects at given speeds around the map, which is highly useful for games
//! like platformers.

mod circle;
mod positioned;
mod rectangle;
mod scalar;
mod shape;
mod tilemap;
mod transform;
mod util;
mod vector;
pub use self::{
    circle::Circle,
    positioned::Positioned,
    rectangle::Rectangle,
    scalar::Scalar,
    shape::Shape,
    tilemap::{Tile, Tilemap},
    transform::Transform,
    util::{about_equal, lerp, lerp_angle},
    vector::Vector,
};
