use util::BLOCK_SIDE;

#[derive(Default, Clone, Copy)]
pub struct Bank {
    pub acc: u8,
    pub block: u8,
    pub coord: u8,
    pub data: u8,
    pub error: bool,
}
impl Bank {
    pub fn set_len(&mut self, len: Option<usize>) {
        if let Some(len) = len {
            let result = u8::try_from(len);
            self.acc = result.unwrap_or(u8::MAX);
            if result.is_ok() {
                return;
            }
        }
        self.error = true;
    }
    fn set_reg(&mut self, reg: u16) {
        let [data, acc] = reg.to_be_bytes();
        (self.data, self.acc) = (data, acc);
    }
    pub fn imm(&mut self, digit: u8) {
        self.data = nibble_combine(self.data, digit);
    }
    pub fn swap(&mut self) {
        (self.data, self.acc) = (self.acc, self.data);
    }
    pub fn hi(&mut self) {
        self.data = self.acc;
    }
    pub fn lo(&mut self) {
        self.acc = self.data;
    }
    pub fn goto(&mut self) {
        self.coord = self.acc;
    }
    pub fn jump(&mut self) {
        self.block = self.data;
    }
    pub fn pos(&mut self) {
        (self.data, self.acc) = (self.block, self.coord);
    }
    pub fn left(&mut self) {
        self.coord = overflow_sub(self.coord, 1);
    }
    pub fn right(&mut self) {
        self.coord = overflow_add(self.coord, 1);
    }
    pub fn up(&mut self) {
        self.coord = overflow_sub(self.coord, BLOCK_SIDE);
    }
    pub fn down(&mut self) {
        self.coord = overflow_add(self.coord, BLOCK_SIDE);
    }
    pub fn inc(&mut self) {
        self.acc = overflow_add(self.acc, 1);
    }
    pub fn dec(&mut self) {
        self.acc = overflow_sub(self.acc, 1);
    }
    pub fn add(&mut self) {
        self.set_reg(u16::from(self.acc) + u16::from(self.data));
    }
    pub fn sub(&mut self) {
        self.set_reg(overflow_sub16(u16::from(self.acc), self.data.into()));
    }
    pub fn mul(&mut self) {
        self.set_reg(u16::from(self.acc) * u16::from(self.data));
    }
    pub fn div(&mut self) {
        if self.data == 0 {
            self.error = true;
        } else {
            (self.data, self.acc) = (self.acc % self.data, self.acc / self.data);
        }
    }
    pub fn clear(&mut self) {
        (self.acc, self.error) = (u8::from(self.error), false);
    }
    pub fn neg(&mut self) {
        self.acc = u8::from(self.data == 0);
    }
    pub fn bool(&mut self) {
        self.acc = u8::from(self.data != 0);
    }
    pub fn eq(&mut self) {
        self.acc = u8::from(self.data == self.acc);
    }
    pub fn lt(&mut self) {
        self.acc = u8::from(self.data < self.acc);
    }
    pub fn gt(&mut self) {
        self.acc = u8::from(self.data > self.acc);
    }
    pub fn not(&mut self) {
        self.acc = !self.acc;
    }
    pub fn and(&mut self) {
        self.acc &= self.data;
    }
    pub fn or(&mut self) {
        self.acc |= self.data;
    }
    pub fn xor(&mut self) {
        self.acc ^= self.data;
    }
    pub fn shl(&mut self) {
        self.acc = shift(self.acc, true);
    }
    pub fn shr(&mut self) {
        self.acc = shift(self.acc, false);
    }
    pub fn rotl(&mut self) {
        self.acc = rot(self.acc, true);
    }
    pub fn rotr(&mut self) {
        self.acc = rot(self.acc, false);
    }
    pub fn argc(&mut self) {
        self.set_len(Some(std::env::args().len()));
    }
    pub fn argv(&mut self, arg: &Option<String>) {
        self.set_len(arg.as_ref().map(String::len));
    }
    pub fn esc(&mut self, key: char) {
        if let Ok(data) = u8::try_from(key) {
            self.data = data;
        }
    }
}

const NIBBLE_SHIFT: u32 = u8::BITS / 2;
fn nibble_cast(bits: u8) -> u8 {
    const MASK: u8 = (1 << NIBBLE_SHIFT) - 1;
    bits & MASK
}
fn nibble_combine(hi: u8, lo: u8) -> u8 {
    (nibble_cast(hi) << NIBBLE_SHIFT) | (nibble_cast(lo))
}
fn shift(val: u8, forward: bool) -> u8 {
    let (val, _) = if forward {
        val.overflowing_shl(1)
    } else {
        val.overflowing_shr(1)
    };
    val
}
fn rot(val: u8, forward: bool) -> u8 {
    let left = if forward { 1 } else { u8::BITS - 1 };
    let right = u8::BITS - left;
    (val << left) | (val >> right)
}
fn overflow_add(lhs: u8, rhs: u8) -> u8 {
    let (val, _) = lhs.overflowing_add(rhs);
    val
}
fn overflow_sub(lhs: u8, rhs: u8) -> u8 {
    let (val, _) = lhs.overflowing_sub(rhs);
    val
}
fn overflow_sub16(lhs: u16, rhs: u16) -> u16 {
    let (val, _) = lhs.overflowing_sub(rhs);
    val
}
