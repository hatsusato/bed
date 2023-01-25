use crate::State;
use inst::{Command, Inst};
use std::collections::VecDeque;
use util::Block;

impl State {
    pub fn new() -> Self {
        Self {
            acc: 0,
            block: 0,
            coord: 0,
            data: 0,
            error: false,
            memory: Block::new(Block::new(0)),
            queue: VecDeque::new(),
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
            Len => self.len(),
            Argc => self.argc(),
            Argv => self.argv(),
            Nop => (),
        }
    }
    pub fn exec_cmd(&mut self, cmd: Command) {
        use Command::*;
        match cmd {
            Imm(_, next) => self.data = next,
            Swap(_, next) => (self.data, self.acc) = next,
            Hi(_, next) => self.data = next,
            Lo(_, next) => self.acc = next,
            Inc(_, next) | Dec(_, next) => self.acc = next,
            Add(_, next) | Sub(_, next) => (self.data, self.acc) = next,
            Mul(_, next) | Div(_, next) => (self.data, self.acc) = next,
            DivErr(_) => self.raise(),
            IsErr(_, next)
            | Bool(_, next)
            | Eq(_, next)
            | Lt(_, next)
            | Gt(_, next)
            | Not(_, next)
            | And(_, next)
            | Or(_, next)
            | Xor(_, next)
            | Shl(_, next)
            | Shr(_, next)
            | Rotl(_, next)
            | Rotr(_, next) => self.acc = next,
            Left(_, next) | Right(_, next) => self.coord = next,
            Down(_, next) | Up(_, next) => self.coord = next,
            Pos(_, next) => (self.data, self.acc) = next,
            Goto(_, next) => self.coord = next,
            Jump(_, next) => self.block = next,
            Load(_, next) => self.data = next,
            Store(_, next) => self.memory[self.block][self.coord] = next,
            Argc(_, next) => (self.acc, self.error) = next,
            Argv(arg) => self.queue.extend(arg),
            NoArg(_) => self.raise(),
        }
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
    pub fn page(&self) -> &Block<u8> {
        &self.memory[self.block]
    }
    pub fn queue(&self) -> &VecDeque<u8> {
        &self.queue
    }
    pub fn set_reg(&mut self, val: u16) {
        (self.data, self.acc) = (trunc(val >> u8::BITS), trunc(val));
    }
    pub fn get_reg(&self) -> u16 {
        (self.data as u16) << u8::BITS | (self.acc as u16)
    }
    pub fn raise(&mut self) {
        self.error = true;
    }
}

fn trunc(val: u16) -> u8 {
    const MASK: u16 = u8::MAX as u16;
    (val & MASK) as u8
}
