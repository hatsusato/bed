use screen::Screen;
use std::ops::{Index, IndexMut};
use util::{Block, BLOCK_SIDE};

const CELL_WIDTH: u16 = 3;
const LINE_OFFSET: u16 = 1;

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
    pub fn print(&self, coord: u8) {
        for y in 0..BLOCK_SIDE {
            for x in 0..BLOCK_SIDE {
                Self::move_cell(x, y);
                self.print_cell(coord, x, y);
            }
        }
    }
    fn print_cell(&self, coord: u8, x: u8, y: u8) {
        let index = x + y * BLOCK_SIDE;
        let msg = format!("{:02x}", self[index]);
        Screen::print_display(msg, coord == index);
    }
    fn move_cell(x: u8, y: u8) {
        let x = u16::from(x) * CELL_WIDTH;
        let y = u16::from(y) + LINE_OFFSET;
        Screen::move_cursor(x, y);
    }
}
