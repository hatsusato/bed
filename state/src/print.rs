use crate::State;
use screen::Screen;
use util::BLOCK_SIDE;

impl State {
    pub fn print(&self, key: char) {
        self.print_header(key);
        self.print_page();
    }
    fn print_header(&self, key: char) {
        Screen::move_cursor(0, 0);
        Screen::print_display(format!(
            "D: {:02x}, A: {:02x}, B: {:02x}, C: {:02x}, E: {:1x}, KEY: {}",
            self.data(),
            self.acc(),
            self.block(),
            self.coord(),
            u8::from(self.error()),
            key
        ));
    }
    fn print_page(&self) {
        (0..BLOCK_SIDE).for_each(|y| (0..BLOCK_SIDE).for_each(|x| self.print_cell(x, y)));
    }
    fn print_cell(&self, x: u8, y: u8) {
        move_cell(x, y);
        let index = x + y * BLOCK_SIDE;
        let msg = format!("{:02x}", self.page()[index]);
        if self.coord() == index {
            Screen::print_highlight(msg);
        } else {
            Screen::print_display(msg);
        }
    }
}

fn move_cell(x: u8, y: u8) {
    const CELL_WIDTH: u16 = 3;
    const LINE_OFFSET: u16 = 1;
    let x = u16::from(x) * CELL_WIDTH;
    let y = u16::from(y) + LINE_OFFSET;
    Screen::move_cursor(x, y);
}
