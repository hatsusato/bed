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
            Imm(digit) => ExecCmd::imm(state, digit),
            Swap => ExecCmd::swap(state),
            Hi => ExecCmd::hi(state),
            Lo => ExecCmd::lo(state),
            Inc => ExecCmd::inc(state),
            Dec => ExecCmd::dec(state),
            Add => ExecCmd::add(state),
            Sub => ExecCmd::sub(state),
            Mul => ExecCmd::mul(state),
            Div => ExecCmd::div(state),
            Eq => ExecCmd::eq(state),
            Le => ExecCmd::lt(state),
            Gr => ExecCmd::gt(state),
            Err => ExecCmd::is_err(state),
            Bool => ExecCmd::bool(state),
            Not => ExecCmd::not(state),
            And => ExecCmd::and(state),
            Or => ExecCmd::or(state),
            Xor => ExecCmd::xor(state),
            Shl => ExecCmd::shl(state),
            Shr => ExecCmd::shr(state),
            Rotl => ExecCmd::rotl(state),
            Rotr => ExecCmd::rotr(state),
            Left => ExecCmd::left(state),
            Right => ExecCmd::right(state),
            Down => ExecCmd::down(state),
            Up => ExecCmd::up(state),
            Pos => ExecCmd::pos(state),
            Goto => ExecCmd::goto(state),
            // Jump => ExecCmd::jump(state),
            Load => ExecCmd::load(state),
            Store => ExecCmd::store(state),
            Argc => ExecCmd::argc(state),
            Argv => ExecCmd::argv(state),
            _ => return state.exec(inst),
        };
        state.exec_cmd(cmd);
    }
}

pub struct ExecCmd {}
