use std::ops::{Index, IndexMut};
use std::slice::Iter;

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

pub type Page = Block<u8>;
impl Page {
    pub fn write(&mut self, input: Iter<u8>) {
        let update = |(i, &src)| self[i as u8] = src;
        input.take(u8::MAX as usize).enumerate().for_each(update);
    }
}
