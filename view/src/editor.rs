use crate::screen::Screen;

pub struct Editor {
    _screen: Screen,
}
impl Editor {
    pub fn new() -> Self {
        Self {
            _screen: Screen::new(),
        }
    }
    pub fn run(&self) {
        while let Some(key) = Screen::getch() {
            Screen::print_string(format!("{}", key));
        }
    }
}
