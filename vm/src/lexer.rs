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
    Exec,
    Register,
    Record,
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
    exec: Mode,
    record: Mode,
}
impl Next {
    fn give(&mut self, next: Mode, prev: Mode) {
        assert!(match next {
            Mode::Ignore => matches!(
                prev,
                Mode::Normal | Mode::Call | Mode::Func | Mode::Body | Mode::Record
            ),
            Mode::Register => matches!(prev, Mode::Normal | Mode::Body),
            _ => matches!(prev, Mode::Normal | Mode::Body | Mode::Record),
        });
        match next {
            Mode::Normal => unreachable!(),
            Mode::Ignore => self.ignore = prev,
            Mode::Call => self.call = prev,
            Mode::Func => assert!(matches!(prev, Mode::Normal)),
            Mode::Body => assert!(matches!(prev, Mode::Func)),
            Mode::Quote => self.quote = prev,
            Mode::Direct => self.direct = prev,
            Mode::Exec => self.exec = prev,
            Mode::Register => self.record = prev,
            Mode::Record => assert!(matches!(prev, Mode::Register)),
        }
    }
    fn take(&mut self, mode: Mode) -> Mode {
        match mode {
            Mode::Ignore => mem::take(&mut self.ignore),
            Mode::Call => mem::take(&mut self.call),
            Mode::Body => Mode::Normal,
            Mode::Quote => mem::take(&mut self.quote),
            Mode::Direct => mem::take(&mut self.direct),
            Mode::Exec => mem::take(&mut self.exec),
            Mode::Register | Mode::Record => mem::take(&mut self.record),
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
const AT: char = '@';
const Q: char = 'q';

#[derive(Default)]
pub struct Lexer {
    mode: Mode,
    next: Next,
    call: String,
    func: String,
    body: Vec<Inst>,
    quote: String,
    register: Option<char>,
    record: Vec<Inst>,
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
            AT => self.consume_at(),
            Q => self.consume_q(),
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
            Mode::Normal | Mode::Body | Mode::Record => self.transit(Mode::Quote),
            Mode::Quote => self.finish_quote(),
            _ => self.consume_other(QUOTE),
        }
    }
    fn consume_hash(&mut self) -> Option<Inst> {
        match self.mode {
            Mode::Normal | Mode::Call | Mode::Func | Mode::Body | Mode::Record => {
                self.transit(Mode::Ignore)
            }
            _ => self.consume_other(HASH),
        }
    }
    fn consume_apostrophe(&mut self) -> Option<Inst> {
        match self.mode {
            Mode::Normal | Mode::Body | Mode::Record => self.transit(Mode::Direct),
            _ => self.consume_other(APOSTROPHE),
        }
    }
    fn consume_colon(&mut self) -> Option<Inst> {
        match self.mode {
            Mode::Normal | Mode::Body | Mode::Record => self.transit(Mode::Call),
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
    fn consume_at(&mut self) -> Option<Inst> {
        match self.mode {
            Mode::Normal | Mode::Body | Mode::Record => self.transit(Mode::Exec),
            _ => self.consume_other(AT),
        }
    }
    fn consume_q(&mut self) -> Option<Inst> {
        match self.mode {
            Mode::Normal | Mode::Body => self.transit(Mode::Register),
            Mode::Record => self.finish_record(),
            _ => self.consume_other(Q),
        }
    }
    fn consume_other(&mut self, input: char) -> Option<Inst> {
        match self.mode {
            Mode::Ignore => None,
            Mode::Normal | Mode::Body | Mode::Record => self.add(Inst::new(input)),
            Mode::Call | Mode::Func | Mode::Quote => self.push(input),
            Mode::Direct => self.finish_direct(input),
            Mode::Exec => self.finish_exec(input),
            Mode::Register => self.finish_register(input),
        }
    }
    fn transit(&mut self, next: Mode) -> Option<Inst> {
        let prev = mem::replace(&mut self.mode, next);
        self.next.give(next, prev);
        None
    }
    fn rewind(&mut self) -> Option<Inst> {
        self.mode = self.next.take(self.mode);
        None
    }
    fn push(&mut self, input: char) -> Option<Inst> {
        match self.mode {
            Mode::Call => self.call.push(input),
            Mode::Func => self.func.push(input),
            Mode::Quote => self.quote.push(input),
            Mode::Register => self.register = Some(input),
            _ => unreachable!(),
        }
        None
    }
    fn add(&mut self, inst: Inst) -> Option<Inst> {
        match self.mode {
            Mode::Normal => return Some(inst),
            Mode::Body => self.body.push(inst),
            Mode::Record => self.record.push(inst),
            _ => unreachable!(),
        }
        None
    }
    fn finish_ignore(&mut self) -> Option<Inst> {
        assert!(matches!(self.mode, Mode::Ignore));
        self.rewind();
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
        self.rewind();
        self.add(Inst::Call(call))
    }
    fn finish_func(&mut self) -> Option<Inst> {
        assert!(matches!(self.mode, Mode::Func));
        self.transit(Mode::Body)
    }
    fn finish_body(&mut self) -> Option<Inst> {
        assert!(matches!(self.mode, Mode::Body));
        let func = mem::take(&mut self.func);
        let body = mem::take(&mut self.body);
        self.rewind();
        self.add(Inst::Define(func, body))
    }
    fn finish_quote(&mut self) -> Option<Inst> {
        assert!(matches!(self.mode, Mode::Quote));
        let quote = mem::take(&mut self.quote);
        self.rewind();
        self.add(Inst::Quote(quote))
    }
    fn finish_direct(&mut self, input: char) -> Option<Inst> {
        assert!(matches!(self.mode, Mode::Direct));
        self.rewind();
        self.add(Inst::immediate(input))
    }
    fn finish_exec(&mut self, input: char) -> Option<Inst> {
        assert!(matches!(self.mode, Mode::Exec));
        if input.is_ascii_graphic() {
            self.rewind();
            self.add(Inst::Exec(input))
        } else {
            self.rewind()
        }
    }
    fn finish_register(&mut self, input: char) -> Option<Inst> {
        assert!(matches!(self.mode, Mode::Register));
        if input.is_ascii_graphic() {
            self.push(input);
            self.transit(Mode::Record)
        } else {
            self.rewind()
        }
    }
    fn finish_record(&mut self) -> Option<Inst> {
        assert!(matches!(self.mode, Mode::Record));
        assert!(self.register.is_some());
        let register = mem::take(&mut self.register).unwrap();
        let record = mem::take(&mut self.record);
        self.rewind();
        self.add(Inst::Macro(register, record))
    }
}
