use crate::inst::Inst;
use std::mem;

#[derive(Clone, Copy)]
enum Mode {
    Normal,
    Ignore,
    Call,
    Name,
    Body,
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
    body: Mode,
}
impl Next {
    fn select(&mut self, mode: Mode) -> &mut Mode {
        match mode {
            Mode::Ignore => &mut self.ignore,
            Mode::Call => &mut self.call,
            Mode::Body => &mut self.body,
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
    body: Vec<Inst>,
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
            Mode::Ignore => self.next_take(),
            Mode::Call => return self.finish_call(),
            Mode::Name => self.mode = Mode::Body,
            Mode::Body => self.body.push(Inst::Nop),
        }
        None
    }
    fn consume_hash(&mut self) -> Option<Inst> {
        match self.mode {
            Mode::Ignore => (),
            Mode::Normal | Mode::Call | Mode::Name | Mode::Body => self.next_replace(Mode::Ignore),
        }
        None
    }
    fn consume_colon(&mut self) -> Option<Inst> {
        match self.mode {
            Mode::Ignore => (),
            Mode::Normal | Mode::Body => self.next_replace(Mode::Call),
            Mode::Call | Mode::Name => self.push(':'),
        }
        None
    }
    fn consume_semicolon(&mut self) -> Option<Inst> {
        match self.mode {
            Mode::Ignore => (),
            Mode::Normal => self.mode = Mode::Name,
            Mode::Call | Mode::Name => self.push(';'),
            Mode::Body => return Some(self.finish_body()),
        }
        None
    }
    fn consume_other(&mut self, input: char) -> Option<Inst> {
        match self.mode {
            Mode::Ignore => (),
            Mode::Normal => return Some(Inst::new(input)),
            Mode::Call | Mode::Name => self.push(input),
            Mode::Body => self.body.push(Inst::new(input)),
        }
        None
    }
    fn next_take(&mut self) {
        self.mode = mem::take(self.next.select(self.mode));
    }
    fn next_replace(&mut self, next: Mode) {
        *self.next.select(next) = mem::replace(&mut self.mode, next);
    }
    fn finish_call(&mut self) -> Option<Inst> {
        match self.next.call {
            Mode::Normal => Some(Inst::Call(mem::take(&mut self.call))),
            _ => unreachable!(),
        }
    }
    fn finish_body(&mut self) -> Inst {
        self.next_take();
        let name = mem::take(&mut self.name);
        let body = mem::take(&mut self.body);
        Inst::Define(name, body)
    }
    fn push(&mut self, input: char) {
        match self.mode {
            Mode::Call => self.call.push(input),
            Mode::Name => self.name.push(input),
            _ => unreachable!(),
        }
    }
}
