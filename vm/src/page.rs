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
        self.regs.data = self.page[self.regs.coord];
    }
    pub fn store(&mut self) {
        self.page[self.regs.coord] = self.regs.data;
    }
    pub fn delete(&mut self) {
        self.page[self.regs.coord] = 0;
    }
    pub fn put(&mut self) {
        use io::Write;
        let buf = &[self.regs.data];
        if io::stdout().write(buf).is_err() {
            self.regs.error = true;
        }
    }
    pub fn get(&mut self) {
        use io::Read;
        let buf = &mut [self.regs.data];
        if io::stdin().read(buf).is_err() {
            self.regs.error = true;
        }
        self.regs.data = buf[0];
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
}

fn floor(lhs: u8, rhs: u8) -> u8 {
    (lhs / rhs) * rhs
}
