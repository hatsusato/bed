use crate::inst::Inst;
use std::mem;

#[derive(Clone, Copy, PartialEq, Debug)]
enum Mode {
    Normal,
    Ignore,
    Call,
    Func,
    Body,
    Quote,
    Direct,
    Exec,
    Repeat,
    Register,
    Record,
}
impl Default for Mode {
    fn default() -> Self {
        Mode::Normal
    }
}

struct Next {
    ignore: Mode,
    call: Mode,
    quote: Mode,
    direct: Mode,
    exec: Mode,
    repeat: Mode,
    record: Mode,
}
impl Default for Next {
    fn default() -> Self {
        Self {
            ignore: Mode::Ignore,
            call: Mode::Call,
            quote: Mode::Quote,
            direct: Mode::default(),
            exec: Mode::default(),
            repeat: Mode::default(),
            record: Mode::default(),
        }
    }
}
impl Next {
    fn toggle_ignore(&mut self, next: &mut Mode) {
        assert!(match self.ignore {
            Mode::Ignore => matches!(next, Mode::Normal | Mode::Body | Mode::Record),
            Mode::Normal | Mode::Body | Mode::Record => matches!(next, Mode::Ignore),
            _ => false,
        });
        mem::swap(&mut self.ignore, next);
    }
    fn toggle_call(&mut self, next: &mut Mode) {
        assert!(match self.call {
            Mode::Call => matches!(next, Mode::Normal | Mode::Body | Mode::Record),
            Mode::Normal | Mode::Body | Mode::Record => matches!(next, Mode::Call),
            _ => false,
        });
        mem::swap(&mut self.call, next);
    }
    fn toggle_quote(&mut self, next: &mut Mode) {
        assert!(match self.quote {
            Mode::Quote => matches!(next, Mode::Normal | Mode::Body | Mode::Record),
            Mode::Normal | Mode::Body | Mode::Record => matches!(next, Mode::Quote),
            _ => false,
        });
        mem::swap(&mut self.quote, next);
    }
    fn toggle(&mut self, select: Mode, next: &mut Mode) {
        match select {
            Mode::Normal => unreachable!(),
            Mode::Ignore => self.toggle_ignore(next),
            Mode::Call => self.toggle_call(next),
            Mode::Quote => self.toggle_quote(next),
            _ => unimplemented!(),
        }
    }
    fn give(&mut self, next: Mode, prev: Mode) {
        assert!(match next {
            Mode::Normal => false,
            Mode::Ignore => matches!(
                prev,
                Mode::Normal | Mode::Call | Mode::Func | Mode::Body | Mode::Record
            ),
            Mode::Func => matches!(prev, Mode::Normal),
            Mode::Body => matches!(prev, Mode::Func),
            Mode::Register => matches!(prev, Mode::Normal | Mode::Body),
            Mode::Record => matches!(prev, Mode::Register),
            _ => matches!(prev, Mode::Normal | Mode::Body | Mode::Record),
        });
        match next {
            Mode::Normal | Mode::Ignore | Mode::Quote => unreachable!(),
            Mode::Call => self.call = prev,
            Mode::Func => assert_eq!(prev, Mode::Normal),
            Mode::Body => assert_eq!(prev, Mode::Func),
            Mode::Direct => self.direct = prev,
            Mode::Exec => self.exec = prev,
            Mode::Repeat => self.repeat = prev,
            Mode::Register => self.record = prev,
            Mode::Record => assert_eq!(prev, Mode::Register),
        }
    }
    fn take(&mut self, mode: Mode) -> Mode {
        match mode {
            Mode::Normal | Mode::Ignore | Mode::Quote => unreachable!(),
            Mode::Call => mem::take(&mut self.call),
            Mode::Func | Mode::Body => Mode::Normal,
            Mode::Direct => mem::take(&mut self.direct),
            Mode::Exec => mem::take(&mut self.exec),
            Mode::Repeat => mem::take(&mut self.repeat),
            Mode::Register | Mode::Record => mem::take(&mut self.record),
        }
    }
}

const NEWLINE: u8 = b'\n';
const QUOTE: u8 = b'"';
const HASH: u8 = b'#';
const PERCENT: u8 = b'%';
const APOSTROPHE: u8 = b'\'';
const COLON: u8 = b':';
const SEMICOLON: u8 = b';';
const AT: u8 = b'@';
const Q: u8 = b'q';

