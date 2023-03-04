use crate::{Bank, Inst, Page};
use std::io;
use util::Block;

#[derive(Default)]
pub struct State {
    bank: Bank,
    memory: Block<Page>,
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
            Inst::Save => (),
            Inst::Restore => (),
            Inst::Quote(_input) => (),
            Inst::Eval | Inst::Meta | Inst::Nop => (),
        }
    }
    fn load(&mut self) {
        self.bank.data = *self.current();
    }
    fn store(&mut self) {
        *self.current_mut() = self.bank.data;
    }
    fn delete(&mut self) {
        *self.current_mut() = 0;
    }
    fn put(&mut self) {
        let flag = put_byte(self.current()).is_err();
        self.bank.set_error(flag);
    }
    fn get(&mut self) {
        let flag = get_byte(self.current_mut()).is_err();
        self.bank.set_error(flag);
    }
    fn current(&self) -> &u8 {
        &self.memory[self.bank.block][self.bank.coord]
    }
    fn current_mut(&mut self) -> &mut u8 {
        &mut self.memory[self.bank.block][self.bank.coord]
    }
}

fn put_byte(data: &u8) -> io::Result<usize> {
    use io::Write;
    io::stdout().write(&[*data])
}
fn get_byte(data: &mut u8) -> io::Result<usize> {
    use io::Read;
    let buf = &mut [0];
    let result = io::stdin().read(buf);
    *data = buf[0];
    result
}
