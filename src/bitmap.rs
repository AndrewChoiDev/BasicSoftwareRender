use nalgebra::*;
use rand::Rng;
type UV = Vector2<f32>;
type Dimensions2D = Vector2<usize>;
pub struct Bitmap {
    pixels : Vec<[u8 ; 4]>,
    dims : Dimensions2D
}

impl Bitmap {
    pub fn new(dims : Dimensions2D) 
    -> Self {
        Self {
            pixels : vec![[0 ; 4] ; dims.x * dims.y],
            dims
        }
    }

    pub fn new_random(dims : Dimensions2D)
    -> Self {
        let mut rng = rand::thread_rng();
        Self {
            pixels : (0..dims.x*dims.y).into_iter().map(|_| [rng.gen_range(0, 255), rng.gen_range(0, 255), rng.gen_range(0, 255), 255]).collect(),
            dims
        }
    }

    
    fn uv_to_usize(&self, uv : UV)
    -> Vector2<usize> {
        [(uv.x * (self.dims.x - 1) as f32).round() as usize,
        (uv.y * (self.dims.y - 1) as f32).round() as usize].into()
    }
    pub fn get_pixel(&self, uv : UV) 
    -> [u8 ; 4] {
        let coords = self.uv_to_usize(uv);
        self.pixels[coords.x + coords.y * self.dims.x]
    }
}
