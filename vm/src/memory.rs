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
