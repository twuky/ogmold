mod decal;
mod entity;
mod layer;
mod level;
mod project;
mod tileset;
mod value;

pub use decal::*;
pub use entity::*;
pub use layer::*;
pub use level::*;
use nanoserde::DeJson;
pub use project::*;
pub use tileset::*;
pub use value::*;

#[derive(Debug, Clone, Copy, DeJson)]
pub struct Vec2 {
    pub x: i32,
    pub y: i32,
}
