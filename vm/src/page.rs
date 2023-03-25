use crate::reg::Registers;
use util::Block;

pub struct Page<'a> {
    pub regs: &'a mut Registers,
    pub page: &'a mut Block<u8>,
}
impl<'a> Page<'a> {
    pub fn new(regs: &'a mut Registers, memory: &'a mut Block<Block<u8>>) -> Self {
        let page = &mut memory[regs.block];
        Self { regs, page }
    }
    pub fn quote(&mut self, input: &[u8]) {
        let pairs = self.page.iter_mut().skip(self.regs.coord.into()).zip(input);
        self.regs.coord += u8::try_from(pairs.len().max(1) - 1).unwrap();
        pairs.for_each(|(dst, src)| *dst = *src);
    }
}
