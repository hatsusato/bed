use util::Block;
use util::Page;

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
    pub fn set_reg(&mut self, val: u16) {
        (self.bank.data, self.bank.acc) = (trunc(val >> u8::BITS), trunc(val));
    }
    pub fn get_reg(&self) -> u16 {
        u16::from(self.bank.data) << u8::BITS | u16::from(self.bank.acc)
    }
    pub fn raise(&mut self) {
        self.bank.error = true;
    }
}

fn trunc(val: u16) -> u8 {
    const MASK: u16 = u8::MAX as u16;
    (val & MASK) as u8
}
