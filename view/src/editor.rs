use screen::Screen;
use vm::Machine;

#[derive(Default)]
pub struct Editor {
    _screen: Screen,
    vm: Machine,
}
impl Editor {
    pub fn run(&mut self) {
        loop {
            self.vm.print();
            match Screen::getch() {
                Some(key) => self.vm.execute(key),
                None => return,
            }
        }
    }
}
