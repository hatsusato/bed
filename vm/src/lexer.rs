use crate::inst::Inst;
use std::mem;

#[derive(Clone, Copy)]
enum Mode {
    Normal,
    Ignore,
    Call,
    Func,
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
        assert!(match next {
            Mode::Ignore => matches!(prev, Mode::Normal | Mode::Call | Mode::Func | Mode::Body),
            _ => matches!(prev, Mode::Normal | Mode::Body),
        });
        match next {
            Mode::Ignore => self.ignore = prev,
            Mode::Call => self.call = prev,
            Mode::Quote => self.quote = prev,
            Mode::Direct => self.direct = prev,
            Mode::Func => assert!(matches!(prev, Mode::Normal)),
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
}

const NEWLINE: char = '\n';
const QUOTE: char = '"';
const HASH: char = '#';
const APOSTROPHE: char = '\'';
const COLON: char = ':';
const SEMICOLON: char = ';';

#[derive(Default)]
pub struct Lexer {
    mode: Mode,
    next: Next,
    call: String,
    func: String,
    body: Vec<Inst>,
    quote: String,
}
impl Lexer {
    pub fn consume(&mut self, input: char) -> Option<Inst> {
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
            Mode::Func => self.finish_func(),
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
            Mode::Normal | Mode::Call | Mode::Func | Mode::Body => self.transit(Mode::Ignore),
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
            Mode::Normal => self.transit(Mode::Func),
            Mode::Body => self.finish_body(),
            _ => self.consume_other(SEMICOLON),
        }
    }
    fn consume_other(&mut self, input: char) -> Option<Inst> {
        match self.mode {
            Mode::Normal => return Some(Inst::new(input)),
            Mode::Ignore => (),
            Mode::Call => self.call.push(input),
            Mode::Func => self.func.push(input),
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
        assert!(matches!(self.mode, Mode::Ignore));
        self.next_take();
        match self.mode {
            Mode::Normal | Mode::Body => None,
            Mode::Call => self.finish_call(),
            Mode::Func => self.finish_func(),
            _ => unreachable!(),
        }
    }
    fn finish_call(&mut self) -> Option<Inst> {
        assert!(matches!(self.mode, Mode::Call));
        let call = mem::take(&mut self.call);
        self.next_take();
        self.finish(Inst::Call(call))
    }
    fn finish_func(&mut self) -> Option<Inst> {
        assert!(matches!(self.mode, Mode::Func));
        self.transit(Mode::Body)
    }
    fn finish_body(&mut self) -> Option<Inst> {
        assert!(matches!(self.mode, Mode::Body));
        let func = mem::take(&mut self.func);
        let body = mem::take(&mut self.body);
        self.next_take();
        self.finish(Inst::Define(func, body))
    }
    fn finish_quote(&mut self) -> Option<Inst> {
        assert!(matches!(self.mode, Mode::Quote));
        let quote = mem::take(&mut self.quote);
        self.next_take();
        self.finish(Inst::Quote(quote))
    }
}
