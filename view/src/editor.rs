use screen::Screen;
use vm::Exec;

#[derive(Default)]
pub struct Editor {
    _screen: Screen,
    exec: Exec,
}
impl Editor {
    pub fn run(&mut self) {
        loop {
            self.exec.print();
            match Screen::getch() {
                Some(key) => self.exec.exec(key),
                None => return,
            }
        }
    }
}
