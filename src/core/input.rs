const KEY_COUNT: usize = 16;

pub struct Input {
    pub keys: [bool; KEY_COUNT]    
}

impl Input {
    pub fn new() -> Self {
        Self {
            keys: [false; KEY_COUNT]
        }
    }

    pub fn update(&mut self, key: usize, value: bool) {
        self.keys[key] = value;
    }
}