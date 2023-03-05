use crate::inst::Inst;

#[derive(Clone, Copy)]
enum Mode {
    Normal,
}
impl Default for Mode {
    fn default() -> Self {
        Mode::Normal
    }
}

#[derive(Default)]
pub struct Lexer {
    mode: Mode,
}
impl Lexer {
    pub fn consume(&mut self, input: char) -> Option<Inst> {
        match self.mode {
            Mode::Normal => Some(Inst::new(input)),
        }
    }
}
