pub trait RenderSystem {
    fn render(&self, context : &mut dyn Renderable);
}
pub trait Renderable {
    fn width(&self) -> i32;
    fn height(&self) -> i32;
    fn plot(&mut self, pixel : [u8 ; 4], x : usize, y : usize);
}
impl<T> Renderable for pixels::Pixels<T> where T : pixels::raw_window_handle::HasRawWindowHandle {
    fn width(&self)
    -> i32 {
        self.context().texture_extent.width as i32
    }

    fn height(&self)
    -> i32 {
        self.context().texture_extent.height as i32
    }

    fn plot(&mut self, pixel : [u8 ; 4], x : usize, y : usize) {
        let frame_index = ((x + y * self.width() as usize) * 4) as usize;
        self.get_frame()[frame_index..frame_index+4].copy_from_slice(&pixel);
    }
    
}