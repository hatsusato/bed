use util::BLOCK_SIDE;

#[derive(Default, Clone, Debug)]
pub struct Registers {
    pub data: u8,
    pub accum: u8,
    pub block: u8,
    pub cell: u8,
    pub error: bool,
}
impl Registers {
    pub fn insert(&mut self, digit: u8) {
        self.accum = nibble_combine(self.accum, digit);
    }
    pub fn high(&mut self) {
        self.data = self.accum;
    }
    pub fn low(&mut self) {
        self.accum = self.data;
    }
    pub fn swap(&mut self) {
        (self.data, self.accum) = (self.accum, self.data);
    }
    pub fn zero(&mut self) {
        self.data = 0;
    }
    pub fn delete(&mut self) {
        self.accum = 0;
    }
    pub fn right(&mut self) {
        self.cell = overflow_add(self.cell, 1);
    }
    pub fn left(&mut self) {
        self.cell = overflow_sub(self.cell, 1);
    }
    pub fn down(&mut self) {
        self.cell = overflow_add(self.cell, BLOCK_SIDE);
    }
    pub fn up(&mut self) {
        self.cell = overflow_sub(self.cell, BLOCK_SIDE);
    }
    pub fn goto(&mut self) {
        self.cell = self.data;
    }
    pub fn jump(&mut self) {
        self.block = self.data;
    }
    pub fn coord(&mut self) {
        self.data = self.cell;
    }
    pub fn page(&mut self) {
        self.data = self.block;
    }
    pub fn origin(&mut self) {
        self.cell = 0;
    }
    pub fn begin(&mut self) {
        self.block = 0;
    }
    pub fn add(&mut self) {
        let val = u16::from(self.accum) + u16::from(self.data);
        [self.data, self.accum] = val.to_be_bytes();
    }
    pub fn sub(&mut self) {
        let (val, _) = u16::from(self.accum).overflowing_sub(self.data.into());
        [self.data, self.accum] = val.to_be_bytes();
    }
    pub fn mul(&mut self) {
        let val = u16::from(self.accum) * u16::from(self.data);
        [self.data, self.accum] = val.to_be_bytes();
    }
    pub fn div(&mut self) {
        if self.data == 0 {
            self.error = true;
        } else {
            (self.data, self.accum) = (self.accum % self.data, self.accum / self.data);
        }
    }
    pub fn inc(&mut self) {
        self.accum = overflow_add(self.accum, 1);
    }
    pub fn dec(&mut self) {
        self.accum = overflow_sub(self.accum, 1);
    }
    pub fn shl(&mut self) {
        self.accum = shift(self.accum, true);
    }
    pub fn shr(&mut self) {
        self.accum = shift(self.accum, false);
    }
    pub fn rotl(&mut self) {
        self.accum = rot(self.accum, true);
    }
    pub fn rotr(&mut self) {
        self.accum = rot(self.accum, false);
    }
    pub fn and(&mut self) {
        self.accum &= self.data;
    }
    pub fn or(&mut self) {
        self.accum |= self.data;
    }
    pub fn xor(&mut self) {
        self.accum ^= self.data;
    }
    pub fn not(&mut self) {
        self.accum = !self.accum;
    }
    pub fn neg(&mut self) {
        self.accum = u8::from(self.accum == 0);
    }
    pub fn bool(&mut self) {
        self.accum = u8::from(self.accum != 0);
    }
    pub fn eq(&mut self) {
        self.accum = u8::from(self.data == self.accum);
    }
    pub fn lt(&mut self) {
        self.accum = u8::from(self.data < self.accum);
    }
    pub fn gt(&mut self) {
        self.accum = u8::from(self.data > self.accum);
    }
    pub fn check(&mut self) {
        self.accum = u8::from(self.error);
    }
    pub fn clear(&mut self) {
        self.error = false;
    }
    pub fn getchar<F: FnOnce() -> Option<u8>>(&mut self, producer: F) {
        match producer() {
            Some(data) => self.data = data,
            None => self.error = true,
        }
    }
    pub fn putchar<F: FnOnce(u8) -> Option<()>>(&mut self, consumer: F) {
        if consumer(self.data).is_none() {
            self.error = true;
        }
    }
    pub fn get_descriptor<F: FnOnce() -> u8>(&mut self, producer: F) {
        self.accum = producer();
    }
    pub fn set_descriptor<F: FnOnce(u8)>(&self, consumer: F) {
        consumer(self.accum);
    }
}

