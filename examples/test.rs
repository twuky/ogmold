use macroquad::prelude::*;

#[macroquad::main("ogmo3-rs")]
async fn main() {
    // load the project from the .ogmo file
    let mut ogmo = ogmold::Ogmo::from_file("./assets/ogmo_test.ogmo").unwrap();
    // the project keeps a list of valid levels by name, but does not load them by default
    // to request a level, use `load_level` and pass the name of the level
    ogmo.load_level("level1");
    // or you may load all levels at once
    // with ogmo.load_all_levels();

    // get the level by name
    // if multiple levels have the same name, the first one will be returned
    // otherwise, use `get_level_by_path` to get a level by a full path
    let level = ogmo.get_level("level1").unwrap();

    let tiles_tex = load_texture("./assets/tiles.png").await.unwrap();

    // the tilemap has a useful function to get draw data for all tiles in a layer
    // but it requires you calculate the number of cells in the tileset texture
    let tileset_cells = tiles_tex.width() as i32 / 16;

    loop {
        clear_background(BLACK);

        for layer in level.tile_layers() {
            // iterate over all tiles in the layer
            // this calcuates the world position and texture coordinates of each tile
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

        next_frame().await
    }
}
