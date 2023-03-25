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
}
