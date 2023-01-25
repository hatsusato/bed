use crate::screen::Screen;
use state::State;
use std::collections::VecDeque;
use util::Page;

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
        Self::print_queue(state.queue());
    }
    fn print_header(data: u8, acc: u8, block: u8, coord: u8, error: u8) {
        Self::move_cursor(0, 0);
        Self::print_string(format!(
            "D: {:02x}, A: {:02x}, B: {:02x}, C: {:02x}, E: {:1x}",
            data, acc, block, coord, error
        ));
    }
    fn print_queue(queue: &VecDeque<u8>) {
        Self::move_cursor(0, LINE_OFFSET + LINE_COUNT);
        let empty = [format!("  ")].into_iter().cycle();
        let q: Vec<_> = queue
            .iter()
            .map(|&c| format!("{:02x}", c))
            .chain(empty)
            .take(LINE_COUNT as usize)
            .collect();
        Self::print_string(q.join(" "));
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
