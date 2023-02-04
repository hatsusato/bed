use crate::inst::Inst;
use crate::{Bank, State};
use util::Page;

pub struct Command {
    pub next: Bank,
    pub page: Option<Page>,
}
impl Command {
    pub fn new(inst: Inst, state: &State) -> Self {
        let mut this = Self {
            next: state.bank(),
            page: None,
        };
        this.update_inst(inst, state);
        this
    }
    fn update_inst(&mut self, inst: Inst, state: &State) {
        match inst {
            Inst::Imm(data) => self.next.imm(data),
            Inst::Ins(digit) => self.next.ins(digit),
            Inst::Swap => self.next.swap(),
            Inst::Hi => self.next.hi(),
            Inst::Lo => self.next.lo(),
            Inst::Goto => self.next.goto(),
            Inst::Jump => self.next.jump(),
            Inst::Pos => self.next.pos(),
            Inst::Left => self.next.left(),
            Inst::Right => self.next.right(),
            Inst::Up => self.next.up(),
            Inst::Down => self.next.down(),
            Inst::Inc => self.next.inc(),
            Inst::Dec => self.next.dec(),
            Inst::Add => self.next.add(),
            Inst::Sub => self.next.sub(),
            Inst::Mul => self.next.mul(),
            Inst::Div => self.next.div(),
            Inst::Clear => self.next.clear(),
            Inst::Neg => self.next.neg(),
            Inst::Bool => self.next.bool(),
            Inst::Eq => self.next.eq(),
            Inst::Lt => self.next.lt(),
            Inst::Gt => self.next.gt(),
            Inst::Not => self.next.not(),
            Inst::And => self.next.and(),
            Inst::Or => self.next.or(),
            Inst::Xor => self.next.xor(),
            Inst::Shl => self.next.shl(),
            Inst::Shr => self.next.shr(),
            Inst::Rotl => self.next.rotl(),
            Inst::Rotr => self.next.rotr(),
            Inst::Load => self.next.load(state.page()),
            Inst::Store => self.page = self.next.store(*state.page()),
            Inst::Push => self.page = self.next.push(*state.page()),
            Inst::Pop => self.next.pop(state.page()),
            Inst::Argc => self.next.argc(),
            Inst::Argv => self.page = self.next.argv(*state.page()),
            Inst::Ctrl | Inst::Nop => (),
        }
    }
}
