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
            exec.print();
            if let Some(key) = Screen::getch() {
                exec.exec(key);
            } else {
                return;
            }
        }
    }
}
