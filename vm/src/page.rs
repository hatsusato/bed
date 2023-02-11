use std::ops::{Index, IndexMut};
use util::Block;

#[derive(Default, Clone, Copy)]
pub struct Page {
    page: Block<u8>,
}
impl Index<u8> for Page {
    type Output = u8;
    fn index(&self, index: u8) -> &Self::Output {
        self.page.index(index)
    }
}
impl IndexMut<u8> for Page {
    fn index_mut(&mut self, index: u8) -> &mut Self::Output {
        self.page.index_mut(index)
    }
}
impl Page {
    pub fn write(&mut self, coord: u8, input: &str) {
        self.page
            .iter_mut()
            .skip(coord.into())
            .zip(input.as_bytes())
            .for_each(|(dst, src)| *dst = *src);
    }
}
