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
}

const NIBBLE_SHIFT: u32 = u8::BITS / 2;
fn nibble_cast(bits: u8) -> u8 {
    const MASK: u8 = (1 << NIBBLE_SHIFT) - 1;
    bits & MASK
}
fn nibble_combine(hi: u8, lo: u8) -> u8 {
    (nibble_cast(hi) << NIBBLE_SHIFT) | (nibble_cast(lo))
}
