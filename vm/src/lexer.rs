use crate::inst::Inst;
use std::mem;

#[derive(Clone, Copy, PartialEq)]
enum Mode {
    Normal,
    Ignore,
    Call,
    Name,
    Body,
    Quote,
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
    name: Mode,
    body: Mode,
    quote: Mode,
}
impl Next {
    fn select(&mut self, mode: Mode) -> &mut Mode {
        match mode {
            Mode::Ignore => &mut self.ignore,
            Mode::Call => &mut self.call,
            Mode::Name => &mut self.name,
            Mode::Body => &mut self.body,
            Mode::Quote => &mut self.quote,
            _ => unreachable!(),
        }
    }
    fn is_normal(&self) -> bool {
        self.ignore == Mode::Normal && self.call == Mode::Normal && self.body == Mode::Normal
    }
}

const NEWLINE: char = '\n';
const QUOTE: char = '"';
const HASH: char = '#';
const COLON: char = ':';
const SEMICOLON: char = ';';

#[derive(Default)]
pub struct Lexer {
    mode: Mode,
    next: Next,
    call: String,
    name: String,
    quote: String,
    body: Vec<Inst>,
}
impl Lexer {
    pub fn consume(&mut self, input: char) -> Option<Inst> {
        if self.mode == Mode::Normal {
            assert!(self.next.is_normal());
        }
        match input {
            NEWLINE => self.consume_newline(),
            QUOTE => self.consume_quote(),
            HASH => self.consume_hash(),
            COLON => self.consume_colon(),
            SEMICOLON => self.consume_semicolon(),
            _ => self.consume_other(input),
        }
    }
    fn consume_newline(&mut self) -> Option<Inst> {
        match self.mode {
            Mode::Normal | Mode::Body | Mode::Quote => self.push(NEWLINE),
            Mode::Ignore => self.finish_ignore(),
            Mode::Call => self.finish(),
            Mode::Name => self.finish_name(),
        }
    }
    fn consume_quote(&mut self) -> Option<Inst> {
        match self.mode {
            Mode::Ignore => None,
            Mode::Normal | Mode::Body => self.transit(Mode::Quote),
            Mode::Call | Mode::Name => self.push(QUOTE),
            Mode::Quote => self.finish(),
        }
    }
    fn consume_hash(&mut self) -> Option<Inst> {
        match self.mode {
            Mode::Ignore => None,
            Mode::Normal | Mode::Call | Mode::Name | Mode::Body => self.transit(Mode::Ignore),
            Mode::Quote => self.push(HASH),
        }
    }
    fn consume_colon(&mut self) -> Option<Inst> {
        match self.mode {
            Mode::Ignore => None,
            Mode::Normal | Mode::Body => self.transit(Mode::Call),
            Mode::Call | Mode::Name | Mode::Quote => self.push(COLON),
        }
    }
    fn consume_semicolon(&mut self) -> Option<Inst> {
        match self.mode {
            Mode::Ignore => None,
            Mode::Normal => self.transit(Mode::Name),
            Mode::Call | Mode::Name | Mode::Quote => self.push(SEMICOLON),
            Mode::Body => self.finish(),
        }
    }
    fn consume_other(&mut self, input: char) -> Option<Inst> {
        match self.mode {
            Mode::Ignore => None,
            Mode::Normal => Some(Inst::new(input)),
            Mode::Call | Mode::Name | Mode::Quote | Mode::Body => self.push(input),
        }
    }
    fn transit(&mut self, next: Mode) -> Option<Inst> {
        *self.next.select(next) = mem::replace(&mut self.mode, next);
        None
    }
    fn next_take(&mut self) {
        self.mode = mem::take(self.next.select(self.mode));
    }
    fn finish(&mut self) -> Option<Inst> {
        let inst = match self.mode {
            Mode::Call => Inst::Call(mem::take(&mut self.call)),
            Mode::Body => Inst::Define(mem::take(&mut self.name), mem::take(&mut self.body)),
            Mode::Quote => Inst::Quote(mem::take(&mut self.quote)),
            _ => unreachable!(),
        };
        self.next_take();
        match self.mode {
            Mode::Normal => return Some(inst),
            Mode::Body => self.body.push(inst),
            _ => unreachable!(),
        }
        None
    }
    fn finish_ignore(&mut self) -> Option<Inst> {
        match self.mode {
            Mode::Ignore => self.next_take(),
            _ => unreachable!(),
        }
        match self.mode {
            Mode::Normal | Mode::Body => None,
            Mode::Call => self.finish(),
            Mode::Name => self.finish_name(),
            _ => unreachable!(),
        }
    }
    fn finish_name(&mut self) -> Option<Inst> {
        match self.mode {
            Mode::Name => self.mode = Mode::Body,
            _ => unreachable!(),
        }
        None
    }
    fn push(&mut self, input: char) -> Option<Inst> {
        match self.mode {
            Mode::Normal => return Some(Inst::new(input)),
            Mode::Call => self.call.push(input),
            Mode::Name => self.name.push(input),
            Mode::Quote => self.quote.push(input),
            Mode::Body => self.body.push(Inst::new(input)),
            _ => unreachable!(),
        }
        None
    }
}
