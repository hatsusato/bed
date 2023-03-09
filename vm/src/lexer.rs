use crate::inst::Inst;
use std::mem;

#[derive(Clone, Copy)]
enum Mode {
    Normal,
    Ignore,
    Call,
    Name,
}
impl Default for Mode {
    fn default() -> Self {
        Mode::Normal
    }
}

#[derive(Default)]
struct Next {
    ignore: Mode,
    call: Mode,
}
impl Next {
    fn replace(&mut self, mode: &mut Mode, next: Mode) {
        match next {
            Mode::Ignore => self.ignore = mem::replace(mode, next),
            Mode::Call => self.call = mem::replace(mode, next),
            _ => unreachable!(),
        }
    }
}

#[derive(Default)]
pub struct Lexer {
    mode: Mode,
    next: Next,
    call: String,
    name: String,
}
impl Lexer {
    pub fn consume(&mut self, input: char) -> Option<Inst> {
        match input {
            '\n' => self.consume_newline(),
            '#' => self.consume_hash(),
            ':' => self.consume_colon(),
            ';' => self.consume_semicolon(),
            _ => self.consume_other(input),
        }
    }
    fn consume_newline(&mut self) -> Option<Inst> {
        match self.mode {
            Mode::Normal => return Some(Inst::Nop),
            Mode::Ignore => self.mode = mem::take(&mut self.next.ignore),
            Mode::Call => return self.finish_call(),
            Mode::Name => (),
        }
        None
    }
    fn consume_hash(&mut self) -> Option<Inst> {
        match self.mode {
            Mode::Ignore => (),
            Mode::Normal | Mode::Call | Mode::Name => self.next_replace(Mode::Ignore),
        }
        None
    }
    fn consume_colon(&mut self) -> Option<Inst> {
        match self.mode {
            Mode::Normal => self.next_replace(Mode::Call),
            Mode::Ignore => (),
            Mode::Call => self.call.push(':'),
            Mode::Name => self.name.push(':'),
        }
        None
    }
    fn consume_semicolon(&mut self) -> Option<Inst> {
        match self.mode {
            Mode::Normal => self.mode = Mode::Name,
            Mode::Ignore => (),
            Mode::Call => self.call.push(';'),
            Mode::Name => self.name.push(';'),
        }
        None
    }
    fn consume_other(&mut self, input: char) -> Option<Inst> {
        match self.mode {
            Mode::Normal => return Some(Inst::new(input)),
            Mode::Ignore => (),
            Mode::Call => self.call.push(input),
            Mode::Name => self.name.push(input),
        }
        None
    }
    fn next_replace(&mut self, next: Mode) {
        self.next.replace(&mut self.mode, next);
    }
    fn finish_call(&mut self) -> Option<Inst> {
        match self.next.call {
            Mode::Normal => Some(Inst::Call(mem::take(&mut self.call))),
            _ => unreachable!(),
        }
    }
}
