use crate::Chip8;

pub trait Frontend {
    fn setup();

    fn render_frame(chip8: &Chip8);
}
