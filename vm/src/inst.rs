#[derive(Clone, Copy)]
pub enum Inst {
    Immediate(u8),
    Insert(u8),
    Swap,
    High,
    Low,
    Zero,
    Origin,
    Start,
    Goto,
    Jump,
    Position,
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
    Read,
    Write,
    Delete,
    Put,
    Get,
    Save,
    Restore,
    Argc,
    Argv,
    Eval,
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
            '$' => Inst::Ctrl,
            '%' => Inst::Ctrl,
            '&' => Inst::And,
            '\'' => Inst::Ctrl,
            '(' => Inst::Rotl,
            ')' => Inst::Rotr,
            '*' => Inst::Mul,
            '+' => Inst::Add,
            ',' => Inst::Get,
            '-' => Inst::Sub,
            '.' => Inst::Put,
            '/' => Inst::Div,
            '0'..='9' => Inst::Insert(translate_hex_digit(key)),
            ':' => Inst::Ctrl,
            ';' => Inst::Ctrl,
            '<' => Inst::Lt,
            '=' => Inst::Eq,
            '>' => Inst::Gt,
            '?' => Inst::Bool,
            '@' => Inst::Ctrl,
            'A'..='Z' => Inst::new(key.to_ascii_lowercase()),
            '[' => Inst::Inc,
            '\\' => Inst::Raise,
            ']' => Inst::Dec,
            '^' => Inst::Xor,
            '_' => Inst::Clear,
            '`' => Inst::Eval,
            'a'..='f' => Inst::Insert(translate_hex_digit(key)),
            'g' => Inst::Origin,
            'h' => Inst::Left,
            'i' => Inst::High,
            'j' => Inst::Down,
            'k' => Inst::Up,
            'l' => Inst::Right,
            'm' => Inst::Position,
            'n' => Inst::Page,
            'o' => Inst::Low,
            'p' => Inst::Goto,
            'q' => Inst::Ctrl,
            'r' => Inst::Read,
            's' => Inst::Start,
            't' => Inst::Restore,
            'u' => Inst::Jump,
            'v' => Inst::Save,
            'w' => Inst::Write,
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
