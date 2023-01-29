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
    fn update_data(&mut self, data: u8) -> &mut Self {
        self.data = data;
        self
    }
    fn update_acc(&mut self, acc: u8) -> &mut Self {
        self.acc = acc;
        self
    }
    fn update_block(&mut self, block: u8) -> &mut Self {
        self.block = block;
        self
    }
    fn update_coord(&mut self, coord: u8) -> &mut Self {
        self.coord = coord;
        self
    }
    fn set_error(&mut self, error: bool) -> &mut Self {
        if error {
            self.error = true;
        }
        self
    }
    fn update_reg(&mut self, reg: u16) -> &mut Self {
        let [data, acc] = reg.to_be_bytes();
        self.update_data(data).update_acc(acc)
    }
    pub fn imm(&mut self, digit: u8) {
        let (hi, lo) = (self.data, digit);
        self.update_data(nibble_combine(hi, lo));
    }
    pub fn swap(&mut self) {
        let (data, acc) = (self.data, self.acc);
        self.update_data(acc).update_acc(data);
    }
    pub fn hi(&mut self) {
        let acc = self.acc;
        self.update_data(acc);
    }
    pub fn lo(&mut self) {
        let data = self.data;
        self.update_acc(data);
    }
    pub fn goto(&mut self) {
        let coord = self.acc;
        self.update_coord(coord);
    }
    pub fn jump(&mut self) {
        let block = self.data;
        self.update_block(block);
    }
    pub fn pos(&mut self) {
        let (data, acc) = (self.block, self.coord);
        self.update_data(data).update_acc(acc);
    }
    pub fn left(&mut self) {
        let (coord, _) = self.coord.overflowing_sub(1);
        self.update_coord(coord);
    }
    pub fn right(&mut self) {
        let (coord, _) = self.coord.overflowing_add(1);
        self.update_coord(coord);
    }
    pub fn up(&mut self) {
        let (coord, _) = self.coord.overflowing_sub(BLOCK_SIDE);
        self.update_coord(coord);
    }
    pub fn down(&mut self) {
        let (coord, _) = self.coord.overflowing_add(BLOCK_SIDE);
        self.update_coord(coord);
    }
    pub fn inc(&mut self) {
        let (acc, _) = self.acc.overflowing_add(1);
        self.update_acc(acc);
    }
    pub fn dec(&mut self) {
        let (acc, _) = self.acc.overflowing_sub(1);
        self.update_acc(acc);
    }
    pub fn add(&mut self) {
        let reg = u16::from(self.acc) + u16::from(self.data);
        self.update_reg(reg);
    }
    pub fn sub(&mut self) {
        let (reg, _) = u16::from(self.acc).overflowing_sub(self.data.into());
        self.update_reg(reg);
    }
    pub fn mul(&mut self) {
        let reg = u16::from(self.acc) * u16::from(self.data);
        self.update_reg(reg);
    }
    pub fn div(&mut self) {
        if self.data != 0 {
            let (quo, rem) = (self.acc / self.data, self.acc % self.data);
            self.update_data(rem).update_acc(quo);
        }
        self.set_error(self.data == 0);
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
