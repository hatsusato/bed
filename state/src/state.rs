use util::Block;

pub struct State {
    block: Block<u8>,
    cursor: u8,
}
impl State {
    pub fn new() -> Self {
        Self {
            block: Block::new(0),
            cursor: 0,
        }
    }
    pub fn push(&mut self, key: char) {
        self.block[self.cursor] = key as u8;
        self.cursor += 1;
    }
    pub fn block(&self) -> &Block<u8> {
        return &self.block;
    }
}
