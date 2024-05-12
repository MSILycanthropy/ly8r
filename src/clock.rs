use crate::{instructions::fetch_instruction, Chip8};

pub trait Clock {
    fn clock(&mut self, ticks: usize) {
        for _ in 0..ticks {
            self.tick();
        }
    }

    fn tick(&mut self);

    fn tick_delay_timer(&mut self);

    fn tick_sound_timer(&mut self);
}

impl Clock for Chip8 {
    fn tick(&mut self) {
        let opcode = self.fetch_opcode();

        let instruction = fetch_instruction(&opcode);

        instruction(opcode, self);
    }

    fn tick_delay_timer(&mut self) {
        if self.delay_timer == 0 {
            return;
        }

        self.delay_timer -= 1;
    }

    fn tick_sound_timer(&mut self) {
        if self.sound_timer == 0 {
            return;
        }

        if self.sound_timer == 1 {
            // TODO: Do sound
        }

        self.sound_timer -= 1;
    }
}
