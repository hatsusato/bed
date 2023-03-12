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
        self.regs.data = self.cur()[0];
    }
    pub fn store(&mut self) {
        self.cur_mut()[0] = self.regs.data;
    }
    pub fn delete(&mut self) {
        self.cur_mut()[0] = 0;
    }
    pub fn put(&mut self) {
        use io::Write;
        let buf = &self.cur()[0..1];
        let result = io::stdout().write(buf);
        self.regs.set_error(result.is_err());
    }
    pub fn get(&mut self) {
        use io::Read;
        let buf = &mut self.cur_mut()[0..1];
        let result = io::stdin().read(buf);
        self.regs.set_error(result.is_err());
    }
    pub fn save(&mut self) {
        let buf = &mut [0; 4];
        self.regs.save(buf);
        Self::copy(self.cur_mut(), buf);
    }
    pub fn restore(&mut self) {
        let buf = &mut [0; 4];
        Self::copy(buf, self.cur());
        self.regs.restore(*buf);
    }
    pub fn quote(&mut self, input: &str) {
        Self::copy(self.cur_mut(), input.as_bytes());
    }
    fn cur(&mut self) -> &[u8] {
        let index = usize::from(self.regs.coord);
        &self.page.iter().as_slice()[index..]
    }
    fn cur_mut(&mut self) -> &mut [u8] {
        let index = usize::from(self.regs.coord);
        &mut self.page.iter_mut().into_slice()[index..]
    }
    fn copy(dst: &mut [u8], src: &[u8]) {
        dst.iter_mut().zip(src).for_each(|(dst, src)| *dst = *src);
    }
}
