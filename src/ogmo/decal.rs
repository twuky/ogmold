use nanoserde::DeJson;

/// Individual Decal data
#[derive(Debug, Clone, DeJson)]
pub struct Decal {
    pub x: i32,
    pub y: i32,
    pub texture: String,
    #[nserde(rename = "scaleX")]
    pub scale_x: Option<f32>,
    #[nserde(rename = "scaleY")]
    pub scale_y: Option<f32>,
    pub rotation: Option<f32>,
}
