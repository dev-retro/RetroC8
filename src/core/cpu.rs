use rand::Rng;

use crate::core::memory_bus::MemoryBus;

const REGISTER_SIZE: usize = 16;
const STACK_COUNT: usize = 12;
const PROGRAM_START_ADDRESS: u16 = 0x200;

pub struct CPU {
    registers: [u16; REGISTER_SIZE],
    i: u16,
    pc: u16,
    sp: usize,
    stack: [u16; STACK_COUNT],
    pub bus: MemoryBus
}

impl CPU {
    pub fn new() -> Self {
        Self {
            registers: [0; REGISTER_SIZE],
            i: 0,
            sp: 0,
            stack: [0; STACK_COUNT],
            bus: MemoryBus::new(),
            pc: PROGRAM_START_ADDRESS
        }
    }

    pub fn load_game(&mut self, file: &[u8]) {
        if Some(file).is_some() {
            if file.len() < 4096 - 512 {
                self.bus.memory[512..512+file.len()].copy_from_slice(&file[..]);
            }
        }
    }

    pub fn tick(&mut self) {
        let opcode = self.fetch();
        self.execute(opcode);
    }

    pub fn tick_timers(&mut self) {
        if self.bus.delay_timer > 0 {
            self.bus.delay_timer -= 1;
        }

        if self.bus.sound_timer > 0 {
            if self.bus.sound_timer == 1 {
                // BEEP
            }
            self.bus.sound_timer -= 1;
        }
    }

    fn fetch(&mut self) -> u16 {
        let higher_byte = self.bus.memory[self.pc as usize] as u16;
        let lower_byte = self.bus.memory[(self.pc + 1) as usize] as u16;
        let opcode = (higher_byte << 8) | lower_byte;
        self.pc += 2;
        opcode
    }


