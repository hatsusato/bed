use inst::Command;
use state::State;

pub struct Exec {}
impl Exec {
    pub fn new() -> Self {
        Self {}
    }
    pub fn exec(key: char, state: &mut State) {
        use Command::Nop;
        let cmd: Command = match key {
            '\n' => Nop,
            '!' => ExecCmd::is_err(state),
            '"' => Nop,
            '#' => Nop,
            '$' => ExecCmd::argv(state),
            '%' => Nop,
            '&' => ExecCmd::and(state),
            '\'' => Nop,
            '(' => ExecCmd::hi(state),
            ')' => ExecCmd::lo(state),
            '*' => ExecCmd::mul(state),
            '+' => ExecCmd::add(state),
            ',' => Nop,
            '-' => ExecCmd::sub(state),
            '.' => Nop,
            '/' => ExecCmd::div(state),
            '0'..='9' => ExecCmd::imm(state, translate_hex_digit(key)),
            ':' => Nop,
            ';' => Nop,
            '<' => ExecCmd::lt(state),
            '=' => ExecCmd::eq(state),
            '>' => ExecCmd::gt(state),
            '?' => ExecCmd::bool(state),
            '@' => ExecCmd::argc(state),
            'A'..='Z' => return Self::exec(key.to_ascii_lowercase(), state),
            '[' => ExecCmd::shl(state),
            '\\' => Nop,
            ']' => ExecCmd::shr(state),
            '^' => ExecCmd::xor(state),
            '_' => Nop,
            '`' => Nop,
            'a'..='f' => ExecCmd::imm(state, translate_hex_digit(key)),
            'g' => ExecCmd::goto(state),
            'h' => ExecCmd::left(state),
            'i' => ExecCmd::load(state),
            'j' => ExecCmd::down(state),
            'k' => ExecCmd::up(state),
            'l' => ExecCmd::right(state),
            'm' => ExecCmd::dec(state),
            'n' => ExecCmd::inc(state),
            'o' => ExecCmd::store(state),
            'p' => Nop,
            'q' => Nop,
            'r' => Nop,
            's' => ExecCmd::swap(state),
            't' => ExecCmd::jump(state),
            'u' => Nop,
            'v' => ExecCmd::pos(state),
            'w' => Nop,
            'x' => Nop,
            'y' => Nop,
            'z' => Nop,
            '{' => ExecCmd::rotl(state),
            '|' => ExecCmd::or(state),
            '}' => ExecCmd::rotr(state),
            '~' => ExecCmd::not(state),
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
