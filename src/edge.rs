type Vertex = Vector4<f32>;
use nalgebra::*;
type UV = Vector2<f32>;
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

type Gradient<U> = Vector2<VectorN<f32, U>>;

struct Interpolant<U : Dim + DimName> 
where 
    DefaultAllocator : nalgebra::allocator::Allocator<f32, U>,
    VectorN<f32, U> : Copy
{
    value : VectorN<f32, U>,
    step : VectorN<f32, U>,
}

impl<U : Dim + DimName> Interpolant<U> 
where 
    DefaultAllocator : nalgebra::allocator::Allocator<f32, U>,
    VectorN<f32, U> : Copy
{
    pub fn new(start_vertex : usize, vertex_positions : &[Vector4<f32>], vertex_values : &[VectorN<f32, U>], dx_over_dy : f32, presteps : Vector2<f32>)
    -> Self {
        let gradient = 
            Interpolant::calculate_gradient(vertex_positions, vertex_values);

        let value = 
            vertex_values[start_vertex]
            + gradient.x * presteps.x + gradient.y * presteps.y;

        let step = (gradient.y * 1.) + (gradient.x * dx_over_dy);

        Self {
            value,
            step
        }
    }

    pub fn step(&mut self) {
        self.value += self.step;
    }

    pub fn value(&self)
    -> VectorN<f32, U> {
        self.value
    }

    fn calculate_gradient(vertex_positions : &[Vertex], vertex_values : &[VectorN<f32, U>])
    -> Gradient<U>
    {
        let vp = vertex_positions;
        let vv = vertex_values;
        let dc_over_dx_numerator= 
            (vv[1] - vv[2]) * (vp[0].y - vp[2].y) 
            - (vv[0] - vv[2]) * (vp[1].y - vp[2].y);
        let dc_over_dy_numerator : VectorN<f32, U> = 
            (vv[1] - vv[2]) * (vp[0].x - vp[2].x) 
            - (vv[0] - vv[2]) * (vp[1].x - vp[2].x);
        let dc_over_dx_denominator = 1.0 / 
            ((vp[1].x - vp[2].x) * (vp[0].y - vp[2].y) 
            - (vp[0].x - vp[2].x) * (vp[1].y - vp[2].y));

        let dc_over_dy_denominator = -dc_over_dx_denominator;

        [
            dc_over_dx_numerator * dc_over_dx_denominator,
            dc_over_dy_numerator * dc_over_dy_denominator
        ].into()
    }
}
