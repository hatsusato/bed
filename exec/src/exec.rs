use vm::{Inst, Machine};

enum Mode {
    Normal,
    Quote,
    Ignore,
    While,
    Direct,
    Call,
    Define,
    Exec,
    Macro,
}
impl Default for Mode {
    fn default() -> Self {
        Self::Normal
    }
}

#[derive(Default)]
pub struct Exec {
    mode: Mode,
    quote: String,
    vm: Machine,
    last: char,
}
impl Exec {
    pub fn execute(&mut self, key: char) {
        match self.mode {
            Mode::Normal => self.execute_normal(key),
            Mode::Quote => self.execute_quote(key),
            Mode::Ignore => self.execute_ignore(key),
            Mode::While => (),
            Mode::Direct => (),
            Mode::Call => (),
            Mode::Define => (),
            Mode::Exec => (),
            Mode::Macro => (),
        }
        self.last = key;
    }
    fn execute_normal(&mut self, key: char) {
        match key {
            '\n' => self.mode = Mode::Normal,
            '"' => self.mode = Mode::Quote,
            '#' => self.mode = Mode::Ignore,
            '%' => self.mode = Mode::While,
            '\'' => self.mode = Mode::Direct,
            ':' => self.mode = Mode::Call,
            ';' => self.mode = Mode::Define,
            '@' => self.mode = Mode::Exec,
            'q' => self.mode = Mode::Macro,
            _ => self.vm.exec_inst(Inst::new(key)),
        }
    }
    fn execute_quote(&mut self, key: char) {
        if key == '"' {
            self.mode = Mode::Normal;
        } else {
            self.quote.push(key);
        }
    }
    fn execute_ignore(&mut self, key: char) {
        if key == '\n' {
            self.mode = Mode::Normal;
        }
    }
    pub fn print(&self) {
        self.vm.print(self.last);
    }
}
