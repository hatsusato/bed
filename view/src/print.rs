use crate::screen::Screen;
use state::State;
use util::BLOCK_SIDE;

const CELL_WIDTH: u16 = 3;
const LINE_OFFSET: u16 = 1;

impl Screen {
    pub fn print_state(state: &State, key: char) {
        Self::print_header(
            state.data(),
            state.acc(),
            state.block(),
            state.coord(),
            u8::from(state.error()),
            key,
        );
        Self::print_page(state);
    }
    fn print_header(data: u8, acc: u8, block: u8, coord: u8, error: u8, key: char) {
        Self::move_cursor(0, 0);
        Self::print_string(format!(
            "D: {:02x}, A: {:02x}, B: {:02x}, C: {:02x}, E: {:1x}, KEY: {}",
            data, acc, block, coord, error, key
        ));
    }
    fn print_page(state: &State) {
        (0..BLOCK_SIDE).for_each(|y| Self::print_line(state, y));
    }
    fn print_line(state: &State, y: u8) {
        (0..BLOCK_SIDE).for_each(|x| Self::print_cell(state, x, y));
    }
    fn print_cell(state: &State, x: u8, y: u8) {
        let page = state.page();
        Self::move_cursor(u16::from(x) * CELL_WIDTH, u16::from(y) + LINE_OFFSET);
        let index = x + y * BLOCK_SIDE;
        let msg = format!("{:02x}", page[index]);
        if state.coord() == index {
            Self::print_highlight(msg);
        } else {
            Self::print_string(msg);
        }
    }
}
