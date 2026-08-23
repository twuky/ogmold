use nanoserde::DeJson;
use nanoserde::DeJsonErr;
use nanoserde::DeJsonState;
use std::collections::HashMap;
use std::str::Chars;

use crate::ogmo::Decal;
use crate::ogmo::Entity;
use crate::ogmo::Value;
use crate::ogmo::Vec2;

#[derive(Debug, Clone)]
pub enum LayerDefinition {
    Tile {
        name: String,
        /// Size of each cell in the layer's grid
        grid_size: Vec2,
        export_id: String,
        /// Enum to determine whether a Tile Layer exports it's Tile Data with IDs or Coords
        export_mode: i64,
        /// Enum to determine whether a Tile or Grid Layer exports it's Data as a 1D Array or a 2D Array
        array_mode: i64,
        /// Name of this Layer's default Tilemap
        default_tileset: String,
    },
    Entity {
        name: String,
        /// Size of each cell in the layer's grid
        grid_size: Vec2,
        export_id: String,
        /// Array of Entity Tags that filters out any Entities that DO NOT have any of the Tags described
        required_tags: Vec<String>,
        /// Array of Entity Tags that filters out any Entities that DO have any of the Tags described
        excluded_tags: Vec<String>,
    },
    Grid {
        /// name of the grid layer
        name: String,
        /// Size of each cell in the layer's grid
        grid_size: Vec2,
        export_id: String,
        array_mode: i64,
        /// maps grid cell ID to a hex color. example:
        /// ```json
        /// "legend": {"0": "#00000000", "1": "#000000ff"}
        /// ```
        legend: HashMap<String, String>,
    },
    Decal {
        name: String,
        /// Size of each cell in the layer's grid
        grid_size: Vec2,
        export_id: String,

        /// Directory to search for Decal images.
        folder: String,
        ///  Flag to set whether image sequences are included as available Decals
        include_image_sequence: bool,
        /// Flag to set whether the Decal can be scaled
        scaleable: bool,
        /// Flag to set whether the Decal can be rotated
        rotatable: bool,
        /// Array of Value Templates for a Decal Layer
        values: Vec<Value>,
    },
}

