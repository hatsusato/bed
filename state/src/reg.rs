use super::State;

impl State {
    pub fn imm(&mut self, digit: u8) {
        self.data = combine_nibbles(self.data, digit);
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
    pub fn inc(&mut self) {
        (self.acc, _) = self.acc.overflowing_add(1);
    }
    pub fn dec(&mut self) {
        (self.acc, _) = self.acc.overflowing_sub(1);
    }
    pub fn add(&mut self) {
        self.set_reg((self.acc as u16) + (self.data as u16));
    }
    pub fn sub(&mut self) {
        let (result, _) = (self.acc as u16).overflowing_sub(self.data as u16);
        self.set_reg(result);
    }
    pub fn mul(&mut self) {
        self.set_reg((self.acc as u16) * (self.data as u16));
    }
    pub fn div(&mut self) {
        if self.data == 0 {
            self.raise();
        } else {
            (self.data, self.acc) = (self.acc % self.data, self.acc / self.data);
        }
    }
    pub fn err(&mut self) {
        self.acc = extend(self.error);
        self.error = false;
    }
    pub fn bool(&mut self) {
        self.acc = extend(self.data != 0);
    }
    pub fn eq(&mut self) {
        self.acc = extend(self.data == self.acc);
    }
    pub fn le(&mut self) {
        self.acc = extend(self.data < self.acc);
    }
    pub fn gr(&mut self) {
        self.acc = extend(self.data > self.acc);
    }
    pub fn not(&mut self) {
        self.acc = !self.data;
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
        self.acc <<= 1;
    }
    pub fn shr(&mut self) {
        self.acc >>= 1;
    }
    pub fn rotl(&mut self) {
        self.acc = rot(self.acc, true);
    }
    pub fn rotr(&mut self) {
        self.acc = rot(self.acc, false);
    }
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
