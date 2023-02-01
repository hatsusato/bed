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
        if let Some(inst) = match &self.mode {
            Mode::Normal => self.exec_normal(key),
            Mode::Ignore => self.exec_ignore(key),
            Mode::Quote(quote) => self.exec_quote(key, &quote.clone()),
            Mode::Escape => Some(self.exec_escape(key)),
        } {
            self.vm.exec_inst(&inst);
        }
        self.last = key;
    }
    pub fn print(&self) {
        self.vm.print(self.last);
    }
    fn exec_normal(&mut self, key: char) -> Option<Inst> {
        match key {
            '\n' => self.mode = Mode::Normal,
            '#' => self.mode = Mode::Ignore,
            '"' => self.mode = Mode::Quote(String::new()),
            '\'' => self.mode = Mode::Escape,
            _ => return Some(Inst::new(key)),
        }
        None
    }
    fn exec_ignore(&mut self, key: char) -> Option<Inst> {
        if key == '\n' {
            self.mode = Mode::Normal;
        }
        None
    }
    fn exec_quote(&mut self, key: char, quote: &String) -> Option<Inst> {
        if key == '"' {
            self.vm.exec_quote(quote);
            self.mode = Mode::Normal;
        } else {
            self.mode = Mode::Quote(format!("{quote}{key}"));
        }
        None
    }
    fn exec_escape(&mut self, key: char) -> Inst {
        self.mode = Mode::Normal;
        Inst::Esc(key)
    }
}
