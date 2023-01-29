use screen::Screen;
use state::State;
use util::BLOCK_SIDE;

const CELL_WIDTH: u16 = 3;
const LINE_OFFSET: u16 = 1;

pub fn print_state(state: &State, key: char) {
    print_header(
        state.data(),
        state.acc(),
        state.block(),
        state.coord(),
        u8::from(state.error()),
        key,
    );
    print_page(state);
}
fn print_header(data: u8, acc: u8, block: u8, coord: u8, error: u8, key: char) {
    screen::Screen::move_cursor(0, 0);
    screen::Screen::print_display(format!(
        "D: {:02x}, A: {:02x}, B: {:02x}, C: {:02x}, E: {:1x}, KEY: {}",
        data, acc, block, coord, error, key
    ));
}
fn print_page(state: &State) {
    (0..BLOCK_SIDE).for_each(|y| print_line(state, y));
}
fn print_line(state: &State, y: u8) {
    (0..BLOCK_SIDE).for_each(|x| print_cell(state, x, y));
}
fn print_cell(state: &State, x: u8, y: u8) {
    let page = state.page();
    Screen::move_cursor(u16::from(x) * CELL_WIDTH, u16::from(y) + LINE_OFFSET);
    let index = x + y * BLOCK_SIDE;
    let msg = format!("{:02x}", page[index]);
    if state.coord() == index {
        Screen::print_highlight(msg);
    } else {
        Screen::print_display(msg);
    }
}
