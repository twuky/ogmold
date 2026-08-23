use crate::{ogmo::{EntityDefinition, LayerDefinition, TilesetDefinition, Value, Vec2}};
use nanoserde::DeJson;

#[derive(Debug, Clone, DeJson)]
pub struct ProjectData {
    /// The Name of the Ogmo Project
    pub name: String,
    /// Array of paths that hold the Project's Levels
    #[nserde(rename = "levelPaths")]
    pub level_paths: Vec<String>,
    /// The Project's background color
    #[nserde(rename = "backgroundColor")]
    pub background_color: String,
    /// The color of the grid displayed in the Project's Editor
    #[nserde(rename = "gridColor")]
    pub grid_color: String,
    /// Flag to set whether the Project describes rotations in Radians or Degrees.
	/// If set to `true`; its in Radians. Otherwise it is in Degrees.
    #[nserde(rename = "anglesRadians")]
    pub angles_radians: bool,
    /// Maximum Depth that the Editor will search for files for its File Tree.
    #[nserde(rename = "directoryDepth")]
    pub directory_depth: i32,
    /// Default size of newly created levels in the Editor.
    #[nserde(rename = "layerGridDefaultSize")]
    pub level_default_size: Vec2,
    /// Minimum size a level can be.
    #[nserde(rename = "levelMinSize")]
    pub level_min_size: Vec2,
    /// Maximum size a level can be.
    #[nserde(rename = "levelMaxSize")]
    pub level_max_size: Vec2,
    /// Array of Value Templates for the Project's Levels.
    #[nserde(rename = "levelValues")]
    pub level_values: Vec<Value>,
    /// Sets the default exported file type of a Level.
    #[nserde(rename = "defaultExportMode")]
    pub default_export_mode: String,
    /// Array containing all of the Project's available Entity Tags.
    #[nserde(rename = "entityTags")]
    pub entity_tags: Vec<String>,
    /// Array containing all of the Project's available Layer Templates.
    pub layers: Vec<LayerDefinition>,
    /// Array containing all of the Project's Entities Templates.
    pub entities: Vec<EntityDefinition>,
    /// Array containing all of the Project's available Tilesets.
    pub tilesets: Vec<TilesetDefinition>,
}
