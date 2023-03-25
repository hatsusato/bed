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
        self.regs.load(self.page);
    }
    pub fn store(&mut self) {
        self.regs.store(self.page);
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
        self.regs.save(self.page);
    }
    pub fn restore(&mut self) {
        self.regs.restore(self.page);
    }
    pub fn quote(&mut self, input: &[u8]) {
        let pairs = self.page.iter_mut().skip(self.regs.coord.into()).zip(input);
        self.regs.coord += u8::try_from(pairs.len().max(1) - 1).unwrap();
        pairs.for_each(|(dst, src)| *dst = *src);
    }
}
