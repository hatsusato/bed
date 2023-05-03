use crate::inst::Inst;
use std::mem;

const NEWLINE: u8 = b'\n';
const QUOTE: u8 = b'"';
const HASH: u8 = b'#';
const DOLLAR: u8 = b'$';
const APOSTROPHE: u8 = b'\'';
const COLON: u8 = b':';
const SEMICOLON: u8 = b';';
const AT: u8 = b'@';
const Q: u8 = b'q';

#[derive(Clone, Copy, PartialEq, Debug)]
enum Mode {
    Normal,
    Comment,
    Invoke,
    Define,
    Function,
    Quote,
    Direct,
    Execute,
    Repeat,
    Register,
    Macro,
}
impl Default for Mode {
    fn default() -> Self {
        Mode::Normal
    }
}
impl Mode {
    fn new(input: u8) -> Self {
        match input {
            HASH => Mode::Comment,
            COLON => Mode::Invoke,
            SEMICOLON => Mode::Define,
            QUOTE => Mode::Quote,
            APOSTROPHE => Mode::Direct,
            AT => Mode::Execute,
            DOLLAR => Mode::Repeat,
            Q => Mode::Register,
            _ => unreachable!(),
        }
    }
}

struct Next {
    comment: Mode,
    invoke: Mode,
    quote: Mode,
    direct: Mode,
    execute: Mode,
    repeat: Mode,
    register: Mode,
}
impl Default for Next {
    fn default() -> Self {
        Self {
            comment: Mode::Comment,
            invoke: Mode::Invoke,
            quote: Mode::Quote,
            direct: Mode::Direct,
            execute: Mode::Execute,
            repeat: Mode::Repeat,
            register: Mode::Register,
        }
    }
}
impl Next {
    fn toggle_comment(&mut self, next: &mut Mode) {
        assert!(match self.comment {
            Mode::Comment => matches!(next, Mode::Normal | Mode::Function | Mode::Macro),
            Mode::Normal | Mode::Function | Mode::Macro => matches!(next, Mode::Comment),
            _ => false,
        });
        mem::swap(&mut self.comment, next);
    }
    fn toggle_invoke(&mut self, next: &mut Mode) {
        assert!(match self.invoke {
            Mode::Invoke => matches!(next, Mode::Normal | Mode::Function | Mode::Macro),
            Mode::Normal | Mode::Function | Mode::Macro => matches!(next, Mode::Invoke),
            _ => false,
        });
        mem::swap(&mut self.invoke, next);
    }
    fn toggle_define(next: &mut Mode) {
        match next {
            Mode::Normal => mem::replace(next, Mode::Define),
            Mode::Define => mem::replace(next, Mode::Function),
            Mode::Function => mem::replace(next, Mode::Normal),
            _ => unreachable!(),
        };
    }
    fn toggle_quote(&mut self, next: &mut Mode) {
        assert!(match self.quote {
            Mode::Quote => matches!(next, Mode::Normal | Mode::Function | Mode::Macro),
            Mode::Normal | Mode::Function | Mode::Macro => matches!(next, Mode::Quote),
            _ => false,
        });
        mem::swap(&mut self.quote, next);
    }
    fn toggle_direct(&mut self, next: &mut Mode) {
        assert!(match self.direct {
            Mode::Direct => matches!(next, Mode::Normal | Mode::Function | Mode::Macro),
            Mode::Normal | Mode::Function | Mode::Macro => matches!(next, Mode::Direct),
            _ => false,
        });
        mem::swap(&mut self.direct, next);
    }
    fn toggle_execute(&mut self, next: &mut Mode) {
        assert!(match self.execute {
            Mode::Execute => matches!(next, Mode::Normal | Mode::Function | Mode::Macro),
            Mode::Normal | Mode::Function | Mode::Macro => matches!(next, Mode::Execute),
            _ => false,
        });
        mem::swap(&mut self.execute, next);
    }
    fn toggle_repeat(&mut self, next: &mut Mode) {
        assert!(match self.repeat {
            Mode::Repeat => matches!(next, Mode::Normal | Mode::Function | Mode::Macro),
            Mode::Normal | Mode::Function | Mode::Macro => matches!(next, Mode::Repeat),
            _ => false,
        });
        mem::swap(&mut self.repeat, next);
    }
    fn toggle_register(&mut self, next: &mut Mode) {
        assert!(match self.register {
            Mode::Normal | Mode::Function => matches!(next, Mode::Register | Mode::Macro),
            Mode::Register | Mode::Macro => matches!(next, Mode::Normal | Mode::Function),
            _ => false,
        });
        match next {
            Mode::Normal | Mode::Function => mem::swap(&mut self.register, next),
            Mode::Register => *next = Mode::Macro,
            Mode::Macro => *next = mem::replace(&mut self.register, Mode::Register),
            _ => unreachable!(),
        };
    }
    fn toggle(&mut self, select: Mode, next: &mut Mode) {
        match select {
            Mode::Normal => unreachable!(),
            Mode::Comment => self.toggle_comment(next),
            Mode::Invoke => self.toggle_invoke(next),
            Mode::Define | Mode::Function => Self::toggle_define(next),
            Mode::Quote => self.toggle_quote(next),
            Mode::Direct => self.toggle_direct(next),
            Mode::Execute => self.toggle_execute(next),
            Mode::Repeat => self.toggle_repeat(next),
            Mode::Register | Mode::Macro => self.toggle_register(next),
        }
    }
}

