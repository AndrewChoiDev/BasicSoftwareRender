pub struct Display {
    width: u32,
    height: u32
}

impl Display {
    pub fn new(width: u32, height: u32)
    -> Self {
        Self {
            width,
            height
        }
    }

}

pub fn create_display(width: u32, height: u32)
-> Display {
    Display::new(width, height)
}