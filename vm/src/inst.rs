#[derive(Clone)]
pub enum Inst {
    Imm(u8),
    Ins(u8),
    Swap,
    High,
    Low,
    Zero,
    Origin,
    Start,
    Goto,
    Jump,
    Pos,
    Page,
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
    Raise,
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
    Delete,
    Put,
    Get,
    Save,
    Restore,
    Eval,
    Quote(String),
    Meta,
    Nop,
}
impl Inst {
    #[allow(clippy::match_same_arms)]
    pub fn new(key: char) -> Self {
        match key {
            '\n' => Inst::Meta,
            '!' => Inst::Neg,
            '"' => Inst::Meta,
            '#' => Inst::Meta,
            '$' => Inst::Nop,
            '%' => Inst::Meta,
            '&' => Inst::And,
            '\'' => Inst::Meta,
            '(' => Inst::Rotl,
            ')' => Inst::Rotr,
            '*' => Inst::Mul,
            '+' => Inst::Add,
            ',' => Inst::Get,
            '-' => Inst::Sub,
            '.' => Inst::Put,
            '/' => Inst::Div,
            '0'..='9' => Inst::Ins(translate_hex_digit(key)),
            ':' => Inst::Meta,
            ';' => Inst::Meta,
            '<' => Inst::Lt,
            '=' => Inst::Eq,
            '>' => Inst::Gt,
            '?' => Inst::Bool,
            '@' => Inst::Meta,
            'A'..='Z' => Inst::new(key.to_ascii_lowercase()),
            '[' => Inst::Inc,
            '\\' => Inst::Raise,
            ']' => Inst::Dec,
            '^' => Inst::Xor,
            '_' => Inst::Clear,
            '`' => Inst::Eval,
            'a'..='f' => Inst::Ins(translate_hex_digit(key)),
            'g' => Inst::Origin,
            'h' => Inst::Left,
            'i' => Inst::High,
            'j' => Inst::Down,
            'k' => Inst::Up,
            'l' => Inst::Right,
            'm' => Inst::Pos,
            'n' => Inst::Page,
            'o' => Inst::Low,
            'p' => Inst::Goto,
            'q' => Inst::Meta,
            'r' => Inst::Load,
            's' => Inst::Start,
            't' => Inst::Restore,
            'u' => Inst::Jump,
            'v' => Inst::Save,
            'w' => Inst::Store,
            'x' => Inst::Delete,
            'y' => Inst::Swap,
            'z' => Inst::Zero,
            '{' => Inst::Shl,
            '|' => Inst::Or,
            '}' => Inst::Shr,
            '~' => Inst::Not,
            _ => Inst::Nop,
        }
    }
    pub fn immediate(input: char) -> Self {
        if let Ok(input) = u8::try_from(input) {
            Inst::Imm(input)
        } else {
            Inst::Nop
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