#[derive(Default)]
pub struct Lexer {
    mode: Mode,
    next: Next,
    invoke: Vec<u8>,
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
            (
                Mode::Normal | Mode::Function | Mode::Macro,
                SEMICOLON,
                false
            )
        ) {
            return self.add(Inst::Nop);
        }
        match (self.mode, input) {
            (Mode::Normal | Mode::Function, Q)
            | (Mode::Normal, SEMICOLON)
            | (
                Mode::Normal | Mode::Function | Mode::Macro,
                HASH | COLON | QUOTE | APOSTROPHE | AT | DOLLAR,
            ) => self.toggle(Mode::new(input), None),
            (Mode::Macro, SEMICOLON) => self.finish(input, true),
            (Mode::Comment | Mode::Invoke | Mode::Define, NEWLINE)
            | (Mode::Function, SEMICOLON)
            | (Mode::Quote, QUOTE)
            | (Mode::Macro, Q)
            | (Mode::Direct | Mode::Execute | Mode::Repeat | Mode::Register, _) => {
                self.finish(input, false)
            }
            (Mode::Invoke | Mode::Define | Mode::Function | Mode::Quote | Mode::Macro, _) => {
                self.push(input)
            }
            (Mode::Comment, _) => Inst::Skip,
            (Mode::Normal, _) => Inst::new(input),
        }
    }
    fn push(&mut self, input: u8) -> Inst {
        match self.mode {
            Mode::Normal | Mode::Comment | Mode::Direct | Mode::Execute | Mode::Repeat => {
                unreachable!()
            }
            Mode::Invoke => self.invoke.push(input),
            Mode::Define => self.func.push(input),
            Mode::Function => self.body.push(Inst::new(input)),
            Mode::Quote => self.quote.push(input),
            Mode::Macro => self.record.push(Inst::new(input)),
            Mode::Register => self.register = Some(input),
        }
        Inst::Skip
    }
    fn take(&mut self, input: u8) -> Inst {
        match self.mode {
            Mode::Comment | Mode::Define => Inst::Skip,
            Mode::Invoke => Inst::Invoke(mem::take(&mut self.invoke)),
            Mode::Function => Inst::Define(mem::take(&mut self.func), mem::take(&mut self.body)),
            Mode::Quote => Inst::Quote(mem::take(&mut self.quote)),
            Mode::Direct => Inst::Direct(input),
            Mode::Execute => Inst::Exec(input),
            Mode::Repeat => Inst::Repeat(input),
            Mode::Register => self.push(input),
            Mode::Macro => Inst::Register(
                mem::take(&mut self.register).unwrap(),
                mem::take(&mut self.record),
            ),
            Mode::Normal => unreachable!(),
        }
    }
    fn add(&mut self, inst: Inst) -> Inst {
        if inst != Inst::Skip {
            match self.mode {
                Mode::Normal => return inst,
                Mode::Function => self.body.push(inst),
                Mode::Macro => self.record.push(inst),
                _ => unreachable!(),
            }
        }
        Inst::Skip
    }
    fn finish(&mut self, input: u8, aborted: bool) -> Inst {
        let inst = self.take(input);
        let inst = self.toggle(self.mode, Some(inst));
        if aborted {
            self.consume(input)
        } else {
            inst
        }
    }
    fn toggle(&mut self, select: Mode, inst: Option<Inst>) -> Inst {
        self.next.toggle(select, &mut self.mode);
        self.add(inst.unwrap_or(Inst::Skip))
    }
}