impl DeJson for LayerDefinition {
    fn de_json(s: &mut DeJsonState, i: &mut Chars) -> Result<Self, DeJsonErr> {
        s.curly_open(i)?;

        let (mut definition, mut name, mut export_id, mut default_tileset): (
            Option<String>,
            Option<String>,
            Option<String>,
            Option<String>,
        ) = (None, None, None, None);
        let mut grid_size: Option<Vec2> = None;
        let (mut export_mode, mut array_mode): (Option<i64>, Option<i64>) = (None, None);
        let (mut required_tags, mut excluded_tags): (Option<Vec<String>>, Option<Vec<String>>) =
            (None, None);
        let mut legend: Option<HashMap<String, String>> = None;

        let mut folder: Option<String> = None;
        let mut include_image_sequence: Option<bool> = None;
        let mut scaleable: Option<bool> = None;
        let mut rotatable: Option<bool> = None;
        let mut values: Option<Vec<Value>> = None;

        while s.next_str().is_some() {
            match AsRef::<str>::as_ref(&s.strbuf) {
                "definition" => {
                    s.next_colon(i)?;
                    definition = Some(DeJson::de_json(s, i)?);
                }
                "name" => {
                    s.next_colon(i)?;
                    name = Some(DeJson::de_json(s, i)?);
                }
                "gridSize" => {
                    s.next_colon(i)?;
                    grid_size = Some(DeJson::de_json(s, i)?);
                }
                "exportID" => {
                    s.next_colon(i)?;
                    export_id = Some(DeJson::de_json(s, i)?);
                }
                "exportMode" => {
                    s.next_colon(i)?;
                    export_mode = Some(DeJson::de_json(s, i)?);
                }
                "arrayMode" => {
                    s.next_colon(i)?;
                    array_mode = Some(DeJson::de_json(s, i)?);
                }
                "defaultTileset" => {
                    s.next_colon(i)?;
                    default_tileset = Some(DeJson::de_json(s, i)?);
                }
                "requiredTags" => {
                    s.next_colon(i)?;
                    required_tags = Some(DeJson::de_json(s, i)?);
                }
                "excludedTags" => {
                    s.next_colon(i)?;
                    excluded_tags = Some(DeJson::de_json(s, i)?);
                }
                "legend" => {
                    s.next_colon(i)?;
                    legend = Some(DeJson::de_json(s, i)?);
                }
                "folder" => {
                    s.next_colon(i)?;
                    folder = Some(DeJson::de_json(s, i)?);
                }
                "includeImageSequence" => {
                    s.next_colon(i)?;
                    include_image_sequence = Some(DeJson::de_json(s, i)?);
                }
                "scaleable" => {
                    s.next_colon(i)?;
                    scaleable = Some(DeJson::de_json(s, i)?);
                }
                "rotatable" => {
                    s.next_colon(i)?;
                    rotatable = Some(DeJson::de_json(s, i)?);
                }
                "values" => {
                    s.next_colon(i)?;
                    values = Some(DeJson::de_json(s, i)?);
                }
                _ => {
                    s.next_colon(i)?;
                    s.whole_field(i)?;
                }
            }
            s.eat_comma_curly(i)?;
        }
        s.curly_close(i)?;

        fn req<T>(v: Option<T>, s: &DeJsonState, name: &str) -> Result<T, DeJsonErr> {
            v.ok_or_else(|| s.err_nf(name))
        }

        // get common fields
        let name = req(name, s, "name")?;
        let grid_size = req(grid_size, s, "gridSize")?;
        let export_id = req(export_id, s, "exportID")?;
        let def = req(definition, s, "definition")?;

        Ok(match def.as_str() {
            "tile" => LayerDefinition::Tile {
                name,
                grid_size,
                export_id,
                export_mode: req(export_mode, s, "exportMode")?,
                array_mode: req(array_mode, s, "arrayMode")?,
                default_tileset: req(default_tileset, s, "defaultTileset")?,
            },
            "entity" => LayerDefinition::Entity {
                name,
                grid_size,
                export_id,
                required_tags: required_tags.unwrap_or_default(),
                excluded_tags: excluded_tags.unwrap_or_default(),
            },
            "grid" => LayerDefinition::Grid {
                name,
                grid_size,
                export_id,
                array_mode: req(array_mode, s, "arrayMode")?,
                legend: legend.unwrap_or_default(),
            },
            "decal" => LayerDefinition::Decal {
                name,
                grid_size,
                export_id,
                folder: folder.unwrap_or_default(),
                include_image_sequence: include_image_sequence.unwrap_or_default(),
                scaleable: scaleable.unwrap_or_default(),
                rotatable: rotatable.unwrap_or_default(),
                values: values.unwrap_or_default(),
            },
            other => return Err(s.err_enum(other)),
        })
    }
}

#[derive(Debug, Clone)]
pub struct TileLayer {
    pub name: String,
    /// Export ID of this Layer
    pub eid: String,
    /// Offset of the Tile Layer in the X-axis in pixels
    pub offset_x: i32,
    /// Offset of the Tile Layer in the Y-axis in pixels
    pub offset_y: i32,
    /// Width of a single Grid Cell in the Tile Layer.
    pub grid_cell_width: i32,
    /// Height of a single Grid Cell in the Tile Layer.
    pub grid_cell_height: i32,
    /// Number of Grid Cells in the X-axis.
    pub grid_cells_x: i32,
    /// Number of Grid Cells in the Y-axis.
    pub grid_cells_y: i32,

    ///  Name of this Layer's Tilemap. May differ from the default tileset of the project's layer definition.
    pub tileset: String,
    /// Enum to determine whether a Tile Layer exports it's Tile Data with IDs or Coords
    pub export_mode: i32,
    /// Enum to determine whether a Tile or Grid Layer exports it's Data as a 1D Array or a 2D Array
    pub array_mode: i32,
    /// Array of Tile IDs
    pub data: Option<Vec<i32>>,
    /// Array of Tile IDs in a 2D Array
    pub data_2d: Option<Vec<Vec<i32>>>,
    pub data_csv: Option<String>,
}

