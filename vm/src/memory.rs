use crate::reg::Registers;
use util::Block;

#[derive(Default, Debug)]
pub struct Memory {
    pub blocks: Block<Block<u8>>,
}
impl Memory {
    fn get_page(&mut self, regs: &Registers) -> &mut Block<u8> {
        &mut self.blocks[regs.block]
    }
    pub fn load(&mut self, regs: &mut Registers) {
        let page = self.get_page(regs);
        regs.data = page[regs.coord];
    }
    pub fn store(&mut self, regs: &Registers) {
        let page = self.get_page(regs);
        page[regs.coord] = regs.data;
    }
    pub fn save(&mut self, regs: &Registers) {
        let page = self.get_page(regs);
        let base = get_base(regs);
        for coord in base..(base + 4) {
            page[coord] = *at(regs, coord);
        }
    }
    pub fn restore(&mut self, regs: &mut Registers) {
        let page = self.get_page(regs);
        let base = get_base(regs);
        for coord in base..(base + 4) {
            *at_mut(regs, coord) = page[coord];
        }
    }
    pub fn quote(&mut self, regs: &mut Registers, input: &[u8]) {
        let page = self.get_page(regs);
        if let Some(src) = input.iter().next() {
            page[regs.coord] = *src;
        }
        for src in &input[1..] {
            if let Some(coord) = regs.coord.checked_add(1) {
                regs.coord = coord;
                page[regs.coord] = *src;
            }
        }
    }
}

fn get_base(regs: &Registers) -> u8 {
    (regs.coord / 4) * 4
}
fn at(regs: &Registers, index: u8) -> &u8 {
    match index % 4 {
        0 => &regs.data,
        1 => &regs.acc,
        2 => &regs.block,
        3 => &regs.coord,
        _ => unreachable!(),
    }
}
fn at_mut(regs: &mut Registers, index: u8) -> &mut u8 {
    match index % 4 {
        0 => &mut regs.data,
        1 => &mut regs.acc,
        2 => &mut regs.block,
        3 => &mut regs.coord,
        _ => unreachable!(),
    }
}

#[cfg(test)]
mod memory_tests {
    use super::{Memory, Registers};

    #[test]
    fn load_store_test() {
        let (mut mem, mut regs) = make();
        for i in 0..=u8::MAX {
            (regs.data, regs.coord) = (i, i);
            mem.store(&regs);
        }
        for i in 0..=u8::MAX {
            assert_eq!(mem.blocks[0][i], i);
            regs.coord = i;
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
        (regs.data, regs.acc, regs.block, regs.coord) = (4, 3, 2, 1);
        mem.save(&regs);
        regs.data = 0;
        for i in 0..4 {
            assert_eq!(mem.blocks[2][i], 4 - i);
            regs.coord = i;
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
