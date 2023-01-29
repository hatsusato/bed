use crate::screen::Screen;
use exec::Exec;
use state::State;
use std::cell::Cell;

#[derive(Default)]
pub struct Editor {
    _screen: Screen,
    state: Cell<State>,
}
impl Editor {
    pub fn run(&mut self) {
        let mut last = ' ';
        let mut exec = Exec::default();
        loop {
            let state = self.state.get_mut();
            Screen::print_state(state, last);
            if let Some(key) = Screen::getch() {
                exec.exec(key, state);
                last = key;
            } else {
                return;
            }
        }
    }
}
