use std::ops::{Add, Sub};

pub mod chunk;
pub mod entity;
pub mod worldgen;
pub mod tile;


#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct ChunkCoord { pub x: i32, pub y: i32 }

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct TileCoord { pub x: i32, pub y: i32 }

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Coordinate { pub x: f32, pub y: f32 }

impl ChunkCoord {
    pub fn new (x: i32, y: i32) -> Self { Self {x, y} }
    pub fn within_chunk(self, tcoord: TileCoord) -> TileCoord {
        let ccoord: TileCoord = self.into();
        let coord = TileCoord::new(tcoord.x - ccoord.x, tcoord.y - ccoord.y);
        let res = TileCoord::new((coord.x + chunk::CHUNK_SIZE as i32) % chunk::CHUNK_SIZE as i32, (coord.y + chunk::CHUNK_SIZE as i32) % chunk::CHUNK_SIZE as i32);

        return res;
    }
}
impl TileCoord {
    pub fn new (x: i32, y: i32) -> Self { Self {x, y} }
    
}
impl Coordinate {
    pub fn new (x: f32, y: f32) -> Self { Self {x, y} }
}


impl From<TileCoord> for ChunkCoord {
    fn from(value: TileCoord) -> Self {
        ChunkCoord::new((value.x as f32 / chunk::CHUNK_SIZE as f32).floor() as i32, (value.y as f32 / chunk::CHUNK_SIZE as f32).floor() as i32)
    }
}
impl From<Coordinate> for ChunkCoord {
    fn from(value: Coordinate) -> Self {
        ChunkCoord::new((value.x / chunk::CHUNK_SIZE as f32).floor() as i32, (value.y / chunk::CHUNK_SIZE as f32).floor() as i32)
    }
}

impl From<ChunkCoord> for TileCoord {
    fn from(value: ChunkCoord) -> Self {
        TileCoord::new(value.x * chunk::CHUNK_SIZE as i32, value.y * chunk::CHUNK_SIZE as i32)
    }
}
impl From<Coordinate> for TileCoord {
    fn from(value: Coordinate) -> Self {
        TileCoord::new(value.x.floor() as i32, value.y.floor() as i32)
    }
}

impl From<ChunkCoord> for Coordinate {
    fn from(value: ChunkCoord) -> Self {
        Coordinate::new( (value.x * chunk::CHUNK_SIZE as i32) as f32, (value.y * chunk::CHUNK_SIZE as i32) as f32)
    }
}
impl From<TileCoord> for Coordinate {
    fn from(value: TileCoord) -> Self {
        Coordinate::new( value.x as f32, value.y as f32)
    }
}


impl Add<Coordinate> for Coordinate {
    type Output = Self;

    fn add(self, rhs: Coordinate) -> Self::Output {
        return Coordinate {x: self.x + rhs.x, y: self.y + rhs.y};
    }
}

impl Sub<Coordinate> for Coordinate {
    type Output = Self;

    fn sub(self, rhs: Coordinate) -> Self::Output {
        return Coordinate {x: self.x - rhs.x, y: self.y - rhs.y};
    }
}



pub struct Surface {
    pub chunks: Vec<chunk::Chunk>,
    pub entities: Vec<entity::Entity>,
    generator: Box<dyn worldgen::Generator>,
    pub camera_pos: Coordinate,
}

impl Surface {
    pub fn new<T>(generator: T) -> Surface
    where T: worldgen::Generator + 'static {
        Surface {
            chunks: vec![],
            entities: vec![],
            generator: Box::new(generator),
            camera_pos: Coordinate::new(0.0, 0.0)
        }
    }
    pub fn gen_chunk(&mut self, coord: ChunkCoord, tileman: &tile::TileManager) {
        let chunk = self.generator.gen_chunk(coord, tileman);
        self.chunks.push(chunk);
    }

    pub fn get_chunk(&self, coord: ChunkCoord) -> Option<&chunk::Chunk> {
        for c in &self.chunks {
            if c.position == coord {
                return Some(c);
            }
        }

        None
    }

    pub fn get_tile(&self, coord: TileCoord) -> Option<tile::Tile> {
        let chunk = self.get_chunk(coord.into());

        if let Some(c) = chunk {
            let tcoord = c.position.within_chunk(coord);
            let tile = c.get_tile(tcoord);
            if let Ok(tile) = tile {
                return Some(tile);
            }
            else {
                return None;
            }
        }
        else {
            //println!("Missing chunk: {:?}", ChunkCoord::from(coord));
            return None;
        }
    }
}