struct Last {
    pub last: u8,
}
impl Default for Last {
    fn default() -> Self {
        Self { last: NEWLINE }
    }
}
impl Last {
    fn is_newline(&self) -> bool {
        self.last == NEWLINE
    }
}

#[derive(Default)]
pub struct Lexer {
    mode: Mode,
    next: Next,
    call: Vec<u8>,
    func: Vec<u8>,
    body: Vec<Inst>,
    quote: Vec<u8>,
    register: Option<u8>,
    record: Vec<Inst>,
    last: Last,
}
impl Lexer {
    pub fn consume(&mut self, input: u8) -> Inst {
        let inst = match input {
            NEWLINE => self.consume_newline(),
            QUOTE => self.consume_quote(),
            HASH => self.consume_hash(),
            PERCENT => self.consume_percent(),
            APOSTROPHE => self.consume_apostrophe(),
            COLON => self.consume_colon(),
            SEMICOLON => self.consume_semicolon(),
            AT => self.consume_at(),
            Q => self.consume_q(),
            _ => self.consume_other(input),
        };
        self.last.last = input;
        inst
    }
    pub fn get_last(&self) -> u8 {
        self.last.last
    }
    fn consume_newline(&mut self) -> Inst {
        match self.mode {
            Mode::Ignore => self.toggle(Mode::Ignore),
            Mode::Call => self.toggle(Mode::Call),
            Mode::Func => self.finish_func(),
            _ => self.consume_other(NEWLINE),
        }
    }
    fn consume_quote(&mut self) -> Inst {
        match self.mode {
            Mode::Normal | Mode::Body | Mode::Record | Mode::Quote => self.toggle(Mode::Quote),
            _ => self.consume_other(QUOTE),
        }
    }
    fn consume_hash(&mut self) -> Inst {
        match self.mode {
            Mode::Normal | Mode::Body | Mode::Record => self.toggle(Mode::Ignore),
            _ => self.consume_other(HASH),
        }
    }
    fn consume_percent(&mut self) -> Inst {
        match self.mode {
            Mode::Normal | Mode::Body | Mode::Record => self.transit(Mode::Repeat),
            _ => self.consume_other(PERCENT),
        }
    }
    fn consume_apostrophe(&mut self) -> Inst {
        match self.mode {
            Mode::Normal | Mode::Body | Mode::Record => self.transit(Mode::Direct),
            _ => self.consume_other(APOSTROPHE),
        }
    }
    fn consume_colon(&mut self) -> Inst {
        match self.mode {
            Mode::Normal | Mode::Body | Mode::Record => self.toggle(Mode::Call),
            _ => self.consume_other(COLON),
        }
    }
    fn consume_semicolon(&mut self) -> Inst {
        match self.mode {
            Mode::Normal => {
                if self.last.is_newline() {
                    self.transit(Mode::Func)
                } else {
                    Inst::Nop
                }
            }
            Mode::Func => self.rewind(),
            Mode::Body => self.finish_body(),
            Mode::Record => Inst::Nop,
            _ => self.consume_other(SEMICOLON),
        }
    }
    fn consume_at(&mut self) -> Inst {
        match self.mode {
            Mode::Normal | Mode::Body | Mode::Record => self.transit(Mode::Exec),
            _ => self.consume_other(AT),
        }
    }
    fn consume_q(&mut self) -> Inst {
        match self.mode {
            Mode::Normal | Mode::Body => self.transit(Mode::Register),
            Mode::Record => self.finish_record(),
            _ => self.consume_other(Q),
        }
    }
    fn consume_other(&mut self, input: u8) -> Inst {
        match self.mode {
            Mode::Ignore => Inst::Skip,
            Mode::Normal | Mode::Body | Mode::Record => self.add(Inst::new(input)),
            Mode::Call | Mode::Func | Mode::Quote => self.push(input),
            Mode::Direct => self.finish_direct(input),
            Mode::Exec => self.finish_exec(input),
            Mode::Repeat => self.finish_repeat(input),
            Mode::Register => self.finish_register(input),
        }
    }
    fn transit(&mut self, next: Mode) -> Inst {
        let prev = mem::replace(&mut self.mode, next);
        self.next.give(next, prev);
        Inst::Skip
    }
    fn rewind(&mut self) -> Inst {
        self.mode = self.next.take(self.mode);
        Inst::Skip
    }
    fn push(&mut self, input: u8) -> Inst {
        match self.mode {
            Mode::Call => self.call.push(input),
            Mode::Func => self.func.push(input),
            Mode::Quote => self.quote.push(input),
            Mode::Register => self.register = Some(input),
            _ => unreachable!(),
        }
        Inst::Skip
    }
    fn add(&mut self, inst: Inst) -> Inst {
        if inst != Inst::Skip {
            match self.mode {
                Mode::Normal => return inst,
                Mode::Body => self.body.push(inst),
                Mode::Record => self.record.push(inst),
                _ => unreachable!(),
            }
        }
        Inst::Skip
    }
    fn take(&mut self, select: Mode) -> Inst {
        match select {
            Mode::Ignore | Mode::Normal | Mode::Body | Mode::Record => Inst::Skip,
            Mode::Call => Inst::Call(mem::take(&mut self.call)),
            Mode::Quote => Inst::Quote(mem::take(&mut self.quote)),
            _ => unimplemented!(),
        }
    }
    fn toggle(&mut self, select: Mode) -> Inst {
        let inst = self.take(self.mode);
        self.next.toggle(select, &mut self.mode);
        self.add(inst)
    }
    fn finish_func(&mut self) -> Inst {
        assert_eq!(self.mode, Mode::Func);
        self.transit(Mode::Body)
    }
    fn finish_body(&mut self) -> Inst {
        assert_eq!(self.mode, Mode::Body);
        let func = mem::take(&mut self.func);
        let body = mem::take(&mut self.body);
        self.rewind();
        self.add(Inst::Define(func, body))
    }
    fn finish_direct(&mut self, input: u8) -> Inst {
        assert_eq!(self.mode, Mode::Direct);
        self.rewind();
        self.add(Inst::Imm(input))
    }
    fn finish_exec(&mut self, input: u8) -> Inst {
        assert_eq!(self.mode, Mode::Exec);
        self.rewind();
        self.add(if input.is_ascii_graphic() {
            Inst::Exec(input)
        } else {
            Inst::Nop
        })
    }
    fn finish_repeat(&mut self, input: u8) -> Inst {
        assert_eq!(self.mode, Mode::Repeat);
        self.rewind();
        self.add(if input.is_ascii_graphic() {
            Inst::Repeat(input)
        } else {
            Inst::Nop
        })
    }
    fn finish_register(&mut self, input: u8) -> Inst {
        assert_eq!(self.mode, Mode::Register);
        if input.is_ascii_graphic() {
            self.push(input);
            self.transit(Mode::Record)
        } else {
            self.rewind()
        }
    }
    fn finish_record(&mut self) -> Inst {
        assert_eq!(self.mode, Mode::Record);
        assert!(self.register.is_some());
        let register = mem::take(&mut self.register).unwrap();
        let record = mem::take(&mut self.record);
        self.rewind();
        self.add(Inst::Macro(register, record))
    }
}

