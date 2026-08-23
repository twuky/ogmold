use nanoserde::DeJson;
use std::collections::HashMap;

use crate::ogmo::EntityValue;
use crate::ogmo::Value;
use crate::ogmo::Vec2;

/// Individual Entity data
#[derive(Debug, Clone, DeJson)]
pub struct Entity {
    pub name: String,
    pub id: i32,
    #[nserde(rename = "_eid")]
    pub eid: String,
    pub x: i32,
    pub y: i32,
    pub width: Option<i32>,
    pub height: Option<i32>,
    #[nserde(rename = "originX")]
    pub origin_x: Option<i32>,
    #[nserde(rename = "originY")]
    pub origin_y: Option<i32>,
    #[nserde(default)]
    pub rotation: f32,
    #[nserde(rename = "flippedX", default)]
    pub flipped_x: bool,
    #[nserde(rename = "flippedY", default)]
    pub flipped_y: bool,
    pub values: HashMap<String, EntityValue>,
}

impl Entity {

    pub fn get_string(&self, key: &str) -> Option<&str> {
        self.values.get(key).and_then(|v| v.as_str())
    }

    pub fn get_bool(&self, key: &str) -> Option<bool> {
        self.values.get(key).and_then(|v| v.as_bool())
    }

    pub fn get_i64(&self, key: &str) -> Option<i64> {
        self.values.get(key).and_then(|v| v.as_i64())
    }

    pub fn get_f64(&self, key: &str) -> Option<f64> {
        self.values.get(key).and_then(|v| v.as_f64())
    }
}

#[derive(Debug, Clone, DeJson)]
pub struct EntityDefinition {
    #[nserde(rename = "exportID")]
    pub export_id: String,
    pub name: String,
    pub limit: i32,
    pub size: Vec2,
    pub origin: Vec2,
    #[nserde(rename = "originAnchored")]
    pub origin_anchored: bool,
    pub shape: EntityShape,
    pub color: String,
    #[nserde(rename = "tileX")]
    pub tile_x: bool,
    #[nserde(rename = "tileY")]
    pub tile_y: bool,
    #[nserde(rename = "tileSize")]
    pub tile_size: Vec2,
    #[nserde(rename = "resizeableX")]
    pub resizeable_x: bool,
    #[nserde(rename = "resizeableY")]
    pub resizeable_y: bool,
    pub rotatable: bool,
    #[nserde(rename = "rotationDegrees")]
    pub rotation_degrees: i32,
    #[nserde(rename = "canFlipX")]
    pub can_flip_x: bool,
    #[nserde(rename = "canFlipY")]
    pub can_flip_y: bool,
    #[nserde(rename = "canSetColor")]
    pub can_set_color: bool,
    #[nserde(rename = "hasNodes")]
    pub has_nodes: bool,
    #[nserde(rename = "nodeLimit")]
    pub node_limit: i32,
    #[nserde(rename = "nodeDisplay")]
    pub node_display: i32,
    #[nserde(rename = "nodeGhost")]
    pub node_ghost: bool,
    pub tags: Vec<String>,
    pub values: Vec<Value>,
}

#[derive(Debug, Clone, DeJson)]
pub struct EntityShape {
    pub label: String,
    pub points: Vec<Vec2>,
}
