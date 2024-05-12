use std::{fs::File, io::Read};

use instructions::{parse_opcode, Opcode};

pub mod clock;
mod frontend;
mod instructions;
mod stack;

pub const SCREEN_HEIGHT: usize = 32;
pub const SCREEN_WIDTH: usize = 64;

const START_ADDRESS: u16 = 0x0200;

const FONT: [u8; 80] = [
    0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
    0x20, 0x60, 0x20, 0x20, 0x70, // 1
    0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
    0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
    0x90, 0x90, 0xF0, 0x10, 0x10, // 4
    0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
    0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
    0xF0, 0x10, 0x20, 0x40, 0x40, // 7
    0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
    0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
    0xF0, 0x90, 0xF0, 0x90, 0x90, // A
    0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
    0xF0, 0x80, 0x80, 0x80, 0xF0, // C
    0xE0, 0x90, 0x90, 0x90, 0xE0, // D
    0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
    0xF0, 0x80, 0xF0, 0x80, 0x80, // F
];

pub struct Chip8 {
    pub ram: [u8; 4096],

    pub display: [bool; SCREEN_HEIGHT * SCREEN_WIDTH],
    pub keys: [bool; 16],

    i_register: u16,
    v_registers: [u8; 16],

    program_counter: u16,
    stack: [u16; 16],
    stack_pointer: u16,

    delay_timer: u8,
    sound_timer: u8,
}

impl Default for Chip8 {
    fn default() -> Self {
        let mut instance = Self {
            ram: [0; 4096],

            display: [false; SCREEN_HEIGHT * SCREEN_WIDTH],

            keys: [false; 16],

            i_register: 0,
            v_registers: [0; 16],

            program_counter: START_ADDRESS,
            stack: [0; 16],
            stack_pointer: 0,

            delay_timer: 0,
            sound_timer: 0,
        };

        instance.ram[..80].copy_from_slice(&FONT);

        instance
    }
}

impl Chip8 {
    fn fetch_opcode(&mut self) -> Opcode {
        let high_byte = self.ram[self.program_counter as usize];
        let low_byte = self.ram[self.program_counter as usize + 1];
        let raw = u16::from_le_bytes([low_byte, high_byte]);

        self.program_counter += 2;

        parse_opcode(raw)
    }

    pub fn load_from_file(&mut self, file_name: &str) {
        let mut rom = File::open(file_name).unwrap();
        let mut buffer = Vec::new();

        rom.read_to_end(&mut buffer).unwrap();

        self.load(&buffer);
    }

    fn load(&mut self, data: &[u8]) {
        let start_addr = START_ADDRESS as usize;
        let end_addr = start_addr + data.len();

        self.ram[start_addr..end_addr].copy_from_slice(data);
    }

    fn reset(&mut self) {
        self.ram = [0; 4096];
        self.display = [false; SCREEN_HEIGHT * SCREEN_WIDTH];
        self.keys = [false; 16];
        self.i_register = 0;
        self.v_registers = [0; 16];
        self.program_counter = START_ADDRESS;
        self.stack = [0; 16];
        self.stack_pointer = 0;
        self.delay_timer = 0;
        self.sound_timer = 0;

        self.ram[..80].copy_from_slice(&FONT);
    }

    fn clear_screen(&mut self) {
        self.display = [false; SCREEN_HEIGHT * SCREEN_WIDTH];
    }
}
