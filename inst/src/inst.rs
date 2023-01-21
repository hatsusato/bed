pub enum Inst {
    Nop,
    Left,
    Down,
    Up,
    Right,
}
impl Inst {
    pub fn new(key: char) -> Inst {
        use Inst::*;
        match key {
            'h' => Left,
            'j' => Down,
            'k' => Up,
            'l' => Right,
            _ => Nop,
        }
    }
}
