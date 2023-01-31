use vm::{Inst, Machine};

enum Mode {
    Normal,
    Ignore,
    Quote(String),
    Escape,
}
impl Default for Mode {
    fn default() -> Self {
        Self::Normal
    }
}

#[derive(Default)]
pub struct Exec {
    mode: Mode,
    vm: Machine,
    last: char,
}
impl Exec {
    pub fn exec(&mut self, key: char) {
        let inst = match &self.mode {
            Mode::Normal => self.exec_normal(key),
            Mode::Ignore => self.exec_ignore(key),
            Mode::Quote(quote) => self.exec_quote(key, quote.clone()),
            Mode::Escape => self.exec_escape(key),
        };
        self.vm.exec_inst(&inst);
        self.last = key;
    }
    pub fn print(&self) {
        self.vm.print();
    }
    fn exec_normal(&mut self, key: char) -> Inst {
        self.vm.exec_normal(key)
    }
    fn exec_ignore(&mut self, key: char) -> Inst {
        self.vm.exec_ignore(key)
    }
    fn exec_quote(&mut self, key: char, quote: String) -> Inst {
        self.vm.exec_quote(key, quote)
    }
    fn exec_escape(&mut self, key: char) -> Inst {
        self.vm.exec_escape(key)
    }
}
