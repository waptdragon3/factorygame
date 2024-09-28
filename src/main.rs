use notan::{draw::{CreateDraw, DrawConfig}, prelude::*};

use notan::math::{Mat3, Vec2};
use tile::TileManager;

mod tile;
mod chunk;
mod entity;
mod worldgen;

mod graphics;


enum Task{
    GenChunk((i32,i32)),
    RemoveEntity(u32),
    PlaceEntity(u32, entity::Entity),

}




struct Surface {
    chunks: Vec<chunk::Chunk>,
    entities: Vec<entity::Entity>,
    generator: Box<dyn worldgen::Generator>
}

impl Surface {
    fn new<T>(generator: T) -> Surface
    where T: worldgen::Generator + 'static {
        Surface {
            chunks: vec![],
            entities: vec![],
            generator: Box::new(generator)
        }
    }
    fn gen_chunk(&mut self, coord: (i32, i32), tileman: &TileManager) {
        let chunk = self.generator.gen_chunk(coord, tileman);
        self.chunks.push(chunk);
    }
}



#[derive(Copy, Clone, Default)]
struct DebugSettings{
    pub chunk_border: bool
}

#[derive(Copy, Clone, Default)]
struct Options {
    pub debugsettings: DebugSettings
}



#[derive(AppState)]
struct State {
    update_time: f32,
    surface: Surface,
    
    player_movement: (i8, i8),
    
    zoom: f32,
    camera_position: Vec2,
    window_size: (u32, u32),
    pub options: Options,

    tileman: tile::TileManager
}

impl State {
    fn new(_gfx: &mut Graphics) -> Self {
        Self {
            update_time: 0.,
            
            surface: Surface::new(worldgen::LabGen{}),

            player_movement: (0,0),

            zoom: 25.,
            camera_position: Vec2::default(),
            window_size: (0,0),
            options: Options::default(),
            tileman: tile::TileManager::new()
        }
    }
}


#[notan_main]
fn main() -> Result<(), String> {

    let windowconfig = WindowConfig::new().set_title("FactoryGame").set_size(1280, 720);

    notan::init_with(setup)
    .add_config(windowconfig)
    .initialize(init)
    .add_config(DrawConfig)
    .update(update)
    .draw(draw)
    .build()
}

fn setup(gfx: &mut Graphics) -> State {

    let mut state = State::new(gfx);
    state.options.debugsettings.chunk_border = true;


    state.tileman.register_tile("grass", r"assets\grass.jpg", gfx);
    state.tileman.register_tile("grass1", r"assets\grass1.jpg", gfx);

    let size = 4;

    for x in -size..size {
        for y in -size..size {
            state.surface.gen_chunk((x,y), &state.tileman);
        }
    }

    println!("{} chunks", state.surface.chunks.len());
    
    return state;
}

fn init(_state: &mut State) {

    

}




fn draw(gfx: &mut Graphics, state: &mut State) {
    let mut draw = gfx.create_draw();
    draw.clear(Color::BLACK);

    

    draw.transform().push(Mat3::from_translation(-state.camera_position * state.zoom));
    draw.transform().push(Mat3::from_translation(Vec2::new(state.window_size.0 as f32, state.window_size.1 as f32) * 0.5));
    
    for chunk in &mut state.surface.chunks {
        chunk.update_texture(gfx);
    }

    for chunk in &state.surface.chunks {
        chunk::draw_chunk(&mut draw, &state, chunk);
    }
    


    gfx.render(&draw);

}

const MOVE_SPEED: f32 = 8.0;
const DT: f32 = 1./60.;

fn update(app: &mut App, state: &mut State) {

    state.update_time += app.timer.delta_f32();
    state.window_size = app.window().size();

    
    handle_input(app, state);

    // update each 300ms
    if state.update_time >= 0.016 {
        state.update_time = 0.0;
    }
    else {
        return;
    }


    handle_movement(state);


    for chunk in &mut state.surface.chunks {
        chunk.update();
    }
}

fn handle_input(app: &mut App, state: &mut State) {
    
    state.player_movement = (0,0);

    if app.keyboard.is_down(KeyCode::W) {
        state.player_movement.1 = -1;
    }

    if app.keyboard.is_down(KeyCode::A) {
        state.player_movement.0 = -1;
    }

    if app.keyboard.is_down(KeyCode::S) {
        state.player_movement.1 += 1;
    }

    if app.keyboard.is_down(KeyCode::D) {
        state.player_movement.0 += 1;
    }

    if app.mouse.is_scrolling() {
        let delta_y = app.mouse.wheel_delta.y * app.timer.delta_f32() * 10.;

        state.zoom = (state.zoom + delta_y).max(10.).min(50.);
    }

    if app.keyboard.was_released(KeyCode::F) {
        state.options.debugsettings.chunk_border = !state.options.debugsettings.chunk_border;
    }

}

fn handle_movement(state: &mut State) {

    let movement = state.player_movement;
    state.camera_position.x += MOVE_SPEED * DT * movement.0 as f32;
    state.camera_position.y += MOVE_SPEED * DT * movement.1 as f32;

}