use vm::Machine;

#[derive(Default)]
pub struct Exec {
    vm: Machine,
}
impl Exec {
    pub fn exec(&mut self, key: char) {
        self.vm.exec(key);
    }
    pub fn print(&self) {
        self.vm.print();
    }
}
