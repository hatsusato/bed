use crate::cmd;
use state::State;

#[derive(Clone)]
enum Mode {
    Normal,
    Ignore,
    Quote(String),
}
impl Default for Mode {
    fn default() -> Self {
        Self::Normal
    }
}

#[derive(Default)]
pub struct Exec {
    mode: Mode,
    state: State,
    last: char,
}
impl Exec {
    pub fn exec(&mut self, key: char) {
        match self.mode.clone() {
            Mode::Normal => self.exec_normal(key),
            Mode::Ignore => self.exec_ignore(key),
            Mode::Quote(quote) => self.exec_quote(key, quote),
        }
        self.last = key;
    }
    fn exec_normal(&mut self, key: char) {
        match key {
            '\n' => self.mode = Mode::Normal,
            '#' => self.mode = Mode::Ignore,
            '"' => self.mode = Mode::Quote(String::new()),
            _ => self.exec_cmd(key),
        }
    }
    fn exec_ignore(&mut self, key: char) {
        if key == '\n' {
            self.mode = Mode::Normal;
        }
    }
    fn exec_quote(&mut self, key: char, quote: String) {
        if key == '"' {
            let count = self.state.acc();
            (0..count).for_each(|_| self.exec_quoted(&quote));
            self.mode = Mode::Normal;
        } else {
            let mut quote = quote;
            quote.push(key);
            self.mode = Mode::Quote(quote);
        }
    }
    fn exec_quoted(&mut self, quote: &str) {
        quote.chars().for_each(|key| self.exec_cmd(key));
    }
    fn exec_cmd(&mut self, key: char) {
        use cmd::Command;
        let state = &self.state;
        let cmd = match key {
            '\n' => unreachable!(),
            '!' => Command::neg(state),
            '"' => unreachable!(),
            '#' => unreachable!(),
            '$' => Command::argv(state),
            '%' => return,
            '&' => Command::and(state),
            '\'' => return,
            '(' => Command::hi(state),
            ')' => Command::lo(state),
            '*' => Command::mul(state),
            '+' => Command::add(state),
            ',' => return,
            '-' => Command::sub(state),
            '.' => return,
            '/' => Command::div(state),
            '0'..='9' => Command::imm(state, translate_hex_digit(key)),
            ':' => return,
            ';' => return,
            '<' => Command::lt(state),
            '=' => Command::eq(state),
            '>' => Command::gt(state),
            '?' => Command::bool(state),
            '@' => Command::argc(state),
            'A'..='Z' => return self.exec_cmd(key.to_ascii_lowercase()),
            '[' => Command::shl(state),
            '\\' => return,
            ']' => Command::shr(state),
            '^' => Command::xor(state),
            '_' => return,
            '`' => return,
            'a'..='f' => Command::imm(state, translate_hex_digit(key)),
            'g' => Command::goto(state),
            'h' => Command::left(state),
            'i' => Command::load(state),
            'j' => Command::down(state),
            'k' => Command::up(state),
            'l' => Command::right(state),
            'm' => Command::dec(state),
            'n' => Command::inc(state),
            'o' => Command::store(state),
            'p' => return,
            'q' => return,
            'r' => return,
            's' => Command::swap(state),
            't' => Command::jump(state),
            'u' => return,
            'v' => Command::pos(state),
            'w' => return,
            'x' => return,
            'y' => return,
            'z' => return,
            '{' => Command::rotl(state),
            '|' => Command::or(state),
            '}' => Command::rotr(state),
            '~' => Command::not(state),
            _ => return,
        };
        self.state.restore_bank(cmd.next);
        self.state.restore_page(cmd.page);
    }
    pub fn print(&self) {
        self.state.print(self.last);
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
