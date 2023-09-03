const GRAPHICS_SIZE: usize = 64 * 32;

pub struct GPU {
    pub memory: [bool; GRAPHICS_SIZE],
    pub draw: bool
}

impl GPU {
    pub fn new() -> Self {
        Self {
            memory: [false; GRAPHICS_SIZE],
            draw: true
        }
    }
}