use crate::lexer::Lexer;
use crate::state::State;
use util::Stream;

#[derive(Default)]
pub struct Machine {
    state: State,
    lexer: Lexer,
}
impl Machine {
    pub fn init(&mut self, input: Stream, output: Stream) {
        self.state.init(input, output);
    }
    #[must_use]
    pub fn get_state(&self) -> &State {
        &self.state
    }
    #[must_use]
    pub fn get_last(&self) -> u8 {
        self.lexer.get_last()
    }
    pub fn execute(&mut self, code: u8) {
        self.state.issue(self.lexer.translate(code));
    }
    pub fn run(code: &[u8], input: Stream, output: Stream) {
        let mut this = Self::default();
        this.init(input, output);
        for input in code {
            this.execute(*input);
        }
        this.execute(b'\n');
    }
}
