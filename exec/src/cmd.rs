use inst::Inst::{self, *};
use state::{Bank, State};
use util::{Page, BLOCK_SIDE};

pub struct Command {
    pub inst: Inst,
    pub next: Bank,
    pub page: Option<Page>,
}
impl Command {
    pub fn new(inst: Inst, state: &State) -> Self {
        let next = state.bank();
        Self {
            inst,
            next,
            page: None,
        }
    }
    fn update_reg(mut self, reg: u16) -> Self {
        [self.next.data, self.next.acc] = reg.to_be_bytes();
        self
    }
    fn update_acc(mut self, acc: u8) -> Self {
        self.next.acc = acc;
        self
    }
    fn update_block(mut self, block: u8) -> Self {
        self.next.block = block;
        self
    }
    fn update_coord(mut self, coord: u8) -> Self {
        self.next.coord = coord;
        self
    }
    fn update_data(mut self, data: u8) -> Self {
        self.next.data = data;
        self
    }
    fn update_error(mut self, error: bool) -> Self {
        self.next.error = error;
        self
    }
    fn update_page(mut self, page: Page) -> Self {
        self.page = Some(page);
        self
    }
    pub fn imm(state: &State, digit: u8) -> Self {
        let next = combine(state.data, digit);
        Self::new(Imm, state).update_data(next)
    }
    pub fn swap(state: &State) -> Self {
        Self::new(Swap, state)
            .update_data(state.acc)
            .update_acc(state.data)
    }
    pub fn hi(state: &State) -> Self {
        Self::new(Hi, state).update_data(state.acc)
    }
    pub fn lo(state: &State) -> Self {
        Self::new(Lo, state).update_acc(state.data)
    }
    pub fn inc(state: &State) -> Self {
        let (next, _) = state.acc.overflowing_add(1);
        Self::new(Inc, state).update_acc(next)
    }
    pub fn dec(state: &State) -> Self {
        let (next, _) = state.acc.overflowing_sub(1);
        Self::new(Dec, state).update_acc(next)
    }
    pub fn add(state: &State) -> Self {
        let next = u16::from(state.acc) + u16::from(state.data);
        Self::new(Add, state).update_reg(next)
    }
    pub fn sub(state: &State) -> Self {
        let (next, _) = u16::from(state.acc).overflowing_sub(state.data.into());
        Self::new(Sub, state).update_reg(next)
    }
    pub fn mul(state: &State) -> Self {
        let next = u16::from(state.data) * u16::from(state.acc);
        Self::new(Mul, state).update_reg(next)
    }
    pub fn div(state: &State) -> Self {
        if state.data == 0 {
            Self::new(DivErr, state).update_error(true)
        } else {
            Self::new(Div, state)
                .update_acc(state.acc / state.data)
                .update_data(state.acc % state.data)
        }
    }
    pub fn neg(state: &State) -> Self {
        Self::new(Neg, state).update_acc(u8::from(state.data == 0))
    }
    pub fn bool(state: &State) -> Self {
        Self::new(Bool, state).update_acc(u8::from(state.data != 0))
    }
    pub fn eq(state: &State) -> Self {
        Self::new(Eq, state).update_acc(u8::from(state.data == state.acc))
    }
    pub fn lt(state: &State) -> Self {
        Self::new(Lt, state).update_acc(u8::from(state.data < state.acc))
    }
    pub fn gt(state: &State) -> Self {
        Self::new(Gt, state).update_acc(u8::from(state.data > state.acc))
    }
    pub fn not(state: &State) -> Self {
        Self::new(Not, state).update_acc(!state.data)
    }
    pub fn and(state: &State) -> Self {
        Self::new(And, state).update_acc(state.data & state.acc)
    }
    pub fn or(state: &State) -> Self {
        Self::new(Or, state).update_acc(state.data | state.acc)
    }
    pub fn xor(state: &State) -> Self {
        Self::new(Xor, state).update_acc(state.data ^ state.acc)
    }
    pub fn shl(state: &State) -> Self {
        Self::new(Shl, state).update_acc(state.acc << 1)
    }
    pub fn shr(state: &State) -> Self {
        Self::new(Shr, state).update_acc(state.acc >> 1)
    }
    pub fn rotl(state: &State) -> Self {
        Self::new(Rotl, state).update_acc(rot(state.acc, true))
    }
    pub fn rotr(state: &State) -> Self {
        Self::new(Rotr, state).update_acc(rot(state.acc, false))
    }
    pub fn left(state: &State) -> Self {
        Self::new(Left, state).update_coord(backward(state, 1))
    }
    pub fn right(state: &State) -> Self {
        Self::new(Right, state).update_coord(forward(state, 1))
    }
    pub fn down(state: &State) -> Self {
        Self::new(Down, state).update_coord(forward(state, BLOCK_SIDE))
    }
    pub fn up(state: &State) -> Self {
        Self::new(Up, state).update_coord(backward(state, BLOCK_SIDE))
    }
    pub fn pos(state: &State) -> Self {
        Self::new(Pos, state)
            .update_data(state.block)
            .update_acc(state.coord)
    }
    pub fn goto(state: &State) -> Self {
        Self::new(Goto, state).update_coord(state.acc)
    }
    pub fn jump(state: &State) -> Self {
        Self::new(Jump, state).update_block(state.data)
    }
    pub fn load(state: &State) -> Self {
        Self::new(Load, state).update_data(state.page()[state.coord])
    }
    pub fn store(state: &State) -> Self {
        let mut next = *state.page();
        next[state.coord] = state.data;
        Self::new(Store, state).update_page(next)
    }
    pub fn argc(state: &State) -> Self {
        const MAX_LEN: usize = u8::MAX as usize;
        let len = std::env::args().len();
        Self::new(Argc, state)
            .update_acc(u8::try_from(len.min(MAX_LEN)).unwrap())
            .update_error(MAX_LEN < len)
    }
    pub fn argv(state: &State) -> Self {
        if let Some(arg) = std::env::args().nth(state.acc as usize) {
            let mut next = *state.page();
            let len = next.write(arg.as_bytes().iter());
            Self::new(Argv, state).update_acc(len).update_page(next)
        } else {
            Self::new(NoArg, state).update_error(true)
        }
    }
}

fn combine(hi: u8, lo: u8) -> u8 {
    const SHIFT: u32 = u8::BITS / 2;
    const MASK: u8 = 0xF;
    ((hi & MASK) << SHIFT) | (lo & MASK)
}
fn rot(val: u8, forward: bool) -> u8 {
    let left = if forward { 1 } else { u8::BITS - 1 };
    let right = u8::BITS - left;
    (val << left) | (val >> right)
}
fn forward(state: &State, shift: u8) -> u8 {
    let (next, _) = state.coord.overflowing_add(shift);
    next
}
fn backward(state: &State, shift: u8) -> u8 {
    let (next, _) = state.coord.overflowing_sub(shift);
    next
}
