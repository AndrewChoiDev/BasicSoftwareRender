use rand::Rng;
pub struct Stars3D {
    spread : f32,
    speed : f32,
    position : Vec<[f32 ; 3]>,
    fov_angle : f32
}
impl Stars3D {
    
    pub fn new(spread : f32, speed : f32, star_num : usize, fov_angle : f32)
    -> Self {
        Self {
            spread : spread,
            speed : speed,
            position : 
                vec![[0f32 ; 3] ; star_num]
                .into_iter()
                .map(|_| Stars3D::init_star(spread))
                .collect(),
            fov_angle : fov_angle,
        }
    }
    fn init_star(spread : f32)
    -> [f32 ; 3] {
        let mut rng = rand::thread_rng();
        [rng.gen_range(-spread, spread), rng.gen_range(-spread, spread), rng.gen_range(f32::EPSILON, spread)]
    }
    pub fn update(&mut self, delta : f32) {
        for pos in self.position.iter_mut() {
            (*pos)[2] -= delta * self.speed;
        }
    }
    pub fn render(&mut self, frame : &mut[u8], width : usize, height : usize) {
        let tan_factor = (self.fov_angle / 2f32).tan();
        for pos in self.position.iter_mut() {
            let x = ((width as f32) * (pos[0] / (pos[2] * tan_factor) * 0.5f32 + 0.5f32)) as i32;
            
            let y = ((height as f32) *  (pos[1] / (pos[2] * tan_factor) * 0.5f32 + 0.5f32)) as i32;

            if !(0..width as i32).contains(&x)
               || !(0..height as i32).contains(&y)
               || pos[2] < 0f32 {
                *pos = Stars3D::init_star(self.spread);
                continue;
            }
            let frame_index = ((x + y * width as i32) * 4) as usize;
            frame[frame_index..frame_index+4].copy_from_slice(&[0x00, 0xff, 0x00, 0xff]); 

        }
    }

}