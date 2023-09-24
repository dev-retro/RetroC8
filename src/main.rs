pub mod core;

use std::fs::File;
use std::io::{Read, Seek};
use crate::core::cpu;

use minifb::{Key, Window, WindowOptions};

const WIDTH: usize = 64;
const HEIGHT: usize = 32;

fn main() {
    let mut chip_8 = cpu::CPU::new();
    let mut game = File::open("/Users/hevey/Development/RetroCade/!debugging/CHIP-8/programs/Chip8 Picture.ch8").expect("File not able to be opened");
    let mut contents = Vec::new();
    game.read_to_end(&mut contents).expect("File could not be read");
    chip_8.load_game(&*contents);

    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];

    let mut window = Window::new(
        "RetroC8",
        WIDTH,
        HEIGHT,
        WindowOptions {
            scale: minifb::Scale::X16,
            borderless: false,
            scale_mode: minifb::ScaleMode::AspectRatioStretch,
            ..WindowOptions::default()
        }
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });

    // Limit to max ~60 fps update rate
    window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));

    while window.is_open() && !window.is_key_down(Key::Escape) {
        let mut counter = 0;
        for i in buffer.iter_mut() {
            *i = if chip_8.bus.graphics.memory[counter] { 0xFFFFFF } else { 0x000000 };
            counter += 1;
        }

        chip_8.tick();

        // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
        window
            .update_with_buffer(&buffer, WIDTH, HEIGHT)
            .unwrap();
    }
}