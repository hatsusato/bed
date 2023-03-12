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
    Call(String),
    Define(String, Vec<Inst>),
    Macro(char, Vec<Inst>),
    Exec(char),
    Repeat(char),
    Nop,
    Skip,
}
impl Inst {
    #[allow(clippy::match_same_arms, clippy::must_use_candidate)]
    pub fn new(key: char) -> Self {
        match key {
            '!' => Inst::Neg,
            '"' => unreachable!(),
            '#' => unreachable!(),
            '$' => Inst::Nop,
            '%' => unreachable!(),
            '&' => Inst::And,
            '\'' => unreachable!(),
            '(' => Inst::Rotl,
            ')' => Inst::Rotr,
            '*' => Inst::Mul,
            '+' => Inst::Add,
            ',' => Inst::Get,
            '-' => Inst::Sub,
            '.' => Inst::Put,
            '/' => Inst::Div,
            '0'..='9' => Self::translate_hex_digit(key),
            ':' => unreachable!(),
            ';' => unreachable!(),
            '<' => Inst::Lt,
            '=' => Inst::Eq,
            '>' => Inst::Gt,
            '?' => Inst::Bool,
            '@' => unreachable!(),
            'A'..='Z' => Self::translate_lowercase(key),
            '[' => Inst::Inc,
            '\\' => Inst::Raise,
            ']' => Inst::Dec,
            '^' => Inst::Xor,
            '_' => Inst::Clear,
            '`' => Inst::Eval,
            'a'..='f' => Self::translate_hex_digit(key),
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
            'q' => unreachable!(),
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
    fn translate_hex_digit(key: char) -> Inst {
        const ZERO: u8 = b'0';
        const A: u8 = b'a';
        let digit = match key {
            '0'..='9' => key as u8 - ZERO,
            'a'..='f' => key as u8 - A + 0xA,
            _ => unreachable!(),
        };
        Inst::Ins(digit)
    }
    fn translate_lowercase(key: char) -> Inst {
        Inst::new(key.to_ascii_lowercase())
    }
}
