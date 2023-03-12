use crate::inst::Inst;
use crate::lexer::Lexer;
use crate::state::State;

#[derive(Default)]
pub struct Machine {
    state: State,
    lexer: Lexer,
    last: char,
}
impl Machine {
    pub fn execute(&mut self, input: u8) {
        let inst = self.lexer.consume(input);
        self.state.issue(inst);
    }
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
    pub fn print(&self) {
        self.state.print(self.last);
    }
}
