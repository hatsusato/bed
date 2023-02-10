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
            Inst::Immediate(data) => self.next.imm(data),
            Inst::Insert(digit) => self.next.ins(digit),
            Inst::Swap => self.next.swap(),
            Inst::High => self.next.hi(),
            Inst::Low => self.next.lo(),
            Inst::Zero => self.next.zero(),
            Inst::Origin => self.next.origin(),
            Inst::Start => self.next.start(),
            Inst::Goto => self.next.goto(),
            Inst::Jump => self.next.jump(),
            Inst::Position => self.next.position(),
            Inst::Page => self.next.page(),
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
            Inst::Raise => self.next.raise(),
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
            Inst::Read => self.next.read(state.page()),
            Inst::Write => self.page = self.next.write(*state.page()),
            Inst::Delete => self.page = self.next.del(*state.page()),
            Inst::Put => self.next.put(state.page()),
            Inst::Get => self.page = self.next.get(*state.page()),
            Inst::Save => self.page = self.next.save(*state.page()),
            Inst::Restore => self.next.restore(state.page()),
            Inst::Argc => self.next.argc(),
            Inst::Argv => self.page = self.next.argv(*state.page()),
            Inst::Eval => self.eval(state),
            Inst::Meta(_) | Inst::Nop => (),
        }
    }
    fn eval(&mut self, state: &State) {
        let inst = Inst::new(char::from(self.next.data));
        self.update_inst(inst, state);
    }
}
