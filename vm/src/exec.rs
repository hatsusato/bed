use crate::lexer::Lexer;
use crate::Machine;

#[derive(Default)]
pub struct Exec {
    vm: Machine,
    last: char,
    lexer: Lexer,
}
impl Exec {
    pub fn execute(&mut self, input: char) {
        let inst = self.lexer.consume(input);
        self.vm.issue_inst(&inst);
    }
    pub fn print(&self) {
        self.vm.print(self.last);
    }
}
