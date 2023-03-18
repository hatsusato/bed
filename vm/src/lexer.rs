use crate::inst::Inst;
use std::mem;

const NEWLINE: u8 = b'\n';
const QUOTE: u8 = b'"';
const HASH: u8 = b'#';
const PERCENT: u8 = b'%';
const APOSTROPHE: u8 = b'\'';
const COLON: u8 = b':';
const SEMICOLON: u8 = b';';
const AT: u8 = b'@';
const Q: u8 = b'q';

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
impl Mode {
    fn new(input: u8) -> Self {
        match input {
            HASH => Mode::Ignore,
            COLON => Mode::Call,
            QUOTE => Mode::Quote,
            APOSTROPHE => Mode::Direct,
            AT => Mode::Exec,
            PERCENT => Mode::Repeat,
            Q => Mode::Register,
            _ => unreachable!(),
        }
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
    last: Option<u8>,
}
impl Lexer {
    pub fn get_last(&self) -> u8 {
        self.last.unwrap_or(NEWLINE)
    }
    pub fn translate(&mut self, input: u8) -> Inst {
        let inst = self.consume(input);
        self.last = if input == NEWLINE { None } else { Some(input) };
        inst
    }
    fn is_head(&self) -> bool {
        self.last.is_none()
    }
    fn consume(&mut self, input: u8) -> Inst {
        if matches!(
            (self.mode, input, self.is_head()),
            (Mode::Normal | Mode::Body | Mode::Record, SEMICOLON, false)
        ) {
            return self.add(Inst::Nop);
        }
        match (self.mode, input) {
            (Mode::Normal | Mode::Body, Q)
            | (
                Mode::Normal | Mode::Body | Mode::Record,
                HASH | COLON | QUOTE | APOSTROPHE | AT | PERCENT,
            ) => self.toggle(Mode::new(input)),
            (Mode::Normal, SEMICOLON) => self.toggle(Mode::Func),
            (Mode::Record, SEMICOLON) => {
                self.finish(input);
                self.consume(input)
            }
            (Mode::Ignore | Mode::Call | Mode::Func, NEWLINE)
            | (Mode::Body, SEMICOLON)
            | (Mode::Quote, QUOTE)
            | (Mode::Record, Q)
            | (Mode::Direct | Mode::Exec | Mode::Repeat | Mode::Register, _) => self.finish(input),
            (
                Mode::Normal
                | Mode::Ignore
                | Mode::Call
                | Mode::Func
                | Mode::Body
                | Mode::Quote
                | Mode::Record,
                _,
            ) => self.push(input),
        }
    }
    fn push(&mut self, input: u8) -> Inst {
        match self.mode {
            Mode::Normal => return Inst::new(input),
            Mode::Ignore => (),
            Mode::Call => self.call.push(input),
            Mode::Func => self.func.push(input),
            Mode::Body => self.body.push(Inst::new(input)),
            Mode::Quote => self.quote.push(input),
            Mode::Record => self.record.push(Inst::new(input)),
            Mode::Direct => return Inst::Imm(input),
            Mode::Exec => return Inst::Exec(input),
            Mode::Repeat => return Inst::Repeat(input),
            Mode::Register => self.register = Some(input),
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
    fn take(&mut self, input: u8) -> Inst {
        match self.mode {
            Mode::Ignore | Mode::Func => Inst::Skip,
            Mode::Call => Inst::Call(mem::take(&mut self.call)),
            Mode::Body => Inst::Define(mem::take(&mut self.func), mem::take(&mut self.body)),
            Mode::Quote => Inst::Quote(mem::take(&mut self.quote)),
            Mode::Direct | Mode::Exec | Mode::Repeat | Mode::Register => self.push(input),
            Mode::Record => Inst::Macro(
                mem::take(&mut self.register).unwrap(),
                mem::take(&mut self.record),
            ),
            Mode::Normal => unreachable!(),
        }
    }
    fn finish(&mut self, input: u8) -> Inst {
        let inst = self.take(input);
        self.toggle(self.mode);
        self.add(inst)
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
            lexer.translate(key);
            assert_eq!(lexer.mode, mode);
        }
    }
}
