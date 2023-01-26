use inst::Inst::{self, *};
use state::Bank;
use state::State;

const BLOCK_SIDE: u8 = 16;

pub struct Command {
    pub inst: Inst,
    pub next: Bank,
}
impl Command {
    pub fn imm(state: &State, digit: u8) -> Self {
        let next = combine(state.data, digit);
        Self {
            inst: Imm,
            next: (state.bank().update_data(next)),
        }
    }
    pub fn swap(state: &State) -> Self {
        Self {
            inst: Swap,
            next: (state.bank().update_acc(state.data).update_data(state.acc)),
        }
    }
    pub fn hi(state: &State) -> Self {
        Self {
            inst: Hi,
            next: (state.bank().update_data(state.acc)),
        }
    }
    pub fn lo(state: &State) -> Self {
        Self {
            inst: Lo,
            next: (state.bank().update_acc(state.data)),
        }
    }
    pub fn inc(state: &State) -> Self {
        let (next, _) = state.acc.overflowing_add(1);
        Self {
            inst: Inc,
            next: (state.bank().update_acc(next)),
        }
    }
    pub fn dec(state: &State) -> Self {
        let (next, _) = state.acc.overflowing_sub(1);
        Self {
            inst: Dec,
            next: (state.bank().update_acc(next)),
        }
    }
    pub fn add(state: &State) -> Self {
        let next = (state.acc as u16) + (state.data as u16);
        Self {
            inst: Add,
            next: (state
                .bank()
                .update_acc(split(next).0)
                .update_data(split(next).1)),
        }
    }
    pub fn sub(state: &State) -> Self {
        let (next, _) = (state.acc as u16).overflowing_sub(state.data as u16);
        Self {
            inst: Sub,
            next: (state
                .bank()
                .update_acc(split(next).0)
                .update_data(split(next).1)),
        }
    }
    pub fn mul(state: &State) -> Self {
        let next = (state.data as u16) * (state.acc as u16);
        Self {
            inst: Mul,
            next: (state
                .bank()
                .update_acc(split(next).0)
                .update_data(split(next).1)),
        }
    }
    pub fn div(state: &State) -> Self {
        if state.data == 0 {
            Self {
                inst: DivErr,
                next: (state.bank().update_error(true)),
            }
        } else {
            Self {
                inst: Div,
                next: (state
                    .bank()
                    .update_acc(state.acc / state.data)
                    .update_data(state.acc % state.data)),
            }
        }
    }
    pub fn neg(state: &State) -> Self {
        Self {
            inst: Neg,
            next: (state.bank().update_acc(extend(state.data == 0))),
        }
    }
    pub fn bool(state: &State) -> Self {
        Self {
            inst: Bool,
            next: (state.bank().update_acc(extend(state.data != 0))),
        }
    }
    pub fn eq(state: &State) -> Self {
        Self {
            inst: Eq,
            next: (state.bank().update_acc(extend(state.data == state.acc))),
        }
    }
    pub fn lt(state: &State) -> Self {
        Self {
            inst: Lt,
            next: (state.bank().update_acc(extend(state.data < state.acc))),
        }
    }
    pub fn gt(state: &State) -> Self {
        Self {
            inst: Gt,
            next: (state.bank().update_acc(extend(state.data > state.acc))),
        }
    }
    pub fn not(state: &State) -> Self {
        Self {
            inst: Not,
            next: (state.bank().update_acc(!state.data)),
        }
    }
    pub fn and(state: &State) -> Self {
        Self {
            inst: And,
            next: (state.bank().update_acc(state.data & state.acc)),
        }
    }
    pub fn or(state: &State) -> Self {
        Self {
            inst: Or,
            next: (state.bank().update_acc(state.data | state.acc)),
        }
    }
    pub fn xor(state: &State) -> Self {
        Self {
            inst: Xor,
            next: (state.bank().update_acc(state.data ^ state.acc)),
        }
    }
    pub fn shl(state: &State) -> Self {
        Self {
            inst: Shl,
            next: (state.bank().update_acc(state.acc << 1)),
        }
    }
    pub fn shr(state: &State) -> Self {
        Self {
            inst: Shr,
            next: (state.bank().update_acc(state.acc >> 1)),
        }
    }
    pub fn rotl(state: &State) -> Self {
        Self {
            inst: Rotl,
            next: (state.bank().update_acc(rot(state.acc, true))),
        }
    }
    pub fn rotr(state: &State) -> Self {
        Self {
            inst: Rotr,
            next: (state.bank().update_acc(rot(state.acc, false))),
        }
    }
    pub fn left(state: &State) -> Self {
        Self {
            inst: Left,
            next: (state.bank().update_coord(backward(state, 1))),
        }
    }
    pub fn right(state: &State) -> Self {
        Self {
            inst: Right,
            next: (state.bank().update_coord(forward(state, 1))),
        }
    }
    pub fn down(state: &State) -> Self {
        Self {
            inst: Down,
            next: (state.bank().update_coord(forward(state, BLOCK_SIDE))),
        }
    }
    pub fn up(state: &State) -> Self {
        Self {
            inst: Up,
            next: (state.bank().update_coord(backward(state, BLOCK_SIDE))),
        }
    }
    pub fn pos(state: &State) -> Self {
        Self {
            inst: Pos,
            next: (state
                .bank()
                .update_data(state.block)
                .update_acc(state.coord)),
        }
    }
    pub fn goto(state: &State) -> Self {
        Self {
            inst: Goto,
            next: (state.bank().update_coord(state.acc)),
        }
    }
    pub fn jump(state: &State) -> Self {
        Self {
            inst: Jump,
            next: (state.bank().update_block(state.data)),
        }
    }
    pub fn load(state: &State) -> Self {
        Self {
            inst: Load,
            next: (state.bank().update_data(state.page()[state.coord])),
        }
    }
    pub fn store(state: &State) -> Self {
        let mut next = state.page().clone();
        next[state.coord] = state.data;
        Self {
            inst: Store,
            next: (state.bank().update_page(next)),
        }
    }
    pub fn argc(state: &State) -> Self {
        const MAX_LEN: usize = u8::MAX as usize;
        let len = std::env::args().len().min(MAX_LEN) as u8;
        let overflow = MAX_LEN < std::env::args().len();
        Self {
            inst: Argc,
            next: (state.bank().update_acc(len).update_error(overflow)),
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
            Self {
                inst: Argv,
                next: (state.bank().update_acc(len).update_page(next)),
            }
        } else {
            Self {
                inst: NoArg,
                next: (state.bank().update_error(true)),
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
