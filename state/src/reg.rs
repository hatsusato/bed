use crate::State;

impl State {
    pub fn imm(&mut self, digit: u8) {
        const DIGIT_BITS: u8 = 4;
        const DIGIT_MAX: u8 = 0xF;
        let hi = self.acc & DIGIT_MAX;
        let lo = digit & DIGIT_MAX;
        self.acc = (hi << DIGIT_BITS) | lo;
    }
    pub fn add(&mut self) {
        let (val, carry) = self.acc.overflowing_add(self.data);
        self.acc = val;
        self.data = if carry { 1 } else { 0 };
    }
    pub fn sub(&mut self) {
        let (val, carry) = self.acc.overflowing_sub(self.data);
        self.acc = val;
        self.data = if carry { u8::MAX } else { 0 };
    }
    pub fn mul(&mut self) {
        const OFFSET: u16 = 1 << u8::BITS;
        let val = (self.data as u16) * (self.acc as u16);
        self.acc = (val / OFFSET) as u8;
        self.data = (val % OFFSET) as u8;
    }
    pub fn div(&mut self) {
        if 0 < self.data {
            (self.acc, self.data) = (self.acc / self.data, self.acc % self.data);
        }
    }
}
