use notan::app::{Graphics, Texture};

#[derive(Clone, Debug)]
pub struct Tile {
    pub name: String,
    pub texture: Texture,
}


pub struct TileManager {
    tiles: Vec<Tile>
}

impl TileManager {
    pub fn new() -> TileManager {
        Self { tiles: vec![] }
    }
    pub fn register_tile(&mut self, tile_name: &str, texture: &str, gfx: &mut Graphics) {

        let bytes = &get_file_as_byte_vec(&texture.to_owned());        


        let texture = gfx
                        .create_texture()
                        .from_image(bytes)
                        .build()
                        .unwrap();

        let tile = Tile {name: tile_name.to_owned(), texture};
        self.tiles.push(tile);

    }

    pub fn get_tile(&self, tile_name: &str) -> Result<Tile, ()> {
        for t in &self.tiles {
            if t.name.eq_ignore_ascii_case(tile_name) {
                return Ok(t.clone());
            }
        }
        return Err(());
    }
}





fn get_file_as_byte_vec(filename: &String) -> Vec<u8> {
    println!("Opening File: {}{}",std::env::current_dir().unwrap().display(), filename);
    let mut f = std::fs::File::open(&filename).expect("no file found");
    let metadata = std::fs::metadata(&filename).expect("unable to read metadata");
    let mut buffer = vec![0; metadata.len() as usize];
    std::io::Read::read(&mut f, &mut buffer).expect("buffer overflow");

    buffer
}