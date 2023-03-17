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
            direct: Mode::Direct,
            exec: Mode::Exec,
            repeat: Mode::Repeat,
            record: Mode::Register,
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
    fn toggle_func(next: &mut Mode) {
        match next {
            Mode::Normal => mem::replace(next, Mode::Func),
            Mode::Func => mem::replace(next, Mode::Body),
            Mode::Body => mem::replace(next, Mode::Normal),
            _ => unreachable!(),
        };
    }
    fn toggle_quote(&mut self, next: &mut Mode) {
        assert!(match self.quote {
            Mode::Quote => matches!(next, Mode::Normal | Mode::Body | Mode::Record),
            Mode::Normal | Mode::Body | Mode::Record => matches!(next, Mode::Quote),
            _ => false,
        });
        mem::swap(&mut self.quote, next);
    }
    fn toggle_direct(&mut self, next: &mut Mode) {
        assert!(match self.direct {
            Mode::Direct => matches!(next, Mode::Normal | Mode::Body | Mode::Record),
            Mode::Normal | Mode::Body | Mode::Record => matches!(next, Mode::Direct),
            _ => false,
        });
        mem::swap(&mut self.direct, next);
    }
    fn toggle_exec(&mut self, next: &mut Mode) {
        assert!(match self.exec {
            Mode::Exec => matches!(next, Mode::Normal | Mode::Body | Mode::Record),
            Mode::Normal | Mode::Body | Mode::Record => matches!(next, Mode::Exec),
            _ => false,
        });
        mem::swap(&mut self.exec, next);
    }
    fn toggle_repeat(&mut self, next: &mut Mode) {
        assert!(match self.repeat {
            Mode::Repeat => matches!(next, Mode::Normal | Mode::Body | Mode::Record),
            Mode::Normal | Mode::Body | Mode::Record => matches!(next, Mode::Repeat),
            _ => false,
        });
        mem::swap(&mut self.repeat, next);
    }
    fn toggle_record(&mut self, next: &mut Mode) {
        assert!(match self.record {
            Mode::Normal | Mode::Body => matches!(next, Mode::Register | Mode::Record),
            Mode::Register | Mode::Record => matches!(next, Mode::Normal | Mode::Body),
            _ => false,
        });
        match next {
            Mode::Normal | Mode::Body => mem::swap(&mut self.record, next),
            Mode::Register => *next = Mode::Record,
            Mode::Record => *next = mem::replace(&mut self.record, Mode::Register),
            _ => unreachable!(),
        };
    }
    fn toggle(&mut self, select: Mode, next: &mut Mode) {
        match select {
            Mode::Normal => unreachable!(),
            Mode::Ignore => self.toggle_ignore(next),
            Mode::Call => self.toggle_call(next),
            Mode::Func | Mode::Body => Self::toggle_func(next),
            Mode::Quote => self.toggle_quote(next),
            Mode::Direct => self.toggle_direct(next),
            Mode::Exec => self.toggle_exec(next),
            Mode::Repeat => self.toggle_repeat(next),
            Mode::Register | Mode::Record => self.toggle_record(next),
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
            Mode::Call => self.finish(Mode::Call, 0),
            Mode::Func => self.toggle(Mode::Func),
            _ => self.consume_other(NEWLINE),
        }
    }
    fn consume_quote(&mut self) -> Inst {
        match self.mode {
            Mode::Normal | Mode::Body | Mode::Record => self.toggle(Mode::Quote),
            Mode::Quote => self.finish(Mode::Quote, 0),
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
            Mode::Normal | Mode::Body | Mode::Record => self.toggle(Mode::Repeat),
            _ => self.consume_other(PERCENT),
        }
    }
    fn consume_apostrophe(&mut self) -> Inst {
        match self.mode {
            Mode::Normal | Mode::Body | Mode::Record => self.toggle(Mode::Direct),
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
        if self.last.is_newline() {
            match self.mode {
                Mode::Normal => self.toggle(Mode::Body),
                Mode::Body => self.finish(Mode::Body, 0),
                Mode::Record => {
                    self.finish(Mode::Record, 0);
                    match self.mode {
                        Mode::Normal => self.toggle(Mode::Body),
                        Mode::Body => self.finish(Mode::Body, 0),
                        _ => unreachable!(),
                    }
                }
                Mode::Quote => self.consume_other(SEMICOLON),
                _ => unreachable!(),
            }
        } else {
            match self.mode {
                Mode::Normal | Mode::Body | Mode::Record => self.push_inst(Inst::Nop),
                _ => self.consume_other(SEMICOLON),
            }
        }
    }
    fn consume_at(&mut self) -> Inst {
        match self.mode {
            Mode::Normal | Mode::Body | Mode::Record => self.toggle(Mode::Exec),
            _ => self.consume_other(AT),
        }
    }
    fn consume_q(&mut self) -> Inst {
        match self.mode {
            Mode::Normal | Mode::Body => self.toggle(Mode::Register),
            Mode::Record => self.finish(Mode::Record, 0),
            _ => self.consume_other(Q),
        }
    }
    fn consume_other(&mut self, input: u8) -> Inst {
        match self.mode {
            Mode::Ignore => Inst::Skip,
            Mode::Normal | Mode::Body | Mode::Record => self.push_inst(Inst::new(input)),
            Mode::Call | Mode::Func | Mode::Quote => self.push_char(input),
            Mode::Direct => self.finish(Mode::Direct, input),
            Mode::Exec => self.finish(Mode::Exec, input),
            Mode::Repeat => self.finish(Mode::Repeat, input),
            Mode::Register => {
                self.register = Some(input);
                self.toggle(Mode::Record)
            }
        }
    }
    fn push_char(&mut self, input: u8) -> Inst {
        match self.mode {
            Mode::Call => self.call.push(input),
            Mode::Func => self.func.push(input),
            Mode::Quote => self.quote.push(input),
            Mode::Record => self.register = Some(input),
            _ => unreachable!(),
        }
        Inst::Skip
    }
    fn push_inst(&mut self, inst: Inst) -> Inst {
        match self.mode {
            Mode::Normal => return inst,
            Mode::Body => self.body.push(inst),
            Mode::Record => self.record.push(inst),
            _ => unreachable!(),
        }
        Inst::Skip
    }
    fn finish(&mut self, select: Mode, input: u8) -> Inst {
        let inst = match select {
            Mode::Call => Inst::Call(mem::take(&mut self.call)),
            Mode::Func | Mode::Body => {
                Inst::Define(mem::take(&mut self.func), mem::take(&mut self.body))
            }
            Mode::Quote => Inst::Quote(mem::take(&mut self.quote)),
            Mode::Direct => Inst::Imm(input),
            Mode::Exec => Inst::Exec(input),
            Mode::Repeat => Inst::Repeat(input),
            Mode::Register | Mode::Record => Inst::Macro(
                mem::take(&mut self.register).unwrap(),
                mem::take(&mut self.record),
            ),
            Mode::Normal | Mode::Ignore => unreachable!(),
        };
        self.toggle(select);
        self.push_inst(inst)
    }
    fn toggle(&mut self, select: Mode) -> Inst {
        self.next.toggle(select, &mut self.mode);
        Inst::Skip
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
        mode_test(";\n#;\n;", &[Func, Body, Ignore, Ignore, Body, Normal]);
        mode_test(
            "q##q\nq",
            &[Register, Record, Ignore, Ignore, Record, Normal],
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
        mode_test(";\n:;\n;", &[Func, Body, Call, Call, Body, Normal]);
        mode_test("q::q\nq", &[Register, Record, Call, Call, Record, Normal]);
    }
    #[test]
    fn func_test() {
        mode_test(";;\n;", &[Func, Func, Body, Normal]);
        mode_test(
            "; \"#%':;@q\nabc\n;",
            &[
                Func, Func, Func, Func, Func, Func, Func, Func, Func, Func, Body, Body, Body, Body,
                Body, Normal,
            ],
        );
        mode_test(
            ";q;\nq;\n ;q\n;",
            &[
                Func, Func, Func, Body, Register, Record, Record, Record, Record, Body, Body,
                Normal,
            ],
        );
        mode_test(
            "q;\n;q\nq;\n;",
            &[
                Register, Record, Record, Func, Func, Body, Register, Record, Record, Normal,
            ],
        );
        mode_test("q;q", &[Register, Record, Normal]);
    }
    #[test]
    fn quote_test() {
        mode_test("\"\"", &[Quote, Normal]);
        mode_test(
            "\"#%':;@q\n;\"",
            &[
                Quote, Quote, Quote, Quote, Quote, Quote, Quote, Quote, Quote, Quote, Normal,
            ],
        );
        mode_test(
            ";\n\";\n;\"\n;",
            &[Func, Body, Quote, Quote, Quote, Quote, Body, Body, Normal],
        );
        mode_test(
            "q\"\"q\"q",
            &[Register, Record, Quote, Quote, Record, Normal],
        );
    }
    #[test]
    fn direct_test() {
        mode_test(
            "' '\"'#'%''':';'@'q'\n",
            &[
                Direct, Normal, Direct, Normal, Direct, Normal, Direct, Normal, Direct, Normal,
                Direct, Normal, Direct, Normal, Direct, Normal, Direct, Normal, Direct, Normal,
            ],
        );
        mode_test(";\n';\n;", &[Func, Body, Direct, Body, Body, Normal]);
        mode_test("q''qq", &[Register, Record, Direct, Record, Normal]);
    }
    #[test]
    fn exec_test() {
        mode_test(
            "@ @\"@#@%@'@:@;@@@q@\n",
            &[
                Exec, Normal, Exec, Normal, Exec, Normal, Exec, Normal, Exec, Normal, Exec, Normal,
                Exec, Normal, Exec, Normal, Exec, Normal, Exec, Normal,
            ],
        );
        mode_test(";\n@;\n;", &[Func, Body, Exec, Body, Body, Normal]);
        mode_test("q@@qq", &[Register, Record, Exec, Record, Normal]);
    }
    #[test]
    fn repeat_test() {
        mode_test(
            "% %\"%#%%%'%:%;%@%q%\n",
            &[
                Repeat, Normal, Repeat, Normal, Repeat, Normal, Repeat, Normal, Repeat, Normal,
                Repeat, Normal, Repeat, Normal, Repeat, Normal, Repeat, Normal, Repeat, Normal,
            ],
        );
        mode_test(";\n%;\n;", &[Func, Body, Repeat, Body, Body, Normal]);
        mode_test("q%%qq", &[Register, Record, Repeat, Record, Normal]);
    }
    #[test]
    fn record_test() {
        mode_test("q q", &[Register, Record, Normal]);
        mode_test("q\n\nq", &[Register, Record, Record, Normal]);
        mode_test("qqq", &[Register, Record, Normal]);
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
