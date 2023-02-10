use crate::Bank;
use screen::Screen;
use util::BLOCK_SIDE;
use util::{Block, Page};

#[derive(Default)]
pub struct State {
    pub bank: Bank,
    pub memory: Block<Page>,
}

impl State {
    #[must_use]
    pub fn page(&self) -> &Page {
        &self.memory[self.bank.block]
    }
    #[must_use]
    pub fn bank(&self) -> Bank {
        self.bank
    }
    pub fn restore_bank(&mut self, bank: Bank) {
        self.bank = bank;
    }
    pub fn restore_page(&mut self, page: Option<Page>) {
        if let Some(page) = page {
            self.memory[self.bank.block] = page;
        }
    }
    pub fn print(&self, key: char) {
        self.bank.print(key);
        self.print_page();
    }
    fn print_page(&self) {
        (0..BLOCK_SIDE).for_each(|y| (0..BLOCK_SIDE).for_each(|x| self.print_cell(x, y)));
    }
    fn print_cell(&self, x: u8, y: u8) {
        move_cell(x, y);
        let index = x + y * BLOCK_SIDE;
        let msg = format!("{:02x}", self.page()[index]);
        if self.bank.coord == index {
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
