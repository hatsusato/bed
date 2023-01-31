use screen::Screen;
use vm::Machine;

#[derive(Default)]
pub struct Editor {
    _screen: Screen,
    exec: Machine,
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
