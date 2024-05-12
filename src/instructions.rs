use crate::{stack::Stack, Chip8, SCREEN_HEIGHT, SCREEN_WIDTH};

pub(crate) struct Opcode {
    full: u16,
    parts: (u16, u16, u16, u16),
}

impl Opcode {
    fn second(&self) -> usize {
        self.parts.1 as usize
    }

    fn third(&self) -> usize {
        self.parts.2 as usize
    }

    fn last(&self) -> usize {
        self.parts.3 as usize
    }

    fn last_3(&self) -> u16 {
        self.full & 0x0FFF
    }

    fn last_2(&self) -> u8 {
        (self.full & 0xFF) as u8
    }
}

const INSTRUCTION_TABLE: [fn(Opcode, &mut Chip8); 35] = [
    sys, clr, rts, jump, call, ske, skne, skre, load, add, move_, or, and, xor, addr, sub, shr,
    subn, shl, skrne, loadi, jumpv, rand, draw, skpr, skup, moved, keyd, loadd, loads, addi, ldspr,
    bcd, stor, read,
];

pub(crate) fn parse_opcode(raw: u16) -> Opcode {
    Opcode {
        full: raw,
        parts: (
            (raw & 0xF000) >> 12,
            (raw & 0x0F00) >> 8,
            (raw & 0x00F0) >> 4,
            raw & 0x000F,
        ),
    }
}

pub(crate) fn fetch_instruction(opcode: &Opcode) -> fn(Opcode, &mut Chip8) {
    let index = decode_instruction(opcode);

    INSTRUCTION_TABLE[index]
}

fn decode_instruction(opcode: &Opcode) -> usize {
    match opcode.parts {
        (0, 0, 0, 0) => 0,
        (0, 0, 0xE, 0) => 1,
        (0, 0, 0xE, 0xE) => 2,
        (1, _, _, _) => 3,
        (2, _, _, _) => 4,
        (3, _, _, _) => 5,
        (4, _, _, _) => 6,
        (5, _, _, 0) => 7,
        (6, _, _, _) => 8,
        (7, _, _, _) => 9,
        (8, _, _, 0) => 10,
        (8, _, _, 1) => 11,
        (8, _, _, 2) => 12,
        (8, _, _, 3) => 13,
        (8, _, _, 4) => 14,
        (8, _, _, 5) => 15,
        (8, _, _, 6) => 16,
        (8, _, _, 7) => 17,
        (8, _, _, 0xE) => 18,
        (9, _, _, 0) => 19,
        (0xA, _, _, _) => 20,
        (0xB, _, _, _) => 21,
        (0xC, _, _, _) => 22,
        (0xD, _, _, _) => 23,
        (0xE, _, 9, 0xE) => 24,
        (0xE, _, 0xA, 1) => 25,
        (0xF, _, 0, 7) => 26,
        (0xF, _, 0, 0xA) => 27,
        (0xF, _, 1, 5) => 28,
        (0xF, _, 1, 8) => 29,
        (0xF, _, 1, 0xE) => 30,
        (0xF, _, 2, 9) => 31,
        (0xF, _, 3, 3) => 32,
        (0xF, _, 5, 5) => 33,
        (0xF, _, 6, 5) => 34,
        _ => unreachable!(),
    }
}

fn sys(_: Opcode, _: &mut Chip8) {}

fn clr(_: Opcode, chip8: &mut Chip8) {
    chip8.clear_screen();
}

fn rts(_: Opcode, chip8: &mut Chip8) {
    let address = chip8.pop();

    chip8.program_counter = address;
}

fn jump(opcode: Opcode, chip8: &mut Chip8) {
    chip8.program_counter = opcode.last_3();
}

fn call(opcode: Opcode, chip8: &mut Chip8) {
    chip8.push(chip8.program_counter);

    chip8.program_counter = opcode.last_3();
}

fn ske(opcode: Opcode, chip8: &mut Chip8) {
    let vx_register = chip8.v_registers[opcode.second()];

    if vx_register == opcode.last_2() {
        chip8.program_counter += 2;
    }
}

fn skne(opcode: Opcode, chip8: &mut Chip8) {
    let vx_register = chip8.v_registers[opcode.second()];

    if vx_register != opcode.last_2() {
        chip8.program_counter += 2;
    }
}

fn skre(opcode: Opcode, chip8: &mut Chip8) {
    let vx = chip8.v_registers[opcode.second()];
    let vy = chip8.v_registers[opcode.third()];

    if vx == vy {
        chip8.program_counter += 2;
    }
}

fn load(opcode: Opcode, chip8: &mut Chip8) {
    chip8.v_registers[opcode.second()] = opcode.last_2();
}

fn add(opcode: Opcode, chip8: &mut Chip8) {
    let vx = chip8.v_registers[opcode.second()];

    chip8.v_registers[opcode.second()] = vx.wrapping_add(opcode.last_2());
}

fn move_(opcode: Opcode, chip8: &mut Chip8) {
    chip8.v_registers[opcode.second()] = chip8.v_registers[opcode.third()];
}

fn or(opcode: Opcode, chip8: &mut Chip8) {
    chip8.v_registers[opcode.second()] |= chip8.v_registers[opcode.third()]
}

fn and(opcode: Opcode, chip8: &mut Chip8) {
    chip8.v_registers[opcode.second()] &= chip8.v_registers[opcode.third()]
}

fn xor(opcode: Opcode, chip8: &mut Chip8) {
    chip8.v_registers[opcode.second()] ^= chip8.v_registers[opcode.third()]
}

