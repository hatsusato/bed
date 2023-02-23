use crate::ctrl::{Ctrl, IssueType, NameType};
use crate::{Inst, Machine};
use std::collections::HashMap;

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
    pub fn execute(&mut self, input: char) {
        match self.ctrl.clone() {
            Ctrl::Normal => self.execute_normal(input),
            Ctrl::Ignore => self.execute_ignore(input),
            Ctrl::Record(key) => self.execute_record(input, key),
            Ctrl::Issue(ty) => self.execute_issue(input, ty),
            Ctrl::Name(ty) => self.execute_name(input, ty),
            Ctrl::Body => self.execute_body(input),
            Ctrl::Quote => self.execute_quote(input),
        }
    }
    fn execute_other(&mut self, input: char) {
        let inst = Inst::new(input);
        self.vm.issue_inst(&inst);
    }
    fn execute_normal(&mut self, input: char) {
        match input {
            '#' => self.ctrl = Ctrl::Ignore,
            _ => self.execute_other(input),
        }
    }
    fn execute_ignore(&mut self, input: char) {
        if '\n' == input {
            self.ctrl = Ctrl::Normal;
        }
    }
    fn execute_record(&mut self, input: char, key: char) {}
    fn execute_issue(&mut self, input: char, ty: IssueType) {}
    fn execute_name(&mut self, input: char, ty: NameType) {}
    fn execute_body(&mut self, input: char) {}
    fn execute_quote(&mut self, input: char) {}
    pub fn print(&self) {
        self.vm.print(self.last);
    }
}
