pub enum Inst {
    Imm,
    Swap,
    Hi,
    Lo,
    Inc,
    Dec,
    Add,
    Sub,
    Mul,
    Div,
    DivErr,
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
    Left,
    Down,
    Up,
    Right,
    Pos,
    Goto,
    Jump,
    Load,
    Store,
    Argc,
    Argv,
    NoArg,
    Nop,
}
