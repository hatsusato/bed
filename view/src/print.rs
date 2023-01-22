use crate::screen::Screen;
use state::State;
use util::Block;

const CELL_WIDTH: u16 = 3;
const LINE_COUNT: u16 = 16;
const LINE_OFFSET: u16 = 1;

impl Screen {
    pub fn print_state(state: &State) {
        Self::print_header(
            state.data(),
            state.acc(),
            state.block(),
            state.coord(),
            state.error() as u8,
        );
        Self::print_page(state.page());
    }
    fn print_header(data: u8, acc: u8, block: u8, coord: u8, error: u8) {
        Self::move_cursor(0, 0);
        Self::print_string(format!(
            "D: {:02x}, A: {:02x}, B: {:02x}, C: {:02x}, E: {:1x}",
            data, acc, block, coord, error
        ));
    }
    fn print_page(block: &Block<u8>) {
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
