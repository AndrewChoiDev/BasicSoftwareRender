type Vertex = Point3<f32>;
use nalgebra::*;
type UV = Vector2<f32>;
pub struct Edge {
    x : f32,
    dx_over_dy : f32,
    y_start : usize,
    y_end : usize,
    uv : UV,
    uv_step : UV,
}

impl Edge {
    pub fn new(start : Vertex, end : Vertex, y_min : i32, y_max : i32, 
        start_uv : UV, gradient : Vector2<UV>)
    -> Self {
        let y_dist = end.y - start.y;
        let x_dist = end.x - start.x;
       
        let dx_over_dy = x_dist / y_dist;
        let y_pre_step = start.y.ceil() - start.y;
        let x = start.x + y_pre_step * dx_over_dy;
        let x_pre_step = x - start.x;

        let uv : _ = 
            start_uv 
            + gradient.x * x_pre_step 
            + gradient.y * y_pre_step;

        let uv_step = (gradient.y * 1.) + (gradient.x * dx_over_dy);
        Self {
            y_start : (start.y.ceil() as i32).max(y_min) as usize,
            y_end : (end.y.ceil() as i32).min(y_max) as usize,
            x,
            dx_over_dy,
            uv,
            uv_step
        }
    }
    pub fn x(&self)
    -> f32 {
        self.x
    }
    pub fn y_start(&self)
    -> usize {
        self.y_start
    }
    pub fn uv(&self)
    -> UV {
        self.uv
    }
    pub fn y_end(&self)
    -> usize {
        self.y_end
    }

    pub fn step(&mut self) {
        self.x += self.dx_over_dy;
        self.uv += self.uv_step;
    }
}