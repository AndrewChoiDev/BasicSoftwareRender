type Vertex = Vector4<f32>;
use nalgebra::*;
type UV = Vector2<f32>;

use super::interpolant::*;
pub struct Edge {
    x : f32,
    dx_over_dy : f32,
    y_start : usize,
    y_end : usize,
    uv : Interpolant<U2>,
    inverse_w : Interpolant<U1>,
}

impl Edge {
    pub fn new(
        vertex_positions : &[Vertex], start : usize, end : usize, y_min : i32, y_max : i32, 
        vertex_uvs : &[UV])
    -> Self {
        let vp = vertex_positions;
        let y_dist = vp[end].y - vp[start].y;
        let x_dist = vp[end].x - vp[start].x;
       
        let dx_over_dy = x_dist / y_dist;
        let y_pre_step = vp[start].y.ceil() - vp[start].y;
        let x = vp[start].x + y_pre_step * dx_over_dy;
        let x_pre_step = x - vp[start].x;

        let presteps = [x_pre_step, y_pre_step].into();

        let inverse_w : [Vector1<f32> ; 3] = [
            Vector1::new(1. / vertex_positions[0][3]), 
            Vector1::new(1. / vertex_positions[1][3]), 
            Vector1::new(1. / vertex_positions[2][3])];

        let uv_vertex_interpolants = [
            vertex_uvs[0] * inverse_w[0],
            vertex_uvs[1] * inverse_w[1],
            vertex_uvs[2] * inverse_w[2],
        ];
        
        Self {
            y_start : (vp[start].y.ceil() as i32).max(y_min) as usize,
            y_end : (vp[end].y.ceil() as i32).min(y_max) as usize,
            x,
            dx_over_dy,
            uv : 
                Interpolant::<U2>::new(
                    start, vertex_positions, 
                    &uv_vertex_interpolants, 
                    dx_over_dy, presteps),
            inverse_w : 
                Interpolant::<U1>::new(
                    start, vertex_positions, 
                    &inverse_w,
                    dx_over_dy, presteps)
        }
    }
    pub fn x(&self)
    -> f32 {
        self.x
    }
    pub fn inverse_w(&self) 
    -> f32 {
        self.inverse_w.value().x
    }
    pub fn y_start(&self)
    -> usize {
        self.y_start
    }
    pub fn uv(&self)
    -> UV {
        self.uv.value()
    }
    pub fn y_end(&self)
    -> usize {
        self.y_end
    }

    pub fn step(&mut self) {
        self.x += self.dx_over_dy;

        self.uv.step();
        self.inverse_w.step();
    }
}


