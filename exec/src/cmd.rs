use inst::Inst::{self, *};
use state::Bank;
use state::State;
use util::Page;

const BLOCK_SIDE: u8 = 16;

enum Data {
    Single(u8, u8),
    Double((u8, u8), (u8, u8)),
    Bool(bool, bool),
    Buffer((Page, u8), (Page, u8)),
    Arg((u8, bool), (u8, bool)),
}
pub struct Command {
    pub inst: Inst,
    prev: Option<Bank>,
    next: Option<Bank>,
    data: Data,
}
impl Command {
    pub fn imm(state: &State, digit: u8) -> Self {
        let next = combine(state.data, digit);
        let inst = Imm(state.data, next);
        let data = Data::Single(state.data, next);
        Self {
            inst,
            prev: Some(state.bank()),
            next: None,
            data,
        }
    }
    pub fn swap(state: &State) -> Self {
        let inst = Swap((state.data, state.acc), (state.acc, state.data));
        let data = Data::Double((state.data, state.acc), (state.acc, state.data));
        Self {
            inst,
            prev: Some(state.bank()),
            next: None,
            data,
        }
    }
    pub fn hi(state: &State) -> Self {
        let inst = Hi(state.data, state.acc);
        let data = Data::Single(state.data, state.acc);
        Self {
            inst,
            prev: Some(state.bank()),
            next: None,
            data,
        }
    }
    pub fn lo(state: &State) -> Self {
        let inst = Lo(state.acc, state.data);
        let data = Data::Single(state.data, state.acc);
        Self {
            inst,
            prev: Some(state.bank()),
            next: None,
            data,
        }
    }
    pub fn inc(state: &State) -> Self {
        let (next, _) = state.acc.overflowing_add(1);
        let inst = Inc(state.acc, next);
        let data = Data::Single(state.acc, next);
        Self {
            inst,
            prev: Some(state.bank()),
            next: None,
            data,
        }
    }
    pub fn dec(state: &State) -> Self {
        let (next, _) = state.acc.overflowing_sub(1);
        let inst = Dec(state.acc, next);
        let data = Data::Single(state.acc, next);
        Self {
            inst,
            prev: Some(state.bank()),
            next: None,
            data,
        }
    }
    pub fn add(state: &State) -> Self {
        let next = (state.acc as u16) + (state.data as u16);
        let inst = Add((state.data, state.acc), split(next));
        let data = Data::Double((state.data, state.acc), split(next));
        Self {
            inst,
            prev: Some(state.bank()),
            next: None,
            data,
        }
    }
    pub fn sub(state: &State) -> Self {
        let (next, _) = (state.acc as u16).overflowing_sub(state.data as u16);
        let inst = Sub((state.data, state.acc), split(next));
        let data = Data::Double((state.data, state.acc), split(next));
        Self {
            inst,
            prev: Some(state.bank()),
            next: None,
            data,
        }
    }
    pub fn mul(state: &State) -> Self {
        let next = (state.data as u16) * (state.acc as u16);
        let inst = Mul((state.data, state.acc), split(next));
        let data = Data::Double((state.data, state.acc), split(next));
        Self {
            inst,
            prev: Some(state.bank()),
            next: None,
            data,
        }
    }
    pub fn div(state: &State) -> Self {
        if state.data == 0 {
            let inst = DivErr(state.error);
            let data = Data::Bool(state.error, true);
            Self {
                inst,
                prev: Some(state.bank()),
                next: None,
                data,
            }
        } else {
            let next = (state.acc % state.data, state.acc / state.data);
            let inst = Div((state.data, state.acc), next);
            let data = Data::Double((state.data, state.acc), next);
            Self {
                inst,
                prev: Some(state.bank()),
                next: None,
                data,
            }
        }
    }
    pub fn neg(state: &State) -> Self {
        let inst = Neg(state.acc, extend(state.data == 0));
        let data = Data::Single(state.acc, extend(state.data == 0));
        Self {
            inst,
            prev: Some(state.bank()),
            next: None,
            data,
        }
    }
    pub fn bool(state: &State) -> Self {
        let inst = Bool(state.acc, extend(state.data != 0));
        let data = Data::Single(state.acc, extend(state.data != 0));
        Self {
            inst,
            prev: Some(state.bank()),
            next: None,
            data,
        }
    }
    pub fn eq(state: &State) -> Self {
        let inst = Eq(state.acc, extend(state.data == state.acc));
        let data = Data::Single(state.acc, extend(state.data == state.acc));
        Self {
            inst,
            prev: Some(state.bank()),
            next: None,
            data,
        }
    }
    pub fn lt(state: &State) -> Self {
        let inst = Lt(state.acc, extend(state.data < state.acc));
        let data = Data::Single(state.acc, extend(state.data < state.acc));
        Self {
            inst,
            prev: Some(state.bank()),
            next: None,
            data,
        }
    }
    pub fn gt(state: &State) -> Self {
        let inst = Lt(state.acc, extend(state.data > state.acc));
        let data = Data::Single(state.acc, extend(state.data > state.acc));
        Self {
            inst,
            prev: Some(state.bank()),
            next: None,
            data,
        }
    }
    pub fn not(state: &State) -> Self {
        let inst = Not(state.acc, !state.data);
        let data = Data::Single(state.acc, !state.data);
        Self {
            inst,
            prev: Some(state.bank()),
            next: None,
            data,
        }
    }
    pub fn and(state: &State) -> Self {
        let inst = And(state.acc, state.data & state.acc);
        let data = Data::Single(state.acc, state.data & state.acc);
        Self {
            inst,
            prev: Some(state.bank()),
            next: None,
            data,
        }
    }
    pub fn or(state: &State) -> Self {
        let inst = Or(state.acc, state.data | state.acc);
        let data = Data::Single(state.acc, state.data | state.acc);
        Self {
            inst,
            prev: Some(state.bank()),
            next: None,
            data,
        }
    }
    pub fn xor(state: &State) -> Self {
        let inst = Xor(state.acc, state.data ^ state.acc);
        let data = Data::Single(state.acc, state.data ^ state.acc);
        Self {
            inst,
            prev: Some(state.bank()),
            next: None,
            data,
        }
    }
    pub fn shl(state: &State) -> Self {
        let inst = Shl(state.acc, state.acc << 1);
        let data = Data::Single(state.acc, state.acc << 1);
        Self {
            inst,
            prev: Some(state.bank()),
            next: None,
            data,
        }
    }
    pub fn shr(state: &State) -> Self {
        let inst = Shr(state.acc, state.acc >> 1);
        let data = Data::Single(state.acc, state.acc >> 1);
        Self {
            inst,
            prev: Some(state.bank()),
            next: None,
            data,
        }
    }
    pub fn rotl(state: &State) -> Self {
        let inst = Rotl(state.acc, rot(state.acc, true));
        let data = Data::Single(state.acc, rot(state.acc, true));
        Self {
            inst,
            prev: Some(state.bank()),
            next: None,
            data,
        }
    }
    pub fn rotr(state: &State) -> Self {
        let inst = Rotr(state.acc, rot(state.acc, true));
        let data = Data::Single(state.acc, rot(state.acc, true));
        Self {
            inst,
            prev: Some(state.bank()),
            next: None,
            data,
        }
    }
    pub fn left(state: &State) -> Self {
        let inst = Left(state.coord, backward(state, 1));
        let data = Data::Single(state.coord, backward(state, 1));
        Self {
            inst,
            prev: Some(state.bank()),
            next: None,
            data,
        }
    }
    pub fn right(state: &State) -> Self {
        let inst = Right(state.coord, forward(state, 1));
        let data = Data::Single(state.coord, forward(state, 1));
        Self {
            inst,
            prev: Some(state.bank()),
            next: None,
            data,
        }
    }
    pub fn down(state: &State) -> Self {
        let inst = Down(state.coord, forward(state, BLOCK_SIDE));
        let data = Data::Single(state.coord, forward(state, BLOCK_SIDE));
        Self {
            inst,
            prev: Some(state.bank()),
            next: None,
            data,
        }
    }
    pub fn up(state: &State) -> Self {
        let inst = Left(state.coord, backward(state, BLOCK_SIDE));
        let data = Data::Single(state.coord, backward(state, BLOCK_SIDE));
        Self {
            inst,
            prev: Some(state.bank()),
            next: None,
            data,
        }
    }
    pub fn pos(state: &State) -> Self {
        let inst = Pos((state.data, state.acc), (state.block, state.coord));
        let data = Data::Double((state.data, state.acc), (state.block, state.coord));
        Self {
            inst,
            prev: Some(state.bank()),
            next: None,
            data,
        }
    }
    pub fn goto(state: &State) -> Self {
        let inst = Goto(state.coord, state.acc);
        let data = Data::Single(state.coord, state.acc);
        Self {
            inst,
            prev: Some(state.bank()),
            next: None,
            data,
        }
    }
    pub fn jump(state: &State) -> Self {
        let inst = Jump(state.block, state.data);
        let data = Data::Single(state.block, state.data);
        Self {
            inst,
            prev: Some(state.bank()),
            next: None,
            data,
        }
    }
    pub fn load(state: &State) -> Self {
        let inst = Load(state.data, state.page()[state.coord]);
        let data = Data::Single(state.data, state.page()[state.coord]);
        Self {
            inst,
            prev: Some(state.bank()),
            next: None,
            data,
        }
    }
    pub fn store(state: &State) -> Self {
        let inst = Load(state.page()[state.coord], state.data);
        let data = Data::Single(state.page()[state.coord], state.data);
        Self {
            inst,
            prev: Some(state.bank()),
            next: None,
            data,
        }
    }
    pub fn argc(state: &State) -> Self {
        const MAX_LEN: usize = u8::MAX as usize;
        let len = std::env::args().len().min(MAX_LEN) as u8;
        let overflow = MAX_LEN < std::env::args().len();
        let inst = Argc((state.acc, state.error), (len, overflow));
        let data = Data::Arg((state.acc, state.error), (len, overflow));
        Self {
            inst,
            prev: Some(state.bank()),
            next: None,
            data,
        }
    }
    pub fn argv(state: &State) -> Self {
        if let Some(arg) = std::env::args().nth(state.acc as usize) {
            let input: Vec<u8> = arg
                .as_bytes()
                .iter()
                .take(u8::MAX as usize)
                .map(|&x| x)
                .collect();
            let len = input.len() as u8;
            let mut next = state.page().clone();
            next.write(input.iter());
            let inst = Argv(*state.page(), next);
            let data = Data::Buffer((*state.page(), state.acc), (next, len));
            Self {
                inst,
                prev: Some(state.bank()),
                next: None,
                data,
            }
        } else {
            let inst = NoArg(state.error);
            let data = Data::Bool(state.error, true);
            Self {
                inst,
                prev: Some(state.bank()),
                next: None,
                data,
            }
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
