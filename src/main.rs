use log::error;
use pixels::{Error, Pixels, SurfaceTexture};
use winit::dpi::LogicalSize;
use winit::event::{Event, VirtualKeyCode};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;
use winit_input_helper::WinitInputHelper;
mod scanline;
use scanline::{Scanline};
mod render_system;
use render_system::{Renderable, RenderSystem};
const WIDTH: u32 = 320;
const HEIGHT: u32 = 240;
use nalgebra as na;
use na::*;
use std::rc::Rc;
mod edge;
mod bitmap;


struct World {
   scanline : Scanline,
   time : f32,
   texture : Rc<bitmap::Bitmap>,
}

fn main() -> Result<(), Error> {
    let event_loop = EventLoop::new();
    let mut input = WinitInputHelper::new();
    let window = {
        let size = LogicalSize::new(WIDTH as f64, HEIGHT as f64);
        WindowBuilder::new()
            .with_title("Hello Pixels")
            .with_inner_size(size)
            .with_min_inner_size(size)
            .build(&event_loop)
            .unwrap()
    };
    
     
    let mut pixels = {
        let window_size = window.inner_size();
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
        Pixels::new(WIDTH, HEIGHT, surface_texture)?
    };
    let mut world = World::new();
    
    let mut pre_time = std::time::Instant::now();
    event_loop.run(move |event, _, control_flow| {
        
        // Draw the current frame
        if let Event::RedrawRequested(_) = event {
            world.draw(&mut pixels);
            if pixels
                .render()
                .map_err(|e| error!("pixels.render() failed: {}", e))
                .is_err()
            {
                *control_flow = ControlFlow::Exit;
                return;
            }
        }

        // Handle input events
        if input.update(&event) {
            // Close events
            if input.key_pressed(VirtualKeyCode::Escape) || input.quit() {
                *control_flow = ControlFlow::Exit;
                return;
            }

            // Resize the window
            if let Some(size) = input.window_resized() {
                pixels.resize(size.width, size.height);
            }
            let dt = pre_time.elapsed().as_secs_f32();
            pre_time = std::time::Instant::now();
            // Update internal state and request a redraw
            world.update(dt);
            window.request_redraw();
        }

    });
}


impl World {
    fn new()-> Self {
        let texture = Rc::new(bitmap::Bitmap::new_random([45, 45].into()));
        Self {
            scanline : Scanline::new(
                [WIDTH as usize, HEIGHT as usize], &[Point3::origin() ; 3], 
                &World::uvs(), texture.clone(), 0.),
            time : 0.0,
            texture
        }
    }

    fn uvs()
    -> [Vector2<f32> ; 3] {
        [  
            Vector2::new(0.1, 0.1),
            Vector2::new(0.5, 0.9),
            Vector2::new(0.9, 0.1),
        ]
    }

    fn vertices() 
    -> [Point3<f32> ; 3] {

        [
            Point3::new(-1.1, -1.2, 0.0), 
            Point3::new(0.0, 1.01, 0.0), 
            Point3::new(1.0, -1.04, 0.0)
        ]
    }
    
    /// Update the `World` internal state; bounce the box around the screen.
    fn update(&mut self, dt : f32) {
        self.time += dt;

        let tri_angle = self.time * 1.1;
        self.scanline = 
            Scanline::new(
                [WIDTH as usize, HEIGHT as usize], 
                &World::vertices(), &World::uvs(), 
                self.texture.clone(), tri_angle);
    }

    
    fn draw(&mut self, context : &mut dyn Renderable) {
        
        for x in 0..WIDTH {
            for y in 0..HEIGHT {
                context.plot([0x00 ; 4], x as usize, y as usize);
            }
        }


        self.scanline.render(context);
    }
}