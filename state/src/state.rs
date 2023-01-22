mod mem;
mod ptr;
mod reg;

use inst::Inst;
use util::Block;

pub struct State {
    memory: Block<u8>,
    _block: u8,
    coord: u8,
    data: u8,
    acc: u8,
    error: bool,
}
impl State {
    pub fn new() -> Self {
        Self {
            memory: Block::new(0),
            _block: 0,
            coord: 0,
            data: 0,
            acc: 0,
            error: false,
        }
    }
    pub fn exec(&mut self, inst: Inst) {
        use Inst::*;
        match inst {
            Imm(digit) => self.imm(digit),
            Add => self.add(),
            Sub => self.sub(),
            Mul => self.mul(),
            Div => self.div(),
            Left => self.left(),
            Down => self.down(),
            Up => self.up(),
            Right => self.right(),
            Load => self.load(),
            Store => self.store(),
            Nop => (),
        }
    }
    pub fn push(&mut self, key: char) {
        self.memory[self.coord] = key as u8;
    }
    pub fn block(&self) -> &Block<u8> {
        return &self.memory;
    }
    fn current(&mut self) -> &mut u8 {
        &mut self.memory[self.coord]
    }
    fn raise(&mut self) {
        self.error = true;
    }
}
