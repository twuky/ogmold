use crate::ogmo::Layer;
use nanoserde::DeJson;

#[derive(Debug, Clone, DeJson)]
pub struct LevelData {
    /// Width of the level in cells
    pub width: i32,
    /// Height of the level in cells
    pub height: i32,
    /// Offset of the Level on the X axis. Useful for loading multiple chunked Levels.
    #[nserde(rename = "offsetX")]
    pub offset_x: i32,
    /// Offset of the Level on the Y axis. Useful for loading multiple chunked Levels.
    #[nserde(rename = "offsetY")]
    pub offset_y: i32,
    /// Array of Layers in the Level
    pub layers: Vec<Layer>,
}
