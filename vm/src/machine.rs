use crate::state::State;
use crate::{Bank, Inst, Page};
use util::Block;

#[derive(Default)]
pub struct Machine {
    state: State,
    bank: Bank,
    memory: Block<Page>,
}
impl Machine {
    pub fn issue_inst(&mut self, inst: &Inst) {
        let page = &mut self.memory[self.bank.block];
        inst.issue(&mut self.bank, page);
    }
    pub fn issue_run(&mut self, seq: &str) {
        let insts: Vec<_> = seq.chars().map(Inst::new).collect();
        self.state.run(insts.as_slice());
    }
    pub fn repeat(&mut self, seq: &str) {
        let insts: Vec<_> = seq.chars().map(Inst::new).collect();
        self.state.repeat(insts.as_slice());
    }
    pub fn print(&self, key: char) {
        self.state.print(key);
    }
}
