use nanoserde::DeJson;

#[derive(Debug, Clone, DeJson)]
pub struct TilesetDefinition {
    /// Name of the Tileset
    pub label: String,
    /// Path to the Tileset image, relative to the Project's path.
    pub path: String,
    /// Base64 version of the Tileset image.
    pub image: String,
    /// Width of a single Tile in the Tileset.
    #[nserde(rename = "tileWidth")]
    pub tile_width: i32,
    /// Height of a single Tile in the Tileset.
    #[nserde(rename = "tileHeight")]
    pub tile_height: i32,
    /// Empty pixels that separate each Tile on the X axis in this Tileset image.
    #[nserde(rename = "tileSeparationX")]
    pub tile_separation_x: i32,
    /// Empty pixels that separate each Tile on the Y axis in this Tileset image.
    #[nserde(rename = "tileSeparationY")]
    pub tile_separation_y: i32,
}
