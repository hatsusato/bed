use crate::state::State;
use crate::Inst;

#[derive(Default)]
pub struct Machine {
    state: State,
}
impl Machine {
    pub fn issue_inst(&mut self, inst: &Inst) {
        self.state.issue(inst.clone());
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
