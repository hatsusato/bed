use std::io::{self, Read, Write};
use util::{Page, BLOCK_SIDE};

#[derive(Default, Clone, Copy)]
pub struct Bank {
    pub acc: u8,
    pub block: u8,
    pub coord: u8,
    pub data: u8,
    pub error: bool,
}
impl Bank {
    pub fn set_len(&mut self, len: Option<usize>) {
        if let Some(len) = len {
            self.acc = u8::try_from(len).unwrap_or(u8::MAX);
        }
        self.set_error(len.map_or(true, |len| usize::from(u8::MAX) < len));
    }
    fn set_error(&mut self, flag: bool) {
        if flag {
            self.error = true;
        }
    }
    fn set_reg(&mut self, reg: u16) {
        let [data, acc] = reg.to_be_bytes();
        (self.data, self.acc) = (data, acc);
    }
    pub fn imm(&mut self, key: u8) {
        self.data = key;
    }
    pub fn ins(&mut self, digit: u8) {
        self.data = nibble_combine(self.data, digit);
    }
    pub fn swap(&mut self) {
        (self.data, self.acc) = (self.acc, self.data);
    }
    pub fn hi(&mut self) {
        self.data = self.acc;
    }
    pub fn lo(&mut self) {
        self.acc = self.data;
    }
    pub fn zero(&mut self) {
        self.acc = 0;
    }
    pub fn origin(&mut self) {
        self.coord = 0;
    }
    pub fn start(&mut self) {
        self.block = 0;
    }
    pub fn goto(&mut self) {
        self.coord = self.acc;
    }
    pub fn jump(&mut self) {
        self.block = self.data;
    }
    pub fn position(&mut self) {
        self.acc = self.coord;
    }
    pub fn page(&mut self) {
        self.data = self.block;
    }
    pub fn left(&mut self) {
        self.coord = overflow_sub(self.coord, 1);
    }
    pub fn right(&mut self) {
        self.coord = overflow_add(self.coord, 1);
    }
    pub fn up(&mut self) {
        self.coord = overflow_sub(self.coord, BLOCK_SIDE);
    }
    pub fn down(&mut self) {
        self.coord = overflow_add(self.coord, BLOCK_SIDE);
    }
    pub fn inc(&mut self) {
        self.acc = overflow_add(self.acc, 1);
    }
    pub fn dec(&mut self) {
        self.acc = overflow_sub(self.acc, 1);
    }
    pub fn add(&mut self) {
        self.set_reg(u16::from(self.acc) + u16::from(self.data));
    }
    pub fn sub(&mut self) {
        self.set_reg(overflow_sub16(u16::from(self.acc), self.data.into()));
    }
    pub fn mul(&mut self) {
        self.set_reg(u16::from(self.acc) * u16::from(self.data));
    }
    pub fn div(&mut self) {
        if self.data == 0 {
            self.error = true;
        } else {
            (self.data, self.acc) = (self.acc % self.data, self.acc / self.data);
        }
    }
    pub fn clear(&mut self) {
        (self.acc, self.error) = (u8::from(self.error), false);
    }
    pub fn raise(&mut self) {
        self.error = true;
    }
    pub fn neg(&mut self) {
        self.acc = u8::from(self.data == 0);
    }
    pub fn bool(&mut self) {
        self.acc = u8::from(self.data != 0);
    }
    pub fn eq(&mut self) {
        self.acc = u8::from(self.data == self.acc);
    }
    pub fn lt(&mut self) {
        self.acc = u8::from(self.data < self.acc);
    }
    pub fn gt(&mut self) {
        self.acc = u8::from(self.data > self.acc);
    }
    pub fn not(&mut self) {
        self.acc = !self.acc;
    }
    pub fn and(&mut self) {
        self.acc &= self.data;
    }
    pub fn or(&mut self) {
        self.acc |= self.data;
    }
    pub fn xor(&mut self) {
        self.acc ^= self.data;
    }
    pub fn shl(&mut self) {
        self.acc = shift(self.acc, true);
    }
    pub fn shr(&mut self) {
        self.acc = shift(self.acc, false);
    }
    pub fn rotl(&mut self) {
        self.acc = rot(self.acc, true);
    }
    pub fn rotr(&mut self) {
        self.acc = rot(self.acc, false);
    }
    pub fn read(&mut self, page: &Page) {
        self.data = page[self.coord];
    }
    pub fn write(&mut self, mut page: Page) -> Option<Page> {
        page[self.coord] = self.data;
        Some(page)
    }
    pub fn del(&mut self, mut page: Page) -> Option<Page> {
        page[self.coord] = 0;
        Some(page)
    }
    pub fn put(&mut self, page: &Page) {
        let buf = &[page[self.coord]];
        self.set_error(io::stdout().write(buf).is_err());
    }
    pub fn get(&mut self, mut page: Page) -> Option<Page> {
        let buf = std::slice::from_mut(&mut page[self.coord]);
        self.set_error(io::stdin().read(buf).is_err());
        None
    }
    pub fn save(&mut self, mut page: Page) -> Option<Page> {
        [self.data, self.acc, self.block, self.coord]
            .iter()
            .enumerate()
            .for_each(|(offset, &src)| page[self.get_index(offset)] = src);
        Some(page)
    }
    pub fn restore(&mut self, page: &Page) {
        [self.data, self.acc, self.block, self.coord]
            .iter_mut()
            .enumerate()
            .for_each(|(offset, dst)| *dst = page[self.get_index(offset)]);
    }
    fn get_index(self, offset: usize) -> u8 {
        overflow_add(self.coord, u8::try_from(offset).unwrap())
    }
}

const NIBBLE_SHIFT: u32 = u8::BITS / 2;
fn nibble_cast(bits: u8) -> u8 {
    const MASK: u8 = (1 << NIBBLE_SHIFT) - 1;
    bits & MASK
}
fn nibble_combine(hi: u8, lo: u8) -> u8 {
    (nibble_cast(hi) << NIBBLE_SHIFT) | (nibble_cast(lo))
}
fn shift(val: u8, forward: bool) -> u8 {
    let (val, _) = if forward {
        val.overflowing_shl(1)
    } else {
        val.overflowing_shr(1)
    };
    val
}
fn rot(val: u8, forward: bool) -> u8 {
    let left = if forward { 1 } else { u8::BITS - 1 };
    let right = u8::BITS - left;
    (val << left) | (val >> right)
}
fn overflow_add(lhs: u8, rhs: u8) -> u8 {
    let (val, _) = lhs.overflowing_add(rhs);
    val
}
fn overflow_sub(lhs: u8, rhs: u8) -> u8 {
    let (val, _) = lhs.overflowing_sub(rhs);
    val
}
fn overflow_sub16(lhs: u16, rhs: u16) -> u16 {
    let (val, _) = lhs.overflowing_sub(rhs);
    val
}
