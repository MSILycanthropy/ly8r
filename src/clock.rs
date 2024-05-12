use crate::{instructions::fetch_instruction, Chip8};

pub trait Clock {
    fn clock(&mut self, ticks: usize) {
        for _ in 0..ticks {
            self.tick();
        }

        self.tick_delay_timer();
        self.tick_sound_timer();
    }

    fn tick(&mut self);

    fn tick_delay_timer(&mut self);

    fn tick_sound_timer(&mut self);
}

impl Clock for Chip8 {
    fn tick(&mut self) {
        if self.waiting {
            for i in 0..self.keys.len() {
                if self.old_keys[i] && !self.keys[i] {
                    self.waiting = false;
                    break;
                }
            }

            self.old_keys = self.keys;
        }

        if self.waiting {
            return;
        }

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
