use macroquad::prelude::*;
use ogmold::Level;

#[macroquad::main("ogmo3-rs")]
async fn main() {
    let level = Level::from_file("./assets/level1.json").unwrap();

    let tiles_tex = load_texture("./assets/tiles.png").await.unwrap();
    let tileset_cells = tiles_tex.width() as i32 / 16;

    loop {
        clear_background(BLACK);

        for layer in level.tile_layers() {
            for tile in layer.get_draw_data(tileset_cells) {
                let x = tile.pos_bl.x as f32;
                let y = tile.pos_bl.y as f32;

                let params = DrawTextureParams { 
                    source: Some(Rect { 
                        x: tile.tex_bl.x as f32,
                        y: tile.tex_bl.y as f32,
                        w: tile.tex_width() as f32,
                        h: tile.tex_height() as f32,
                     }), 
                    ..Default::default()
                };
                draw_texture_ex(&tiles_tex, x, y, WHITE, params);
            }
        }

        for layer in level.entity_layers() {
            for entity in layer.entities.iter() {
              let mut color = WHITE;
              // values on entities are stored as KV pairs
              let c = entity.values.get("color").unwrap();
              println!("color: {:?}", c);
              // each value may only be one of the following types
              // String, Bool, Float, Int
              // this is independent of the type of the value in the .ogmo file, which includes
              // only the definitions for the editor
              match c {
                ogmold::EntityValue::String(hex) => {
                  color = color_from_hex(hex);
                },
                ogmold::EntityValue::Bool(_) => {},
                ogmold::EntityValue::Float(_) => {},
                ogmold::EntityValue::Int(_) => {},
              }

              draw_circle(entity.x as f32 + 8.0, entity.y as f32 + 8.0, 8.0, color);
            }
        }
        next_frame().await
    }
}

fn color_from_hex(hex: &str) -> Color {
  let r = u8::from_str_radix(&hex[1..3], 16).unwrap();
  let g = u8::from_str_radix(&hex[3..5], 16).unwrap();
  let b = u8::from_str_radix(&hex[5..7], 16).unwrap();
  let a = u8::from_str_radix(&hex[7..9], 16).unwrap();
  Color::from_rgba(r, g, b, a)
}
