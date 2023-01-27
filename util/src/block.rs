use std::ops::{Index, IndexMut};
use std::slice::Iter;

const BLOCK_SIZE: usize = 1 << u8::BITS;

#[derive(Clone, Copy)]
pub struct Block<T> {
    block: [T; BLOCK_SIZE],
}
impl<T: Copy + Default> Default for Block<T> {
    fn default() -> Self {
        let block = [Default::default(); BLOCK_SIZE];
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
    pub fn write(&mut self, input: Iter<u8>) -> u8 {
        let mut len = 0;
        self.block.iter_mut().zip(input).for_each(|(dst, src)| {
            *dst = *src;
            len += 1;
        });
        len
    }
}
