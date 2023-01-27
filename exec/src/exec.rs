use crate::cmd::Command;
use state::State;

pub struct Exec {}
impl Exec {
    pub fn new() -> Self {
        Self {}
    }
    pub fn exec(key: char, state: &mut State) {
        let cmd = match key {
            '\n' => return,
            '!' => Command::neg(state),
            '"' => return,
            '#' => return,
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
            'A'..='Z' => return Self::exec(key.to_ascii_lowercase(), state),
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
        state.restore_bank(&cmd.next);
    }
}

fn translate_hex_digit(key: char) -> u8 {
    const ZERO: u8 = '0' as u8;
    const A: u8 = 'a' as u8;
    match key {
        '0'..='9' => key as u8 - ZERO + 0,
        'a'..='f' => key as u8 - A + 0xA,
        _ => unreachable!(),
    }
}
