use crate::{Bank, State};
use util::Page;

pub struct Command {
    pub next: Bank,
    pub page: Option<Page>,
}
impl Command {
    pub fn from_key(key: char, state: &State) -> Self {
        let mut result = Command::new(state);
        result.update_key(key, state);
        result
    }
    pub fn new(state: &State) -> Self {
        Self {
            next: state.bank(),
            page: None,
        }
    }
    fn update_data(mut self, data: u8) -> Self {
        self.next.data = data;
        self
    }
    fn update_key(&mut self, key: char, state: &State) {
        match key {
            '\n' => (),
            '!' => self.next.neg(),
            '"' => (),
            '#' => (),
            '$' => self.argv(state),
            '%' => self.next.argc(),
            '&' => self.next.and(),
            '\'' => (),
            '(' => self.next.hi(),
            ')' => self.next.lo(),
            '*' => self.next.mul(),
            '+' => self.next.add(),
            ',' => (),
            '-' => self.next.sub(),
            '.' => (),
            '/' => self.next.div(),
            '0'..='9' => self.next.imm(translate_hex_digit(key)),
            ':' => (),
            ';' => (),
            '<' => self.next.lt(),
            '=' => self.next.eq(),
            '>' => self.next.gt(),
            '?' => self.next.bool(),
            '@' => (),
            'A'..='Z' => self.update_key(key.to_ascii_lowercase(), state),
            '[' => self.next.shl(),
            '\\' => (),
            ']' => self.next.shr(),
            '^' => self.next.xor(),
            '_' => (),
            '`' => (),
            'a'..='f' => self.next.imm(translate_hex_digit(key)),
            'g' => self.next.goto(),
            'h' => self.next.left(),
            'i' => self.load(state),
            'j' => self.next.down(),
            'k' => self.next.up(),
            'l' => self.next.right(),
            'm' => self.next.dec(),
            'n' => self.next.inc(),
            'o' => self.store(state),
            'p' => (),
            'q' => (),
            'r' => (),
            's' => self.next.swap(),
            't' => self.next.jump(),
            'u' => (),
            'v' => self.next.pos(),
            'w' => (),
            'x' => (),
            'y' => (),
            'z' => (),
            '{' => self.next.rotl(),
            '|' => self.next.or(),
            '}' => self.next.rotr(),
            '~' => self.next.not(),
            _ => (),
        }
    }
    pub fn load(&mut self, state: &State) {
        self.next.data = state.page()[self.next.coord];
    }
    pub fn store(&mut self, state: &State) {
        let mut page = *state.page();
        page[self.next.coord] = self.next.data;
        self.page = Some(page);
    }
    pub fn argv(&mut self, state: &State) {
        let arg = std::env::args().nth(self.next.acc.into());
        if let Some(input) = &arg {
            let mut page = *state.page();
            let len = page.write(input.as_bytes().iter());
            (self.next.acc, self.page) = (len, Some(page));
        }
        self.next.set_error(arg.is_none());
    }
    pub fn esc(state: &State, key: char) -> Self {
        match u8::try_from(key) {
            Ok(data) => Self::new(state).update_data(data),
            Err(_) => Self::new(state),
        }
    }
}

fn translate_hex_digit(key: char) -> u8 {
    const ZERO: u8 = b'0';
    const A: u8 = b'a';
    match key {
        '0'..='9' => key as u8 - ZERO,
        'a'..='f' => key as u8 - A + 0xA,
        _ => unreachable!(),
    }
}
