use macroquad::prelude::*;

#[macroquad::main("ogmo3-rs")]
async fn main() {
    let mut ogmo = ogmold::Ogmo::from_file("./assets/ogmo_test.ogmo").unwrap();
    ogmo.load_level("level1");
    let level = ogmo.get_level("level1").unwrap();

    let tiles_tex = load_texture("./assets/tiles.png").await.unwrap();
    let tileset_cells = tiles_tex.width() as i32 / 16;

    loop {
        clear_background(BLACK);

        for layer in level.tile_layers() {
            // meshes are greedy, chunking the output into similarly typed tiles
            // this is useful if you have an easier way to draw a texture with repeated tiles
            // however, mesh.get_draw_data() will return the same data type as tile.get_draw_data()
            for mesh in layer.mesh(tileset_cells) {
                let color = GRAY;
                for tile in mesh.get_draw_data(layer.grid_cell_width, layer.grid_cell_height) {
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
          
                  draw_texture_ex(&tiles_tex, x, y, DARKGRAY, params);
                }

                // here we also draw the shape of the mesh
                draw_rectangle_lines(mesh.pos_bl.x as f32 + 2.0, mesh.pos_bl.y as f32 + 2.0, mesh.width() as f32 - 4.0, mesh.height() as f32 - 4.0, 1.4, color);
            }
        }

        // similarly, grid layers have a mesh function that returns a list of meshes
        // more useful for collision objects, or other objects that are not tiles
        for layer in level.grid_layers() {
            for mesh in layer.mesh() {
                let mut color = BLUE;
                if mesh.value == "1" { color = RED; }
                if mesh.value == "2" { color = GREEN; }

                draw_rectangle_lines(mesh.tl().x as f32 + 1.0, mesh.tl().y as f32 + 1.0, mesh.tex_width() as f32 - 2.0, mesh.tex_height() as f32 - 2.0, 1.9, color);
            }
        }

        next_frame().await
    }
}