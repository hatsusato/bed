use crate::inst::Inst;
use crate::{Bank, State};
use util::Page;

pub struct Command {
    pub next: Bank,
    pub page: Option<Page>,
}
impl Command {
    pub fn new(inst: &Inst, state: &State) -> Self {
        let mut this = Self {
            next: state.bank(),
            page: None,
        };
        this.update_inst(inst, state);
        this
    }
    fn update_inst(&mut self, inst: &Inst, state: &State) {
        match inst {
            Inst::Imm(digit) => self.next.imm(*digit),
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
            Inst::Load => self.load(state),
            Inst::Store => self.store(state),
            Inst::Argc => self.next.argc(),
            Inst::Argv => self.argv(state),
            Inst::Esc(ch) => self.next.esc(*ch),
            Inst::Nop => (),
        }
    }
    fn load(&mut self, state: &State) {
        self.next.data = state.page()[self.next.coord];
    }
    fn store(&mut self, state: &State) {
        let mut page = *state.page();
        page[self.next.coord] = self.next.data;
        self.page = Some(page);
    }
    fn argv(&mut self, state: &State) {
        let arg = std::env::args().nth(self.next.acc.into());
        self.next.argv(&arg);
        if let Some(input) = arg {
            let mut page = *state.page();
            page.write(input.as_bytes().iter());
            self.page = Some(page);
        }
    }
}
