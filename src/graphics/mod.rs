use notan::{app::{Color, Graphics, Texture}, draw::{CreateDraw, DrawImages, DrawShapes}, math::{Mat3, Vec2}};

use crate::world::{Coordinate, Surface, TileCoord};


pub struct GraphicsData {    
    tilebuffer: Option<Texture>,
    buffer_size: (f32, f32),
    pub window_size: (f32, f32),
    pub prev_cam_pos: Coordinate,
    pub zoom: f32,
}

impl GraphicsData {
    pub fn new () -> Self {
        Self {
            tilebuffer: None,
            buffer_size: (0.0, 0.0),
            window_size: (0.0, 0.0),
            prev_cam_pos: Coordinate::new(0.0, 0.0),
            zoom: 1.0
        }
    }
}

struct ScreenCoord {
    x: f32,
    y: f32
}

impl ScreenCoord {
    fn new(x: f32, y: f32) -> ScreenCoord {
        ScreenCoord { x, y }
    }
}

impl From<(f32, f32)> for ScreenCoord {
    fn from(value: (f32, f32)) -> Self {
        ScreenCoord { x: value.0, y: value.1 }
    }
}

impl From<ScreenCoord> for (f32, f32) {
    fn from(value: ScreenCoord) -> Self {
        (value.x, value.y)
    }
}


struct ScreenWorldConverter {
    cam_pos: Coordinate,
    window_size: (f32, f32),
    scale: f32,
    offset: Coordinate
}

impl ScreenWorldConverter {
    fn from_world(&self, coord: Coordinate) -> ScreenCoord {
        let dpos = coord - self.cam_pos + self.offset;
        let s = (dpos.x * self.scale, dpos.y * self.scale);
        return ScreenCoord::new(s.0 + self.window_size.0 * 0.5, self.window_size.1 * 0.5 + s.1);
    }

    fn from_screen(&self, coord: ScreenCoord) -> Coordinate {
        let screen_pos = (coord.x - self.window_size.0 * 0.5, coord.y - self.window_size.1 * 0.5);
        let pos = (screen_pos.0 / self.scale, screen_pos.1 / self.scale);
        return Coordinate::new(pos.0, pos.1) + self.cam_pos;
    }
}

trait SignedFloor {
    ///rounds the number "in" towards 0 regardless of sign
    fn signed_floor(self) -> Self;
}
impl SignedFloor for f32 {
    ///rounds the number "in" towards 0 regardless of sign
    fn signed_floor(self) -> Self {
        if self < 0.0 { return self.ceil(); }
        else { return self.floor(); }
    }
}

trait SignedCeil {
    ///rounds the number "out" away from 0 regardless of sign
    fn signed_ceil(self) -> Self;
}
impl SignedCeil for f32 {
    ///rounds the number "out" away from 0 regardless of sign
    fn signed_ceil(self) -> Self {
        if self < 0.0 { return self.floor(); }
        else { return self.ceil(); }
    }
}



