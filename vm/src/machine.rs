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
    pub fn issue_inst(&mut self, inst: &Inst) {
        let mut bank = self.state.bank;
        let mut page = *self.state.page();
        inst.issue(&mut bank, &mut page);
    }
    pub fn issue(&mut self, seq: &str) {
        seq.chars().map(Inst::new).for_each(|inst| {
            let Command { next, page } = Command::new(inst, &self.state);
            self.state.restore_bank(next);
            self.state.restore_page(page);
        });
    }
    pub fn repeat(&mut self, seq: &str) {
        let count = self.state.bank().acc;
        let mut bank = self.state.bank();
        for i in 0..count {
            bank.acc = i;
            self.state.restore_bank(bank);
            self.issue(seq);
        }
    }
    pub fn exec_repeat(&mut self, block: &[Inst]) {
        let count = self.state.bank().acc;
        (0..count).for_each(|_| self.exec_block(block));
    }
    fn exec_block(&mut self, block: &[Inst]) {
        block.iter().for_each(|inst| self.exec_inst(inst.clone()));
    }
    pub fn print(&self, key: char) {
        self.state.print(key);
    }
}
