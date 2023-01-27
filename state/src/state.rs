use util::Block;
use util::Page;

#[derive(Default)]
pub struct Bank {
    pub acc: u8,
    pub block: u8,
    pub coord: u8,
    pub data: u8,
    pub error: bool,
    pub page: Option<Page>,
}

#[derive(Default)]
pub struct State {
    pub acc: u8,
    pub block: u8,
    pub coord: u8,
    pub data: u8,
    pub error: bool,
    pub memory: Block<Page>,
}

impl State {
    pub fn restore_bank(&mut self, bank: &Bank) {
        self.acc = bank.acc;
        self.block = bank.block;
        self.coord = bank.coord;
        self.data = bank.data;
        self.error = bank.error;
        if let Some(page) = bank.page {
            self.memory[self.block] = page;
        }
    }
    pub fn data(&self) -> u8 {
        self.data
    }
    pub fn acc(&self) -> u8 {
        self.acc
    }
    pub fn block(&self) -> u8 {
        self.block
    }
    pub fn coord(&self) -> u8 {
        self.coord
    }
    pub fn error(&self) -> bool {
        self.error
    }
    pub fn page(&self) -> &Page {
        &self.memory[self.block]
    }
    pub fn bank(&self) -> Bank {
        Bank {
            acc: self.acc,
            block: self.block,
            coord: self.coord,
            data: self.data,
            error: self.error,
            page: None,
        }
    }
    pub fn set_reg(&mut self, val: u16) {
        (self.data, self.acc) = (trunc(val >> u8::BITS), trunc(val));
    }
    pub fn get_reg(&self) -> u16 {
        u16::from(self.data) << u8::BITS | u16::from(self.acc)
    }
    pub fn raise(&mut self) {
        self.error = true;
    }
}

fn trunc(val: u16) -> u8 {
    const MASK: u16 = u8::MAX as u16;
    (val & MASK) as u8
}
