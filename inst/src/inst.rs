pub enum Inst {
    Imm(u8),
    Swap,
    Inc,
    Dec,
    Add,
    Sub,
    Mul,
    Div,
    Equal,
    Less,
    Greater,
    Left,
    Down,
    Up,
    Right,
    Pos,
    Goto,
    Load,
    Store,
    Nop,
}
impl Inst {
    pub fn new(key: char) -> Self {
        use Inst::*;
        match key {
            '\n' => Nop,
            '!' => Nop,
            '"' => Nop,
            '#' => Nop,
            '$' => Nop,
            '%' => Nop,
            '&' => Nop,
            '\'' => Nop,
            '(' => Nop,
            ')' => Nop,
            '*' => Mul,
            '+' => Add,
            ',' => Nop,
            '-' => Sub,
            '.' => Nop,
            '/' => Div,
            '0'..='9' => Imm(translate_hex_digit(key)),
            ':' => Nop,
            ';' => Nop,
            '<' => Less,
            '=' => Equal,
            '>' => Greater,
            '?' => Nop,
            '@' => Nop,
            'A'..='Z' => Self::new(key.to_ascii_lowercase()),
            '[' => Nop,
            '\\' => Nop,
            ']' => Nop,
            '^' => Nop,
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
            'p' => Nop,
            'q' => Nop,
            'r' => Nop,
            's' => Swap,
            't' => Nop,
            'u' => Nop,
            'v' => Nop,
            'w' => Pos,
            'x' => Nop,
            'y' => Nop,
            'z' => Nop,
            '{' => Nop,
            '|' => Nop,
            '}' => Nop,
            '~' => Nop,
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
