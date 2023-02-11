use crate::{Bank, Inst, Page};
use util::Block;

#[derive(Default)]
pub struct Machine {
    bank: Bank,
    memory: Block<Page>,
}
impl Machine {
    pub fn exec_inst(&mut self, inst: Inst) {
        self.issue_inst(&inst);
    }
    pub fn issue_inst(&mut self, inst: &Inst) {
        let page = &mut self.memory[self.bank.block];
        inst.issue(&mut self.bank, page);
    }
    pub fn issue_seq(&mut self, seq: &str) {
        seq.chars()
            .map(Inst::new)
            .for_each(|inst| self.issue_inst(&inst));
    }
    pub fn repeat(&mut self, seq: &str) {
        let count = self.bank.acc;
        for i in 0..count {
            self.bank.acc = i;
            self.issue_seq(seq);
        }
    }
    pub fn exec_repeat(&mut self, block: &[Inst]) {
        let count = self.bank.acc;
        (0..count).for_each(|_| self.exec_block(block));
    }
    fn exec_block(&mut self, block: &[Inst]) {
        block.iter().for_each(|inst| self.exec_inst(inst.clone()));
    }
    pub fn print(&self, key: char) {
        self.bank.print(key);
        self.memory[self.bank.block].print(self.bank.coord);
    }
}
