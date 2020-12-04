use nalgebra::*;

type Gradient<U> = Vector2<VectorN<f32, U>>;

pub struct Interpolant<U : Dim + DimName> 
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

    fn calculate_gradient(vertex_positions : &[Vector4<f32>], vertex_values : &[VectorN<f32, U>])
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