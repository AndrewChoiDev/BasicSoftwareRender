use nalgebra::*;
use super::edge::Edge;
use super::bitmap::*;
use std::rc::Rc;
type Vertex = Vector4<f32>;
type Vect = Vector3<f32>;

pub fn triangle_signed_area(v_min_y : Vector3<f32>, v_max_y : Vector3<f32>, v_mid_y : Vector3<f32>) 
-> f32 {
    xy_cross_product_magnitude(v_min_y - v_max_y, v_min_y - v_mid_y)
}
pub fn new_vertex(position : &Point3<f32>)
-> Vertex {
    let p = position;

    Vector4::new(p.x, p.y, p.z, 1.)
}
pub fn xy_cross_product_magnitude(v1 : Vect, v2 : Vect) 
-> f32 {
    (v1.x * v2.y - v2.x * v1.y) * 0.5f32
}
type UV = Vector2<f32>;
pub struct Scanline {
    vertices : Vec<Vertex>, 
    uvs : Vec<UV>,
    texture : Rc<Bitmap>,
    transform_matrix : Matrix4<f32>
}

impl Scanline {
    
    pub fn new(
        dimensions : [usize ; 2], positions : &[Point3<f32>], vertex_uvs : &[UV], 
        texture : Rc<Bitmap>, model_angle : f32) 
    -> Scanline {
    
        let vertices = 
            positions.iter()
            .map(|p| new_vertex(p)).collect();

        let uvs = 
            vertex_uvs.iter()
            .map(|uv| uv.clone()).collect();

        let transform_matrix = 
            Scanline::construct_transform_matrix(dimensions, model_angle);

        Scanline {vertices, uvs, texture, transform_matrix}
    }


    
    
    fn construct_transform_matrix(dimensions : [usize ; 2], model_angle : f32) 
    -> Matrix4<f32> {
        let screen_space : Matrix4<f32> = 
            Matrix4::new_nonuniform_scaling(
                &Vector3::new(dimensions[0] as f32 / 2.0, dimensions[1] as f32 / 2.0, 0.0))
            * Matrix4::new_translation(&Vector3::new(1.0, 1.0, 0.0));

        let model = Isometry3::new(
        Vector3::new(0.0, 0.1, 1.5), Vector3::y() * model_angle);

        let aspect_ratio = dimensions[0] as f32 / dimensions[1] as f32;

        let perspective_matrix = 
            Matrix::new_perspective(
                aspect_ratio, 110f32.to_radians(),
                0.01, 200.0
            );

        screen_space * (perspective_matrix * model.to_homogeneous())
    }

    fn transformed_vertices(&self)
    -> Vec<Vector4<f32>> {
        self.vertices.iter()
        .map(
            |v| {

                let transformed_vertex = self.transform_matrix * v;


                let mut perspective_divided_vertex : _ = transformed_vertex / transformed_vertex[3];
                perspective_divided_vertex[3] = transformed_vertex[3]; // save w component for later
                perspective_divided_vertex
            }
        )
        .collect()
    }

    fn scan_convert_triangle(&self, context : &mut dyn Renderable) {

        let trans_verts = self.transformed_vertices();

        // sort indices to transformed vertices by y value ascending
        let indices : Vec<usize> = {
            let mut v : Vec<usize> = (0..3).collect(); 
            v.sort_by(|&ia, &ib| 
                trans_verts[ia].y.partial_cmp(&trans_verts[ib].y).unwrap()); 
            v
        };

        //ordered vertices and ordered vertex values
        let ovs : Vec<Vertex> = indices.iter().map(|&i| trans_verts[i]).collect();
        let ovvs : Vec<UV> = indices.iter().map(|&i| self.uvs[i]).collect();

        let handedness = 
            triangle_signed_area(
                ovs[0].xyz(), ovs[2].xyz(), ovs[1].xyz()) 
            >= 0f32;

        let y_min = ovs[0].y.ceil() as usize;
        let y_max = (ovs[2].y.ceil() as usize).min(context.height() as usize);

        let mut top_to_bottom = 
            Edge::new(
                &ovs, 0, 2, 
                y_min as i32, y_max as i32, &ovvs);

        let mut top_to_middle = 
            Edge::new(
                &ovs, 0, 1, 
                y_min as i32, y_max as i32, &ovvs);

        let mut middle_to_bottom = 
            Edge::new(
                &ovs, 1, 2, 
                y_min as i32, y_max as i32, &ovvs);

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

        let min_inverse_w = left.inverse_w();
        let max_inverse_w = right.inverse_w();

        let uv_delta = max_uv - min_uv;
        
        let inverse_w_delta = max_inverse_w - min_inverse_w;

        let x_delta_float = (x_max - x_min) as f32;

        for i in x_min..x_max {

            let lerp_value = (i - x_min) as f32 / x_delta_float;

            let uv = min_uv + lerp_value * uv_delta;

            let z = 1. / (min_inverse_w + lerp_value * inverse_w_delta);

            let color_bytes = self.texture.get_pixel(uv * z);

            context.plot(color_bytes, i, j);

        }
    }
    
}



use super::render_system as rs;
use rs::RenderSystem;
use rs::Renderable;
impl RenderSystem for Scanline {
    fn render(&self, context : &mut dyn Renderable) {
        self.scan_convert_triangle(context);   
    }
}