    fn execute(&mut self, opcode: u16) {
        let op1 = (opcode & 0xF000) >> 12;
        let op2 = (opcode & 0x0F00) >> 8;
        let op3 = (opcode & 0x00F0) >> 4;
        let op4 = opcode & 0x000F;

        match (op1, op2, op3, op4) {

            (0x0, 0x0, 0x0, 0x0) => return,
            (0x0, 0x0, 0xE, 0x0) => self.bus.graphics.memory = [false; 32 * 64],
            (0x0, 0x0, 0xE, 0xE) => {
                self.sp -= 2;
                self.pc = self.stack[self.sp];
            }
            (0x1, _, _, _) => { self.pc = opcode & 0x0FFF; }
            (0x2, _, _, _) => {
                self.stack[self.sp] = self.pc;
                self.sp += 2;
                self.pc = opcode & 0x0FFF;
            }
            (0x3, _, _, _) => {
                if self.registers[op2 as usize] == opcode & 0x0FF {
                    self.pc += 2;
                }
            }
            (0x4, _, _, _) => {
                if self.registers[op2 as usize] != opcode & 0x0FF {
                    self.pc += 2;
                }
            }
            (0x5, _, _, _) => {
                if self.registers[op2 as usize] == self.registers[op3 as usize]  {
                    self.pc += 2;
                }
            }
            (0x6, _, _, _) => {
                self.registers[op2 as usize] = opcode & 0x00FF;
            }
            (0x7, _, _, _) => {
                self.registers[op2 as usize] += opcode & 0x00FF;
            }
            (0x8, _, _, 0x0) => {
                self.registers[op2 as usize] = self.registers[op3 as usize];
            }
            (0x8, _, _, 0x1) => {
                self.registers[op2 as usize] |= self.registers[op3 as usize];
            }
            (0x8, _, _, 0x2) => {
                self.registers[op2 as usize] &= self.registers[op3 as usize];
            }
            (0x8, _, _, 0x3) => {
                self.registers[op2 as usize] ^= self.registers[op3 as usize];
            }
            (0x8, _, _, 0x4) => {
                let (value, carry) = self.registers[op2 as usize].overflowing_add(self.registers[op3 as usize]);
                self.registers[op2 as usize] = value as u8 as u16;
                self.registers[0xF] = if carry { 0xF } else { 0x0 };
            }
            (0x8, _, _, 0x5) => {
                let (value, borrow) = self.registers[op2 as usize].overflowing_sub(self.registers[op3 as usize]);
                self.registers[op2 as usize] = value as u8 as u16;
                self.registers[0xF] = if !borrow { 0xF } else { 0x0 };
            }
            (0x8, _, _, 0x6) => {
                self.registers[0xF] = self.registers[op3 as usize] & 0x1;
                self.registers[op2 as usize] = self.registers[op3 as usize] >> 1;
            }
            (0x8, _, _, 0x7) => {
                let (value, borrow) = self.registers[op3 as usize].overflowing_sub(self.registers[op2 as usize]);
                self.registers[0xF] = if borrow { 0x0 } else { 0xF };
                self.registers[op2 as usize] = value as u8 as u16;
            }
            (0x8, _, _, 0xE) => {
                // self.registers[op2 as usize] = self.registers[op3 as usize];
                self.registers[0xF] = (self.registers[op2 as usize] >> 7) as u8 as u16;
                self.registers[op2 as usize] <<= 1u8 as u16;
            }
            (0x9, _, _, 0x0) => {
                if self.registers[02 as usize] != self.registers[03 as usize] {
                    self.pc += 2;
                }
            }
            (0xA, _, _, _) => {
                self.i = opcode & 0x0FFF;
            }
            (0xB, _, _, _) => {
                self.pc = (opcode & 0x0FFF) + self.registers[0x0];
            }
            (0xC, _, _, _) => {
                let random = rand::thread_rng().gen_range(0..255);
                self.registers[op2 as usize] = random & (opcode & 0x00FF);
            }
            (0xD, _, _, _) => {
                let x = self.registers[op2 as usize];
                let y = self.registers[op3 as usize];
                let height = op4;

                self.registers[0xF] = 0;

                for y_line in 0..height {
                    let pixel = self.bus.memory[(self.i + y_line) as usize];

                    for x_line in 0..8 {
                        if pixel & (0x80 >> x_line) != 0 {
                            if self.bus.graphics.memory[(x + x_line + ((y + y_line) * 64)) as usize] {
                                self.registers[0xF] = 1;
                            }
                            self.bus.graphics.memory[(x + x_line + ((y + y_line) * 64)) as usize] = !self.bus.graphics.memory[(x + x_line + ((y + y_line) * 64)) as usize];
                        }
                    }

                    self.bus.graphics.draw = true;
                }
            }
            (0xE, _, 0x9, 0xE) => {
                if self.bus.input.keys[self.registers[op2 as usize] as usize] {
                    self.pc += 2;
                }
            }
            (0xE, _, 0xA, 0x1) => {
                if !self.bus.input.keys[self.registers[op2 as usize] as usize] {
                    self.pc += 2;
                }
            }
            (0xF, _, 0x0, 0x7) => {
                self.registers[op2 as usize] = self.bus.delay_timer;
            }
            (0xF, _, 0x0, 0xA) => {
                let mut key_pressed = false;

                for key in self.bus.input.keys {
                    if key {
                        key_pressed = true;
                    }

                    if !key_pressed {
                        self.pc -= 2;
                    }
                }
            }
            (0xF, _, 0x1, 0x5) => {
                self.bus.delay_timer = self.registers[op2 as usize];
            }
            (0xF, _, 0x1, 0x8) => {
                self.bus.sound_timer = self.registers[op2 as usize];
            }
            (0xF, _, 0x1, 0xE) => {
                self.i = self.i + self.registers[op2 as usize];
            }
            (0xF, _, 0x2, 0x9) => {
                self.i = self.registers[op2 as usize] * 0x5;
            }
            (0xF, _, 0x3, 0x3) => {
                self.bus.memory[self.i as usize] = (self.registers[op2 as usize] / 100) as u8;
                self.bus.memory[(self.i + 1) as usize] = ((self.registers[op2 as usize] / 10) % 10) as u8;
                self.bus.memory[(self.i + 2) as usize] = ((self.registers[op2 as usize] % 100) % 10) as u8;
            }
            (0xF, _, 0x5, 0x5) => {
                for i in 0..= op2 {
                    self.bus.memory[(self.i + i) as usize] = self.registers[i as usize] as u8;
                }
            }
            (0xF, _, 0x6, 0x5) => {
                for i in 0..= op2 {
                    self.registers[i as usize] = self.bus.memory[(self.i + i) as usize] as u16;
                }
            }
            (_, _, _, _) => unimplemented!("Unimplemented opcode: {}", opcode),
        }
    }
}