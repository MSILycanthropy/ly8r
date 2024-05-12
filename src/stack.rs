use crate::Chip8;

pub(crate) trait Stack {
    fn push(&mut self, value: u16);
    fn pop(&mut self) -> u16;
    fn read_stack(&mut self) -> u16;
    fn write_stack(&mut self, value: u16);
}

impl Stack for Chip8 {
    #[inline]
    fn pop(&mut self) -> u16 {
        self.stack_pointer -= 1;

        self.read_stack()
    }

    #[inline]
    fn push(&mut self, value: u16) {
        self.write_stack(value);

        self.stack_pointer += 1;
    }

    #[inline]
    fn write_stack(&mut self, value: u16) {
        self.stack[self.stack_pointer as usize] = value;
    }

    #[inline]
    fn read_stack(&mut self) -> u16 {
        self.stack[self.stack_pointer as usize]
    }
}
