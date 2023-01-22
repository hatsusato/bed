use super::State;

impl State {
    pub fn imm(&mut self, digit: u8) {
        self.data = combine_nibbles(self.data, digit);
    }
    pub fn swap(&mut self) {
        self.set_reg(self.acc, self.data);
    }
    pub fn inc(&mut self) {
        (self.acc, _) = self.acc.overflowing_add(1);
    }
    pub fn dec(&mut self) {
        (self.acc, _) = self.acc.overflowing_sub(1);
    }
    pub fn add(&mut self) {
        let (result, carry) = self.acc.overflowing_add(self.data);
        self.set_reg(extend(carry), result);
    }
    pub fn sub(&mut self) {
        let (result, carry) = self.acc.overflowing_sub(self.data);
        self.set_reg(u8::MAX * extend(carry), result);
    }
    pub fn mul(&mut self) {
        const SHIFT: u32 = u8::BITS;
        let val = (self.acc as u16) * (self.data as u16);
        self.set_reg(trunc(val >> SHIFT), trunc(val));
    }
    pub fn div(&mut self) {
        if self.data == 0 {
            self.raise();
        } else {
            self.set_reg(self.acc % self.data, self.acc / self.data);
        }
    }
    pub fn equal(&mut self) {
        self.acc = extend(self.data == self.acc);
    }
    pub fn less(&mut self) {
        self.acc = extend(self.data < self.acc);
    }
    pub fn greater(&mut self) {
        self.acc = extend(self.data > self.acc);
    }
}

fn combine_nibbles(hi: u8, lo: u8) -> u8 {
    const SHIFT: u32 = u8::BITS / 2;
    const MASK: u8 = 0xF;
    ((hi & MASK) << SHIFT) | (lo & MASK)
}
fn trunc(val: u16) -> u8 {
    const MASK: u16 = u8::MAX as u16;
    (val & MASK) as u8
}
fn extend(cond: bool) -> u8 {
    if cond {
        1
    } else {
        0
    }
}