fn addr(opcode: Opcode, chip8: &mut Chip8) {
    let vx = chip8.v_registers[opcode.second()];
    let vy = chip8.v_registers[opcode.third()];

    let (result, carry) = vx.overflowing_add(vy);

    chip8.v_registers[opcode.second()] = result;

    chip8.v_registers[0xF] = carry.into();
}

fn sub(opcode: Opcode, chip8: &mut Chip8) {
    let vx = chip8.v_registers[opcode.second()];
    let vy = chip8.v_registers[opcode.third()];

    let (result, borrow) = vx.overflowing_sub(vy);

    chip8.v_registers[opcode.second()] = result;

    chip8.v_registers[0xF] = (!borrow).into();
}

fn shr(opcode: Opcode, chip8: &mut Chip8) {
    let new_vf = chip8.v_registers[opcode.second()] & 1;

    chip8.v_registers[opcode.second()] >>= 1;
    chip8.v_registers[0xF] = new_vf;
}

fn subn(opcode: Opcode, chip8: &mut Chip8) {
    let vx = chip8.v_registers[opcode.second()];
    let vy = chip8.v_registers[opcode.third()];

    let (result, borrow) = vy.overflowing_sub(vx);

    chip8.v_registers[opcode.second()] = result;

    chip8.v_registers[0xF] = (!borrow).into();
}

fn shl(opcode: Opcode, chip8: &mut Chip8) {
    let new_vf = (chip8.v_registers[opcode.second()] >> 7) & 1;

    chip8.v_registers[opcode.second()] <<= 1;
    chip8.v_registers[0xF] = new_vf;
}

fn skrne(opcode: Opcode, chip8: &mut Chip8) {
    if chip8.v_registers[opcode.second()] != chip8.v_registers[opcode.third()] {
        chip8.program_counter += 2;
    }
}

fn loadi(opcode: Opcode, chip8: &mut Chip8) {
    chip8.i_register = opcode.last_3();
}

fn jumpv(opcode: Opcode, chip8: &mut Chip8) {
    chip8.program_counter = chip8.v_registers[0] as u16 + opcode.last_3()
}

fn rand(opcode: Opcode, chip8: &mut Chip8) {
    let random: u8 = rand::random();

    chip8.v_registers[opcode.second()] = random & opcode.last_2();
}

fn draw(opcode: Opcode, chip8: &mut Chip8) {
    let x_start = chip8.v_registers[opcode.second()] as usize;
    let y_start = chip8.v_registers[opcode.third()] as usize;

    let rows = opcode.last();

    let mut flipped = false;

    for row in 0..rows {
        let addr = chip8.i_register + row as u16;
        let pixels = chip8.ram[addr as usize];

        for col in 0..8 {
            if (pixels & (0b1000_0000 >> col)) != 0 {
                let x = (x_start + col) % SCREEN_WIDTH;
                let y = (y_start + row) % SCREEN_HEIGHT;

                let index = x + SCREEN_WIDTH * y;

                flipped |= chip8.display[index];

                chip8.display[index] = true;
            }
        }
    }

    chip8.v_registers[0xF] = flipped.into();
}

fn skpr(opcode: Opcode, chip8: &mut Chip8) {
    let vx = chip8.v_registers[opcode.second()] as usize;

    if chip8.keys[vx] {
        chip8.program_counter += 2;
    }
}

fn skup(opcode: Opcode, chip8: &mut Chip8) {
    let vx = chip8.v_registers[opcode.second()] as usize;

    if !chip8.keys[vx] {
        chip8.program_counter += 2;
    }
}

fn moved(opcode: Opcode, chip8: &mut Chip8) {
    chip8.v_registers[opcode.second()] = chip8.delay_timer;
}

fn keyd(opcode: Opcode, chip8: &mut Chip8) {
    let mut pressed = false;

    for i in 0..chip8.keys.len() {
        if chip8.keys[i] {
            chip8.v_registers[opcode.second()] = i as u8;

            pressed = true;
            break;
        }
    }

    if !pressed {
        chip8.program_counter -= 2;
    }
}

fn loadd(opcode: Opcode, chip8: &mut Chip8) {
    let vx = chip8.v_registers[opcode.second()];

    chip8.delay_timer = vx;
}

fn loads(opcode: Opcode, chip8: &mut Chip8) {
    let vx = chip8.v_registers[opcode.second()];

    chip8.sound_timer = vx;
}

fn addi(opcode: Opcode, chip8: &mut Chip8) {
    let vx = chip8.v_registers[opcode.second()] as u16;

    chip8.i_register = chip8.i_register.wrapping_add(vx);
}

fn ldspr(opcode: Opcode, chip8: &mut Chip8) {
    let vx = chip8.v_registers[opcode.second()] as u16;

    chip8.i_register = vx * 5;
}

fn bcd(opcode: Opcode, chip8: &mut Chip8) {
    let vx = chip8.v_registers[opcode.second()] as f32;

    let hundreds = (vx / 100.).floor() as u8;
    let tens = (vx / 10.).floor() as u8;
    let ones = (vx % 10.).floor() as u8;

    let start = chip8.i_register as usize;

    chip8.ram[start] = hundreds;
    chip8.ram[start + 1] = tens;
    chip8.ram[start + 2] = ones;
}

fn stor(opcode: Opcode, chip8: &mut Chip8) {
    let start = chip8.i_register as usize;

    for index in 0..=opcode.second() {
        chip8.ram[start + index] = chip8.v_registers[index];
    }
}

fn read(opcode: Opcode, chip8: &mut Chip8) {
    let start = chip8.i_register as usize;

    for index in 0..=opcode.second() {
        chip8.v_registers[index] = chip8.ram[start + index];
    }
}
