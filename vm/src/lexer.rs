use crate::inst::Inst;
use std::mem;

#[derive(Clone, Copy)]
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
    quote: String,
    body: Vec<Inst>,
}
impl Lexer {
    pub fn consume(&mut self, input: char) -> Option<Inst> {
        match input {
            '\n' => self.consume_newline(),
            '"' => self.consume_quote(),
            '#' => self.consume_hash(),
            ':' => self.consume_colon(),
            ';' => self.consume_semicolon(),
            _ => self.consume_other(input),
        }
    }
    fn consume_newline(&mut self) -> Option<Inst> {
        match self.mode {
            Mode::Normal | Mode::Body | Mode::Quote => return self.push('\n'),
            Mode::Ignore => self.next_take(),
            Mode::Call => return self.finish(),
            Mode::Name => self.mode = Mode::Body,
        }
        None
    }
    fn consume_quote(&mut self) -> Option<Inst> {
        match self.mode {
            Mode::Normal => self.mode = Mode::Quote,
            Mode::Ignore => (),
            Mode::Call | Mode::Name => return self.push('"'),
            Mode::Body => self.next_replace(Mode::Quote),
            Mode::Quote => return self.finish(),
        }
        None
    }
    fn consume_hash(&mut self) -> Option<Inst> {
        match self.mode {
            Mode::Ignore => (),
            Mode::Normal | Mode::Call | Mode::Name | Mode::Body => self.next_replace(Mode::Ignore),
            Mode::Quote => return self.push('#'),
        }
        None
    }
    fn consume_colon(&mut self) -> Option<Inst> {
        match self.mode {
            Mode::Ignore => (),
            Mode::Normal | Mode::Body => self.next_replace(Mode::Call),
            Mode::Call | Mode::Name | Mode::Quote => return self.push(':'),
        }
        None
    }
    fn consume_semicolon(&mut self) -> Option<Inst> {
        match self.mode {
            Mode::Ignore => (),
            Mode::Normal => self.mode = Mode::Name,
            Mode::Call | Mode::Name | Mode::Quote => return self.push(';'),
            Mode::Body => return self.finish(),
        }
        None
    }
    fn consume_other(&mut self, input: char) -> Option<Inst> {
        match self.mode {
            Mode::Ignore => (),
            Mode::Normal => return Some(Inst::new(input)),
            Mode::Call | Mode::Name | Mode::Quote | Mode::Body => return self.push(input),
        }
        None
    }
    fn next_take(&mut self) {
        self.mode = mem::take(self.next.select(self.mode));
    }
    fn next_replace(&mut self, next: Mode) {
        *self.next.select(next) = mem::replace(&mut self.mode, next);
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
