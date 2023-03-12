use crate::mem::Page;
use crate::{Inst, Registers};
use screen::Screen;
use std::io;
use util::{Block, BLOCK_SIDE};

#[derive(Default)]
pub struct State {
    regs: Registers,
    memory: Block<Page>,
}
impl State {
    pub fn issue(&mut self, inst: Inst) {
        let regs = &mut self.regs;
        match inst {
            Inst::Imm(data) => regs.imm(data),
            Inst::Ins(digit) => regs.ins(digit),
            Inst::Swap => regs.swap(),
            Inst::High => regs.high(),
            Inst::Low => regs.low(),
            Inst::Zero => regs.zero(),
            Inst::Origin => regs.origin(),
            Inst::Start => regs.start(),
            Inst::Goto => regs.goto(),
            Inst::Jump => regs.jump(),
            Inst::Pos => regs.pos(),
            Inst::Page => regs.page(),
            Inst::Left => regs.left(),
            Inst::Right => regs.right(),
            Inst::Up => regs.up(),
            Inst::Down => regs.down(),
            Inst::Inc => regs.inc(),
            Inst::Dec => regs.dec(),
            Inst::Add => regs.add(),
            Inst::Sub => regs.sub(),
            Inst::Mul => regs.mul(),
            Inst::Div => regs.div(),
            Inst::Clear => regs.clear(),
            Inst::Raise => regs.raise(),
            Inst::Neg => regs.neg(),
            Inst::Bool => regs.bool(),
            Inst::Eq => regs.eq(),
            Inst::Lt => regs.lt(),
            Inst::Gt => regs.gt(),
            Inst::Not => regs.not(),
            Inst::And => regs.and(),
            Inst::Or => regs.or(),
            Inst::Xor => regs.xor(),
            Inst::Shl => regs.shl(),
            Inst::Shr => regs.shr(),
            Inst::Rotl => regs.rotl(),
            Inst::Rotr => regs.rotr(),
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
            | Inst::Exec(_)
            | Inst::Repeat(_)
            | Inst::Eval
            | Inst::Nop
            | Inst::Skip => (),
        }
    }
    pub fn run(&mut self, insts: &[Inst]) {
        insts.iter().for_each(|i| self.issue(i.clone()));
    }
    pub fn repeat(&mut self, insts: &[Inst]) {
        let count = self.regs.acc;
        for i in 0..count {
            self.regs.acc = i;
            self.run(insts);
        }
        self.regs.acc = count;
    }
    fn load(&mut self) {
        self.regs.data = self.current()[0];
    }
    fn store(&mut self) {
        self.current_mut()[0] = self.regs.data;
    }
    fn delete(&mut self) {
        self.current_mut()[0] = 0;
    }
    fn put(&mut self) {
        use io::Write;
        let src = &self.current()[0..1];
        let result = io::stdout().write(src);
        self.regs.set_error(result.is_err());
    }
    fn get(&mut self) {
        use io::Read;
        let dst = &mut self.current_mut()[0..1];
        let result = io::stdin().read(dst);
        self.regs.set_error(result.is_err());
    }
    fn save(&mut self) {
        let buf = &mut [0; 4];
        self.regs.save(buf);
        copy(self.current_mut(), buf);
    }
    fn restore(&mut self) {
        let buf = &mut [0; 4];
        copy(buf, self.current());
        self.regs.restore(buf);
    }
    fn quote(&mut self, input: &str) {
        copy(self.current_mut(), input.as_bytes());
    }
    fn page(&self) -> &[u8] {
        self.memory[self.regs.block].get().iter().as_slice()
    }
    fn page_mut(&mut self) -> &mut [u8] {
        self.memory[self.regs.block]
            .get_mut()
            .iter_mut()
            .into_slice()
    }
    fn current(&self) -> &[u8] {
        let coord = usize::from(self.regs.coord);
        &self.page()[coord..]
    }
    fn current_mut(&mut self) -> &mut [u8] {
        let coord = usize::from(self.regs.coord);
        &mut self.page_mut()[coord..]
    }
    pub fn print(&self, key: char) {
        self.regs.print(key);
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
        Screen::print_display(util::as_hex(val), self.regs.coord == index);
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
