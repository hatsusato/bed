use super::State;

const BLOCK_SIDE: u8 = 16;

impl State {
    pub fn right(&mut self) {
        self.forward(1);
    }
    pub fn left(&mut self) {
        self.backward(1);
    }
    pub fn down(&mut self) {
        self.forward(BLOCK_SIDE);
    }
    pub fn up(&mut self) {
        self.backward(BLOCK_SIDE);
    }
    pub fn pos(&mut self) {
        (self.data, self.acc) = (self.block, self.coord);
    }
    pub fn goto(&mut self) {
        self.coord = self.acc;
    }
    pub fn jump(&mut self) {
        self.block = self.data;
    }
    fn forward(&mut self, shift: u8) {
        (self.coord, _) = self.coord.overflowing_add(shift);
    }
    fn backward(&mut self, shift: u8) {
        (self.coord, _) = self.coord.overflowing_sub(shift)
    }
}
