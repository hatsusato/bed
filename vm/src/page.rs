use crate::Bank;
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
    pub fn load(&self, bank: &mut Bank) {
        bank.load(&self[bank.coord]);
    }
    pub fn store(&mut self, bank: &Bank) {
        bank.store(&mut self[bank.coord]);
    }
    pub fn delete(&mut self, bank: &Bank) {
        self[bank.coord] = 0;
    }
    pub fn put(&self, bank: &mut Bank) {
        let mut buf = [0];
        buf[0] = self[bank.coord];
        bank.put(&buf);
    }
    pub fn get(&mut self, bank: &mut Bank) {
        let mut buf = [0];
        bank.get(&mut buf);
        self[bank.coord] = buf[0];
    }
    pub fn save(&mut self, bank: &mut Bank) {
        let mut buf = [0; 4];
        bank.save(&mut buf);
        copy(self.page.iter_mut().skip(bank.coord.into()), buf.iter());
    }
    pub fn restore(&self, bank: &mut Bank) {
        let mut buf = [0; 4];
        copy(buf.iter_mut(), self.page.iter().skip(bank.coord.into()));
        bank.restore(&buf);
    }
    pub fn quote(&mut self, input: &str, bank: &Bank) {
        let input = input.as_bytes().iter();
        copy(self.page.iter_mut().skip(bank.coord.into()), input);
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

fn copy<'a, 'b>(dst: impl Iterator<Item = &'a mut u8>, src: impl Iterator<Item = &'b u8>) {
    dst.zip(src).for_each(|(dst, src)| *dst = *src);
}
