use crate::screen::Screen;
use state::State;
use util::Block;

const CELL_WIDTH: u16 = 3;
const LINE_COUNT: u16 = 16;
const LINE_OFFSET: u16 = 1;

impl Screen {
    pub fn print_state(state: &State) {
        Self::print_header(state.hi(), state.lo());
        Self::print_block(state.block());
    }
    fn print_header(hi: u8, lo: u8) {
        Self::move_cursor(0, 0);
        Self::print_string(format!("X: {:02x}, A: {:02x}", hi, lo));
    }
    fn print_block(block: &Block<u8>) {
        (0..16).for_each(|y| Self::print_line(block, y));
    }
    fn print_line(block: &Block<u8>, y: u16) {
        (0..16).for_each(|x| Self::print_cell(block, x, y));
    }
    fn print_cell(block: &Block<u8>, x: u16, y: u16) {
        Self::move_cursor(x * CELL_WIDTH, y + LINE_OFFSET);
        let index = (x + y * LINE_COUNT) as u8;
        Self::print_string(format!("{:02x}", block[index]));
    }
}
