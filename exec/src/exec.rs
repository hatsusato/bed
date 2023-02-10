use vm::{Ctrl, Inst, Machine};

#[derive(Default)]
pub struct Exec {
    ctrl: Ctrl,
    quote: String,
    vm: Machine,
    last: char,
}
impl Exec {
    pub fn execute(&mut self, key: char) {
        match self.ctrl {
            Ctrl::Enter => self.execute_enter(key),
            Ctrl::Quote => self.execute_quote(key),
            Ctrl::Ignore => self.execute_ignore(key),
            Ctrl::While => (),
            Ctrl::Direct => (),
            Ctrl::Call => (),
            Ctrl::Define => (),
            Ctrl::Exec => (),
            Ctrl::Macro => (),
        }
        self.last = key;
    }
    fn execute_enter(&mut self, key: char) {
        match Inst::new(key) {
            Inst::Meta(ctrl) => self.ctrl = ctrl,
            inst => self.vm.exec_inst(inst),
        }
    }
    fn execute_quote(&mut self, key: char) {
        match key {
            '"' => self.ctrl = Ctrl::Enter,
            _ => self.quote.push(key),
        }
    }
    fn execute_ignore(&mut self, key: char) {
        if key == '\n' {
            self.ctrl = Ctrl::Enter;
        }
    }
    pub fn print(&self) {
        self.vm.print(self.last);
    }
}
