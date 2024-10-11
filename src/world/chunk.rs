use crate::world::tile::Tile;

use super::{ChunkCoord, TileCoord};


pub const CHUNK_SIZE: usize = 32;



pub struct Chunk {
    pub tiles: [Tile; CHUNK_SIZE*CHUNK_SIZE],
    pub position: ChunkCoord
}

impl Chunk {
    pub fn new(position: ChunkCoord, tiles: [Tile; CHUNK_SIZE*CHUNK_SIZE]) -> Chunk {

        Chunk {
            tiles,
            position
        }
    }

    pub fn update(&mut self) {
        
    }

    pub fn get_tile(&self, coord: TileCoord) -> Result<Tile, ()> {
        if coord.x < 0 || coord.x > (CHUNK_SIZE-1) as i32 ||
            coord.y < 0 || coord.y > (CHUNK_SIZE-1) as i32 {
                return Err(());
        }
        else {
            return Ok(self.tiles[(coord.x * CHUNK_SIZE as i32 + coord.y) as usize].clone());
        }
    }
}