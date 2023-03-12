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
    Repeat,
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
    repeat: Mode,
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
            Mode::Repeat => self.repeat = prev,
            Mode::Register => self.record = prev,
            Mode::Record => assert!(matches!(prev, Mode::Register)),
        }
    }
    fn take(&mut self, mode: Mode) -> Mode {
        match mode {
            Mode::Normal | Mode::Func => unreachable!(),
            Mode::Ignore => mem::take(&mut self.ignore),
            Mode::Call => mem::take(&mut self.call),
            Mode::Body => Mode::Normal,
            Mode::Quote => mem::take(&mut self.quote),
            Mode::Direct => mem::take(&mut self.direct),
            Mode::Exec => mem::take(&mut self.exec),
            Mode::Repeat => mem::take(&mut self.repeat),
            Mode::Register | Mode::Record => mem::take(&mut self.record),
        }
    }
}

const NEWLINE: char = '\n';
const QUOTE: char = '"';
const HASH: char = '#';
const PERCENT: char = '%';
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
    pub fn consume(&mut self, input: u8) -> Inst {
        let input = input as char;
        match input {
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
        }
    }
    fn consume_newline(&mut self) -> Inst {
        match self.mode {
            Mode::Ignore => self.finish_ignore(),
            Mode::Call => self.finish_call(),
            Mode::Func => self.finish_func(),
            _ => self.consume_other(NEWLINE),
        }
    }
    fn consume_quote(&mut self) -> Inst {
        match self.mode {
            Mode::Normal | Mode::Body | Mode::Record => self.transit(Mode::Quote),
            Mode::Quote => self.finish_quote(),
            _ => self.consume_other(QUOTE),
        }
    }
    fn consume_hash(&mut self) -> Inst {
        match self.mode {
            Mode::Normal | Mode::Call | Mode::Func | Mode::Body | Mode::Record => {
                self.transit(Mode::Ignore)
            }
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
            Mode::Normal | Mode::Body | Mode::Record => self.transit(Mode::Call),
            _ => self.consume_other(COLON),
        }
    }
    fn consume_semicolon(&mut self) -> Inst {
        match self.mode {
            Mode::Normal => self.transit(Mode::Func),
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
    fn consume_other(&mut self, input: char) -> Inst {
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
    fn push(&mut self, input: char) -> Inst {
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
        match self.mode {
            Mode::Normal => return inst,
            Mode::Body => self.body.push(inst),
            Mode::Record => self.record.push(inst),
            _ => unreachable!(),
        }
        Inst::Skip
    }
    fn finish_ignore(&mut self) -> Inst {
        assert!(matches!(self.mode, Mode::Ignore));
        self.rewind();
        match self.mode {
            Mode::Normal | Mode::Body => Inst::Skip,
            Mode::Call => self.finish_call(),
            Mode::Func => self.finish_func(),
            _ => unreachable!(),
        }
    }
    fn finish_call(&mut self) -> Inst {
        assert!(matches!(self.mode, Mode::Call));
        let call = mem::take(&mut self.call);
        self.rewind();
        self.add(Inst::Call(call))
    }
    fn finish_func(&mut self) -> Inst {
        assert!(matches!(self.mode, Mode::Func));
        self.transit(Mode::Body)
    }
    fn finish_body(&mut self) -> Inst {
        assert!(matches!(self.mode, Mode::Body));
        let func = mem::take(&mut self.func);
        let body = mem::take(&mut self.body);
        self.rewind();
        self.add(Inst::Define(func, body))
    }
    fn finish_quote(&mut self) -> Inst {
        assert!(matches!(self.mode, Mode::Quote));
        let quote = mem::take(&mut self.quote);
        self.rewind();
        self.add(Inst::Quote(quote))
    }
    fn finish_direct(&mut self, input: char) -> Inst {
        assert!(matches!(self.mode, Mode::Direct));
        let inst = if let Ok(input) = u8::try_from(input) {
            Inst::Imm(input)
        } else {
            Inst::Nop
        };
        self.rewind();
        self.add(inst)
    }
    fn finish_exec(&mut self, input: char) -> Inst {
        assert!(matches!(self.mode, Mode::Exec));
        self.rewind();
        self.add(if input.is_ascii_graphic() {
            Inst::Exec(input)
        } else {
            Inst::Nop
        })
    }
    fn finish_repeat(&mut self, input: char) -> Inst {
        assert!(matches!(self.mode, Mode::Repeat));
        self.rewind();
        self.add(if input.is_ascii_graphic() {
            Inst::Repeat(input)
        } else {
            Inst::Nop
        })
    }
    fn finish_register(&mut self, input: char) -> Inst {
        assert!(matches!(self.mode, Mode::Register));
        if input.is_ascii_graphic() {
            self.push(input);
            self.transit(Mode::Record)
        } else {
            self.rewind()
        }
    }
    fn finish_record(&mut self) -> Inst {
        assert!(matches!(self.mode, Mode::Record));
        assert!(self.register.is_some());
        let register = mem::take(&mut self.register).unwrap();
        let record = mem::take(&mut self.record);
        self.rewind();
        self.add(Inst::Macro(register, record))
    }
}
