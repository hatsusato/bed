use crate::{Bank, Inst, Page};
use util::Block;

#[derive(Default)]
pub struct State {
    bank: Bank,
    memory: Block<Page>,
}
impl State {
    pub fn issue(&mut self, inst: Inst) {
        match inst {
            Inst::Immediate(_data) => (),
            Inst::Insert(_digit) => (),
            Inst::Swap => (),
            Inst::High => (),
            Inst::Low => (),
            Inst::Zero => (),
            Inst::Origin => (),
            Inst::Start => (),
            Inst::Goto => (),
            Inst::Jump => (),
            Inst::Position => (),
            Inst::Page => (),
            Inst::Left => (),
            Inst::Right => (),
            Inst::Up => (),
            Inst::Down => (),
            Inst::Inc => (),
            Inst::Dec => (),
            Inst::Add => (),
            Inst::Sub => (),
            Inst::Mul => (),
            Inst::Div => (),
            Inst::Clear => (),
            Inst::Raise => (),
            Inst::Neg => (),
            Inst::Bool => (),
            Inst::Eq => (),
            Inst::Lt => (),
            Inst::Gt => (),
            Inst::Not => (),
            Inst::And => (),
            Inst::Or => (),
            Inst::Xor => (),
            Inst::Shl => (),
            Inst::Shr => (),
            Inst::Rotl => (),
            Inst::Rotr => (),
            Inst::Load => (),
            Inst::Store => (),
            Inst::Delete => (),
            Inst::Put => (),
            Inst::Get => (),
            Inst::Save => (),
            Inst::Restore => (),
            Inst::Quote(_input) => (),
            Inst::Eval | Inst::Meta | Inst::Nop => (),
        }
    }
}
