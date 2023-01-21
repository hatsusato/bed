use inst::Inst;
use util::Block;

pub struct State {
    memory: Block<u8>,
    coord: u8,
}
impl State {
    pub fn new() -> Self {
        Self {
            memory: Block::new(0),
            coord: 0,
        }
    }
    pub fn exec(&mut self, inst: Inst) {
        use Inst::*;
        match inst {
            Nop => (),
            Left => self.coord -= 1,
            Down => self.coord += 16,
            Up => self.coord -= 16,
            Right => self.coord += 1,
        }
    }
    pub fn push(&mut self, key: char) {
        self.memory[self.coord] = key as u8;
    }
    pub fn block(&self) -> &Block<u8> {
        return &self.memory;
    }
}