#[cfg(test)]
mod lexer_tests {
    use super::{Lexer, Mode};
    #[allow(clippy::enum_glob_use)]
    use Mode::*;
    #[test]
    fn comment_test() {
        mode_test("", &[]);
        mode_test(" #\n", &[Normal, Comment, Normal]);
        mode_test(
            "# \"#$':;@q\n",
            &[
                Comment, Comment, Comment, Comment, Comment, Comment, Comment, Comment, Comment,
                Comment, Normal,
            ],
        );
        mode_test(
            ";\n#;\n;",
            &[Define, Function, Comment, Comment, Function, Normal],
        );
        mode_test(
            "q##q\nq",
            &[Register, Macro, Comment, Comment, Macro, Normal],
        );
    }
    #[test]
    fn invoke_test() {
        mode_test(":\n", &[Invoke, Normal]);
        mode_test(": a\n", &[Invoke, Invoke, Invoke, Normal]);
        mode_test(
            ":\"#$':;@q\n",
            &[
                Invoke, Invoke, Invoke, Invoke, Invoke, Invoke, Invoke, Invoke, Invoke, Normal,
            ],
        );
        mode_test(
            ";\n:;\n;",
            &[Define, Function, Invoke, Invoke, Function, Normal],
        );
        mode_test("q::q\nq", &[Register, Macro, Invoke, Invoke, Macro, Normal]);
    }
    #[test]
    fn define_test() {
        mode_test(";;\n;", &[Define, Define, Function, Normal]);
        mode_test(
            "; \"#$':;@q\nabc\n;",
            &[
                Define, Define, Define, Define, Define, Define, Define, Define, Define, Define,
                Function, Function, Function, Function, Function, Normal,
            ],
        );
        mode_test(
            ";q;\nq;\n ;q\n;",
            &[
                Define, Define, Define, Function, Register, Macro, Macro, Macro, Macro, Function,
                Function, Normal,
            ],
        );
        mode_test(
            "q;\n;q\nq;\n;",
            &[
                Register, Macro, Macro, Define, Define, Function, Register, Macro, Macro, Normal,
            ],
        );
        mode_test("q;q", &[Register, Macro, Normal]);
    }
    #[test]
    fn quote_test() {
        mode_test("\"\"", &[Quote, Normal]);
        mode_test(
            "\"#$':;@q\n;\"",
            &[
                Quote, Quote, Quote, Quote, Quote, Quote, Quote, Quote, Quote, Quote, Normal,
            ],
        );
        mode_test(
            ";\n\";\n;\"\n;",
            &[
                Define, Function, Quote, Quote, Quote, Quote, Function, Function, Normal,
            ],
        );
        mode_test("q\"\"q\"q", &[Register, Macro, Quote, Quote, Macro, Normal]);
    }
    #[test]
    fn direct_test() {
        mode_test(
            "' '\"'#'$''':';'@'q'\n",
            &[
                Direct, Normal, Direct, Normal, Direct, Normal, Direct, Normal, Direct, Normal,
                Direct, Normal, Direct, Normal, Direct, Normal, Direct, Normal, Direct, Normal,
            ],
        );
        mode_test(
            ";\n';'\n;",
            &[Define, Function, Direct, Function, Direct, Function, Normal],
        );
        mode_test(
            "q''q'\nq",
            &[Register, Macro, Direct, Macro, Direct, Macro, Normal],
        );
    }
    #[test]
    fn execute_test() {
        mode_test(
            "@ @\"@#@$@'@:@;@@@q@\n",
            &[
                Execute, Normal, Execute, Normal, Execute, Normal, Execute, Normal, Execute,
                Normal, Execute, Normal, Execute, Normal, Execute, Normal, Execute, Normal,
                Execute, Normal,
            ],
        );
        mode_test(
            ";\n@;@\n;",
            &[
                Define, Function, Execute, Function, Execute, Function, Normal,
            ],
        );
        mode_test(
            "q@@q@\nq",
            &[Register, Macro, Execute, Macro, Execute, Macro, Normal],
        );
    }
    #[test]
    fn repeat_test() {
        mode_test(
            "$ $\"$#$$$'$:$;$@$q$\n",
            &[
                Repeat, Normal, Repeat, Normal, Repeat, Normal, Repeat, Normal, Repeat, Normal,
                Repeat, Normal, Repeat, Normal, Repeat, Normal, Repeat, Normal, Repeat, Normal,
            ],
        );
        mode_test(
            ";\n$;$\n;",
            &[Define, Function, Repeat, Function, Repeat, Function, Normal],
        );
        mode_test(
            "q$$q$\nq",
            &[Register, Macro, Repeat, Macro, Repeat, Macro, Normal],
        );
    }
    #[test]
    fn register_test() {
        mode_test("q q", &[Register, Macro, Normal]);
        mode_test("q\nq", &[Register, Macro, Normal]);
        mode_test("qqq", &[Register, Macro, Normal]);
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
