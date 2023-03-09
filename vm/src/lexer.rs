use crate::inst::Inst;
use std::mem;

#[derive(Clone, Copy)]
enum Mode {
    Normal,
    Ignore,
    Call,
}
impl Default for Mode {
    fn default() -> Self {
        Mode::Normal
    }
}

#[derive(Default)]
pub struct Lexer {
    mode: Mode,
    call: String,
}
impl Lexer {
    pub fn consume(&mut self, input: char) -> Option<Inst> {
        match input {
            '\n' => self.consume_newline(),
            '#' => self.consume_hash(),
            ':' => self.consume_colon(),
            _ => self.consume_other(input),
        }
    }
    fn consume_newline(&mut self) -> Option<Inst> {
        match self.mode {
            Mode::Normal => return Some(Inst::Nop),
            Mode::Ignore => self.mode = Mode::Normal,
            Mode::Call => return Some(self.finish_call()),
        }
        None
    }
    fn consume_hash(&mut self) -> Option<Inst> {
        match self.mode {
            Mode::Normal => self.mode = Mode::Ignore,
            Mode::Ignore => (),
            Mode::Call => self.call.push('#'),
        }
        None
    }
    fn consume_colon(&mut self) -> Option<Inst> {
        match self.mode {
            Mode::Normal => self.mode = Mode::Call,
            Mode::Ignore => (),
            Mode::Call => self.call.push(':'),
        }
        None
    }
    fn consume_other(&mut self, input: char) -> Option<Inst> {
        match self.mode {
            Mode::Normal => return Some(Inst::new(input)),
            Mode::Ignore => (),
            Mode::Call => self.call.push(input),
        }
        None
    }
    fn finish_call(&mut self) -> Inst {
        self.mode = Mode::Normal;
        Inst::Call(mem::take(&mut self.call))
    }
}
