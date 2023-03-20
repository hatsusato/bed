use crate::reg::Registers;
use std::io;
use util::Block;

pub struct Page<'a> {
    regs: &'a mut Registers,
    page: &'a mut Block<u8>,
}
impl<'a> Page<'a> {
    pub fn new(regs: &'a mut Registers, memory: &'a mut Block<Block<u8>>) -> Self {
        let page = &mut memory[regs.block];
        Self { regs, page }
    }
    pub fn load(&mut self) {
        self.regs.data = self.cur();
    }
    pub fn store(&mut self) {
        self.cur_mut(self.regs.data);
    }
    pub fn delete(&mut self) {
        self.cur_mut(0);
    }
    pub fn put(&mut self) {
        use io::Write;
        let buf = &[self.cur()];
        let result = io::stdout().write(buf);
        self.set_error(result.is_err());
    }
    pub fn get(&mut self) {
        use io::Read;
        let buf = &mut [0];
        let result = io::stdin().read(buf);
        self.cur_mut(buf[0]);
        self.set_error(result.is_err());
    }
    fn set_error(&mut self, flag: bool) {
        if flag {
            self.regs.error = true;
        }
    }
    pub fn save(&mut self) {
        let index = floor(self.regs.coord, 4);
        for offset in 0..4 {
            self.page[index + offset] = *self.regs.at(offset);
        }
    }
    pub fn restore(&mut self) {
        let index = floor(self.regs.coord, 4);
        for offset in 0..4 {
            *self.regs.at(offset) = self.page[index + offset];
        }
    }
    pub fn quote(&mut self, input: &[u8]) {
        for (dst, src) in self.page.iter_mut().zip(input) {
            *dst = *src;
        }
    }
    fn cur(&mut self) -> u8 {
        self.page[self.regs.coord]
    }
    fn cur_mut(&mut self, src: u8) {
        self.page[self.regs.coord] = src;
    }
}

fn floor(lhs: u8, rhs: u8) -> u8 {
    (lhs / rhs) * rhs
}
