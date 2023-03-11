use crate::{Inst, Regs};
use screen::Screen;
use std::io;
use util::{Block, BLOCK_SIDE};

#[derive(Default)]
pub struct State {
    bank: Regs,
    memory: Block<Block<u8>>,
}
impl State {
    pub fn issue(&mut self, inst: Inst) {
        let bank = &mut self.bank;
        match inst {
            Inst::Imm(data) => bank.imm(data),
            Inst::Ins(digit) => bank.ins(digit),
            Inst::Swap => bank.swap(),
            Inst::High => bank.high(),
            Inst::Low => bank.low(),
            Inst::Zero => bank.zero(),
            Inst::Origin => bank.origin(),
            Inst::Start => bank.start(),
            Inst::Goto => bank.goto(),
            Inst::Jump => bank.jump(),
            Inst::Pos => bank.pos(),
            Inst::Page => bank.page(),
            Inst::Left => bank.left(),
            Inst::Right => bank.right(),
            Inst::Up => bank.up(),
            Inst::Down => bank.down(),
            Inst::Inc => bank.inc(),
            Inst::Dec => bank.dec(),
            Inst::Add => bank.add(),
            Inst::Sub => bank.sub(),
            Inst::Mul => bank.mul(),
            Inst::Div => bank.div(),
            Inst::Clear => bank.clear(),
            Inst::Raise => bank.raise(),
            Inst::Neg => bank.neg(),
            Inst::Bool => bank.bool(),
            Inst::Eq => bank.eq(),
            Inst::Lt => bank.lt(),
            Inst::Gt => bank.gt(),
            Inst::Not => bank.not(),
            Inst::And => bank.and(),
            Inst::Or => bank.or(),
            Inst::Xor => bank.xor(),
            Inst::Shl => bank.shl(),
            Inst::Shr => bank.shr(),
            Inst::Rotl => bank.rotl(),
            Inst::Rotr => bank.rotr(),
            Inst::Load => self.load(),
            Inst::Store => self.store(),
            Inst::Delete => self.delete(),
            Inst::Put => self.put(),
            Inst::Get => self.get(),
            Inst::Save => self.save(),
            Inst::Restore => self.restore(),
            Inst::Quote(input) => self.quote(&input),
            Inst::Call(_)
            | Inst::Define(_, _)
            | Inst::Macro(_, _)
            | Inst::Eval
            | Inst::Meta
            | Inst::Nop => (),
        }
    }
    pub fn run(&mut self, insts: &[Inst]) {
        insts.iter().for_each(|i| self.issue(i.clone()));
    }
    pub fn repeat(&mut self, insts: &[Inst]) {
        let count = self.bank.acc;
        for i in 0..count {
            self.bank.acc = i;
            self.run(insts);
        }
        self.bank.acc = count;
    }
    fn load(&mut self) {
        self.bank.data = self.current()[0];
    }
    fn store(&mut self) {
        self.current_mut()[0] = self.bank.data;
    }
    fn delete(&mut self) {
        self.current_mut()[0] = 0;
    }
    fn put(&mut self) {
        use io::Write;
        let src = &self.current()[0..1];
        let result = io::stdout().write(src);
        self.bank.set_error(result.is_err());
    }
    fn get(&mut self) {
        use io::Read;
        let dst = &mut self.current_mut()[0..1];
        let result = io::stdin().read(dst);
        self.bank.set_error(result.is_err());
    }
    fn save(&mut self) {
        let buf = &mut [0; 4];
        self.bank.save(buf);
        copy(self.current_mut(), buf);
    }
    fn restore(&mut self) {
        let buf = &mut [0; 4];
        copy(buf, self.current());
        self.bank.restore(buf);
    }
    fn quote(&mut self, input: &str) {
        copy(self.current_mut(), input.as_bytes());
    }
    fn page(&self) -> &[u8] {
        self.memory[self.bank.block].iter().as_slice()
    }
    fn page_mut(&mut self) -> &mut [u8] {
        self.memory[self.bank.block].iter_mut().into_slice()
    }
    fn current(&self) -> &[u8] {
        let coord = usize::from(self.bank.coord);
        &self.page()[coord..]
    }
    fn current_mut(&mut self) -> &mut [u8] {
        let coord = usize::from(self.bank.coord);
        &mut self.page_mut()[coord..]
    }
    pub fn print(&self, key: char) {
        self.bank.print(key);
        for y in 0..BLOCK_SIDE {
            for x in 0..BLOCK_SIDE {
                Self::move_cell(x, y);
                self.print_cell(x, y);
            }
        }
    }
    fn print_cell(&self, x: u8, y: u8) {
        let index = x + y * BLOCK_SIDE;
        let val = self.page()[usize::from(index)];
        Screen::print_display(util::as_hex(val), self.bank.coord == index);
    }
    fn move_cell(x: u8, y: u8) {
        const CELL_WIDTH: u16 = 3;
        const LINE_OFFSET: u16 = 1;
        let x = u16::from(x) * CELL_WIDTH;
        let y = u16::from(y) + LINE_OFFSET;
        Screen::move_cursor(x, y);
    }
}

fn copy(dst: &mut [u8], src: &[u8]) {
    dst.iter_mut().zip(src).for_each(|(dst, src)| *dst = *src);
}
