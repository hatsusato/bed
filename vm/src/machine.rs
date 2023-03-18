use crate::lexer::Lexer;
use crate::state::State;

#[derive(Default)]
pub struct Machine {
    state: State,
    lexer: Lexer,
}
impl Machine {
    #[must_use]
    pub fn get_state(&self) -> &State {
        &self.state
    }
    #[must_use]
    pub fn get_last(&self) -> u8 {
        self.lexer.get_last()
    }
    pub fn execute(&mut self, input: u8) {
        let inst = self.lexer.translate(input);
        self.state.issue(&inst);
    }
}
