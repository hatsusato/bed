use crate::lexer::Lexer;
use crate::state::State;
use util::Stream;

pub struct Machine {
    state: State,
    lexer: Lexer,
}
impl Machine {
    #[must_use]
    pub fn new(input: Stream, output: Stream) -> Self {
        let state = State::new(input, output);
        let lexer = Lexer::default();
        Self { state, lexer }
    }
    #[must_use]
    pub fn get_state(&self) -> &State {
        &self.state
    }
    #[must_use]
    pub fn get_last(&self) -> u8 {
        self.lexer.get_last()
    }
    pub fn execute(&mut self, input: u8) {
        self.state.issue(self.lexer.translate(input));
    }
    pub fn run(&mut self, code: &[u8]) {
        for input in code {
            self.execute(*input);
        }
    }
}
