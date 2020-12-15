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
const WIDTH: u32 = 360;
const HEIGHT: u32 = 240;
use nalgebra as na;
use na::*;
use std::rc::Rc;
mod edge;
mod bitmap;
mod interpolant;
mod mesh_loader;
struct World {
   scanline : Scanline,
   time : f32,
   texture : Rc<bitmap::Bitmap>,
   mesh : Rc<mesh_loader::Mesh>,
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
        let texture = Rc::new(bitmap::Bitmap::new_random([20, 20].into()));
        let mesh = Rc::new(
            mesh_loader::Mesh::new("resources/teapot/teapot.obj")
        );
        Self {
            scanline : Scanline::new(
                [WIDTH as usize, HEIGHT as usize], 
                mesh.clone(), texture.clone(), 0.),
            time : 0.0,
            texture,
            mesh,
        }
    }

    /// Update the `World` internal state; bounce the box around the screen.
    fn update(&mut self, dt : f32) {
        self.time += dt;

        let tri_angle = self.time * 1.1;
        self.scanline = 
            Scanline::new(
                [WIDTH as usize, HEIGHT as usize], 
                self.mesh.clone(),
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