pub struct TileDrawData {
    /// position of the tile's bottom left corner in the world
    pub pos_bl: Vec2,
    /// position of the tile's top right corner in the world
    pub pos_tr: Vec2,

    /// position of the tile's bottom left corner in the tileset texture (pixels)
    pub tex_bl: Vec2,
    /// position of the tile's top right corner in the tileset texture (pixels)
    pub tex_tr: Vec2,

    /// ID of the tile in the Tile Layer
    pub id: i32,
}

impl TileDrawData {
    pub fn tex_width(&self) -> i32 {
        self.tex_tr.x - self.tex_bl.x
    }

    pub fn tex_height(&self) -> i32 {
        self.tex_tr.y - self.tex_bl.y
    }
}

impl TileLayer {
    /// Iterate over all Tiles in this layer in order, returning their position and texture coordinates
    ///
    /// `tileset_cells_x` - Number of cells in the tileset texture. 
    /// Used to calculate the texture coordinates of each tile. 
    /// If your tileset image contains 8 tiles in a row, enter 8, etc
    /// or `texture.width / tile_size.x`
    pub fn get_draw_data(&self, tileset_cells_x: i32) -> impl Iterator<Item = TileDrawData> {
        self.data.as_ref().unwrap().iter().enumerate().map(move |(i, id)| {
            let x = i as i32 % self.grid_cells_x;
            let y = i as i32 / self.grid_cells_x;

            let pos_x = x * self.grid_cell_width + self.offset_x;
            let pos_y = y * self.grid_cell_height + self.offset_y;

            let pos_bl = Vec2{ x: pos_x, y: pos_y };
            let pos_tr = Vec2{ x: pos_bl.x + self.grid_cell_width, y: pos_bl.y + self.grid_cell_height };

            let tex_bl = Vec2{ x: *id % tileset_cells_x * self.grid_cell_width, y: *id / tileset_cells_x * self.grid_cell_height };
            let tex_tr = Vec2{ x: tex_bl.x + self.grid_cell_height, y: tex_bl.y + self.grid_cell_height };

            TileDrawData{ pos_bl, pos_tr, tex_bl, tex_tr, id: *id }
        })
    }

    /// Iterate over all Tiles in this layer in order, returning their position and texture coordinates.
    /// This flips the map vertically so that the "origin" is at the bottom left of the map.
    ///
    /// `tileset_cells_x` - Number of cells in the tileset texture. 
    /// Used to calculate the texture coordinates of each tile. 
    /// If your tileset image contains 8 tiles in a row, enter 8, etc
    /// or `texture.width / tile_size.x`
    pub fn get_draw_data_flipped(&self, tileset_cells_x: i32) -> impl Iterator<Item = TileDrawData> {
        self.data.as_ref().unwrap().iter().enumerate().map(move |(i, id)| {
            let x = i as i32 % self.grid_cells_x;
            let y = i as i32 / self.grid_cells_x;

            let pos_x = x * self.grid_cell_width + self.offset_x;
            let pos_y = -(y * self.grid_cell_height + self.offset_y) + self.grid_cells_y * self.grid_cell_height;

            let pos_bl = Vec2{ x: pos_x, y: pos_y };
            let pos_tr = Vec2{ x: pos_bl.x + self.grid_cell_width, y: pos_bl.y - self.grid_cell_height };

            let tex_bl = Vec2{ x: *id % tileset_cells_x * self.grid_cell_width, y: *id / tileset_cells_x * self.grid_cell_height };
            let tex_tr = Vec2{ x: tex_bl.x + self.grid_cell_height, y: tex_bl.y + self.grid_cell_height };

            TileDrawData{ pos_bl, pos_tr, tex_bl, tex_tr, id: *id }
        })
    }

