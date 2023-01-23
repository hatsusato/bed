use inst::Command;
use state::State;

pub struct ExecReg {}
impl ExecReg {
    pub fn imm(state: &State, digit: u8) -> Command {
        let prev = state.data;
        let next = combine_nibbles(state.data, digit);
        Command::Imm(prev, next)
    }
    pub fn swap(state: &State) -> Command {
        let prev = combine(state.data, state.acc);
        let next = combine(state.acc, state.data);
        Command::Swap(prev, next)
    }
    pub fn hi(state: &State) -> Command {
        let prev = state.data;
        let next = state.acc;
        Command::Hi(prev, next)
    }
    pub fn lo(state: &State) -> Command {
        let prev = state.acc;
        let next = state.data;
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
    pub fn is_err(state: &State) -> Command {
        let prev = state.acc;
        let next = extend(state.error);
        Command::IsErr(prev, next)
    }
    pub fn bool(state: &State) -> Command {
        let prev = state.acc;
        let next = extend(state.data != 0);
        Command::Bool(prev, next)
    }
    pub fn eq(state: &State) -> Command {
        let prev = state.acc;
        let next = extend(state.data == state.acc);
        Command::Eq(prev, next)
    }
    pub fn lt(state: &State) -> Command {
        let prev = state.acc;
        let next = extend(state.data < state.acc);
        Command::Lt(prev, next)
    }
    pub fn gt(state: &State) -> Command {
        let prev = state.acc;
        let next = extend(state.data > state.acc);
        Command::Lt(prev, next)
    }
    pub fn not(state: &State) -> Command {
        let prev = state.acc;
        let next = !state.data;
        Command::Not(prev, next)
    }
    pub fn and(state: &State) -> Command {
        let prev = state.acc;
        let next = state.data & state.acc;
        Command::And(prev, next)
    }
    pub fn or(state: &State) -> Command {
        let prev = state.acc;
        let next = state.data | state.acc;
        Command::Or(prev, next)
    }
    pub fn xor(state: &State) -> Command {
        let prev = state.acc;
        let next = state.data ^ state.acc;
        Command::Xor(prev, next)
    }
    pub fn shl(state: &State) -> Command {
        let prev = state.acc;
        let next = state.acc << 1;
        Command::Shl(prev, next)
    }
    pub fn shr(state: &State) -> Command {
        let prev = state.acc;
        let next = state.acc >> 1;
        Command::Shr(prev, next)
    }
    pub fn rotl(state: &State) -> Command {
        let prev = state.acc;
        let next = rot(state.acc, true);
        Command::Rotl(prev, next)
    }
    pub fn rotr(state: &State) -> Command {
        let prev = state.acc;
        let next = rot(state.acc, true);
        Command::Rotr(prev, next)
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
fn extend(cond: bool) -> u8 {
    if cond {
        1
    } else {
        0
    }
}
fn rot(val: u8, forward: bool) -> u8 {
    let shl = if forward { 1 } else { u8::BITS - 1 };
    let shr = u8::BITS - shl;
    (val << shl) | (val >> shr)
}
