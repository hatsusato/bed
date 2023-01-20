use std::ops::{Index, IndexMut};

const BLOCK_SIZE: usize = 1 << u8::BITS;

#[derive(Clone, Copy)]
pub struct Block<T> {
    block: [T; BLOCK_SIZE],
}
impl<T: Copy> Block<T> {
    pub fn new(init: T) -> Self {
        let block = [init; BLOCK_SIZE];
        Self { block }
    }
}
impl<T> Index<u8> for Block<T> {
    type Output = T;
    fn index(&self, index: u8) -> &Self::Output {
        &self.block[usize::from(index)]
    }
}
impl<T> IndexMut<u8> for Block<T> {
    fn index_mut(&mut self, index: u8) -> &mut Self::Output {
        &mut self.block[usize::from(index)]
    }
}
