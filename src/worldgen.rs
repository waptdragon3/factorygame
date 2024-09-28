use crate::{chunk::{self, CHUNK_SIZE}, tile::TileManager};



pub trait Generator {
    fn gen_chunk(&mut self, coords:(i32, i32), tileman: &TileManager)-> chunk::Chunk;
}

#[derive(Default, Debug, Clone, Copy)]
pub struct LabGen;

impl Generator for LabGen {
    fn gen_chunk (&mut self, position:(i32, i32), tileman: &TileManager)-> chunk::Chunk {
        let grass = tileman.get_tile("grass").unwrap();
        let grass2 = tileman.get_tile("grass1").unwrap();

        let mut tiles = [(); CHUNK_SIZE*CHUNK_SIZE].map(|_| grass.clone());
        for i in 0..CHUNK_SIZE {
            for j in 0..CHUNK_SIZE {
                if i%2 != j%2 {
                    tiles[i*CHUNK_SIZE+j] = grass2.clone();
                }
            }
        }

        return chunk::Chunk::new(position, tiles);
    }
}