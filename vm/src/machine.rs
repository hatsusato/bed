use crate::lexer::Lexer;
use crate::state::State;

#[derive(Default)]
pub struct Machine {
    state: State,
    lexer: Lexer,
}
impl Machine {
    pub fn execute(&mut self, input: u8) {
        let inst = self.lexer.consume(input);
        self.state.issue(&inst);
    }
    pub fn print(&self) {
        self.state.print(self.lexer.get_last());
    }
}
