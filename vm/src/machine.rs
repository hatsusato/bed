use crate::cmd::Command;
use crate::inst::Inst;
use crate::State;

#[derive(Default)]
pub struct Machine {
    state: State,
}
impl Machine {
    pub fn exec_inst(&mut self, inst: Inst) {
        let cmd = Command::new(inst, &self.state);
        self.state.restore_bank(cmd.next);
        self.state.restore_page(cmd.page);
    }
    pub fn exec_repeat(&mut self, block: &[Inst]) {
        let count = self.state.bank().acc;
        (0..count).for_each(|_| self.exec_block(block));
    }
    fn exec_block(&mut self, block: &[Inst]) {
        block.iter().for_each(|inst| self.exec_inst(*inst));
    }
    pub fn print(&self, key: char) {
        self.state.print(key);
    }
}
