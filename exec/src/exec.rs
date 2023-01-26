use crate::cmd::Command;
use inst::Inst;
use state::State;

pub struct Exec {}
impl Exec {
    pub fn new() -> Self {
        Self {}
    }
    pub fn exec(key: char, state: &mut State) {
        use Inst::Nop;
        let cmd: Inst = match key {
            '\n' => Nop,
            '!' => Command::neg(state).inst,
            '"' => Nop,
            '#' => Nop,
            '$' => Command::argv(state).inst,
            '%' => Nop,
            '&' => Command::and(state).inst,
            '\'' => Nop,
            '(' => Command::hi(state).inst,
            ')' => Command::lo(state).inst,
            '*' => Command::mul(state).inst,
            '+' => Command::add(state).inst,
            ',' => Nop,
            '-' => Command::sub(state).inst,
            '.' => Nop,
            '/' => Command::div(state).inst,
            '0'..='9' => Command::imm(state, translate_hex_digit(key)).inst,
            ':' => Nop,
            ';' => Nop,
            '<' => Command::lt(state).inst,
            '=' => Command::eq(state).inst,
            '>' => Command::gt(state).inst,
            '?' => Command::bool(state).inst,
            '@' => Command::argc(state).inst,
            'A'..='Z' => return Self::exec(key.to_ascii_lowercase(), state),
            '[' => Command::shl(state).inst,
            '\\' => Nop,
            ']' => Command::shr(state).inst,
            '^' => Command::xor(state).inst,
            '_' => Nop,
            '`' => Nop,
            'a'..='f' => Command::imm(state, translate_hex_digit(key)).inst,
            'g' => Command::goto(state).inst,
            'h' => Command::left(state).inst,
            'i' => Command::load(state).inst,
            'j' => Command::down(state).inst,
            'k' => Command::up(state).inst,
            'l' => Command::right(state).inst,
            'm' => Command::dec(state).inst,
            'n' => Command::inc(state).inst,
            'o' => Command::store(state).inst,
            'p' => Nop,
            'q' => Nop,
            'r' => Nop,
            's' => Command::swap(state).inst,
            't' => Command::jump(state).inst,
            'u' => Nop,
            'v' => Command::pos(state).inst,
            'w' => Nop,
            'x' => Nop,
            'y' => Nop,
            'z' => Nop,
            '{' => Command::rotl(state).inst,
            '|' => Command::or(state).inst,
            '}' => Command::rotr(state).inst,
            '~' => Command::not(state).inst,
            _ => Nop,
        };
        state.exec_cmd(cmd);
    }
}

pub struct ExecCmd {}

fn translate_hex_digit(key: char) -> u8 {
    const ZERO: u8 = '0' as u8;
    const A: u8 = 'a' as u8;
    match key {
        '0'..='9' => key as u8 - ZERO + 0,
        'a'..='f' => key as u8 - A + 0xA,
        _ => unreachable!(),
    }
}
