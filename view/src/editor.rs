use exec::Exec;
use screen::Screen;

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
                Some(key) => self.exec.execute(key),
                None => return,
            }
        }
    }
}
