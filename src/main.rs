use core::f32;
use std::time::Instant;

use graphics::GraphicsData;
use notan::{draw::DrawConfig, prelude::*};
use prototype::{EntityManager, PrototypeManager};
use world::{tile::TileManager, ChunkCoord, Surface};

mod graphics;
mod world;

mod prototype;


enum Task{
    GenChunk(ChunkCoord),
    RemoveEntity(u32),
    PlaceEntity(u32, world::entity::Entity),

}

struct UpdateData {
    pub update_time: f32,
}
impl UpdateData {
    pub fn new() -> Self {
        Self {
            update_time: 0.0,
        }
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
#[allow(dead_code)]
struct State {
    tileman: TileManager,
    protoman: PrototypeManager,
    entityman: EntityManager,
    //recipeman: RecipeManager,
    //scriptman: ScriptManager,

    surface: Surface,
    
    options: Options,
    graphicsdata: GraphicsData,
    updatedata: UpdateData,

    tasks: Vec<Task>,
}


impl State {
    fn new(_gfx: &mut Graphics) -> Self {
        Self {
            tileman: world::tile::TileManager::new(),
            protoman: PrototypeManager::new(),
            entityman: EntityManager::new(),

            surface: world::Surface::new(world::worldgen::LabGen{}),
            
            
            options: Options::default(),
            graphicsdata: GraphicsData::default(),
            updatedata: UpdateData::new(),

            tasks: vec![]
        }
    }

    fn add_task(&mut self, task: Task) {
        self.tasks.push(task);
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

    let size = 10;

    for x in -size..size {
        for y in -size..size {
            state.add_task(Task::GenChunk(ChunkCoord::new(x,y)));
        }
    }

    
    //state.add_task(Task::GenChunk(ChunkCoord::new(0, 0)));
    println!("{} chunks", state.surface.chunks.len());
    
    return state;
}

fn init(_state: &mut State) {

    

}




fn draw(_app: &mut App, _assets: &mut Assets, gfx: &mut Graphics, state: &mut State) {
    
    let before = Instant::now();
   
    graphics::render_surface(gfx, &state.surface, &mut state.graphicsdata);


    println!("time: {}us", before.elapsed().as_micros());
    //render UI

    //println!("fps: {}", app.timer.fps());
}


fn update(app: &mut App, state: &mut State) {

    state.updatedata.update_time += app.timer.delta_f32();
    let tmpsize = app.window().size();
    state.graphicsdata.window_size = (tmpsize.0 as f32, tmpsize.1 as f32);
    
    const MAX_TASKS: i32 = 1000;
    for _ in 0..MAX_TASKS {
        let task = state.tasks.pop();
        if let Some(task) = task {
            handle_task(task, state);
        }

        else { break; }
    }


    // update each 300ms
    if state.updatedata.update_time >= 0.016 {
        state.updatedata.update_time = 0.0;
    }
    else {
        return;
    }

    let speed = 0.125;
    let rad = 11.0;

    state.surface.camera_pos.x = (app.timer.elapsed().as_secs_f32() * speed * f32::consts::PI).cos() * rad;
    state.surface.camera_pos.y = (app.timer.elapsed().as_secs_f32() * speed * f32::consts::PI).sin() * rad;
    //state.surface.camera_pos.x = app.timer.elapsed().as_secs_f32() * rad;



    for chunk in &mut state.surface.chunks {
        chunk.update();
    }


}


fn handle_task(task: Task, state: &mut State) {
    match task {
        Task::GenChunk(position) => {
            println!("Generating chunk {:?}", position);
            state.surface.gen_chunk(position, &state.tileman);
        },
        Task::RemoveEntity(_) => todo!(),
        Task::PlaceEntity(_, _entity) => todo!(),
    }
}