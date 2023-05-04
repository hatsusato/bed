use crate::reg::Registers;
use util::Block;

#[derive(Default, Debug)]
pub struct Memory {
    blocks: Block<Block<u8>>,
}
impl Memory {
    pub fn get_memory(&self) -> &Block<Block<u8>> {
        &self.blocks
    }
    pub fn load(&mut self, regs: &mut Registers) {
        regs.data = self.blocks[regs.block][regs.cell];
    }
    pub fn store(&mut self, regs: &Registers) {
        self.blocks[regs.block][regs.cell] = regs.data;
    }
    pub fn restore(&mut self, regs: &mut Registers) {
        regs.cell = self.blocks[regs.block][regs.data];
    }
    pub fn save(&mut self, regs: &Registers) {
        self.blocks[regs.block][regs.data] = regs.cell;
    }
    pub fn direct(&mut self, regs: &Registers, data: u8) {
        self.blocks[regs.block][regs.cell] = data;
    }
    pub fn quote(&mut self, regs: &mut Registers, seq: &[u8]) {
        if let Some(src) = seq.iter().next() {
            self.blocks[regs.block][regs.cell] = *src;
        }
        for src in seq.iter().skip(1) {
            if let Some(cell) = regs.cell.checked_add(1) {
                regs.cell = cell;
                self.blocks[regs.block][regs.cell] = *src;
            } else {
                regs.error = true;
                return;
            }
        }
    }
}

#[cfg(test)]
mod memory_tests {
    use super::{Memory, Registers};

    #[test]
    fn load_store_test() {
        let (mut mem, mut regs) = make();
        for i in 0..=u8::MAX {
            (regs.data, regs.cell) = (i, i);
            mem.store(&regs);
        }
        for i in 0..=u8::MAX {
            assert_eq!(mem.blocks[0][i], i);
            regs.cell = i;
            mem.load(&mut regs);
            assert_eq!(regs.data, i);
            regs.data = 0;
            mem.store(&regs);
        }
        zero_test(&mem);
    }
    #[test]
    fn save_test() {
        let (mut mem, mut regs) = make();
        regs.cell = 42;
        mem.save(&regs);
        assert_eq!(mem.blocks[0][0], 42);
        regs.cell = 0;
        mem.blocks[0][0] = 0;
        zero_test(&mem);
    }
    #[test]
    fn restore_test() {
        let (mut mem, mut regs) = make();
        mem.blocks[0][0] = 42;
        mem.restore(&mut regs);
        assert_eq!(regs.cell, 42);
        regs.cell = 0;
        mem.blocks[0][0] = 0;
        zero_test(&mem);
    }
    #[test]
    fn direct_test() {
        let (mut mem, mut regs) = make();
        for i in 0..=u8::MAX {
            regs.cell = i;
            mem.direct(&regs, i);
        }
        regs.cell = 0;
        for i in 0..=u8::MAX {
            assert_eq!(mem.blocks[0][i], i);
            mem.blocks[0][i] = 0;
        }
        zero_test(&mem);
    }
    #[test]
    fn quote_test() {
        let (mut mem, mut regs) = make();
        mem.quote(&mut regs, &[1, 2, 3, 4]);
        assert_eq!(regs.cell, 3);
        for i in 0..4 {
            assert_eq!(mem.blocks[0][i], i + 1);
            regs.cell = i;
            mem.store(&regs);
        }
        zero_test(&mem);
    }
    fn make() -> (Memory, Registers) {
        let mem = Memory::default();
        zero_test(&mem);
        (mem, Registers::default())
    }
    fn zero_test(mem: &Memory) {
        for b in 0..=u8::MAX {
            for c in 0..=u8::MAX {
                assert_eq!(mem.blocks[b][c], 0);
            }
        }
    }
}
