use util::{Block, Page};

#[derive(Default, Clone, Copy)]
pub struct Bank {
    pub acc: u8,
    pub block: u8,
    pub coord: u8,
    pub data: u8,
    pub error: bool,
}

#[derive(Default)]
pub struct State {
    bank: Bank,
    memory: Block<Page>,
}

impl State {
    pub fn data(&self) -> u8 {
        self.bank.data
    }
    pub fn acc(&self) -> u8 {
        self.bank.acc
    }
    pub fn block(&self) -> u8 {
        self.bank.block
    }
    pub fn coord(&self) -> u8 {
        self.bank.coord
    }
    pub fn error(&self) -> bool {
        self.bank.error
    }
    pub fn page(&self) -> &Page {
        &self.memory[self.bank.block]
    }
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
}
