use util::BLOCK_SIDE;

#[derive(Default, Clone)]
pub struct Registers {
    pub acc: u8,
    pub block: u8,
    pub coord: u8,
    pub data: u8,
    pub error: bool,
}
impl Registers {
    pub fn imm(&mut self, key: u8) {
        self.data = key;
    }
    pub fn ins(&mut self, digit: u8) {
        self.data = nibble_combine(self.data, digit);
    }
    pub fn swap(&mut self) {
        (self.data, self.acc) = (self.acc, self.data);
    }
    pub fn zero(&mut self) {
        self.acc = 0;
    }
    pub fn high(&mut self) {
        self.data = self.acc;
    }
    pub fn low(&mut self) {
        self.acc = self.data;
    }
    pub fn goto(&mut self) {
        self.coord = self.acc;
    }
    pub fn jump(&mut self) {
        self.block = self.data;
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
    pub fn pos(&mut self) {
        self.acc = self.coord;
    }
    pub fn page(&mut self) {
        self.data = self.block;
    }
    pub fn origin(&mut self) {
        self.coord = 0;
    }
    pub fn start(&mut self) {
        self.block = 0;
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
    pub fn raise(&mut self) {
        self.error = true;
    }
    pub fn neg(&mut self) {
        self.acc = u8::from(self.acc == 0);
    }
    pub fn bool(&mut self) {
        self.acc = u8::from(self.acc != 0);
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
    pub fn save(&self, buf: &mut [u8; 4]) {
        (buf[0], buf[1], buf[2], buf[3]) = (self.data, self.acc, self.block, self.coord);
    }
    pub fn restore(&mut self, buf: [u8; 4]) {
        (self.data, self.acc, self.block, self.coord) = (buf[0], buf[1], buf[2], buf[3]);
    }
    pub fn set_error(&mut self, flag: bool) {
        if flag {
            self.error = true;
        }
    }
    fn set_reg(&mut self, reg: u16) {
        let [data, acc] = reg.to_be_bytes();
        (self.data, self.acc) = (data, acc);
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

#[cfg(test)]
mod register_tests {
    use super::Registers;

    #[test]
    fn imm_ins_test() {
        let mut reg = make();
        reg.imm(0x42);
        assert_eq!(reg.data, 0x42);
        reg.ins(1);
        assert_eq!(reg.data, 0x21);
        reg.ins(0);
        assert_eq!(reg.data, 0x10);
        reg.imm(0);
        default_test(&reg);
    }
    #[test]
    fn swap_zero_test() {
        let mut reg = make();
        reg.imm(42);
        assert_eq!((reg.data, reg.acc), (42, 0));
        reg.swap();
        assert_eq!((reg.data, reg.acc), (0, 42));
        reg.zero();
        default_test(&reg);
    }
    #[test]
    fn high_low_test() {
        let mut reg = make();
        reg.imm(42);
        assert_eq!((reg.data, reg.acc), (42, 0));
        reg.low();
        assert_eq!((reg.data, reg.acc), (42, 42));
        reg.imm(0);
        assert_eq!((reg.data, reg.acc), (0, 42));
        reg.swap();
        assert_eq!((reg.data, reg.acc), (42, 0));
        reg.high();
        default_test(&reg);
    }
    #[test]
    fn goto_jump_test() {
        let mut reg = make();
        reg.imm(0x42);
        reg.low();
        reg.imm(42);
        assert_eq!((reg.data, reg.acc), (42, 0x42));
        reg.goto();
        reg.jump();
        assert_eq!((reg.block, reg.coord), (42, 0x42));
        reg.zero();
        reg.high();
        reg.goto();
        reg.jump();
        default_test(&reg);
    }
    #[test]
    fn left_right_up_down_test() {
        let mut reg = make();
        reg.left();
        assert_eq!(reg.coord, 0xff);
        reg.down();
        assert_eq!(reg.coord, 0x0f);
        reg.right();
        assert_eq!(reg.coord, 0x10);
        reg.up();
        default_test(&reg);
    }
    #[test]
    fn pos_origin_test() {
        let mut reg = make();
        reg.right();
        assert_eq!((reg.acc, reg.coord), (0x00, 0x01));
        reg.down();
        assert_eq!((reg.acc, reg.coord), (0x00, 0x11));
        reg.pos();
        assert_eq!((reg.acc, reg.coord), (0x11, 0x11));
        reg.origin();
        assert_eq!((reg.acc, reg.coord), (0x11, 0x00));
        reg.pos();
        default_test(&reg);
    }
    #[test]
    fn page_start() {
        let mut reg = make();
        reg.imm(42);
        assert_eq!((reg.data, reg.block), (42, 0));
        reg.jump();
        assert_eq!((reg.data, reg.block), (42, 42));
        reg.start();
        assert_eq!((reg.data, reg.block), (42, 0));
        reg.imm(0);
        default_test(&reg);
    }
    #[test]
    fn page_inc_dec() {
        let mut reg = make();
        reg.dec();
        assert_eq!(reg.acc, 0xff);
        reg.inc();
        reg.inc();
        assert_eq!(reg.acc, 0x01);
        reg.zero();
        default_test(&reg);
    }
    #[test]
    fn add_test() {
        let mut reg = make();
        reg.imm(0x42);
        assert_eq!((reg.data, reg.acc), (0x42, 0x00));
        reg.add();
        assert_eq!((reg.data, reg.acc), (0x00, 0x42));
        reg.imm(0xcc);
        assert_eq!((reg.data, reg.acc), (0xcc, 0x42));
        reg.add();
        assert_eq!((reg.data, reg.acc), (0x01, 0x0e));
        reg.add();
        assert_eq!((reg.data, reg.acc), (0x00, 0x0f));
        reg.zero();
        default_test(&reg);
    }
    #[test]
    fn sub_test() {
        let mut reg = make();
        reg.imm(0x42);
        assert_eq!((reg.data, reg.acc), (0x42, 0x00));
        reg.sub();
        assert_eq!((reg.data, reg.acc), (0xff, 0xbe));
        reg.high();
        assert_eq!((reg.data, reg.acc), (0xbe, 0xbe));
        reg.sub();
        default_test(&reg);
    }
    #[test]
    fn mul_test() {
        let mut reg = make();
        reg.imm(0x42);
        assert_eq!((reg.data, reg.acc), (0x42, 0x00));
        reg.inc();
        assert_eq!((reg.data, reg.acc), (0x42, 0x01));
        reg.mul();
        assert_eq!((reg.data, reg.acc), (0x00, 0x42));
        reg.high();
        assert_eq!((reg.data, reg.acc), (0x42, 0x42));
        reg.mul();
        assert_eq!((reg.data, reg.acc), (0x11, 0x04));
        reg.mul();
        assert_eq!((reg.data, reg.acc), (0x00, 0x44));
        reg.mul();
        default_test(&reg);
    }
    #[test]
    fn div_clear_raise_test() {
        let mut reg = make();
        reg.raise();
        assert_eq!((reg.data, reg.acc, reg.error), (0x00, 0x00, true));
        reg.clear();
        assert_eq!((reg.data, reg.acc, reg.error), (0x00, 0x01, false));
        reg.inc();
        reg.inc();
        reg.imm(0x42);
        assert_eq!((reg.data, reg.acc, reg.error), (0x42, 0x03, false));
        reg.swap();
        assert_eq!((reg.data, reg.acc, reg.error), (0x03, 0x42, false));
        reg.div();
        assert_eq!((reg.data, reg.acc, reg.error), (0x00, 0x16, false));
        reg.div();
        assert_eq!((reg.data, reg.acc, reg.error), (0x00, 0x16, true));
        reg.clear();
        assert_eq!((reg.data, reg.acc, reg.error), (0x00, 0x01, false));
        reg.clear();
        default_test(&reg);
    }
    #[test]
    fn neg_bool_test() {
        let mut reg = make();
        reg.neg();
        assert_eq!((reg.data, reg.acc), (0, 1));
        reg.inc();
        assert_eq!((reg.data, reg.acc), (0, 2));
        reg.bool();
        assert_eq!((reg.data, reg.acc), (0, 1));
        reg.neg();
        assert_eq!((reg.data, reg.acc), (0, 0));
        reg.bool();
        default_test(&reg);
    }
    #[test]
    fn eq_lt_gt_test() {
        let mut reg = make();
        reg.eq();
        assert_eq!((reg.data, reg.acc), (0, 1));
        reg.lt();
        assert_eq!((reg.data, reg.acc), (0, 1));
        reg.gt();
        default_test(&reg);
    }
    #[test]
    fn not_and_test() {
        let mut reg = make();
        reg.not();
        assert_eq!((reg.data, reg.acc), (0x00, 0xff));
        reg.imm(0x42);
        assert_eq!((reg.data, reg.acc), (0x42, 0xff));
        reg.and();
        assert_eq!((reg.data, reg.acc), (0x42, 0x42));
        reg.imm(0);
        assert_eq!((reg.data, reg.acc), (0x00, 0x42));
        reg.not();
        assert_eq!((reg.data, reg.acc), (0x00, 0xbd));
        reg.and();
        default_test(&reg);
    }
    #[test]
    fn or_xor_test() {
        let mut reg = make();
        reg.imm(0x42);
        assert_eq!((reg.data, reg.acc), (0x42, 0x00));
        reg.or();
        assert_eq!((reg.data, reg.acc), (0x42, 0x42));
        reg.xor();
        assert_eq!((reg.data, reg.acc), (0x42, 0x00));
        reg.imm(0x00);
        default_test(&reg);
    }
    #[test]
    fn shl_test() {
        let mut reg = make();
        reg.imm(0x5a);
        reg.swap();
        assert_eq!(reg.acc, 0x5a);
        reg.shl();
        assert_eq!(reg.acc, 0xb4);
        reg.shl();
        assert_eq!(reg.acc, 0x68);
        reg.shl();
        assert_eq!(reg.acc, 0xd0);
        reg.shl();
        assert_eq!(reg.acc, 0xa0);
        reg.shl();
        assert_eq!(reg.acc, 0x40);
        reg.shl();
        assert_eq!(reg.acc, 0x80);
        reg.shl();
        default_test(&reg);
    }
    #[test]
    fn shr_test() {
        let mut reg = make();
        reg.imm(0x5a);
        reg.swap();
        assert_eq!(reg.acc, 0x5a);
        reg.shr();
        assert_eq!(reg.acc, 0x2d);
        reg.shr();
        assert_eq!(reg.acc, 0x16);
        reg.shr();
        assert_eq!(reg.acc, 0x0b);
        reg.shr();
        assert_eq!(reg.acc, 0x05);
        reg.shr();
        assert_eq!(reg.acc, 0x02);
        reg.shr();
        assert_eq!(reg.acc, 0x01);
        reg.shr();
        default_test(&reg);
    }
    #[test]
    fn rotl_test() {
        let mut reg = make();
        reg.imm(0x5a);
        reg.swap();
        assert_eq!(reg.acc, 0x5a);
        reg.rotl();
        assert_eq!(reg.acc, 0xb4);
        reg.rotl();
        assert_eq!(reg.acc, 0x69);
        reg.rotl();
        assert_eq!(reg.acc, 0xd2);
        reg.rotl();
        assert_eq!(reg.acc, 0xa5);
        reg.rotl();
        assert_eq!(reg.acc, 0x4b);
        reg.rotl();
        assert_eq!(reg.acc, 0x96);
        reg.rotl();
        assert_eq!(reg.acc, 0x2d);
        reg.rotl();
        assert_eq!(reg.acc, 0x5a);
        reg.zero();
        default_test(&reg);
    }
    #[test]
    fn rotr_test() {
        let mut reg = make();
        reg.imm(0x5a);
        reg.swap();
        assert_eq!(reg.acc, 0x5a);
        reg.rotr();
        assert_eq!(reg.acc, 0x2d);
        reg.rotr();
        assert_eq!(reg.acc, 0x96);
        reg.rotr();
        assert_eq!(reg.acc, 0x4b);
        reg.rotr();
        assert_eq!(reg.acc, 0xa5);
        reg.rotr();
        assert_eq!(reg.acc, 0xd2);
        reg.rotr();
        assert_eq!(reg.acc, 0x69);
        reg.rotr();
        assert_eq!(reg.acc, 0xb4);
        reg.rotr();
        assert_eq!(reg.acc, 0x5a);
        reg.zero();
        default_test(&reg);
    }
    fn make() -> Registers {
        let reg = Registers::default();
        default_test(&reg);
        reg
    }
    fn default_test(reg: &Registers) {
        assert_eq!(reg.data, 0);
        assert_eq!(reg.acc, 0);
        assert_eq!(reg.block, 0);
        assert_eq!(reg.coord, 0);
        assert!(!reg.error);
    }
}
