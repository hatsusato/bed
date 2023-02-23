use crate::ctrl::{Ctrl, DelayType, IssueType, NameType};
use crate::{Inst, Machine};
use std::collections::HashMap;
use std::mem;

#[derive(Default)]
pub struct Exec {
    ctrl: Ctrl,
    queue: String,
    map: HashMap<char, String>,
    vm: Machine,
    last: char,
}
impl Exec {
    pub fn execute(&mut self, input: char) {
        match self.ctrl.clone() {
            Ctrl::Normal => self.execute_normal(input),
            Ctrl::Ignore => self.execute_ignore(input),
            Ctrl::Delay(ty) => self.execute_delay(input, &ty),
            Ctrl::Record(key) => self.execute_record(input, key),
            Ctrl::Issue(ty) => self.execute_issue(input, ty),
            Ctrl::Name(ty) => self.execute_name(input, ty),
            Ctrl::Body => self.execute_body(input),
            Ctrl::Quote => self.execute_quote(input),
        }
    }
    fn execute_inst(&mut self, inst: &Inst) {
        self.vm.issue_inst(inst);
    }
    fn execute_macro(&mut self, input: char) {
        let seq = self.map.get(&input);
        self.vm.issue_run(seq.unwrap_or(&String::default()));
    }
    fn execute_normal(&mut self, input: char) {
        match input {
            '"' => self.ctrl = Ctrl::Quote,
            '#' => self.ctrl = Ctrl::Ignore,
            '\'' => self.ctrl = Ctrl::Delay(DelayType::Immediate),
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
        use DelayType::{Immediate, Macro, Record};
        match ty {
            Immediate => self.execute_inst(&Inst::immediate(input)),
            Record => self.ctrl = Ctrl::Record(input),
            Macro => self.execute_macro(input),
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
    fn execute_issue(&mut self, input: char, ty: IssueType) {}
    fn execute_name(&mut self, input: char, ty: NameType) {}
    fn execute_body(&mut self, input: char) {}
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