use nalgebra::*;

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

pub struct Scanline {
    vertices : [Vertex ; 3], 
    color_vertices : [VertexColor ; 3], 
}

impl Scanline {
    pub fn new(vertices : [Vertex ; 3], color_vertices : [VertexColor ; 3]) -> Scanline {
       Scanline {vertices, color_vertices}
    }
    pub fn update_vertices(&mut self, vertices : [Vertex ; 3]) {
        self.vertices = vertices;
    }
    
    
    pub fn scan_convert_triangle(&self, context : &mut dyn Renderable, min_y_vert : Vertex, 
        mid_y_vert : Vertex, max_y_vert : Vertex, handedness : bool) {
       let y_min = min_y_vert.y.ceil() as usize;
       let y_max = (max_y_vert.y.ceil() as usize).min(context.height() as usize);

       let mut top_to_bottom = Edge::new(min_y_vert, max_y_vert, 
        y_min as i32, y_max as i32);

       let mut top_to_middle = Edge::new(min_y_vert, mid_y_vert, 
        y_min as i32, y_max as i32);

       let mut middle_to_bottom = Edge::new(mid_y_vert, max_y_vert, 
        y_min as i32, y_max as i32);

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
                left.x.ceil() as usize, right.x.ceil() as usize, j);
            left.step();
            right.step();
        }
    }

    fn draw_scan_line(&self, context : &mut dyn Renderable, x_min : usize, x_max : usize, j : usize) {            
        for i in x_min..x_max {
            context.plot([255u8, 0, 0, 255u8], i, j);
        }
    }
    
}
struct Edge {
    x : f32,
    dx_over_dy : f32,
    y_start : usize,
    y_end : usize
}

impl Edge {
    fn new(start : Vertex, end : Vertex, y_min : i32, y_max : i32)
    -> Self {
        let y_dist = end.y - start.y;
        let x_dist = end.x - start.x;
       
        let dx_over_dy = x_dist / y_dist;
        let y_pre_step = start.y.ceil() - start.y;
        
        Self {
            y_start : (start.y.ceil() as i32).max(y_min) as usize,
            y_end : (end.y.ceil() as i32).min(y_max) as usize,
            x : start.x + y_pre_step * dx_over_dy,
            dx_over_dy,
        }
    }

    fn step(&mut self) {
        self.x += self.dx_over_dy;
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