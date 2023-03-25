use crate::reg::Registers;
use util::Block;

#[derive(Default, Debug)]
pub struct Memory {
    pub regs: Registers,
    pub blocks: Block<Block<u8>>,
}
