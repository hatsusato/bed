use vm::{Inst, Machine};

enum Mode {
    Normal,
    Ignore,
    Block(Vec<Inst>),
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
    vm: Machine,
    last: char,
}
impl Exec {
    pub fn exec(&mut self, key: char) {
        if let Some(inst) = match &self.mode {
            Mode::Normal => self.exec_normal(key),
            Mode::Ignore => self.exec_ignore(key),
            Mode::Block(block) => self.exec_block(key, block.clone()),
            Mode::Quote => self.exec_quote(key),
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
            '"' => self.mode = Mode::Block(Vec::new()),
            '\'' => self.mode = Mode::Quote,
            _ => return Some(Inst::new(key)),
        }
        None
    }
    fn exec_ignore(&mut self, key: char) -> Option<Inst> {
        if key == '\n' {
            self.mode = Mode::Normal;
        } else {
            self.mode = Mode::Ignore;
        }
        None
    }
    fn exec_block(&mut self, key: char, mut block: Vec<Inst>) -> Option<Inst> {
        if key == '"' {
            self.vm.exec_repeat(&block);
            self.mode = Mode::Normal;
        } else {
            block.push(Inst::new(key));
            self.mode = Mode::Block(block);
        }
        None
    }
    fn exec_quote(&mut self, key: char) -> Option<Inst> {
        self.mode = Mode::Normal;
        u8::try_from(key).ok().map(Inst::Imm)
    }
}