    pub fn get_tile(&self, x: usize, y: usize) -> Option<i32> {
        let id = x + y * self.grid_cells_x as usize;
        if let Some(data) = &self.data {
            if id >= data.len() {
                return None;
            }
            return Some(data[id as usize]);
        }
        if let Some(data_2d) = &self.data_2d {
            return data_2d.get(x as usize).and_then(|v| v.get(y as usize).copied());
        }
        None
    }
}

#[derive(Debug, Clone)]
pub struct GridLayer {
    pub name: String,
    pub eid: String,
    pub offset_x: i32,
    pub offset_y: i32,
    pub grid_cell_width: i32,
    pub grid_cell_height: i32,
    pub grid_cells_x: i32,
    pub grid_cells_y: i32,

    pub grid: Option<Vec<String>>,
    pub grid_2d: Option<Vec<Vec<String>>>,
    /// Enum to determine whether a Grid Layer exports it's Data as a 1D Array or a 2D Array
    pub array_mode: i32,
}

#[derive(Debug, Clone)]
pub struct EntityLayer {
    pub name: String,
    pub eid: String,
    pub offset_x: i32,
    pub offset_y: i32,
    pub grid_cell_width: i32,
    pub grid_cell_height: i32,
    pub grid_cells_x: i32,
    pub grid_cells_y: i32,

    pub entities: Vec<Entity>,
}

#[derive(Debug, Clone)]
pub struct DecalLayer {
    pub name: String,
    pub eid: String,
    pub offset_x: i32,
    pub offset_y: i32,
    pub grid_cell_width: i32,
    pub grid_cell_height: i32,
    pub grid_cells_x: i32,
    pub grid_cells_y: i32,

    pub decals: Vec<Decal>,
}

#[derive(Debug, Clone)]
pub enum Layer {
    Tile(TileLayer),
    Grid(GridLayer),
    Entity(EntityLayer),
    Decal(DecalLayer),
}

impl Layer {
    pub fn as_tile(&self) -> Option<&TileLayer> {
        if let Layer::Tile(l) = self {
            Some(l)
        } else {
            None
        }
    }

    pub fn as_grid(&self) -> Option<&GridLayer> {
        if let Layer::Grid(l) = self {
            Some(l)
        } else {
            None
        }
    }
    
    pub fn as_entity(&self) -> Option<&EntityLayer> {
        if let Layer::Entity(l) = self {
            Some(l)
        } else {
            None
        }
    }

    pub fn as_decal(&self) -> Option<&DecalLayer> {
        if let Layer::Decal(l) = self {
            Some(l)
        } else {
            None
        }
    }  
}

