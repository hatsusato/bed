use crate::reg::ExecReg;
use inst::Inst;
use state::State;

pub struct Exec {}
impl Exec {
    pub fn new() -> Self {
        Self {}
    }
    pub fn exec(key: char, state: &mut State) {
        let inst = Inst::new(key);
        use Inst::*;
        let cmd = match inst {
            Imm(digit) => ExecReg::imm(state, digit),
            Swap => ExecReg::swap(state),
            Hi => ExecReg::hi(state),
            Lo => ExecReg::lo(state),
            Inc => ExecReg::inc(state),
            Dec => ExecReg::dec(state),
            Add => ExecReg::add(state),
            Sub => ExecReg::sub(state),
            Mul => ExecReg::mul(state),
            Div => ExecReg::div(state),
            _ => return state.exec(inst),
        };
        state.exec_cmd(cmd);
    }
}
