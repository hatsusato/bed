use crate::screen::Screen;
use state::State;
use util::Page;

const CELL_WIDTH: u16 = 3;
const LINE_COUNT: u16 = 16;
const LINE_OFFSET: u16 = 1;

impl Screen {
    pub fn print_state(state: &State, key: char) {
        Self::print_header(
            state.data(),
            state.acc(),
            state.block(),
            state.coord(),
            state.error() as u8,
            key,
        );
        Self::print_page(state.page());
    }
    fn print_header(data: u8, acc: u8, block: u8, coord: u8, error: u8, key: char) {
        Self::move_cursor(0, 0);
        Self::print_string(format!(
            "D: {:02x}, A: {:02x}, B: {:02x}, C: {:02x}, E: {:1x}, KEY: {}",
            data, acc, block, coord, error, key
        ));
    }
    fn print_page(page: &Page) {
        (0..16).for_each(|y| Self::print_line(page, y));
    }
    fn print_line(page: &Page, y: u16) {
        (0..16).for_each(|x| Self::print_cell(page, x, y));
    }
    fn print_cell(page: &Page, x: u16, y: u16) {
        Self::move_cursor(x * CELL_WIDTH, y + LINE_OFFSET);
        let index = (x + y * LINE_COUNT) as u8;
        Self::print_string(format!("{:02x}", page[index]));
    }
}
