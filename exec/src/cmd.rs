use crate::exec::ExecCmd;
use inst::Command;
use state::State;

const BLOCK_SIDE: u8 = 16;

impl ExecCmd {
    pub fn imm(state: &State, digit: u8) -> Command {
        Command::Imm(state.data, combine(state.data, digit))
    }
    pub fn swap(state: &State) -> Command {
        Command::Swap((state.data, state.acc), (state.acc, state.data))
    }
    pub fn hi(state: &State) -> Command {
        Command::Hi(state.data, state.acc)
    }
    pub fn lo(state: &State) -> Command {
        Command::Lo(state.acc, state.data)
    }
    pub fn inc(state: &State) -> Command {
        let (next, _) = state.acc.overflowing_add(1);
        Command::Inc(state.acc, next)
    }
    pub fn dec(state: &State) -> Command {
        let (next, _) = state.acc.overflowing_sub(1);
        Command::Dec(state.acc, next)
    }
    pub fn add(state: &State) -> Command {
        let next = (state.acc as u16) + (state.data as u16);
        Command::Add((state.data, state.acc), split(next))
    }
    pub fn sub(state: &State) -> Command {
        let (next, _) = (state.acc as u16).overflowing_sub(state.data as u16);
        Command::Sub((state.data, state.acc), split(next))
    }
    pub fn mul(state: &State) -> Command {
        let next = (state.data as u16) * (state.acc as u16);
        Command::Mul((state.data, state.acc), split(next))
    }
    pub fn div(state: &State) -> Command {
        if state.data == 0 {
            Command::DivErr(state.error)
        } else {
            let next = (state.acc % state.data, state.acc / state.data);
            Command::Div((state.data, state.acc), next)
        }
    }
    pub fn neg(state: &State) -> Command {
        Command::Neg(state.acc, extend(state.data == 0))
    }
    pub fn bool(state: &State) -> Command {
        Command::Bool(state.acc, extend(state.data != 0))
    }
    pub fn eq(state: &State) -> Command {
        Command::Eq(state.acc, extend(state.data == state.acc))
    }
    pub fn lt(state: &State) -> Command {
        Command::Lt(state.acc, extend(state.data < state.acc))
    }
    pub fn gt(state: &State) -> Command {
        Command::Lt(state.acc, extend(state.data > state.acc))
    }
    pub fn not(state: &State) -> Command {
        Command::Not(state.acc, !state.data)
    }
    pub fn and(state: &State) -> Command {
        Command::And(state.acc, state.data & state.acc)
    }
    pub fn or(state: &State) -> Command {
        Command::Or(state.acc, state.data | state.acc)
    }
    pub fn xor(state: &State) -> Command {
        Command::Xor(state.acc, state.data ^ state.acc)
    }
    pub fn shl(state: &State) -> Command {
        Command::Shl(state.acc, state.acc << 1)
    }
    pub fn shr(state: &State) -> Command {
        Command::Shr(state.acc, state.acc >> 1)
    }
    pub fn rotl(state: &State) -> Command {
        Command::Rotl(state.acc, rot(state.acc, true))
    }
    pub fn rotr(state: &State) -> Command {
        Command::Rotr(state.acc, rot(state.acc, true))
    }
    pub fn left(state: &State) -> Command {
        Command::Left(state.coord, backward(state, 1))
    }
    pub fn right(state: &State) -> Command {
        Command::Right(state.coord, forward(state, 1))
    }
    pub fn down(state: &State) -> Command {
        Command::Down(state.coord, forward(state, BLOCK_SIDE))
    }
    pub fn up(state: &State) -> Command {
        Command::Left(state.coord, backward(state, BLOCK_SIDE))
    }
    pub fn pos(state: &State) -> Command {
        Command::Pos((state.data, state.acc), (state.block, state.coord))
    }
    pub fn goto(state: &State) -> Command {
        Command::Goto(state.coord, state.acc)
    }
    pub fn jump(state: &State) -> Command {
        Command::Jump(state.block, state.data)
    }
    pub fn load(state: &State) -> Command {
        Command::Load(state.data, state.page()[state.coord])
    }
    pub fn store(state: &State) -> Command {
        Command::Load(state.page()[state.coord], state.data)
    }
    pub fn argc(state: &State) -> Command {
        const MAX_LEN: usize = u8::MAX as usize;
        let len = std::env::args().len().min(MAX_LEN) as u8;
        let overflow = MAX_LEN < std::env::args().len();
        Command::Argc((state.acc, state.error), (len, overflow))
    }
    pub fn argv(state: &State) -> Command {
        if let Some(arg) = std::env::args().nth(state.acc as usize) {
            let mut next = state.page().clone();
            next.write(arg.as_bytes().iter());
            Command::Argv(*state.page(), next)
        } else {
            Command::NoArg(state.error)
        }
    }
}

fn split(val: u16) -> (u8, u8) {
    let hi = val >> u8::BITS;
    let lo = val & (u8::MAX as u16);
    (hi as u8, lo as u8)
}
fn combine(hi: u8, lo: u8) -> u8 {
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
fn forward(state: &State, shift: u8) -> u8 {
    let (next, _) = state.coord.overflowing_add(shift);
    next
}
fn backward(state: &State, shift: u8) -> u8 {
    let (next, _) = state.coord.overflowing_sub(shift);
    next
}
