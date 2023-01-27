use crate::screen::Screen;
use exec::Exec;
use state::State;
use std::cell::Cell;

pub struct Editor {
    _screen: Screen,
    state: Cell<State>,
}
impl Editor {
    pub fn new() -> Self {
        Self {
            _screen: Screen::new(),
            state: Cell::new(Default::default()),
        }
    }
    pub fn run(&mut self) {
        let mut last = ' ';
        loop {
            let state = self.state.get_mut();
            Screen::print_state(state, last);
            match Screen::getch() {
                Some(key) => {
                    last = key;
                    Exec::exec(key, state);
                }
                None => return,
            }
        }
    }
}
