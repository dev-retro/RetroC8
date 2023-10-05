pub mod core;

use std::fs::File;
use std::io::{Read, Seek};
use std::time::{Duration, Instant};
use crate::core::cpu;

use minifb::{Key, Window, WindowOptions};

const WIDTH: usize = 64;
const HEIGHT: usize = 32;

fn main() {
    let mut chip_8 = cpu::CPU::new();
    let mut game = File::open("/Users/hevey/Development/RetroCade/!debugging/chip8-test-suite/bin/4-flags.ch8").expect("File not able to be opened");
    let mut contents = Vec::new();
    game.read_to_end(&mut contents).expect("File could not be read");
    chip_8.load_game(&contents);

    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];

    let mut window = Window::new(
        "RetroC8",
        WIDTH,
        HEIGHT,
        WindowOptions {
            scale: minifb::Scale::X16,
            ..WindowOptions::default()
        }
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });

    // Limit to max ~60 fps update rate
    //window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));

    let mut last_instruction_run_time = Instant::now();

    while window.is_open() && !window.is_key_down(Key::Escape) {
        if last_instruction_run_time.elapsed() > Duration::from_millis(5) {
            last_instruction_run_time = Instant::now();

            if window.is_key_down(Key::Key1) {
                chip_8.bus.input.update(0x1, true);
            }
            if window.is_key_down(Key::Key2) {
                chip_8.bus.input.update(0x2, true);
            }
            if window.is_key_down(Key::Key3) {
                chip_8.bus.input.update(0x3, true);
            }
            if window.is_key_down(Key::Key4) {
                chip_8.bus.input.update(0xC, true);
            }
            if window.is_key_down(Key::Q) {
                chip_8.bus.input.update(0x4, true);
            }
            if window.is_key_down(Key::W) {
                chip_8.bus.input.update(0x5, true);
            }
            if window.is_key_down(Key::E) {
                chip_8.bus.input.update(0x6, true);
            }
            if window.is_key_down(Key::R) {
                chip_8.bus.input.update(0xD, true);
            }
            if window.is_key_down(Key::A) {
                chip_8.bus.input.update(0x7, true);
            }
            if window.is_key_down(Key::S) {
                chip_8.bus.input.update(0x8, true);
            }
            if window.is_key_down(Key::D) {
                chip_8.bus.input.update(0x9, true);
            }
            if window.is_key_down(Key::F) {
                chip_8.bus.input.update(0xE, true);
            }
            if window.is_key_down(Key::Z) {
                chip_8.bus.input.update(0xA, true);
            }
            if window.is_key_down(Key::X) {
                chip_8.bus.input.update(0x0, true);
            }
            if window.is_key_down(Key::C) {
                chip_8.bus.input.update(0xB, true);
            }
            if window.is_key_down(Key::V) {
                chip_8.bus.input.update(0xF, true);
            }

            if !window.is_key_down(Key::Key1) {
                chip_8.bus.input.update(0x1, false);
            }
            if !window.is_key_down(Key::Key2) {
                chip_8.bus.input.update(0x2, false);
            }
            if !window.is_key_down(Key::Key3) {
                chip_8.bus.input.update(0x3, false);
            }
            if !window.is_key_down(Key::Key4) {
                chip_8.bus.input.update(0xC, false);
            }
            if !window.is_key_down(Key::Q) {
                chip_8.bus.input.update(0x4, false);
            }
            if !window.is_key_down(Key::W) {
                chip_8.bus.input.update(0x5, false);
            }
            if !window.is_key_down(Key::E) {
                chip_8.bus.input.update(0x6, false);
            }
            if !window.is_key_down(Key::R) {
                chip_8.bus.input.update(0xD, false);
            }
            if !window.is_key_down(Key::A) {
                chip_8.bus.input.update(0x7, false);
            }
            if !window.is_key_down(Key::S) {
                chip_8.bus.input.update(0x8, false);
            }
            if !window.is_key_down(Key::D) {
                chip_8.bus.input.update(0x9, false);
            }
            if !window.is_key_down(Key::F) {
                chip_8.bus.input.update(0xE, false);
            }
            if !window.is_key_down(Key::Z) {
                chip_8.bus.input.update(0xA, false);
            }
            if !window.is_key_down(Key::X) {
                chip_8.bus.input.update(0x0, false);
            }
            if !window.is_key_down(Key::C) {
                chip_8.bus.input.update(0xB, false);
            }
            if !window.is_key_down(Key::V) {
                chip_8.bus.input.update(0xF, false);
            }

            chip_8.tick_timers();
            chip_8.tick();

            if chip_8.bus.graphics.draw {
                let mut counter = 0;
                for i in buffer.iter_mut() {
                    *i = if chip_8.bus.graphics.memory[counter] { 0xFFFFFF } else { 0x000000 };
                    counter += 1;
                }

                chip_8.bus.graphics.draw = false;
            }
        }
            // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
            window
                .update_with_buffer(&buffer, WIDTH, HEIGHT)
                .unwrap();
    }
}