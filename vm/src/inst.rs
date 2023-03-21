pub type Name = Vec<u8>;
pub type Seq = Vec<Inst>;

#[derive(Clone, PartialEq, Debug)]
pub enum Inst {
    Direct(u8),
    Insert(u8),
    Swap,
    Zero,
    Delete,
    Start,
    Origin,
    High,
    Low,
    Pos,
    Page,
    Goto,
    Jump,
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
    Put,
    Get,
    Save,
    Restore,
    Eval,
    Quote(Name),
    Call(Name),
    Func(Name, Seq),
    Macro(u8, Seq),
    Exec(u8),
    Repeat(u8),
    Nop,
    Skip,
}
impl Inst {
    #[allow(clippy::match_same_arms)]
    pub fn new(key: u8) -> Self {
        match key as char {
            '!' => Inst::Neg,
            '"' => unreachable!(),
            '#' => unreachable!(),
            '$' => unreachable!(),
            '%' => Inst::Nop,
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
    fn translate_hex_digit(key: u8) -> Inst {
        const ZERO: u8 = b'0';
        const NINE: u8 = b'9';
        const A: u8 = b'a';
        const F: u8 = b'f';
        let digit = match key {
            ZERO..=NINE => key - ZERO,
            A..=F => key - A + 0xA,
            _ => unreachable!(),
        };
        Inst::Insert(digit)
    }
    fn translate_lowercase(key: u8) -> Inst {
        Inst::new(key.to_ascii_lowercase())
    }
}