pub fn render_surface(gfx: &mut Graphics, surface: &Surface, graphicsdata: &mut GraphicsData) {

    let extrasize = 4.0;


    //min zoom = 10, max zoom = 100
    let scale = 30.0;
    let buffersize = (graphicsdata.window_size.0 + extrasize*scale, graphicsdata.window_size.1 + extrasize*scale);
    let dpos = surface.camera_pos - graphicsdata.prev_cam_pos;

    if dpos.x.abs() > 5.0 || dpos.y.abs() > 5.0 {
        graphicsdata.tilebuffer = None;
        println!("refresh: {dpos:?}");
        graphicsdata.prev_cam_pos = surface.camera_pos;
    }
    

    if let Some(tex) = &graphicsdata.tilebuffer {       

        let rtex = gfx.create_render_texture(buffersize.0 as u32, buffersize.1 as u32).build().unwrap();
        let mut tdraw =  rtex.create_draw();
        tdraw.clear(Color::RED);

        if dpos.x.abs() >= 1.0 || dpos.y.abs() >= 1.0 {
            println!("MOVED!");

            graphicsdata.prev_cam_pos = surface.camera_pos;

            tdraw.image(tex)
            .position(-dpos.x * scale, -dpos.y * scale)
            .size(tex.width(), tex.height());

            let center = surface.camera_pos;

            let x0 = center.x - (graphicsdata.window_size.0 * 0.5 / scale);
            let x1 = center.x + (graphicsdata.window_size.0 * 0.5 / scale);
            let y0 = center.y - (graphicsdata.window_size.1 * 0.5 / scale);
            let y1 = center.y + (graphicsdata.window_size.1 * 0.5 / scale);

            println!("({x0},{y0}) - ({x1},{y1})");


            //TODO: render new tiles

            let mut dx: f32 = 0.0;
            while dx.abs() < dpos.x.abs() {
                println!("Rendering tile x: {dx}");

                dx += dpos.x.signum();
            }

            let mut dy: f32 = 0.0;
            while dy.abs() < dpos.y.abs() {
                println!("Rendering tile y: {dy}");

                dy += dpos.y.signum();
            }



        
            gfx.render_to(&rtex, &tdraw);
        
            graphicsdata.tilebuffer = Some(rtex.texture().clone());

        }

    }
    else {

        let rtex = gfx.create_render_texture(buffersize.0 as u32, buffersize.1 as u32).build().unwrap();
        let mut tdraw =  rtex.create_draw();

        tdraw.transform().push(Mat3::from_translation(Vec2::new(graphicsdata.window_size.0, graphicsdata.window_size.1) * 0.5));

        let center = surface.camera_pos;

        let x0 = (center.x - extrasize - (graphicsdata.window_size.0 * 0.5 / scale)).signed_ceil() as i32;
        let x1 = (center.x + extrasize + (graphicsdata.window_size.0 * 0.5 / scale)).signed_ceil() as i32;
        let y0 = (center.y - extrasize - (graphicsdata.window_size.1 * 0.5 / scale)).signed_ceil() as i32;
        let y1 = (center.y + extrasize + (graphicsdata.window_size.1 * 0.5 / scale)).signed_ceil() as i32;

        //println!("({x0},{y0}) - ({x1},{y1})");
        
        for x in x0..x1 {
            for y in y0..y1 {
                let t = surface.get_tile(TileCoord::new(x, y));

                if let Some(tile) = t {
                    let dpos = Coordinate::new(x as f32+extrasize*0.5, y as f32+extrasize*0.5) - graphicsdata.prev_cam_pos;
                    let tcoord = (dpos.x * scale, dpos.y * scale);

                    tdraw.image(&tile.texture)
                    .position(tcoord.0, tcoord.1)
                    .size(scale, scale);
                }
            }
        }

        gfx.render_to(&rtex, &tdraw);

        //let _ = rtex.to_file(gfx, "test.png");

        graphicsdata.tilebuffer = Some(rtex.texture().clone());
        graphicsdata.prev_cam_pos = surface.camera_pos;

    }
    let windowsize = 0.9;

    let mut draw = gfx.create_draw();
    draw.clear(Color::BLACK);

    draw.transform().push(Mat3::from_translation(Vec2::new(graphicsdata.window_size.0, graphicsdata.window_size.1) * 0.5));
    draw.transform().push(Mat3::from_scale(Vec2::new(1.0, 1.0) * windowsize));

    draw.transform().push(Mat3::from_translation(Vec2::new(graphicsdata.window_size.0, graphicsdata.window_size.1) * -0.5));
    
    let offset = graphicsdata.prev_cam_pos - surface.camera_pos;


    draw.image(&graphicsdata.tilebuffer.clone().unwrap())
    .position(scale * (-extrasize*0.5 +offset.x), scale * (-extrasize*0.5 +offset.y));

    if windowsize < 1.0 {
        draw.rect((0.0, 0.0), graphicsdata.window_size)
        .color(Color::BLUE)
        .stroke(4.0);

        
        draw.circle(10.0)
        .color(Color::RED)
        .position(graphicsdata.window_size.0 * 0.5 + offset.x*scale, graphicsdata.window_size.1 * 0.5 + offset.y*scale);

        

    }
    //draw.transform().clear();

    let convert = ScreenWorldConverter {cam_pos: graphicsdata.prev_cam_pos, window_size: graphicsdata.window_size, scale: scale, offset: offset};

    let dotpos = convert.from_world(Coordinate::new(0.0, 0.0));
    draw.circle(5.0)
    .color(Color::PURPLE)
    .position(dotpos.x, dotpos.y);

    let dotpos = convert.from_world(Coordinate::new(1.0, 0.0));
    draw.circle(5.0)
    .color(Color::BLUE)
    .position(dotpos.x, dotpos.y);

    let dotpos = convert.from_world(Coordinate::new(0.0, 1.0));
    draw.circle(5.0)
    .color(Color::GREEN)
    .position(dotpos.x, dotpos.y);

    
    draw.rect(convert.from_world(Coordinate::new(5.0, 5.0)).into(), (3.0*scale,3.0*scale))
    .fill_color(Color::GREEN);



    gfx.render(&draw);
}
