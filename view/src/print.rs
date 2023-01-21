use crate::screen::Screen;
use util::Block;

const CELL_WIDTH: u16 = 3;
const LINE_COUNT: u16 = 16;

impl Screen {
    pub fn print_block(block: &Block<u8>) {
        (0..16).for_each(|y| Self::print_line(block, y));
    }
    fn print_line(block: &Block<u8>, y: u16) {
        (0..16).for_each(|x| Self::print_cell(block, x, y));
    }
    fn print_cell(block: &Block<u8>, x: u16, y: u16) {
        Self::move_cursor(x * CELL_WIDTH, y);
        let index = (x + y * LINE_COUNT) as u8;
        Self::print_string(format!("{:02x}", block[index]));
    }
}
