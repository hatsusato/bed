use crate::{Bank, State};
use util::{Page, BLOCK_SIDE};

pub struct Command {
    pub next: Bank,
    pub page: Option<Page>,
}
impl Command {
    pub fn from_key(key: char, state: &State) -> Self {
        match key {
            '\n' => (),
            '!' => return Command::neg(state),
            '"' => (),
            '#' => (),
            '$' => return Command::argv(state),
            '%' => (),
            '&' => return Command::and(state),
            '\'' => (),
            '(' => return Command::hi(state),
            ')' => return Command::lo(state),
            '*' => return Command::mul(state),
            '+' => return Command::add(state),
            ',' => (),
            '-' => return Command::sub(state),
            '.' => (),
            '/' => return Command::div(state),
            '0'..='9' => return Command::imm(state, translate_hex_digit(key)),
            ':' => (),
            ';' => (),
            '<' => return Command::lt(state),
            '=' => return Command::eq(state),
            '>' => return Command::gt(state),
            '?' => return Command::bool(state),
            '@' => return Command::argc(state),
            'A'..='Z' => return Self::from_key(key.to_ascii_lowercase(), state),
            '[' => return Command::shl(state),
            '\\' => (),
            ']' => return Command::shr(state),
            '^' => return Command::xor(state),
            '_' => (),
            '`' => (),
            'a'..='f' => return Command::imm(state, translate_hex_digit(key)),
            'g' => return Command::goto(state),
            'h' => return Command::left(state),
            'i' => return Command::load(state),
            'j' => return Command::down(state),
            'k' => return Command::up(state),
            'l' => return Command::right(state),
            'm' => return Command::dec(state),
            'n' => return Command::inc(state),
            'o' => return Command::store(state),
            'p' => (),
            'q' => (),
            'r' => (),
            's' => return Command::swap(state),
            't' => return Command::jump(state),
            'u' => (),
            'v' => return Command::pos(state),
            'w' => (),
            'x' => (),
            'y' => (),
            'z' => (),
            '{' => return Command::rotl(state),
            '|' => return Command::or(state),
            '}' => return Command::rotr(state),
            '~' => return Command::not(state),
            _ => (),
        }
        Command::new(state)
    }
    pub fn new(state: &State) -> Self {
        Self {
            next: state.bank(),
            page: None,
        }
    }
    fn update_reg(mut self, reg: u16) -> Self {
        [self.next.data, self.next.acc] = reg.to_be_bytes();
        self
    }
    fn update_acc(mut self, acc: u8) -> Self {
        self.next.acc = acc;
        self
    }
    fn update_block(mut self, block: u8) -> Self {
        self.next.block = block;
        self
    }
    fn update_coord(mut self, coord: u8) -> Self {
        self.next.coord = coord;
        self
    }
    fn update_data(mut self, data: u8) -> Self {
        self.next.data = data;
        self
    }
    fn update_error(mut self, error: bool) -> Self {
        if error {
            self.next.error = true;
        }
        self
    }
    fn update_page(mut self, page: Page) -> Self {
        self.page = Some(page);
        self
    }
    fn from_bank(next: Bank) -> Self {
        Self { next, page: None }
    }
    pub fn imm(state: &State, digit: u8) -> Self {
        Self::from_bank(state.bank().imm(digit))
    }
    pub fn swap(state: &State) -> Self {
        Self::from_bank(state.bank().swap())
    }
    pub fn hi(state: &State) -> Self {
        Self::from_bank(state.bank().hi())
    }
    pub fn lo(state: &State) -> Self {
        Self::from_bank(state.bank().lo())
    }
    pub fn inc(state: &State) -> Self {
        let (next, _) = state.acc().overflowing_add(1);
        Self::new(state).update_acc(next)
    }
    pub fn dec(state: &State) -> Self {
        let (next, _) = state.acc().overflowing_sub(1);
        Self::new(state).update_acc(next)
    }
    pub fn add(state: &State) -> Self {
        let next = u16::from(state.acc()) + u16::from(state.data());
        Self::new(state).update_reg(next)
    }
    pub fn sub(state: &State) -> Self {
        let (next, _) = u16::from(state.acc()).overflowing_sub(state.data().into());
        Self::new(state).update_reg(next)
    }
    pub fn mul(state: &State) -> Self {
        let next = u16::from(state.data()) * u16::from(state.acc());
        Self::new(state).update_reg(next)
    }
    pub fn div(state: &State) -> Self {
        if state.data() == 0 {
            Self::new(state).update_error(true)
        } else {
            Self::new(state)
                .update_acc(state.acc() / state.data())
                .update_data(state.acc() % state.data())
        }
    }
    pub fn neg(state: &State) -> Self {
        Self::new(state).update_acc(u8::from(state.data() == 0))
    }
    pub fn bool(state: &State) -> Self {
        Self::new(state).update_acc(u8::from(state.data() != 0))
    }
    pub fn eq(state: &State) -> Self {
        Self::new(state).update_acc(u8::from(state.data() == state.acc()))
    }
    pub fn lt(state: &State) -> Self {
        Self::new(state).update_acc(u8::from(state.data() < state.acc()))
    }
    pub fn gt(state: &State) -> Self {
        Self::new(state).update_acc(u8::from(state.data() > state.acc()))
    }
    pub fn not(state: &State) -> Self {
        Self::new(state).update_acc(!state.data())
    }
    pub fn and(state: &State) -> Self {
        Self::new(state).update_acc(state.data() & state.acc())
    }
    pub fn or(state: &State) -> Self {
        Self::new(state).update_acc(state.data() | state.acc())
    }
    pub fn xor(state: &State) -> Self {
        Self::new(state).update_acc(state.data() ^ state.acc())
    }
    pub fn shl(state: &State) -> Self {
        Self::new(state).update_acc(state.acc() << 1)
    }
    pub fn shr(state: &State) -> Self {
        Self::new(state).update_acc(state.acc() >> 1)
    }
    pub fn rotl(state: &State) -> Self {
        Self::new(state).update_acc(rot(state.acc(), true))
    }
    pub fn rotr(state: &State) -> Self {
        Self::new(state).update_acc(rot(state.acc(), false))
    }
    pub fn left(state: &State) -> Self {
        Self::new(state).update_coord(backward(state, 1))
    }
    pub fn right(state: &State) -> Self {
        Self::new(state).update_coord(forward(state, 1))
    }
    pub fn down(state: &State) -> Self {
        Self::new(state).update_coord(forward(state, BLOCK_SIDE))
    }
    pub fn up(state: &State) -> Self {
        Self::new(state).update_coord(backward(state, BLOCK_SIDE))
    }
    pub fn pos(state: &State) -> Self {
        Self::new(state)
            .update_data(state.block())
            .update_acc(state.coord())
    }
    pub fn goto(state: &State) -> Self {
        Self::new(state).update_coord(state.acc())
    }
    pub fn jump(state: &State) -> Self {
        Self::new(state).update_block(state.data())
    }
    pub fn load(state: &State) -> Self {
        let next = state.page()[state.coord()];
        Self::new(state).update_data(next)
    }
    pub fn store(state: &State) -> Self {
        let mut next = *state.page();
        next[state.coord()] = state.data();
        Self::new(state).update_page(next)
    }
    pub fn argc(state: &State) -> Self {
        let len = u8::try_from(std::env::args().len());
        Self::new(state)
            .update_acc(len.unwrap_or(u8::MAX))
            .update_error(len.is_err())
    }
    pub fn argv(state: &State) -> Self {
        if let Some(arg) = std::env::args().nth(state.acc().into()) {
            let mut next = *state.page();
            let len = next.write(arg.as_bytes().iter());
            Self::new(state).update_acc(len).update_page(next)
        } else {
            Self::new(state).update_error(true)
        }
    }
    pub fn esc(state: &State, key: char) -> Self {
        match u8::try_from(key) {
            Ok(data) => Self::new(state).update_data(data),
            Err(_) => Self::new(state),
        }
    }
}

fn rot(val: u8, forward: bool) -> u8 {
    let left = if forward { 1 } else { u8::BITS - 1 };
    let right = u8::BITS - left;
    (val << left) | (val >> right)
}
fn forward(state: &State, shift: u8) -> u8 {
    let (next, _) = state.coord().overflowing_add(shift);
    next
}
fn backward(state: &State, shift: u8) -> u8 {
    let (next, _) = state.coord().overflowing_sub(shift);
    next
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
