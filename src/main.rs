use std::time::SystemTime;
use sdl3::{self, EventPump, keyboard::Keycode, pixels::Color, render::{Canvas, FRect}, sys::render::SDL_RendererLogicalPresentation, video::Window};
use nalgebra::{self, Vector2};

mod sim;

use sim::{HEIGHT, PixelMatrix, WIDTH};
use sim::particles::ParticleType;


struct World 
{

    canvas: Canvas<Window>,
    event_pump: EventPump,

    pixel_matrix: PixelMatrix,

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
   
        let pixel_matrix = PixelMatrix::new();
        let sand_flip = false;
        let liquid_flip = false;
        let time = SystemTime::now();

        Self { canvas, event_pump, pixel_matrix, sand_flip, liquid_flip, time}

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

        //let pb = &mut self.pixelbuf;
        let mut sand_pixel_draw_buf: Vec<FRect> = Vec::new();
        let mut water_pixel_draw_buf: Vec<FRect> = Vec::new();
        let mut gas_pixel_draw_buf: Vec<FRect> = Vec::new();

        'y: for y in (0..HEIGHT as usize).rev()
        {
            'x: for x in (0..WIDTH as usize).rev()
            {
                let mut p = self.pixel_matrix.get(x, y);
                p.update(x, y, &mut self.pixel_matrix);
                
                match &self.pixel_matrix.screen[x][y].ptype
                {
                    ParticleType::Sand => {sand_pixel_draw_buf.push(FRect::new(x as f32, y as f32, 1.0, 1.0));},
                    ParticleType::Water => {water_pixel_draw_buf.push(FRect::new(x as f32, y as f32, 1.0, 1.0));},
                    ParticleType::Empty => {},
                }
                continue 'x;

            }
        }

        for w in self.pixel_matrix.screen.iter_mut()
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
