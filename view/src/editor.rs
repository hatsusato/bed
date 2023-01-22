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
            state: Cell::new(State::new()),
        }
    }
    pub fn run(&mut self) {
        loop {
            let state = self.state.get_mut();
            Screen::print_state(state);
            match Screen::getch() {
                Some(key) => Exec::exec(key, state),
                None => return,
            }
        }
    }
}
