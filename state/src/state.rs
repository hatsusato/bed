mod mem;
mod ptr;
mod queue;
mod reg;

use inst::Inst;
use util::Block;

pub struct State {
    acc: u8,
    block: u8,
    coord: u8,
    data: u8,
    error: bool,
    memory: Block<Block<u8>>,
    queue: Vec<u8>,
}
impl State {
    pub fn new() -> Self {
        Self {
            acc: 0,
            block: 0,
            coord: 0,
            data: 0,
            error: false,
            memory: Block::new(Block::new(0)),
            queue: Vec::new(),
        }
    }
    pub fn exec(&mut self, inst: Inst) {
        use Inst::*;
        match inst {
            Imm(digit) => self.imm(digit),
            Swap => self.swap(),
            Hi => self.hi(),
            Lo => self.lo(),
            Inc => self.inc(),
            Dec => self.dec(),
            Add => self.add(),
            Sub => self.sub(),
            Mul => self.mul(),
            Div => self.div(),
            Eq => self.eq(),
            Le => self.le(),
            Gr => self.gr(),
            Err => self.err(),
            Bool => self.bool(),
            Not => self.not(),
            And => self.and(),
            Or => self.or(),
            Xor => self.xor(),
            Shl => self.shl(),
            Shr => self.shr(),
            Rotl => self.rotl(),
            Rotr => self.rotr(),
            Left => self.left(),
            Down => self.down(),
            Up => self.up(),
            Right => self.right(),
            Pos => self.pos(),
            Goto => self.goto(),
            Load => self.load(),
            Store => self.store(),
            Push => self.push(),
            Pop => self.pop(),
            Nop => (),
        }
    }
    pub fn page(&self) -> &Block<u8> {
        &self.memory[self.block]
    }
    pub fn data(&self) -> u8 {
        self.data
    }
    pub fn acc(&self) -> u8 {
        self.acc
    }
    pub fn block(&self) -> u8 {
        self.block
    }
    pub fn coord(&self) -> u8 {
        self.coord
    }
    pub fn error(&self) -> bool {
        self.error
    }
    fn set_reg(&mut self, hi: u8, lo: u8) {
        (self.data, self.acc) = (hi, lo);
    }
    fn current(&mut self) -> &mut u8 {
        &mut self.memory[self.block][self.coord]
    }
    fn raise(&mut self) {
        self.error = true;
    }
}
