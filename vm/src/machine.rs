use crate::{Bank, Inst, Page};
use util::Block;

#[derive(Default)]
pub struct Machine {
    bank: Bank,
    memory: Block<Page>,
}
impl Machine {
    pub fn issue_inst(&mut self, inst: &Inst) {
        let page = &mut self.memory[self.bank.block];
        inst.issue(&mut self.bank, page);
    }
    pub fn issue_run(&mut self, seq: &str) {
        seq.chars()
            .map(Inst::new)
            .for_each(|inst| self.issue_inst(&inst));
    }
    pub fn repeat(&mut self, seq: &str) {
        let count = self.bank.acc;
        for i in 0..count {
            self.bank.acc = i;
            self.issue_run(seq);
        }
        self.bank.acc = count;
    }
    pub fn print(&self, key: char) {
        self.bank.print(key);
        self.memory[self.bank.block].print(self.bank.coord);
    }
}
