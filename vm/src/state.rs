use crate::{Bank, Page};
use util::Block;

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
        self.page().print(self.bank.coord);
    }
}
