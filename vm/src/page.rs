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
    pub fn put(&mut self) {
        use io::Write;
        let buf = &[self.regs.data];
        match io::stdout().write(buf) {
            Ok(1) => (),
            _ => self.regs.error = true,
        }
    }
    pub fn get(&mut self) {
        use io::Read;
        let buf = &mut [self.regs.data];
        match io::stdin().read(buf) {
            Ok(1) => self.regs.data = buf[0],
            _ => self.regs.error = true,
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
        let pairs = self.page.iter_mut().skip(self.regs.coord.into()).zip(input);
        self.regs.coord += u8::try_from(pairs.len().max(1) - 1).unwrap();
        pairs.for_each(|(dst, src)| *dst = *src);
    }
}

fn floor(lhs: u8, rhs: u8) -> u8 {
    (lhs / rhs) * rhs
}
