use inst::Command;
use state::State;

pub struct ExecReg {}
impl ExecReg {
    pub fn imm(state: &State, digit: u8) -> Command {
        let prev = combine(state.data, state.acc);
        let next = combine(combine_nibbles(state.data, digit), state.acc);
        Command::Imm(prev, next)
    }
    pub fn swap(state: &State) -> Command {
        let prev = combine(state.data, state.acc);
        let next = combine(state.acc, state.data);
        Command::Swap(prev, next)
    }
    pub fn hi(state: &State) -> Command {
        let prev = combine(state.data, state.acc);
        let next = combine(state.acc, state.acc);
        Command::Hi(prev, next)
    }
    pub fn lo(state: &State) -> Command {
        let prev = combine(state.data, state.acc);
        let next = combine(state.data, state.data);
        Command::Lo(prev, next)
    }
    pub fn inc(state: &State) -> Command {
        let prev = combine(state.data, state.acc);
        let (next, _) = state.acc.overflowing_add(1);
        Command::Inc(prev, combine(state.data, next))
    }
    pub fn dec(state: &State) -> Command {
        let prev = combine(state.data, state.acc);
        let (next, _) = state.acc.overflowing_sub(1);
        Command::Dec(prev, combine(state.data, next))
    }
    pub fn add(state: &State) -> Command {
        let prev = combine(state.data, state.acc);
        let next = (state.acc as u16) + (state.data as u16);
        Command::Add(prev, next)
    }
    pub fn sub(state: &State) -> Command {
        let prev = combine(state.data, state.acc);
        let (next, _) = (state.acc as u16).overflowing_sub(state.data as u16);
        Command::Sub(prev, next)
    }
    pub fn mul(state: &State) -> Command {
        let prev = combine(state.data, state.acc);
        let next = (state.data as u16) * (state.acc as u16);
        Command::Mul(prev, next)
    }
    pub fn div(state: &State) -> Command {
        if state.data == 0 {
            Command::DivErr(state.error, true)
        } else {
            let prev = combine(state.data, state.acc);
            let next = combine(state.acc % state.data, state.acc / state.data);
            Command::Div(prev, next)
        }
    }
}

fn combine(hi: u8, lo: u8) -> u16 {
    ((hi as u16) << u8::BITS) | (lo as u16)
}
fn combine_nibbles(hi: u8, lo: u8) -> u8 {
    const SHIFT: u32 = u8::BITS / 2;
    const MASK: u8 = 0xF;
    ((hi & MASK) << SHIFT) | (lo & MASK)
}
