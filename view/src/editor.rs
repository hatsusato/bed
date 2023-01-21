use crate::screen::Screen;
use util::Block;

pub struct Editor {
    _screen: Screen,
    block: Block<u8>,
    cursor: u8,
}
impl Editor {
    pub fn new() -> Self {
        Self {
            _screen: Screen::new(),
            block: Block::new(0),
            cursor: 0,
        }
    }
    pub fn run(&mut self) {
        loop {
            Screen::print_block(&self.block);
            match Screen::getch() {
                Some(key) => self.block[self.cursor] = key as u8,
                None => return,
            }
            self.cursor += 1;
        }
    }
}
