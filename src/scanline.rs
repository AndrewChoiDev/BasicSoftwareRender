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
    vertices : [Vertex ; 3], 
    uvs : [UV ; 3],
    gradient : Vector2<UV>,
    texture : Rc<Bitmap>
}

impl Scanline {

    
    pub fn new(vertices : [Vertex ; 3], uvs : [UV ; 3], texture : Rc<Bitmap>) -> Scanline {
    
        let indices : Vec<usize> = {
            let mut v : Vec<usize> = (0..3).collect(); 
            v.sort_by(|&ia, &ib| vertices[ia].y.partial_cmp(&vertices[ib].y).unwrap()); 
            v
        };
        let ovs : Vec<Vertex> = indices.iter().map(|&i| vertices[i]).collect();
        let ocvs : Vec<UV> = indices.iter().map(|&i| uvs[i]).collect();
        let vv = [ocvs[0], ocvs[1], ocvs[2]];
        let vp = [ovs[0], ovs[1], ovs[2]];

        let gradient : _ = Scanline::gradient_of_triangle(vp, vv).into();
        Scanline {vertices, uvs, gradient, texture}
    }


    fn gradient_of_triangle<U : Dim + DimName>(
        vertex_positions : [Vertex ; 3], 
        vertex_vectors : [VectorN<f32, U> ; 3])
    -> [VectorN<f32, U> ; 2] 
    where DefaultAllocator : nalgebra::allocator::Allocator<f32, U>
    {
        let vp = vertex_positions;
        let vv = vertex_vectors;
        let dc_over_dx_numerator= 
            (vv[1].clone() - vv[2].clone()) * (vp[0].y - vp[2].y) 
            - (vv[0].clone() - vv[2].clone()) * (vp[1].y - vp[2].y);
        let dc_over_dy_numerator : VectorN<f32, U> = 
            (vv[1].clone() - vv[2].clone()) * (vp[0].x - vp[2].x) 
            - (vv[0].clone() - vv[2].clone()) * (vp[1].x - vp[2].x);
        let dc_over_dx_denominator = 1.0 / 
            ((vp[1].x - vp[2].x) * (vp[0].y - vp[2].y) 
            - (vp[0].x - vp[2].x) * (vp[1].y - vp[2].y));

        let dc_over_dy_denominator = -dc_over_dx_denominator;

        [
            dc_over_dx_numerator * dc_over_dx_denominator,
            dc_over_dy_numerator * dc_over_dy_denominator
        ]


    }
    
    pub fn scan_convert_triangle(&self, context : &mut dyn Renderable, min_y_vert : Vertex, 
        mid_y_vert : Vertex, max_y_vert : Vertex, handedness : bool) {
       let y_min = min_y_vert.y.ceil() as usize;
       let y_max = (max_y_vert.y.ceil() as usize).min(context.height() as usize);
                
        let indices : Vec<usize> = {
            let mut v : Vec<usize> = (0..3).collect(); 
            v.sort_by(|&ia, &ib| self.vertices[ia].y.partial_cmp(&self.vertices[ib].y).unwrap()); 
            v
        };
        let ocvs : Vec<UV> = indices.iter().map(|&i| self.uvs[i]).collect();

       let mut top_to_bottom = Edge::new(min_y_vert, max_y_vert, 
        y_min as i32, y_max as i32,
        ocvs[0], self.gradient);

       let mut top_to_middle = Edge::new(min_y_vert, mid_y_vert, 
        y_min as i32, y_max as i32,
        ocvs[0], self.gradient);

       let mut middle_to_bottom = Edge::new(mid_y_vert, max_y_vert, 
        y_min as i32, y_max as i32,
        ocvs[1], self.gradient);

        self.scan_convert_edge_pair(context, &mut top_to_bottom, &mut top_to_middle, handedness);
        self.scan_convert_edge_pair(context, &mut top_to_bottom, &mut middle_to_bottom, handedness);
    }

    fn scan_convert_edge_pair(&self, context : &mut dyn Renderable, a : &mut Edge, b : &mut Edge, 
        handedness : bool) {            
                 
        let (y_start, y_end) = (b.y_start, b.y_end);

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
        let (x_min, x_max) = (left.x.ceil() as usize, right.x.ceil() as usize);
        
        let min_uv = left.uv;
        let max_uv = right.uv;

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
        let mut ovs = self.vertices.to_vec();
        ovs.sort_by(|va, vb| va.y.partial_cmp(&vb.y).unwrap());
        let signed_area = triangle_signed_area(ovs[0], ovs[2], ovs[1]);

        let handedness = signed_area >= 0f32;
        self.scan_convert_triangle(context, ovs[0], ovs[1], ovs[2], handedness);   
    }
}