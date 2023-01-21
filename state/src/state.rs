use inst::Inst;
use util::Block;

pub struct State {
    memory: Block<u8>,
    pub block: u8,
    pub coord: u8,
    pub data: u8,
    pub acc: u8,
}
impl State {
    pub fn new() -> Self {
        Self {
            memory: Block::new(0),
            block: 0,
            coord: 0,
            data: 0,
            acc: 0,
        }
    }
    pub fn exec(&mut self, inst: Inst) {
        use Inst::*;
        match inst {
            Nop => (),
            Left => self.left(),
            Down => self.down(),
            Up => self.up(),
            Right => self.right(),
            Add => self.add(),
            Sub => self.sub(),
            Mul => self.mul(),
            Div => self.div(),
        }
    }
    pub fn push(&mut self, key: char) {
        self.memory[self.coord] = key as u8;
    }
    pub fn block(&self) -> &Block<u8> {
        return &self.memory;
    }
}
