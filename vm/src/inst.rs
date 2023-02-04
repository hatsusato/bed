#[derive(Clone, Copy)]
pub enum Inst {
    Imm(u8),
    Ins(u8),
    Swap,
    Hi,
    Lo,
    Goto,
    Jump,
    Pos,
    Left,
    Right,
    Up,
    Down,
    Inc,
    Dec,
    Add,
    Sub,
    Mul,
    Div,
    Clear,
    Neg,
    Bool,
    Eq,
    Lt,
    Gt,
    Not,
    And,
    Or,
    Xor,
    Shl,
    Shr,
    Rotl,
    Rotr,
    Load,
    Store,
    Push,
    Pop,
    Argc,
    Argv,
    Ctrl,
    Nop,
}
impl Inst {
    #[allow(clippy::match_same_arms)]
    pub fn new(key: char) -> Self {
        match key {
            '\n' => Inst::Ctrl,
            '!' => Inst::Neg,
            '"' => Inst::Ctrl,
            '#' => Inst::Ctrl,
            '$' => Inst::Argv,
            '%' => Inst::Argc,
            '&' => Inst::And,
            '\'' => Inst::Ctrl,
            '(' => Inst::Hi,
            ')' => Inst::Lo,
            '*' => Inst::Mul,
            '+' => Inst::Add,
            ',' => Inst::Nop,
            '-' => Inst::Sub,
            '.' => Inst::Nop,
            '/' => Inst::Div,
            '0'..='9' => Inst::Ins(translate_hex_digit(key)),
            ':' => Inst::Nop,
            ';' => Inst::Nop,
            '<' => Inst::Lt,
            '=' => Inst::Eq,
            '>' => Inst::Gt,
            '?' => Inst::Bool,
            '@' => Inst::Nop,
            'A'..='Z' => Inst::new(key.to_ascii_lowercase()),
            '[' => Inst::Shl,
            '\\' => Inst::Nop,
            ']' => Inst::Shr,
            '^' => Inst::Xor,
            '_' => Inst::Clear,
            '`' => Inst::Nop,
            'a'..='f' => Inst::Ins(translate_hex_digit(key)),
            'g' => Inst::Goto,
            'h' => Inst::Left,
            'i' => Inst::Load,
            'j' => Inst::Down,
            'k' => Inst::Up,
            'l' => Inst::Right,
            'm' => Inst::Dec,
            'n' => Inst::Inc,
            'o' => Inst::Store,
            'p' => Inst::Pop,
            'q' => Inst::Nop,
            'r' => Inst::Nop,
            's' => Inst::Swap,
            't' => Inst::Jump,
            'u' => Inst::Push,
            'v' => Inst::Pos,
            'w' => Inst::Nop,
            'x' => Inst::Nop,
            'y' => Inst::Nop,
            'z' => Inst::Nop,
            '{' => Inst::Rotl,
            '|' => Inst::Or,
            '}' => Inst::Rotr,
            '~' => Inst::Not,
            _ => Inst::Nop,
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
