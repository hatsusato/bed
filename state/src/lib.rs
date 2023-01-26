mod state;

use std::collections::VecDeque;
use util::{Block, Page};
pub struct State {
    pub acc: u8,
    pub block: u8,
    pub coord: u8,
    pub data: u8,
    pub error: bool,
    pub memory: Block<Page>,
    pub queue: VecDeque<u8>,
}
#[derive(Clone)]
pub struct Bank {
    pub acc: u8,
    pub block: u8,
    pub coord: u8,
    pub data: u8,
    pub error: bool,
}
impl Bank {
    pub fn update_acc(&self, acc: u8) -> Self {
        let mut bank = self.clone();
        bank.acc = acc;
        bank
    }
    pub fn update_block(&self, block: u8) -> Self {
        let mut bank = self.clone();
        bank.block = block;
        bank
    }
    pub fn update_coord(&self, coord: u8) -> Self {
        let mut bank = self.clone();
        bank.coord = coord;
        bank
    }
    pub fn update_data(&self, data: u8) -> Self {
        let mut bank = self.clone();
        bank.data = data;
        bank
    }
    pub fn update_error(&self, error: bool) -> Self {
        let mut bank = self.clone();
        bank.error = error;
        bank
    }
}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
