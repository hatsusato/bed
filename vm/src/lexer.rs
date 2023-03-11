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
    Direct,
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
    quote: Mode,
    direct: Mode,
}
impl Next {
    fn give(&mut self, next: Mode, prev: Mode) {
        match next {
            Mode::Ignore => self.ignore = prev,
            Mode::Call => self.call = prev,
            Mode::Quote => self.quote = prev,
            Mode::Direct => self.direct = prev,
            Mode::Name => assert!(prev == Mode::Normal),
            Mode::Normal | Mode::Body => unreachable!(),
        }
    }
    fn take(&mut self, mode: Mode) -> Mode {
        match mode {
            Mode::Ignore => mem::take(&mut self.ignore),
            Mode::Call => mem::take(&mut self.call),
            Mode::Body => Mode::Normal,
            Mode::Quote => mem::take(&mut self.quote),
            _ => unreachable!(),
        }
    }
    fn is_valid(&self) -> bool {
        matches!(
            self.ignore,
            Mode::Normal | Mode::Call | Mode::Name | Mode::Body
        ) && matches!(self.call, Mode::Name | Mode::Body)
            && matches!(self.quote, Mode::Name | Mode::Body)
            && matches!(self.direct, Mode::Name | Mode::Body)
    }
}

const NEWLINE: char = '\n';
const QUOTE: char = '"';
const APOSTROPHE: char = '\'';
const HASH: char = '#';
const COLON: char = ':';
const SEMICOLON: char = ';';

#[derive(Default)]
pub struct Lexer {
    mode: Mode,
    next: Next,
    call: String,
    name: String,
    body: Vec<Inst>,
    quote: String,
}
impl Lexer {
    pub fn consume(&mut self, input: char) -> Option<Inst> {
        assert!(self.next.is_valid());
        match input {
            NEWLINE => self.consume_newline(),
            QUOTE => self.consume_quote(),
            HASH => self.consume_hash(),
            APOSTROPHE => self.consume_apostrophe(),
            COLON => self.consume_colon(),
            SEMICOLON => self.consume_semicolon(),
            _ => self.consume_other(input),
        }
    }
    fn consume_newline(&mut self) -> Option<Inst> {
        match self.mode {
            Mode::Ignore => self.finish_ignore(),
            Mode::Call => self.finish_call(),
            Mode::Name => self.finish_name(),
            _ => self.consume_other(NEWLINE),
        }
    }
    fn consume_quote(&mut self) -> Option<Inst> {
        match self.mode {
            Mode::Normal | Mode::Body => self.transit(Mode::Quote),
            Mode::Quote => self.finish_quote(),
            _ => self.consume_other(QUOTE),
        }
    }
    fn consume_hash(&mut self) -> Option<Inst> {
        match self.mode {
            Mode::Normal | Mode::Call | Mode::Name | Mode::Body => self.transit(Mode::Ignore),
            _ => self.consume_other(HASH),
        }
    }
    fn consume_apostrophe(&mut self) -> Option<Inst> {
        match self.mode {
            Mode::Normal | Mode::Body => self.transit(Mode::Direct),
            _ => self.consume_other(APOSTROPHE),
        }
    }
    fn consume_colon(&mut self) -> Option<Inst> {
        match self.mode {
            Mode::Normal | Mode::Body => self.transit(Mode::Call),
            _ => self.consume_other(COLON),
        }
    }
    fn consume_semicolon(&mut self) -> Option<Inst> {
        match self.mode {
            Mode::Normal => self.transit(Mode::Name),
            Mode::Body => self.finish_body(),
            _ => self.consume_other(SEMICOLON),
        }
    }
    fn consume_other(&mut self, input: char) -> Option<Inst> {
        match self.mode {
            Mode::Normal => return Some(Inst::new(input)),
            Mode::Ignore => (),
            Mode::Call => self.call.push(input),
            Mode::Name => self.name.push(input),
            Mode::Body => self.body.push(Inst::new(input)),
            Mode::Quote => self.quote.push(input),
            Mode::Direct => return self.finish(Inst::immediate(input)),
        }
        None
    }
    fn transit(&mut self, next: Mode) -> Option<Inst> {
        let prev = mem::replace(&mut self.mode, next);
        self.next.give(next, prev);
        None
    }
    fn next_take(&mut self) {
        self.mode = self.next.take(self.mode);
    }
    fn finish(&mut self, inst: Inst) -> Option<Inst> {
        match self.mode {
            Mode::Normal => return Some(inst),
            Mode::Body => self.body.push(inst),
            _ => unreachable!(),
        }
        None
    }
    fn finish_ignore(&mut self) -> Option<Inst> {
        assert!(self.mode == Mode::Ignore);
        self.next_take();
        match self.mode {
            Mode::Normal | Mode::Body => None,
            Mode::Call => self.finish_call(),
            Mode::Name => self.finish_name(),
            _ => unreachable!(),
        }
    }
    fn finish_call(&mut self) -> Option<Inst> {
        assert!(self.mode == Mode::Call);
        let call = mem::take(&mut self.call);
        self.next_take();
        self.finish(Inst::Call(call))
    }
    fn finish_name(&mut self) -> Option<Inst> {
        assert!(self.mode == Mode::Name);
        self.transit(Mode::Body)
    }
    fn finish_body(&mut self) -> Option<Inst> {
        assert!(self.mode == Mode::Body);
        let name = mem::take(&mut self.name);
        let body = mem::take(&mut self.body);
        self.next_take();
        self.finish(Inst::Define(name, body))
    }
    fn finish_quote(&mut self) -> Option<Inst> {
        assert!(self.mode == Mode::Quote);
        let quote = mem::take(&mut self.quote);
        self.next_take();
        self.finish(Inst::Quote(quote))
    }
}