impl DeJson for Layer {
    fn de_json(s: &mut DeJsonState, i: &mut Chars) -> Result<Self, DeJsonErr> {
        s.curly_open(i)?;

        let mut name = None;
        let mut eid = None;
        let mut offset_x = None;
        let mut offset_y = None;
        let mut grid_cell_width = None;
        let mut grid_cell_height = None;
        let mut grid_cells_x = None;
        let mut grid_cells_y = None;

        let mut tileset = None;
        let mut export_mode = None;
        let mut array_mode = None;
        let mut data = None;
        let mut data_2d = None;
        let mut data_csv = None;
        let mut grid = None;
        let mut grid_2d = None;
        let mut decals = None;
        let mut entities = None;

        while s.next_str().is_some() {
            match AsRef::<str>::as_ref(&s.strbuf) {
                "name" => {
                    s.next_colon(i)?;
                    name = Some(DeJson::de_json(s, i)?);
                }
                "_eid" => {
                    s.next_colon(i)?;
                    eid = Some(DeJson::de_json(s, i)?);
                }
                "offsetX" => {
                    s.next_colon(i)?;
                    offset_x = Some(DeJson::de_json(s, i)?);
                }
                "offsetY" => {
                    s.next_colon(i)?;
                    offset_y = Some(DeJson::de_json(s, i)?);
                }
                "gridCellWidth" => {
                    s.next_colon(i)?;
                    grid_cell_width = Some(DeJson::de_json(s, i)?);
                }
                "gridCellHeight" => {
                    s.next_colon(i)?;
                    grid_cell_height = Some(DeJson::de_json(s, i)?);
                }
                "gridCellsX" => {
                    s.next_colon(i)?;
                    grid_cells_x = Some(DeJson::de_json(s, i)?);
                }
                "gridCellsY" => {
                    s.next_colon(i)?;
                    grid_cells_y = Some(DeJson::de_json(s, i)?);
                }

                "tileset" => {
                    s.next_colon(i)?;
                    tileset = Some(DeJson::de_json(s, i)?);
                }
                "exportMode" => {
                    s.next_colon(i)?;
                    export_mode = Some(DeJson::de_json(s, i)?);
                }
                "arrayMode" => {
                    s.next_colon(i)?;
                    array_mode = Some(DeJson::de_json(s, i)?);
                }
                "data" => {
                    s.next_colon(i)?;
                    data = Some(DeJson::de_json(s, i)?);
                }
                "data2D" => {
                    s.next_colon(i)?;
                    data_2d = Some(DeJson::de_json(s, i)?);
                }
                "dataCSV" => {
                    s.next_colon(i)?;
                    data_csv = Some(DeJson::de_json(s, i)?);
                }
                "grid" => {
                    s.next_colon(i)?;
                    grid = Some(DeJson::de_json(s, i)?);
                }
                "grid2D" => {
                    s.next_colon(i)?;
                    grid_2d = Some(DeJson::de_json(s, i)?);
                }
                "decals" => {
                    s.next_colon(i)?;
                    decals = Some(DeJson::de_json(s, i)?);
                }
                "entities" => {
                    s.next_colon(i)?;
                    entities = Some(DeJson::de_json(s, i)?);
                }
                _ => {
                    s.next_colon(i)?;
                    s.whole_field(i)?;
                }
            }
            s.eat_comma_curly(i)?;
        }
        s.curly_close(i)?;

        fn req<T>(v: Option<T>, s: &DeJsonState, name: &str) -> Result<T, DeJsonErr> {
            v.ok_or_else(|| s.err_nf(name))
        }

        let name = req(name, s, "name")?;
        let eid = req(eid, s, "eid")?;
        let offset_x = req(offset_x, s, "offsetX")?;
        let offset_y = req(offset_y, s, "offsetY")?;
        let grid_cell_width = req(grid_cell_width, s, "gridCellWidth")?;
        let grid_cell_height = req(grid_cell_height, s, "gridCellHeight")?;
        let grid_cells_x = req(grid_cells_x, s, "gridCellsX")?;
        let grid_cells_y = req(grid_cells_y, s, "gridCellsY")?;

        if let Some(tileset) = tileset {
            let export_mode = req(export_mode, s, "exportMode")?;
            let array_mode = req(array_mode, s, "arrayMode")?;
            Ok(Layer::Tile(TileLayer {
                name,
                eid,
                offset_x,
                offset_y,
                grid_cell_width,
                grid_cell_height,
                grid_cells_x,
                grid_cells_y,
                tileset,
                export_mode,
                array_mode,
                data,
                data_2d,
                data_csv,
            }))
        } else if let Some(grid) = grid {
            let array_mode = req(array_mode, s, "arrayMode")?;
            Ok(Layer::Grid(GridLayer {
                name,
                eid,
                offset_x,
                offset_y,
                grid_cell_width,
                grid_cell_height,
                grid_cells_x,
                grid_cells_y,
                grid,
                array_mode,
                grid_2d,
            }))
        } else if let Some(decals) = decals {
            Ok(Layer::Decal(DecalLayer {
                name,
                eid,
                offset_x,
                offset_y,
                grid_cell_width,
                grid_cell_height,
                grid_cells_x,
                grid_cells_y,
                decals,
            }))
        } else if let Some(entities) = entities {
            Ok(Layer::Entity(EntityLayer {
                name,
                eid,
                offset_x,
                offset_y,
                grid_cell_width,
                grid_cell_height,
                grid_cells_x,
                grid_cells_y,
                entities,
            }))
        } else {
            Err(s.err_enum("TileLayer, GridLayer, DecalLayer, EntityLayer"))
        }
    }
}
