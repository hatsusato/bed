mod mem;
mod ptr;
mod queue;
mod reg;
mod state;

use std::collections::VecDeque;
use util::Block;
pub struct State {
    pub acc: u8,
    pub block: u8,
    pub coord: u8,
    pub data: u8,
    pub error: bool,
    pub memory: Block<Block<u8>>,
    pub queue: VecDeque<u8>,
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
