use crate::{Bank, State};
use inst::Inst;
use std::collections::VecDeque;
use util::{Block, Page};

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
    pub fn restore_bank(&mut self, bank: Bank) {
        self.acc = bank.acc;
        self.block = bank.block;
        self.coord = bank.coord;
        self.data = bank.data;
        self.error = bank.error;
    }
    pub fn exec_cmd(&mut self, cmd: Inst) {
        use Inst::*;
        match cmd {
            Imm(_, next) => self.data = next,
            Swap(_, next) => (self.data, self.acc) = next,
            Hi(_, next) => self.data = next,
            Lo(_, next) => self.acc = next,
            Inc(_, next) | Dec(_, next) => self.acc = next,
            Add(_, next) | Sub(_, next) => (self.data, self.acc) = next,
            Mul(_, next) | Div(_, next) => (self.data, self.acc) = next,
            DivErr(_) => self.raise(),
            Neg(_, next)
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
            Argv(_, next) => self.memory[self.block] = next,
            NoArg(_) => self.raise(),
            Nop => (),
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
    pub fn page(&self) -> &Page {
        &self.memory[self.block]
    }
    pub fn queue(&self) -> &VecDeque<u8> {
        &self.queue
    }
    pub fn bank(&self) -> Bank {
        Bank {
            acc: self.acc,
            block: self.block,
            coord: self.coord,
            data: self.data,
            error: self.error,
        }
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
