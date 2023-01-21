use crate::screen::Screen;
use state::State;

pub struct Editor {
    _screen: Screen,
    state: State,
}
impl Editor {
    pub fn new() -> Self {
        Self {
            _screen: Screen::new(),
            state: State::new(),
        }
    }
    pub fn run(&mut self) {
        loop {
            Screen::print_block(self.state.block());
            match Screen::getch() {
                Some(key) => self.state.push(key),
                None => return,
            }
        }
    }
}
