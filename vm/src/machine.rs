use crate::cmd::Command;
use crate::inst::Inst;
use crate::State;

#[derive(Clone)]
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
pub struct Machine {
    mode: Mode,
    state: State,
    last: char,
}
impl Machine {
    pub fn exec(&mut self, key: char) {
        let inst = match self.mode.clone() {
            Mode::Normal => self.exec_normal(key),
            Mode::Ignore => self.exec_ignore(key),
            Mode::Quote(quote) => self.exec_quote(&quote),
            Mode::Escape => self.exec_escape(key),
        };
        self.exec_inst(&inst);
        self.last = key;
    }
    pub fn exec_inst(&mut self, inst: &Inst) {
        let cmd = Command::new(inst, &self.state);
        self.state.restore_bank(cmd.next);
        self.state.restore_page(cmd.page);
    }
    pub fn exec_normal(&mut self, key: char) -> Inst {
        match key {
            '\n' => self.mode = Mode::Normal,
            '#' => self.mode = Mode::Ignore,
            '"' => self.mode = Mode::Quote(String::new()),
            '\'' => self.mode = Mode::Escape,
            _ => return Inst::new(key),
        }
        Inst::Nop
    }
    pub fn exec_ignore(&mut self, key: char) -> Inst {
        if key == '\n' {
            self.mode = Mode::Normal;
        }
        Inst::Nop
    }
    pub fn exec_quote(&mut self, quote: &str) -> Inst {
        let count = self.state.bank().acc;
        (0..count).for_each(|_| self.exec_quoted(quote));
        Inst::Nop
    }
    pub fn exec_quoted(&mut self, quote: &str) {
        self.mode = Mode::Normal;
        quote.chars().for_each(|key| self.exec(key));
    }
    pub fn exec_escape(&mut self, key: char) -> Inst {
        self.mode = Mode::Normal;
        Inst::Esc(key)
    }
    pub fn print(&self, key: char) {
        self.state.print(key);
    }
}
