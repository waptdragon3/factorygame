use notan::{app::{Color, Graphics, Texture}, draw::*};

use crate::{tile::Tile, State};


pub const CHUNK_SIZE: usize = 32;

pub struct Chunk {
    is_dirty: bool,
    texture: Option<Texture>,
    tiles: [Tile; CHUNK_SIZE*CHUNK_SIZE],
    position: (i32, i32)
}

impl Chunk {
    pub fn new(position: (i32, i32), tiles: [Tile; CHUNK_SIZE*CHUNK_SIZE]) -> Chunk {

        Chunk {
            is_dirty: true,
            texture: None,
            tiles,
            position
        }
    }

    pub fn update(&mut self) {
        
    }

    pub fn update_texture(&mut self, gfx: &mut Graphics) {
        if self.is_dirty {
            println!("Rendering chunk ({},{})", self.position.0, self.position.1);
            const TILE_SIZE: f32 = 64.;
            let render_texture = gfx.create_render_texture(CHUNK_SIZE as u32 * TILE_SIZE as u32, CHUNK_SIZE as u32 * TILE_SIZE as u32).build().unwrap();
            let mut draw = render_texture.create_draw();
            for i in 0..CHUNK_SIZE {
                for j in 0..CHUNK_SIZE {
                    let current_tile = &self.tiles[i*CHUNK_SIZE+j];

                    draw.image(&current_tile.texture)
                    .position(i as f32*TILE_SIZE, j as f32*TILE_SIZE)
                    .size(TILE_SIZE, TILE_SIZE);

                }
            }
            gfx.render_to(&render_texture, &draw);
            self.texture = Some(render_texture.texture().clone());
            
            self.is_dirty = false;
        }
    }
}


pub fn draw_chunk(draw: &mut notan::draw::Draw, state: &State, chunk: &Chunk) {
    
    if let Some(tex) = &chunk.texture {

        let chunk_size_zoomed = state.zoom*CHUNK_SIZE as f32;

        draw.image(&tex)
        .position(chunk.position.0 as f32 * chunk_size_zoomed, chunk.position.1 as f32 * chunk_size_zoomed)
        .size(chunk_size_zoomed, chunk_size_zoomed);

        if state.options.debugsettings.chunk_border {

            for _i in 0..CHUNK_SIZE {
                /*
                draw.line(
                    (chunk.position.0 as f32 * chunk_size_zoomed + i as f32*state.zoom, chunk.position.1 as f32 * chunk_size_zoomed),
                    (chunk.position.0 as f32 * chunk_size_zoomed + i as f32*state.zoom, (chunk.position.1+1) as f32 * chunk_size_zoomed))
                    .color(Color::BLACK);

                draw.line(
                    (chunk.position.0 as f32 * chunk_size_zoomed, chunk.position.1 as f32 * chunk_size_zoomed + i as f32*state.zoom),
                    ((chunk.position.0+1) as f32 * chunk_size_zoomed, chunk.position.1 as f32 * chunk_size_zoomed + i as f32*state.zoom))
                    .color(Color::BLACK);
                */
            }

            draw.rect((chunk.position.0 as f32 * chunk_size_zoomed, chunk.position.1 as f32 * chunk_size_zoomed), (chunk_size_zoomed, chunk_size_zoomed))
            .fill_color(Color::TRANSPARENT)
            .stroke(2.)
            .stroke_color(Color::RED);
        }
    }
}
