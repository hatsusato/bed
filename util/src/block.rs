use crate::BYTE_COUNT;
use std::ops::{Index, IndexMut};
use std::slice::{Iter, IterMut};

#[derive(Clone, Copy, Debug)]
pub struct Block<T> {
    block: [T; BYTE_COUNT],
}
impl<T: Copy + Default> Default for Block<T> {
    fn default() -> Self {
        let block = [Default::default(); BYTE_COUNT];
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
impl<T> Block<T> {
    pub fn iter(&self) -> Iter<T> {
        self.block.iter()
    }
    pub fn iter_mut(&mut self) -> IterMut<T> {
        self.block.iter_mut()
    }
}