fn nibble_combine(hi: u8, lo: u8) -> u8 {
    const NIBBLE_SHIFT: u32 = u8::BITS / 2;
    const MASK: u8 = (1 << NIBBLE_SHIFT) - 1;
    ((hi & MASK) << NIBBLE_SHIFT) | (lo & MASK)
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

#[cfg(test)]
mod register_tests {
    use super::Registers;

    #[test]
    fn swap_zero_delete_test() {
        let mut reg = make();
        reg.insert(4);
        reg.insert(2);
        assert_eq!((reg.data, reg.accum), (0, 0x42));
        reg.swap();
        assert_eq!((reg.data, reg.accum), (0x42, 0));
        reg.insert(4);
        reg.insert(2);
        assert_eq!((reg.data, reg.accum), (0x42, 0x42));
        reg.delete();
        assert_eq!((reg.data, reg.accum), (0x42, 0));
        reg.zero();
        zero_test(&reg);
    }
    #[test]
    fn high_low_test() {
        let mut reg = make();
        reg.insert(4);
        reg.insert(2);
        assert_eq!((reg.data, reg.accum), (0, 0x42));
        reg.high();
        assert_eq!((reg.data, reg.accum), (0x42, 0x42));
        reg.zero();
        assert_eq!((reg.data, reg.accum), (0, 0x42));
        reg.swap();
        assert_eq!((reg.data, reg.accum), (0x42, 0));
        reg.high();
        zero_test(&reg);
    }
    #[test]
    fn goto_jump_test() {
        let mut reg = make();
        reg.insert(4);
        reg.insert(2);
        assert_eq!((reg.data, reg.accum, reg.block, reg.cell), (0, 0x42, 0, 0));
        reg.swap();
        assert_eq!((reg.data, reg.accum, reg.block, reg.cell), (0x42, 0, 0, 0));
        reg.goto();
        assert_eq!(
            (reg.data, reg.accum, reg.block, reg.cell),
            (0x42, 0, 0, 0x42)
        );
        reg.insert(3);
        reg.insert(1);
        assert_eq!(
            (reg.data, reg.accum, reg.block, reg.cell),
            (0x42, 0x31, 0, 0x42)
        );
        reg.swap();
        assert_eq!(
            (reg.data, reg.accum, reg.block, reg.cell),
            (0x31, 0x42, 0, 0x42)
        );
        reg.jump();
        assert_eq!(
            (reg.data, reg.accum, reg.block, reg.cell),
            (0x31, 0x42, 0x31, 0x42)
        );
        reg.zero();
        assert_eq!(
            (reg.data, reg.accum, reg.block, reg.cell),
            (0, 0x42, 0x31, 0x42)
        );
        reg.goto();
        assert_eq!(
            (reg.data, reg.accum, reg.block, reg.cell),
            (0, 0x42, 0x31, 0)
        );
        reg.jump();
        assert_eq!((reg.data, reg.accum, reg.block, reg.cell), (0, 0x42, 0, 0));
        reg.delete();
        zero_test(&reg);
    }
    #[test]
    fn left_right_up_down_test() {
        let mut reg = make();
        reg.left();
        assert_eq!(reg.cell, 0xff);
        reg.down();
        assert_eq!(reg.cell, 0x0f);
        reg.right();
        assert_eq!(reg.cell, 0x10);
        reg.up();
        zero_test(&reg);
    }
    #[test]
    fn pos_origin_test() {
        let mut reg = make();
        reg.right();
        assert_eq!((reg.data, reg.cell), (0x00, 0x01));
        reg.down();
        assert_eq!((reg.data, reg.cell), (0x00, 0x11));
        reg.coord();
        assert_eq!((reg.data, reg.cell), (0x11, 0x11));
        reg.origin();
        assert_eq!((reg.data, reg.cell), (0x11, 0x00));
        reg.coord();
        zero_test(&reg);
    }
    #[test]
    fn page_start() {
        let mut reg = make();
        reg.insert(4);
        reg.insert(2);
        assert_eq!((reg.data, reg.accum, reg.block), (0, 0x42, 0));
        reg.swap();
        assert_eq!((reg.data, reg.accum, reg.block), (0x42, 0, 0));
        reg.jump();
        assert_eq!((reg.data, reg.accum, reg.block), (0x42, 0, 0x42));
        reg.begin();
        assert_eq!((reg.data, reg.accum, reg.block), (0x42, 0, 0));
        reg.zero();
        zero_test(&reg);
    }
    #[test]
    fn page_inc_dec() {
        let mut reg = make();
        reg.dec();
        assert_eq!(reg.accum, 0xff);
        reg.inc();
        reg.inc();
        assert_eq!(reg.accum, 0x01);
        reg.delete();
        zero_test(&reg);
    }
    #[test]
    fn add_test() {
        let mut reg = make();
        reg.insert(4);
        reg.insert(2);
        assert_eq!((reg.data, reg.accum), (0x00, 0x42));
        reg.swap();
        assert_eq!((reg.data, reg.accum), (0x42, 0x00));
        reg.add();
        assert_eq!((reg.data, reg.accum), (0x00, 0x42));
        reg.swap();
        assert_eq!((reg.data, reg.accum), (0x42, 0x00));
        reg.insert(0xc);
        reg.insert(0xc);
        assert_eq!((reg.data, reg.accum), (0x42, 0xcc));
        reg.add();
        assert_eq!((reg.data, reg.accum), (0x01, 0x0e));
        reg.add();
        assert_eq!((reg.data, reg.accum), (0x00, 0x0f));
        reg.delete();
        zero_test(&reg);
    }
    #[test]
    fn sub_test() {
        let mut reg = make();
        reg.insert(4);
        reg.insert(2);
        assert_eq!((reg.data, reg.accum), (0x00, 0x42));
        reg.swap();
        assert_eq!((reg.data, reg.accum), (0x42, 0x00));
        reg.sub();
        assert_eq!((reg.data, reg.accum), (0xff, 0xbe));
        reg.high();
        assert_eq!((reg.data, reg.accum), (0xbe, 0xbe));
        reg.sub();
        zero_test(&reg);
    }
    #[test]
    fn mul_test() {
        let mut reg = make();
        reg.insert(4);
        reg.insert(2);
        assert_eq!((reg.data, reg.accum), (0x00, 0x42));
        reg.swap();
        assert_eq!((reg.data, reg.accum), (0x42, 0x00));
        reg.inc();
        assert_eq!((reg.data, reg.accum), (0x42, 0x01));
        reg.mul();
        assert_eq!((reg.data, reg.accum), (0x00, 0x42));
        reg.high();
        assert_eq!((reg.data, reg.accum), (0x42, 0x42));
        reg.mul();
        assert_eq!((reg.data, reg.accum), (0x11, 0x04));
        reg.mul();
        assert_eq!((reg.data, reg.accum), (0x00, 0x44));
        reg.mul();
        zero_test(&reg);
    }
    #[test]
    fn div_clear_raise_test() {
        let mut reg = make();
        reg.insert(4);
        reg.insert(2);
        assert_eq!((reg.data, reg.accum, reg.error), (0, 0x42, false));
        reg.swap();
        assert_eq!((reg.data, reg.accum, reg.error), (0x42, 0, false));
        reg.inc();
        reg.inc();
        reg.inc();
        assert_eq!((reg.data, reg.accum, reg.error), (0x42, 0x03, false));
        reg.swap();
        assert_eq!((reg.data, reg.accum, reg.error), (0x03, 0x42, false));
        reg.div();
        assert_eq!((reg.data, reg.accum, reg.error), (0x00, 0x16, false));
        reg.div();
        assert_eq!((reg.data, reg.accum, reg.error), (0x00, 0x16, true));
        reg.check();
        assert_eq!((reg.data, reg.accum, reg.error), (0x00, 0x01, true));
        reg.clear();
        assert_eq!((reg.data, reg.accum, reg.error), (0x00, 0x01, false));
        reg.delete();
        zero_test(&reg);
    }
    #[test]
    fn neg_bool_test() {
        let mut reg = make();
        reg.neg();
        assert_eq!((reg.data, reg.accum), (0, 1));
        reg.inc();
        assert_eq!((reg.data, reg.accum), (0, 2));
        reg.bool();
        assert_eq!((reg.data, reg.accum), (0, 1));
        reg.neg();
        assert_eq!((reg.data, reg.accum), (0, 0));
        reg.bool();
        zero_test(&reg);
    }
    #[test]
    fn eq_lt_gt_test() {
        let mut reg = make();
        reg.eq();
        assert_eq!((reg.data, reg.accum), (0, 1));
        reg.lt();
        assert_eq!((reg.data, reg.accum), (0, 1));
        reg.gt();
        zero_test(&reg);
    }
    #[test]
    fn not_and_test() {
        let mut reg = make();
        reg.not();
        assert_eq!((reg.data, reg.accum), (0x00, 0xff));
        reg.swap();
        assert_eq!((reg.data, reg.accum), (0xff, 0x00));
        reg.insert(4);
        reg.insert(2);
        assert_eq!((reg.data, reg.accum), (0xff, 0x42));
        reg.swap();
        assert_eq!((reg.data, reg.accum), (0x42, 0xff));
        reg.and();
        assert_eq!((reg.data, reg.accum), (0x42, 0x42));
        reg.zero();
        assert_eq!((reg.data, reg.accum), (0x00, 0x42));
        reg.not();
        assert_eq!((reg.data, reg.accum), (0x00, 0xbd));
        reg.and();
        zero_test(&reg);
    }
    #[test]
    fn or_xor_test() {
        let mut reg = make();
        reg.insert(4);
        reg.insert(2);
        assert_eq!((reg.data, reg.accum), (0x00, 0x42));
        reg.swap();
        assert_eq!((reg.data, reg.accum), (0x42, 0x00));
        reg.or();
        assert_eq!((reg.data, reg.accum), (0x42, 0x42));
        reg.xor();
        assert_eq!((reg.data, reg.accum), (0x42, 0x00));
        reg.high();
        zero_test(&reg);
    }
    #[test]
    fn shl_test() {
        let mut reg = make();
        reg.insert(5);
        reg.insert(0xa);
        assert_eq!(reg.accum, 0x5a);
        reg.shl();
        assert_eq!(reg.accum, 0xb4);
        reg.shl();
        assert_eq!(reg.accum, 0x68);
        reg.shl();
        assert_eq!(reg.accum, 0xd0);
        reg.shl();
        assert_eq!(reg.accum, 0xa0);
        reg.shl();
        assert_eq!(reg.accum, 0x40);
        reg.shl();
        assert_eq!(reg.accum, 0x80);
        reg.shl();
        zero_test(&reg);
    }
    #[test]
    fn shr_test() {
        let mut reg = make();
        reg.insert(5);
        reg.insert(0xa);
        assert_eq!(reg.accum, 0x5a);
        reg.shr();
        assert_eq!(reg.accum, 0x2d);
        reg.shr();
        assert_eq!(reg.accum, 0x16);
        reg.shr();
        assert_eq!(reg.accum, 0x0b);
        reg.shr();
        assert_eq!(reg.accum, 0x05);
        reg.shr();
        assert_eq!(reg.accum, 0x02);
        reg.shr();
        assert_eq!(reg.accum, 0x01);
        reg.shr();
        zero_test(&reg);
    }
    #[test]
    fn rotl_test() {
        let mut reg = make();
        reg.insert(5);
        reg.insert(0xa);
        assert_eq!(reg.accum, 0x5a);
        reg.rotl();
        assert_eq!(reg.accum, 0xb4);
        reg.rotl();
        assert_eq!(reg.accum, 0x69);
        reg.rotl();
        assert_eq!(reg.accum, 0xd2);
        reg.rotl();
        assert_eq!(reg.accum, 0xa5);
        reg.rotl();
        assert_eq!(reg.accum, 0x4b);
        reg.rotl();
        assert_eq!(reg.accum, 0x96);
        reg.rotl();
        assert_eq!(reg.accum, 0x2d);
        reg.rotl();
        assert_eq!(reg.accum, 0x5a);
        reg.delete();
        zero_test(&reg);
    }
    #[test]
    fn rotr_test() {
        let mut reg = make();
        reg.insert(5);
        reg.insert(0xa);
        assert_eq!(reg.accum, 0x5a);
        reg.rotr();
        assert_eq!(reg.accum, 0x2d);
        reg.rotr();
        assert_eq!(reg.accum, 0x96);
        reg.rotr();
        assert_eq!(reg.accum, 0x4b);
        reg.rotr();
        assert_eq!(reg.accum, 0xa5);
        reg.rotr();
        assert_eq!(reg.accum, 0xd2);
        reg.rotr();
        assert_eq!(reg.accum, 0x69);
        reg.rotr();
        assert_eq!(reg.accum, 0xb4);
        reg.rotr();
        assert_eq!(reg.accum, 0x5a);
        reg.delete();
        zero_test(&reg);
    }
    fn make() -> Registers {
        let reg = Registers::default();
        zero_test(&reg);
        reg
    }
    fn zero_test(reg: &Registers) {
        assert_eq!(reg.data, 0);
        assert_eq!(reg.accum, 0);
        assert_eq!(reg.block, 0);
        assert_eq!(reg.cell, 0);
        assert!(!reg.error);
    }
}
