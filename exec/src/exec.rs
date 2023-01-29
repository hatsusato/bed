use crate::cmd::Command;
use crate::State;
use inst::Inst;

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
pub struct Exec {
    mode: Mode,
    state: State,
    last: char,
}
impl Exec {
    pub fn exec(&mut self, key: char) {
        use Mode::{Escape, Ignore, Normal, Quote};
        match self.mode.clone() {
            Normal => self.exec_normal(key),
            Ignore => self.exec_ignore(key),
            Quote(quote) => self.exec_quote(key, quote),
            Escape => self.exec_escape(key),
        }
        self.last = key;
    }
    fn exec_normal(&mut self, key: char) {
        match key {
            '\n' => self.mode = Mode::Normal,
            '#' => self.mode = Mode::Ignore,
            '"' => self.mode = Mode::Quote(String::new()),
            '\'' => self.mode = Mode::Escape,
            _ => self.exec_cmd(key),
        }
    }
    fn exec_ignore(&mut self, key: char) {
        if key == '\n' {
            self.mode = Mode::Normal;
        }
    }
    fn exec_quote(&mut self, key: char, quote: String) {
        if key == '"' {
            let count = self.state.acc();
            (0..count).for_each(|_| self.exec_quoted(&quote));
            self.mode = Mode::Normal;
        } else {
            let mut quote = quote;
            quote.push(key);
            self.mode = Mode::Quote(quote);
        }
    }
    fn exec_quoted(&mut self, quote: &str) {
        quote.chars().for_each(|key| self.exec_cmd(key));
    }
    fn exec_escape(&mut self, key: char) {
        let cmd = Command::esc(&self.state, key);
        self.state.restore_bank(cmd.next);
        self.mode = Mode::Normal;
    }
    fn exec_cmd(&mut self, key: char) {
        let cmd = Command::from_inst(&Inst::new(key), &self.state);
        self.state.restore_bank(cmd.next);
        self.state.restore_page(cmd.page);
    }
    pub fn print(&self) {
        self.state.print(self.last);
    }
}
