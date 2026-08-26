use std::time::SystemTime;

use sdl3::{self, EventPump, keyboard::Keycode, pixels::Color, render::{Canvas, FRect}, sys::render::SDL_RendererLogicalPresentation, video::Window};
use nalgebra::{self, Vector2};

use crate::ParticleType::Liquid;


const WIDTH: u32 = 400;
const HEIGHT: u32 = 200;


struct World 
{

    canvas: Canvas<Window>,
    event_pump: EventPump,

    pixelbuf: [[Pixel; HEIGHT as usize]; WIDTH as usize],
    sand_flip: bool,
    liquid_flip: bool,

    time: SystemTime,
}
impl World
{

    pub fn new() -> Self
    {
    
        let context = sdl3::init().expect("Cannot initialize SDL3");
        let mut video_subsys = context.video().expect("Failed to initialize SDL3 Video Subsystem");



        let mut window = video_subsys.window("Sand", WIDTH, HEIGHT)
            .resizable()
            .position_centered()
            .build()
            .unwrap();

        window.set_fullscreen(true);

        let mut canvas = window.into_canvas();
        let _ = canvas.set_logical_size(WIDTH, HEIGHT, SDL_RendererLogicalPresentation::LETTERBOX);

        canvas.clear();
        canvas.present();

        let event_pump = context.event_pump().unwrap();
   
        let mut pixelbuf = [[ Pixel::new(false, ParticleType::None); HEIGHT as usize]; WIDTH as usize];

        for i in 100..200
        {
            for j in HEIGHT as usize-100..HEIGHT as usize
            {
                pixelbuf[i][j].ptype = ParticleType::Sand;
            }
        }
        for i in 300..350
        {
            for j in 0..45 as usize
            {
                pixelbuf[i][j].ptype = ParticleType::Sand;
            }
        }

        for i in 300..350
        {
            for j in HEIGHT as usize-100..HEIGHT as usize
            {
                pixelbuf[i][j].ptype = ParticleType::Liquid;
            }
        }
        for i in 100..150
        {
            for j in 0..50 as usize
            {
                pixelbuf[i][j].ptype = ParticleType::Liquid;
            }
        }

        let sand_flip = false;
        let liquid_flip = false;
        let time = SystemTime::now();

        Self { canvas, event_pump, pixelbuf, sand_flip, liquid_flip, time}

    }


