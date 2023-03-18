use crate::lexer::Lexer;
use crate::state::State;

#[derive(Default)]
pub struct Machine {
    state: State,
    lexer: Lexer,
}
impl Machine {
    pub fn get_state(&self) -> &State {
        &self.state
    }
    pub fn get_last(&self) -> u8 {
        self.lexer.get_last()
    }
    pub fn execute(&mut self, input: u8) {
        let inst = self.lexer.translate(input);
        self.state.issue(&inst);
    }
    pub fn print(&self) {
        self.state.print(self.lexer.get_last());
    }
}
