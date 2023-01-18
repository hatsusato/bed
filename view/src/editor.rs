use crate::screen::Screen;

struct Editor {
    _screen: Screen,
}
impl Editor {
    fn run(&self) {
        while let Some(key) = Screen::getch() {
            Screen::print_string(format!("{}", key));
        }
    }
}
