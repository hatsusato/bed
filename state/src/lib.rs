mod mem;
mod ptr;
mod queue;
mod reg;
mod state;

use std::collections::VecDeque;
use util::Block;
pub struct State {
    acc: u8,
    block: u8,
    coord: u8,
    data: u8,
    error: bool,
    memory: Block<Block<u8>>,
    queue: VecDeque<u8>,
}
impl State {
    fn set_reg(&mut self, hi: u8, lo: u8) {
        (self.data, self.acc) = (hi, lo);
    }
    fn raise(&mut self) {
        self.error = true;
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
