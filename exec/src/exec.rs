use std::collections::HashMap;
use std::mem;
use vm::{Ctrl, Inst, Machine};

#[derive(Default)]
pub struct Exec {
    ctrl: Ctrl,
    quote: String,
    record: String,
    key: Option<char>,
    map: HashMap<char, String>,
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
            Ctrl::Direct => self.execute_direct(key),
            Ctrl::Call => (),
            Ctrl::Define => (),
            Ctrl::Run => self.execute_run(key),
            Ctrl::Macro => self.execute_macro(key),
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
        if key == '"' {
            let quote = mem::take(&mut self.quote);
            self.vm.exec_inst(Inst::Quote(quote));
            self.ctrl = Ctrl::Enter;
        } else {
            self.quote.push(key);
        }
    }
    fn execute_ignore(&mut self, key: char) {
        if key == '\n' {
            self.ctrl = Ctrl::Enter;
        }
    }
    fn execute_direct(&mut self, key: char) {
        self.ctrl = Ctrl::Enter;
        if let Ok(key) = u8::try_from(key) {
            self.vm.exec_inst(Inst::Immediate(key));
        }
    }
    fn execute_run(&mut self, key: char) {
        self.ctrl = Ctrl::Enter;
        if let Some(val) = self.map.get(&key) {
            val.clone().chars().for_each(|key| self.execute(key));
        }
    }
    fn execute_macro(&mut self, key: char) {
        if let Some(k) = self.key {
            if key == 'q' {
                let v = mem::take(&mut self.record);
                self.map.insert(k, v);
                self.ctrl = Ctrl::Enter;
            } else {
                self.record.push(key);
            }
        } else {
            self.key = Some(key);
        }
    }
    pub fn print(&self) {
        self.vm.print(self.last);
    }
}
