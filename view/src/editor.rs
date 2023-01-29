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
        let mut exec = Exec::default();
        loop {
            let state = self.state.get_mut();
            exec.print(state);
            if let Some(key) = Screen::getch() {
                exec.exec(key, state);
            } else {
                return;
            }
        }
    }
}
