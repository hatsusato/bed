use crate::ctrl::{Ctrl, DelayType, NameType};
use crate::lexer::Lexer;
use crate::{Inst, Machine};
use std::collections::HashMap;
use std::mem;

#[derive(Default)]
pub struct Exec {
    ctrl: Ctrl,
    queue: String,
    map: HashMap<char, String>,
    routines: HashMap<String, String>,
    vm: Machine,
    last: char,
    lexer: Lexer,
}
impl Exec {
    pub fn execute(&mut self, input: char) {
        if let Some(inst) = self.lexer.consume(input) {
            self.vm.issue_inst(&inst);
            return;
        }
        match self.ctrl.clone() {
            Ctrl::Normal => self.execute_normal(input),
            Ctrl::Ignore => self.execute_ignore(input),
            Ctrl::Delay(ty) => self.execute_delay(input, &ty),
            Ctrl::Record(key) => self.execute_record(input, key),
            Ctrl::Name(ty) => self.execute_name(input, &ty),
            Ctrl::Body(name) => self.execute_body(input, name),
            Ctrl::Quote => self.execute_quote(input),
        }
        self.last = input;
    }
    fn execute_inst(&mut self, inst: &Inst) {
        self.vm.issue_inst(inst);
    }
    fn execute_macro(&mut self, key: char) {
        if let Some(seq) = self.map.get(&key) {
            self.vm.issue_run(seq);
        }
    }
    fn execute_while(&mut self, key: char) {
        if let Some(seq) = self.map.get(&key) {
            self.vm.repeat(seq);
        }
    }
    fn execute_call(&mut self, name: &str) {
        if let Some(seq) = self.routines.get(name) {
            self.vm.issue_run(seq);
        }
    }
    fn execute_normal(&mut self, input: char) {
        match input {
            '"' => self.ctrl = Ctrl::Quote,
            '#' => self.ctrl = Ctrl::Ignore,
            '%' => self.ctrl = Ctrl::Delay(DelayType::While),
            '\'' => self.ctrl = Ctrl::Delay(DelayType::Immediate),
            ':' => self.ctrl = Ctrl::Name(NameType::Call),
            ';' => self.ctrl = Ctrl::Name(NameType::Define),
            '@' => self.ctrl = Ctrl::Delay(DelayType::Macro),
            'q' => self.ctrl = Ctrl::Delay(DelayType::Record),
            _ => self.execute_inst(&Inst::new(input)),
        }
    }
    fn execute_ignore(&mut self, input: char) {
        if '\n' == input {
            self.ctrl = Ctrl::Normal;
        }
    }
    fn execute_delay(&mut self, input: char, ty: &DelayType) {
        use DelayType::{Immediate, Macro, Record, While};
        match ty {
            Immediate => self.execute_inst(&Inst::immediate(input)),
            Record => self.ctrl = Ctrl::Record(input),
            Macro => self.execute_macro(input),
            While => self.execute_while(input),
        }
    }
    fn execute_record(&mut self, input: char, key: char) {
        if 'q' == input {
            let queue = mem::take(&mut self.queue);
            self.map.insert(key, queue);
        } else {
            self.queue.push(input);
        }
    }
    fn execute_name(&mut self, input: char, ty: &NameType) {
        use NameType::{Call, Define};
        if '\n' == input {
            let name = mem::take(&mut self.queue);
            match ty {
                Define => self.ctrl = Ctrl::Body(name),
                Call => self.execute_call(&name),
            }
        } else {
            self.queue.push(input);
        }
    }
    fn execute_body(&mut self, input: char, name: String) {
        if self.last == '\n' && input == '\n' {
            self.routines.insert(name, mem::take(&mut self.queue));
            self.ctrl = Ctrl::Normal;
        } else if input == ';' {
            self.routines.insert(name, mem::take(&mut self.queue));
            self.ctrl = Ctrl::Name(NameType::Define);
        } else {
            self.queue.push(input);
        }
    }
    fn execute_quote(&mut self, input: char) {
        if '"' == input {
            let queue = mem::take(&mut self.queue);
            self.execute_inst(&Inst::Quote(queue));
        } else {
            self.queue.push(input);
        }
    }
    pub fn print(&self) {
        self.vm.print(self.last);
    }
}
