use crate::mem::Page;
use crate::{Inst, Registers};
use screen::Screen;
use util::{Block, BLOCK_SIDE};

#[derive(Default)]
pub struct State {
    regs: Registers,
    memory: Block<Block<u8>>,
}
impl State {
    pub fn issue(&mut self, inst: Inst) {
        let regs = &mut self.regs;
        let mut page = Page::new(regs, &mut self.memory);
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
            Inst::Load => page.load(),
            Inst::Store => page.store(),
            Inst::Delete => page.delete(),
            Inst::Put => page.put(),
            Inst::Get => page.get(),
            Inst::Save => page.save(),
            Inst::Restore => page.restore(),
            Inst::Quote(input) => page.quote(&input),
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
        let val = self.memory[self.regs.block][index];
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
