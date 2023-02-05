use vm::{Inst, Machine};

enum Mode {
    Normal,
    Ignore,
    Quote,
}
impl Default for Mode {
    fn default() -> Self {
        Self::Normal
    }
}

#[derive(Default)]
pub struct Exec {
    mode: Mode,
    block: Option<Vec<Inst>>,
    vm: Machine,
    last: char,
}
impl Exec {
    pub fn execute(&mut self, key: char) {
        match self.mode {
            Mode::Normal => self.execute_normal(key),
            Mode::Ignore => self.execute_ignore(key),
            Mode::Quote => self.execute_quote(key),
        }
        self.last = key;
    }
    fn execute_normal(&mut self, key: char) {
        match key {
            '\n' => (),
            '#' => {
                if self.block.is_none() {
                    self.mode = Mode::Ignore;
                }
            }
            '"' => {
                if let Some(block) = &self.block {
                    self.vm.exec_repeat(block);
                    self.block = None;
                } else {
                    self.block = Some(Vec::new());
                }
            }
            '\'' => self.mode = Mode::Quote,
            _ => {
                let inst = Inst::new(key);
                if let Some(block) = &mut self.block {
                    block.push(inst);
                } else {
                    self.vm.exec_inst(inst);
                }
            }
        }
    }
    fn execute_ignore(&mut self, key: char) {
        if key == '\n' {
            self.mode = Mode::Normal;
        }
    }
    fn execute_quote(&mut self, key: char) {
        if let Ok(inst) = u8::try_from(key).map(Inst::Immediate) {
            self.vm.exec_inst(inst);
        }
        self.mode = Mode::Normal;
    }
    pub fn print(&self) {
        self.vm.print(self.last);
    }
}