    pub fn run(&mut self)
    {


        'run: loop
        {

            for event in self.event_pump.poll_iter()
            {
                
                match event 
                {
                    sdl3::event::Event::KeyDown { keycode: Some(Keycode::Escape), .. } 
                    | sdl3::event::Event::Quit { .. } => { break 'run },
                    _ => {}
                } // match event
            } // for event

           
            let now = SystemTime::now();

            if now.duration_since(self.time).unwrap().as_millis() > 10
            {
                self.step();
                self.time = now;
                self.canvas.present();
            }
                

        } // 'run loop

    } // fn run
      

    fn step(&mut self)
    {
        self.canvas.set_draw_color(Color::BLACK);
        self.canvas.clear();

        let pb = &mut self.pixelbuf;
        let mut sand_pixel_draw_buf: Vec<FRect> = Vec::new();
        let mut water_pixel_draw_buf: Vec<FRect> = Vec::new();
        let mut gas_pixel_draw_buf: Vec<FRect> = Vec::new();

        'y: for y in (0..HEIGHT as usize).rev()
        {
            'x: for x in (0..WIDTH as usize).rev()
            {
                if pb[x][y].just_moved { continue 'x; }
                match pb[x][y].ptype
                {
                    ParticleType::Sand =>
                    {
                        self.sand_flip = !self.sand_flip;
                        sand_pixel_draw_buf.push(FRect::new(x as f32, y as f32, 1.0, 1.0));

                        if y+1 >= HEIGHT as usize { continue 'x; }

                        if pb[x][y+1].ptype == ParticleType::None
                        {
                            pb[x][y] = pb[x][y+1];
                            pb[x][y+1].ptype = ParticleType::Sand;
                            pb[x][y+1].just_moved = true;
                            continue 'x;
                        }

                        if self.sand_flip
                        {
                            if x+1 < WIDTH as usize && pb[x+1][y+1].ptype != ParticleType::Sand
                            {
                                if pb[x+1][y+1].ptype == ParticleType::Liquid && pb[x+1][y].ptype == ParticleType::None
                                {
                                    pb[x+1][y+1].ptype = ParticleType::None;
                                    pb[x+1][y].ptype = ParticleType::Liquid;
                                }
                                pb[x][y] = pb[x+1][y+1];
                                pb[x+1][y+1].ptype = ParticleType::Sand;
                                pb[x+1][y+1].just_moved = true;
                                continue 'x;
                            }
                            if x > 0 && pb[x-1][y+1].ptype == ParticleType::None
                            {
                                if pb[x-1][y+1].ptype == ParticleType::Liquid && pb[x-1][y].ptype == ParticleType::None
                                {
                                    pb[x-1][y+1].ptype = ParticleType::None;
                                    pb[x-1][y].ptype = ParticleType::Liquid;
                                }
                                pb[x][y] = pb[x-1][y+1];
                                pb[x-1][y+1].ptype = ParticleType::Sand;
                                pb[x-1][y+1].just_moved = true;
                                continue 'x;
                            }
                        }
                        else
                        {
                            if x > 0 && pb[x-1][y+1].ptype == ParticleType::None
                            {
                                if pb[x-1][y+1].ptype == ParticleType::Liquid && pb[x-1][y].ptype == ParticleType::None
                                {
                                    pb[x-1][y+1].ptype = ParticleType::None;
                                    pb[x-1][y].ptype = ParticleType::Liquid;
                                }
                                pb[x][y] = pb[x-1][y+1];
                                pb[x-1][y+1].ptype = ParticleType::Sand;
                                pb[x-1][y+1].just_moved = true;
                                continue 'x;
                            }
                            if x+1 < WIDTH as usize && pb[x+1][y+1].ptype != ParticleType::Sand
                            {
                                if pb[x+1][y+1].ptype == ParticleType::Liquid && pb[x+1][y].ptype == ParticleType::None
                                {
                                    pb[x+1][y+1].ptype = ParticleType::None;
                                    pb[x+1][y].ptype = ParticleType::Liquid;
                                }
                                pb[x][y] = pb[x+1][y+1];
                                pb[x+1][y+1].ptype = ParticleType::Sand;
                                pb[x+1][y+1].just_moved = true;
                                continue 'x;
                            }
                        }
                    },
                    ParticleType::Liquid =>
                    {
                        self.liquid_flip = !self.liquid_flip;
                        water_pixel_draw_buf.push(FRect::new(x as f32, y as f32, 1.0, 1.0));

                        if y+1 >= HEIGHT as usize { continue 'x; }

                        if pb[x][y+1].ptype == ParticleType::None
                        {
                            pb[x][y] = pb[x][y+1];
                            pb[x][y+1].ptype = ParticleType::Liquid;
                            pb[x][y+1].just_moved = true;
                            continue 'x;
                        }

                        if y-1 > 0 && pb[x][y-1].ptype == ParticleType::Sand
                        {
                            pb[x][y] = pb[x][y-1];
                            pb[x][y-1].ptype = ParticleType::Liquid;
                            pb[x][y-1].just_moved = true;
                            continue 'x;
                        }

                        if self.liquid_flip
                        {
                            if x+1 < WIDTH as usize && passable_for_liquid(pb[x+1][y+1].ptype)
                            {
                                pb[x][y] = pb[x+1][y+1];
                                pb[x+1][y+1].ptype = ParticleType::Liquid;
                                pb[x+1][y+1].just_moved = true;
                                continue 'x;
                            }
                            if x > 0 && passable_for_liquid(pb[x-1][y+1].ptype)
                            {
                                pb[x][y] = pb[x-1][y+1];
                                pb[x-1][y+1].ptype = ParticleType::Liquid;
                                pb[x-1][y+1].just_moved = true;
                                continue 'x;
                            }
                            if x+1 < WIDTH as usize && passable_for_liquid(pb[x+1][y].ptype)
                            {
                                pb[x][y] = pb[x+1][y];
                                pb[x+1][y].ptype = ParticleType::Liquid;
                                pb[x+1][y].just_moved = true;
                                continue 'x;
                            }
                            if x > 0 && passable_for_liquid(pb[x-1][y].ptype)
                            {
                                pb[x][y] = pb[x-1][y];
                                pb[x-1][y].ptype = ParticleType::Liquid;
                                pb[x-1][y].just_moved = true;
                                continue 'x;
                            }
                        }
                        else
                        {
                            if x > 0 && passable_for_liquid(pb[x-1][y+1].ptype)
                            {
                                pb[x][y] = pb[x-1][y+1];
                                pb[x-1][y+1].ptype = ParticleType::Liquid;
                                pb[x-1][y+1].just_moved = true;
                                continue 'x;
                            }
                            if x+1 < WIDTH as usize && passable_for_liquid(pb[x+1][y+1].ptype)
                            {
                                pb[x][y] = pb[x+1][y+1];
                                pb[x+1][y+1].ptype = ParticleType::Liquid;
                                pb[x+1][y+1].just_moved = true;
                                continue 'x;
                            }
                            if x > 0 && passable_for_liquid(pb[x-1][y].ptype)
                            {
                                pb[x][y] = pb[x-1][y];
                                pb[x-1][y].ptype = ParticleType::Liquid;
                                pb[x-1][y].just_moved = true;
                                continue 'x;
                            }
                            if x+1 < WIDTH as usize && passable_for_liquid(pb[x+1][y].ptype)
                            {
                                pb[x][y] = pb[x+1][y];
                                pb[x+1][y].ptype = ParticleType::Liquid;
                                pb[x+1][y].just_moved = true;
                                continue 'x;
                            }
                        }
                    }

                    _ => { continue; }
                }
            }
        }

        for w in pb.iter_mut()
        {
            for h in w.iter_mut()
            {
                h.just_moved = false;
            }
        }

        self.canvas.set_draw_color(Color::YELLOW);
        self.canvas.draw_rects(&sand_pixel_draw_buf.as_slice());

        self.canvas.set_draw_color(Color::BLUE);
        self.canvas.draw_rects(&water_pixel_draw_buf.as_slice());
    }


}



fn main() {
    println!("Hello, world!");

    let mut world = World::new();

    world.run();
}



#[derive(Clone, Copy, Debug)]
struct Pixel
{
    pub just_moved: bool,
    pub ptype: ParticleType
}
impl Pixel
{
    pub fn new(b: bool, t: ParticleType) -> Self
    {
        Self { just_moved: b, ptype: t }
    }
}


#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum ParticleType
{
    None,
    Sand,
    Liquid,
    Gas,
}

fn passable_for_liquid(p: ParticleType) -> bool
{
    p == ParticleType::Gas || p == ParticleType::None
}
