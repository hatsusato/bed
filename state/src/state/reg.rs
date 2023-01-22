use super::State;

impl State {
    pub fn imm(&mut self, digit: u8) {
        const DIGIT_BITS: u8 = 4;
        const DIGIT_MAX: u8 = 0xF;
        let hi = self.acc & DIGIT_MAX;
        let lo = digit & DIGIT_MAX;
        self.acc = (hi << DIGIT_BITS) | lo;
    }
    pub fn swap(&mut self) {
        (self.data, self.acc) = (self.acc, self.data);
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
        const SHIFT: u16 = 1 << u8::BITS;
        self.set_reg((self.data as u16) * (self.acc as u16), SHIFT);
    }
    pub fn div(&mut self) {
        if self.data == 0 {
            self.raise();
        } else {
            self.set_reg(self.acc as u16, self.data as u16);
        }
    }
    fn set_reg(&mut self, dividend: u16, divisor: u16) {
        let (quo, rem) = (dividend / divisor, dividend % divisor);
        (self.data, self.acc) = (quo as u8, rem as u8);
    }
}
