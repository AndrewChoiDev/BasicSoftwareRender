use nalgebra::*;
use super::edge::Edge;
use super::bitmap::*;
use std::rc::Rc;
type Vertex = Point3<f32>;
type Vect = Vector3<f32>;
type VertexColor = Vector4<f32>;
pub fn triangle_signed_area(v_min_y : Vertex, v_max_y : Vertex, v_mid_y : Vertex) 
-> f32 {
    xy_cross_product_magnitude(v_min_y - v_max_y, v_min_y - v_mid_y)
}

pub fn xy_cross_product_magnitude(v1 : Vect, v2 : Vect) 
-> f32 {
    (v1.x * v2.y - v2.x * v1.y) * 0.5f32
}
type UV = Vector2<f32>;
pub struct Scanline {
    ordered_vertices : Vec<Vertex>, 
    ordered_uvs : Vec<UV>,
    gradient : Vector2<UV>,
    texture : Rc<Bitmap>
}

impl Scanline {
    
    pub fn new(unordered_vertices : [Vertex ; 3], unordered_uvs : [UV ; 3], texture : Rc<Bitmap>) -> Scanline {
    
        // sort indices to positional vertices by y value ascending
        let indices : Vec<usize> = {
            let mut v : Vec<usize> = (0..3).collect(); 
            v.sort_by(|&ia, &ib| 
                unordered_vertices[ia].y.partial_cmp(&unordered_vertices[ib].y).unwrap()); 
            v
        };

        //ordered vertices and ordered vertex values
        let ovs : Vec<Vertex> = indices.iter().map(|&i| unordered_vertices[i]).collect();
        let ovvs : Vec<UV> = indices.iter().map(|&i| unordered_uvs[i]).collect();

        let gradient : _ = 
            Scanline::gradient_of_triangle(&ovs[0..3], &ovvs[0..3]).into();

        Scanline {ordered_vertices : ovs, ordered_uvs : ovvs, gradient, texture}
    }


    fn gradient_of_triangle<U : Dim + DimName>(
        vertex_positions : &[Vertex], 
        vertex_vectors : &[VectorN<f32, U>])
    -> [VectorN<f32, U> ; 2] 
    where 
        DefaultAllocator : nalgebra::allocator::Allocator<f32, U>,
        VectorN<f32, U> : Copy
    {
        let vp = vertex_positions;
        let vv = vertex_vectors;
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
        ]


    }
    
    fn scan_convert_triangle(&self, context : &mut dyn Renderable, handedness : bool) {
       let y_min = self.ordered_vertices[0].y.ceil() as usize;
       let y_max = (self.ordered_vertices[2].y.ceil() as usize).min(context.height() as usize);
                
       let mut top_to_bottom = Edge::new(self.ordered_vertices[0], self.ordered_vertices[2], 
        y_min as i32, y_max as i32,
        self.ordered_uvs[0], self.gradient);

       let mut top_to_middle = Edge::new(self.ordered_vertices[0], self.ordered_vertices[1], 
        y_min as i32, y_max as i32,
        self.ordered_uvs[0], self.gradient);

       let mut middle_to_bottom = Edge::new(self.ordered_vertices[1], self.ordered_vertices[2], 
        y_min as i32, y_max as i32,
        self.ordered_uvs[1], self.gradient);

        self.scan_convert_edge_pair(context, &mut top_to_bottom, &mut top_to_middle, handedness);
        self.scan_convert_edge_pair(context, &mut top_to_bottom, &mut middle_to_bottom, handedness);
    }

    fn scan_convert_edge_pair(&self, context : &mut dyn Renderable, a : &mut Edge, b : &mut Edge, 
        handedness : bool) {            
                 
        let (y_start, y_end) = (b.y_start(), b.y_end());

        let (left, right) = 
            if handedness {(b, a)} 
            else {(a, b)};            

        for j in y_start..y_end {
            self.draw_scan_line(context, 
                left, right, j);
            left.step();
            right.step();
        }
    }

    fn draw_scan_line(&self, context : &mut dyn Renderable, left : &Edge, right : &Edge, j : usize) {
        let (x_min, x_max) = (left.x().ceil() as usize, right.x().ceil() as usize);
        
        let min_uv = left.uv();
        let max_uv = right.uv();

        let mut lerp_value = 0.;
        let lerp_step = 1. / (x_max - x_min) as f32;

        let uv_delta = max_uv - min_uv;

        for i in x_min..x_max {

            let uv = min_uv + lerp_value * uv_delta;

            let color_bytes = self.texture.get_pixel(uv);
            // let color_bytes = color.map(|c| (c * 255f32 + 0.5f32) as u8).into();                
            context.plot(color_bytes, i, j);

            lerp_value += lerp_step;
        }
    }
    
}



use super::render_system as rs;
use rs::RenderSystem;
use rs::Renderable;
impl RenderSystem for Scanline {
    fn render(&self, context : &mut dyn Renderable) {
        
        let signed_area = triangle_signed_area(self.ordered_vertices[0],
             self.ordered_vertices[2], self.ordered_vertices[1]);

        let handedness = signed_area >= 0f32;

        self.scan_convert_triangle(context, handedness);   
    }
}