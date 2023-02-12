use crate::Bank;
use screen::Screen;
use std::io::{self, Read, Write};
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
    pub fn load(&self, bank: &mut Bank) {
        bank.data = self[bank.coord];
    }
    pub fn store(&mut self, bank: &Bank) {
        self[bank.coord] = bank.data;
    }
    pub fn delete(&mut self, bank: &Bank) {
        self[bank.coord] = 0;
    }
    pub fn put(&self, bank: &mut Bank) {
        let buf = &[self[bank.coord]];
        bank.set_error(io::stdout().write(buf).is_err());
    }
    pub fn get(&mut self, bank: &mut Bank) {
        let buf = &mut [u8::from(0)];
        bank.set_error(io::stdin().read(buf).is_err());
        self[bank.coord] = buf[0];
    }
    pub fn save(&mut self, bank: &mut Bank) {
        self.page
            .iter_mut()
            .skip(bank.coord.into())
            .zip(bank.save().iter())
            .for_each(|(dst, src)| *dst = *src);
    }
    pub fn restore(&self, bank: &mut Bank) {
        let mut buf = [0; 4];
        self.page
            .iter()
            .skip(bank.coord.into())
            .zip(buf.iter_mut())
            .for_each(|(src, dst)| *dst = *src);
        bank.restore(buf);
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