#[cfg(test)]
mod tests {
    use super::{Lexer, Mode};
    #[allow(clippy::enum_glob_use)]
    use Mode::*;
    #[test]
    fn ignore_test() {
        mode_test("", &[]);
        mode_test(" #\n", &[Normal, Ignore, Normal]);
        mode_test(
            "# \"#%':;@q\n",
            &[
                Ignore, Ignore, Ignore, Ignore, Ignore, Ignore, Ignore, Ignore, Ignore, Ignore,
                Normal,
            ],
        );
    }
    #[test]
    fn call_test() {
        mode_test(":\n", &[Call, Normal]);
        mode_test(": a\n", &[Call, Call, Call, Normal]);
        mode_test(
            ":\"#%':;@q\n",
            &[Call, Call, Call, Call, Call, Call, Call, Call, Call, Normal],
        );
    }
    #[test]
    fn quote_test() {
        mode_test("\"\"", &[Quote, Normal]);
        mode_test(
            "\n\"#%':;@q\n\"",
            &[
                Normal, Quote, Quote, Quote, Quote, Quote, Quote, Quote, Quote, Quote, Normal,
            ],
        );
    }
    fn mode_test(input: &str, modes: &[Mode]) {
        assert_eq!(input.len(), modes.len());
        let mut lexer = Lexer::default();
        assert_eq!(lexer.mode, Mode::Normal);
        for (&key, &mode) in input.as_bytes().iter().zip(modes.iter()) {
            lexer.consume(key);
            assert_eq!(lexer.mode, mode);
        }
    }
}
