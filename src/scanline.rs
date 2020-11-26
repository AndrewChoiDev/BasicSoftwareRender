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
    color_gradient : Vector2<VertexColor>,
}

impl Scanline {

    pub fn color_vertices(&self)
    -> [VertexColor ; 3] {
        self.color_vertices
    }
    pub fn new(vertices : [Vertex ; 3], color_vertices : [VertexColor ; 3]) -> Scanline {
    
        let indices : Vec<usize> = {
            let mut v : Vec<usize> = (0..3).collect(); 
            v.sort_by(|&ia, &ib| vertices[ia].y.partial_cmp(&vertices[ib].y).unwrap()); 
            v
        };
        let ovs : Vec<Vertex> = indices.iter().map(|&i| vertices[i]).collect();
        let ocvs : Vec<VertexColor> = indices.iter().map(|&i| color_vertices[i]).collect();

        let dc_over_dx_numerator : VertexColor = 
            (ocvs[1] - ocvs[2]) * (ovs[0].y - ovs[2].y) 
            - (ocvs[0] - ocvs[2]) * (ovs[1].y - ovs[2].y);
        let dc_over_dy_numerator : VertexColor = 
            (ocvs[1] - ocvs[2]) * (ovs[0].x - ovs[2].x) 
            - (ocvs[0] - ocvs[2]) * (ovs[1].x - ovs[2].x);
        let dc_over_dx_denominator = 1.0 / 
            ((ovs[1].x - ovs[2].x) * (ovs[0].y - ovs[2].y) 
            - (ovs[0].x - ovs[2].x) * (ovs[1].y - ovs[2].y));

        let dc_over_dy_denominator = -dc_over_dx_denominator;

        let color_gradient = Vector2::new(dc_over_dx_numerator * dc_over_dx_denominator, dc_over_dy_numerator * dc_over_dy_denominator);

        Scanline {vertices, color_vertices, color_gradient}
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
        let ocvs : Vec<VertexColor> = indices.iter().map(|&i| self.color_vertices[i]).collect();

       let mut top_to_bottom = Edge::new(min_y_vert, max_y_vert, 
        y_min as i32, y_max as i32,
        ocvs[0], self.color_gradient);

       let mut top_to_middle = Edge::new(min_y_vert, mid_y_vert, 
        y_min as i32, y_max as i32,
        ocvs[0], self.color_gradient);

       let mut middle_to_bottom = Edge::new(mid_y_vert, max_y_vert, 
        y_min as i32, y_max as i32,
        ocvs[1], self.color_gradient);

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
        
        let min_color = left.color;
        let max_color = right.color;

        let mut lerp_value = 0.;
        let lerp_step = 1. / (x_max - x_min) as f32;

        let color_delta = max_color - min_color;

        for i in x_min..x_max {

            let color = min_color + lerp_value * color_delta;

            let color_bytes = color.map(|c| (c * 255f32 + 0.5f32) as u8).into();                
            context.plot(color_bytes, i, j);

            lerp_value += lerp_step;
        }
    }
    
}
struct Edge {
    x : f32,
    dx_over_dy : f32,
    y_start : usize,
    y_end : usize,
    color : VertexColor,
    color_step : VertexColor,
}

impl Edge {
    fn new(start : Vertex, end : Vertex, y_min : i32, y_max : i32, start_color : VertexColor, color_gradient : Vector2<VertexColor>)
    -> Self {
        let y_dist = end.y - start.y;
        let x_dist = end.x - start.x;
       
        let dx_over_dy = x_dist / y_dist;
        let y_pre_step = start.y.ceil() - start.y;
        let x = start.x + y_pre_step * dx_over_dy;
        let x_pre_step = x - start.x;

        let color : VertexColor = 
            start_color 
            + color_gradient.x * x_pre_step 
            + color_gradient.y * y_pre_step;

        let color_step = (color_gradient.y * 1.) + (color_gradient.x * dx_over_dy);
        Self {
            y_start : (start.y.ceil() as i32).max(y_min) as usize,
            y_end : (end.y.ceil() as i32).min(y_max) as usize,
            x,
            dx_over_dy,
            color,
            color_step
        }
    }

    fn step(&mut self) {
        self.x += self.dx_over_dy;
        self.color += self.color_step;
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