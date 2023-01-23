pub enum Inst {
    Imm(u8),
    Swap,
    Hi,
    Lo,
    Inc,
    Dec,
    Add,
    Sub,
    Mul,
    Div,
    Err,
    Bool,
    Eq,
    Le,
    Gr,
    Not,
    And,
    Or,
    Xor,
    Shl,
    Shr,
    Rotl,
    Rotr,
    Left,
    Down,
    Up,
    Right,
    Pos,
    Goto,
    Load,
    Store,
    Push,
    Pop,
    Len,
    Argc,
    Argv,
    Nop,
}
impl Inst {
    pub fn new(key: char) -> Self {
        use Inst::*;
        match key {
            '\n' => Nop,
            '!' => Err,
            '"' => Nop,
            '#' => Nop,
            '$' => Argv,
            '%' => Nop,
            '&' => And,
            '\'' => Nop,
            '(' => Hi,
            ')' => Lo,
            '*' => Mul,
            '+' => Add,
            ',' => Nop,
            '-' => Sub,
            '.' => Nop,
            '/' => Div,
            '0'..='9' => Imm(translate_hex_digit(key)),
            ':' => Nop,
            ';' => Nop,
            '<' => Le,
            '=' => Eq,
            '>' => Gr,
            '?' => Bool,
            '@' => Argc,
            'A'..='Z' => Self::new(key.to_ascii_lowercase()),
            '[' => Shl,
            '\\' => Nop,
            ']' => Shr,
            '^' => Xor,
            '_' => Nop,
            '`' => Nop,
            'a'..='f' => Imm(translate_hex_digit(key)),
            'g' => Goto,
            'h' => Left,
            'i' => Load,
            'j' => Down,
            'k' => Up,
            'l' => Right,
            'm' => Dec,
            'n' => Inc,
            'o' => Store,
            'p' => Pop,
            'q' => Len,
            'r' => Nop,
            's' => Swap,
            't' => Nop,
            'u' => Push,
            'v' => Pos,
            'w' => Nop,
            'x' => Nop,
            'y' => Nop,
            'z' => Nop,
            '{' => Rotl,
            '|' => Or,
            '}' => Rotr,
            '~' => Not,
            _ => Nop,
        }
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

pub enum Command {
    Imm(u8, u8),
    Swap(u16, u16),
    Hi(u8, u8),
    Lo(u8, u8),
    Inc(u16, u16),
    Dec(u16, u16),
    Add(u16, u16),
    Sub(u16, u16),
    Mul(u16, u16),
    Div(u16, u16),
    DivErr(bool, bool),
    IsErr(u8, u8),
    Bool(u8, u8),
    Eq(u8, u8),
    Lt(u8, u8),
    Gt(u8, u8),
    Not(u8, u8),
    And(u8, u8),
    Or(u8, u8),
    Xor(u8, u8),
    Shl(u8, u8),
    Shr(u8, u8),
    Rotl(u8, u8),
    Rotr(u8, u8),
}
