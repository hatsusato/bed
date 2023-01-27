use crate::{Bank, State};
use util::Page;

impl State {
    pub fn new() -> Self {
        Self {
            acc: 0,
            block: 0,
            coord: 0,
            data: 0,
            error: false,
            memory: Default::default(),
        }
    }
    pub fn restore_bank(&mut self, bank: Bank) {
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
        (self.data as u16) << u8::BITS | (self.acc as u16)
    }
    pub fn raise(&mut self) {
        self.error = true;
    }
}

fn trunc(val: u16) -> u8 {
    const MASK: u16 = u8::MAX as u16;
    (val & MASK) as u8
}
