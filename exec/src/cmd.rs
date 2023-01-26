use crate::exec::ExecCmd;
use inst::Inst::{self, *};
use state::State;

const BLOCK_SIDE: u8 = 16;

enum Data {
    Single(u8, u8),
    Double((u8, u8), (u8, u8)),
    Bool(bool, bool),
}
pub struct Command {
    pub inst: Inst,
    data: Data,
}
impl Command {
    pub fn imm(state: &State, digit: u8) -> Self {
        let next = combine(state.data, digit);
        let inst = Imm(state.data, next);
        let data = Data::Single(state.data, next);
        Self { inst, data }
    }
    pub fn swap(state: &State) -> Self {
        let inst = Swap((state.data, state.acc), (state.acc, state.data));
        let data = Data::Double((state.data, state.acc), (state.acc, state.data));
        Self { inst, data }
    }
    pub fn hi(state: &State) -> Self {
        let inst = Hi(state.data, state.acc);
        let data = Data::Single(state.data, state.acc);
        Self { inst, data }
    }
    pub fn lo(state: &State) -> Self {
        let inst = Lo(state.acc, state.data);
        let data = Data::Single(state.data, state.acc);
        Self { inst, data }
    }
    pub fn inc(state: &State) -> Self {
        let (next, _) = state.acc.overflowing_add(1);
        let inst = Inc(state.acc, next);
        let data = Data::Single(state.acc, next);
        Self { inst, data }
    }
    pub fn dec(state: &State) -> Self {
        let (next, _) = state.acc.overflowing_sub(1);
        let inst = Dec(state.acc, next);
        let data = Data::Single(state.acc, next);
        Self { inst, data }
    }
    pub fn add(state: &State) -> Self {
        let next = (state.acc as u16) + (state.data as u16);
        let inst = Add((state.data, state.acc), split(next));
        let data = Data::Double((state.data, state.acc), split(next));
        Self { inst, data }
    }
    pub fn sub(state: &State) -> Self {
        let (next, _) = (state.acc as u16).overflowing_sub(state.data as u16);
        let inst = Sub((state.data, state.acc), split(next));
        let data = Data::Double((state.data, state.acc), split(next));
        Self { inst, data }
    }
    pub fn mul(state: &State) -> Self {
        let next = (state.data as u16) * (state.acc as u16);
        let inst = Mul((state.data, state.acc), split(next));
        let data = Data::Double((state.data, state.acc), split(next));
        Self { inst, data }
    }
    pub fn div(state: &State) -> Self {
        if state.data == 0 {
            let inst = DivErr(state.error);
            let data = Data::Bool(state.error, true);
            Self { inst, data }
        } else {
            let next = (state.acc % state.data, state.acc / state.data);
            let inst = Div((state.data, state.acc), next);
            let data = Data::Double((state.data, state.acc), next);
            Self { inst, data }
        }
    }
    pub fn neg(state: &State) -> Self {
        let inst = Neg(state.acc, extend(state.data == 0));
        let data = Data::Single(state.acc, extend(state.data == 0));
        Self { inst, data }
    }
    pub fn bool(state: &State) -> Self {
        let inst = Bool(state.acc, extend(state.data != 0));
        let data = Data::Single(state.acc, extend(state.data != 0));
        Self { inst, data }
    }
    pub fn eq(state: &State) -> Self {
        let inst = Eq(state.acc, extend(state.data == state.acc));
        let data = Data::Single(state.acc, extend(state.data == state.acc));
        Self { inst, data }
    }
    pub fn lt(state: &State) -> Self {
        let inst = Lt(state.acc, extend(state.data < state.acc));
        let data = Data::Single(state.acc, extend(state.data < state.acc));
        Self { inst, data }
    }
    pub fn gt(state: &State) -> Self {
        let inst = Lt(state.acc, extend(state.data > state.acc));
        let data = Data::Single(state.acc, extend(state.data > state.acc));
        Self { inst, data }
    }
}

impl ExecCmd {
    pub fn not(state: &State) -> Inst {
        Not(state.acc, !state.data)
    }
    pub fn and(state: &State) -> Inst {
        And(state.acc, state.data & state.acc)
    }
    pub fn or(state: &State) -> Inst {
        Or(state.acc, state.data | state.acc)
    }
    pub fn xor(state: &State) -> Inst {
        Xor(state.acc, state.data ^ state.acc)
    }
    pub fn shl(state: &State) -> Inst {
        Shl(state.acc, state.acc << 1)
    }
    pub fn shr(state: &State) -> Inst {
        Shr(state.acc, state.acc >> 1)
    }
    pub fn rotl(state: &State) -> Inst {
        Rotl(state.acc, rot(state.acc, true))
    }
    pub fn rotr(state: &State) -> Inst {
        Rotr(state.acc, rot(state.acc, true))
    }
    pub fn left(state: &State) -> Inst {
        Left(state.coord, backward(state, 1))
    }
    pub fn right(state: &State) -> Inst {
        Right(state.coord, forward(state, 1))
    }
    pub fn down(state: &State) -> Inst {
        Down(state.coord, forward(state, BLOCK_SIDE))
    }
    pub fn up(state: &State) -> Inst {
        Left(state.coord, backward(state, BLOCK_SIDE))
    }
    pub fn pos(state: &State) -> Inst {
        Pos((state.data, state.acc), (state.block, state.coord))
    }
    pub fn goto(state: &State) -> Inst {
        Goto(state.coord, state.acc)
    }
    pub fn jump(state: &State) -> Inst {
        Jump(state.block, state.data)
    }
    pub fn load(state: &State) -> Inst {
        Load(state.data, state.page()[state.coord])
    }
    pub fn store(state: &State) -> Inst {
        Load(state.page()[state.coord], state.data)
    }
    pub fn argc(state: &State) -> Inst {
        const MAX_LEN: usize = u8::MAX as usize;
        let len = std::env::args().len().min(MAX_LEN) as u8;
        let overflow = MAX_LEN < std::env::args().len();
        Argc((state.acc, state.error), (len, overflow))
    }
    pub fn argv(state: &State) -> Inst {
        if let Some(arg) = std::env::args().nth(state.acc as usize) {
            let mut next = state.page().clone();
            next.write(arg.as_bytes().iter());
            Argv(*state.page(), next)
        } else {
            NoArg(state.error)
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
