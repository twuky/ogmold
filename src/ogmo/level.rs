use crate::{Decal, Entity, ogmo::{Layer}};
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

pub enum LevelItem {
    Tile(i32),
    Grid(String),
    Entity(Entity),
    Decal(Decal),
}

impl LevelData {
    /// Get all intersections of the level with the given coordinates
    /// 
    /// Returns a list of all intersections of the level with the given coordinates
    /// 
    /// example:
    /// ```rust
    /// let intersections = level.get_intersections(0, 0);
    /// for intersection in intersections {
    ///     match intersection {
    ///         LevelItem::Tile(id) => todo!(),
    ///         LevelItem::Grid(id) => todo!(),
    ///         LevelItem::Entity(entity) => todo!(),
    ///         LevelItem::Decal(decal) => todo!(),
    ///     }
    /// }
    /// ```
    pub fn get_intersections(&self, x: i32, y: i32) -> Vec<LevelItem> {
        let mut intersections: Vec<LevelItem> = Vec::new();
        for layer in &self.layers {
            match layer {
                Layer::Tile(tile_layer) => {
                    let tile = tile_layer.get_tile(x, y);
                    if let Some(id) = tile {
                        intersections.push(LevelItem::Tile(id));
                    }
                },
                Layer::Grid(grid_layer) => {
                    let tile = grid_layer.get_tile(x, y);
                    if let Some(id) = tile {
                        intersections.push(LevelItem::Grid(id));
                    }
                },
                Layer::Entity(entity_layer) => {
                    for entity in &entity_layer.entities {
                        if entity.x / entity_layer.grid_cell_width == x
                            && entity.y / entity_layer.grid_cell_height == y {
                            intersections.push(LevelItem::Entity(entity.clone()));
                        }
                    }
                },
                Layer::Decal(decal_layer) => {
                    for decal in &decal_layer.decals {
                        if decal.x / decal_layer.grid_cell_width == x
                            && decal.y / decal_layer.grid_cell_height == y {
                            intersections.push(LevelItem::Decal(decal.clone()));
                        }
                    }
                },
            }
        };

        intersections
    }

    /// Get all tile or grid intersections of the level with the given coordinates,
    /// skipping any entities or decals
    /// 
    /// useful for applying properties to tiles, like collision
    pub fn get_tile_grid_intersections(&self, x: i32, y: i32) -> Vec<LevelItem> {
        let mut intersections: Vec<LevelItem> = Vec::new();
        for layer in &self.layers {
            match layer {
                Layer::Tile(tile_layer) => {
                    let tile = tile_layer.get_tile(x, y);
                    if let Some(id) = tile {
                        intersections.push(LevelItem::Tile(id));
                    }
                },
                Layer::Grid(grid_layer) => {
                    let tile = grid_layer.get_tile(x, y);
                    if let Some(id) = tile {
                        intersections.push(LevelItem::Grid(id));
                    }
                },
                Layer::Entity(_) => {},
                Layer::Decal(_) => {},
            }
        };

        intersections
    }
}