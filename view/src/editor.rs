use exec::Exec;
use screen::Screen;
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
            Exec::print_state(state, last);
            if let Some(key) = screen::Screen::getch() {
                exec.exec(key, state);
                last = key;
            } else {
                return;
            }
        }
    }
}
