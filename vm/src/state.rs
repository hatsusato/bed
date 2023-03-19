use crate::inst::{Inst, Name, Seq};
use crate::page::Page;
use crate::reg::Registers;
use std::collections::HashMap;
use util::Block;

#[derive(Default)]
pub struct State {
    regs: Registers,
    memory: Block<Block<u8>>,
    macros: HashMap<u8, Seq>,
    funcs: HashMap<Name, Seq>,
}
impl State {
    #[must_use]
    pub fn get_regs(&self) -> &Registers {
        &self.regs
    }
    #[must_use]
    pub fn get_memory(&self) -> &Block<Block<u8>> {
        &self.memory
    }
    pub fn issue(&mut self, inst: Inst) {
        let regs = &mut self.regs;
        let mut page = Page::new(regs, &mut self.memory);
        match inst {
            Inst::Direct(data) => regs.direct(data),
            Inst::Insert(digit) => regs.insert(digit),
            Inst::Swap => regs.swap(),
            Inst::High => regs.high(),
            Inst::Low => regs.low(),
            Inst::Zero => regs.zero(),
            Inst::Origin => regs.origin(),
            Inst::Start => regs.start(),
            Inst::Goto => regs.goto(),
            Inst::Jump => regs.jump(),
            Inst::Pos => regs.pos(),
            Inst::Page => regs.page(),
            Inst::Left => regs.left(),
            Inst::Right => regs.right(),
            Inst::Up => regs.up(),
            Inst::Down => regs.down(),
            Inst::Inc => regs.inc(),
            Inst::Dec => regs.dec(),
            Inst::Add => regs.add(),
            Inst::Sub => regs.sub(),
            Inst::Mul => regs.mul(),
            Inst::Div => regs.div(),
            Inst::Clear => regs.clear(),
            Inst::Raise => regs.raise(),
            Inst::Neg => regs.neg(),
            Inst::Bool => regs.bool(),
            Inst::Eq => regs.eq(),
            Inst::Lt => regs.lt(),
            Inst::Gt => regs.gt(),
            Inst::Not => regs.not(),
            Inst::And => regs.and(),
            Inst::Or => regs.or(),
            Inst::Xor => regs.xor(),
            Inst::Shl => regs.shl(),
            Inst::Shr => regs.shr(),
            Inst::Rotl => regs.rotl(),
            Inst::Rotr => regs.rotr(),
            Inst::Load => page.load(),
            Inst::Store => page.store(),
            Inst::Delete => page.delete(),
            Inst::Put => page.put(),
            Inst::Get => page.get(),
            Inst::Save => page.save(),
            Inst::Restore => page.restore(),
            Inst::Quote(input) => page.quote(input.as_slice()),
            Inst::Func(name, body) => self.define_func(name, body),
            Inst::Call(name) => self.call_func(&name),
            Inst::Macro(key, val) => self.register_macro(key, val),
            Inst::Exec(key) => self.exec_macro(key),
            Inst::Repeat(key) => self.repeat_macro(key),
            Inst::Eval => self.eval(),
            Inst::Nop | Inst::Skip => (),
        }
    }
    fn run(&mut self, seq: &[Inst]) {
        seq.iter().for_each(|i| self.issue(i.clone()));
    }
    fn repeat(&mut self, seq: &[Inst]) {
        let count = self.regs.acc;
        for i in 0..count {
            self.regs.acc = i;
            self.run(seq);
        }
        self.regs.acc = count;
    }
    fn eval(&mut self) {
        self.exec_macro(self.regs.data);
    }
    fn register_macro(&mut self, key: u8, val: Seq) {
        self.macros.insert(key, val);
    }
    fn exec_macro(&mut self, key: u8) {
        if let Some(record) = self.macros.get(&key).cloned() {
            self.run(record.as_slice());
        }
    }
    fn repeat_macro(&mut self, key: u8) {
        if let Some(record) = self.macros.get(&key).cloned() {
            self.repeat(record.as_slice());
        }
    }
    fn call_func(&mut self, name: &Name) {
        if let Some(body) = self.funcs.get(name).cloned() {
            self.run(body.as_slice());
        }
    }
    fn define_func(&mut self, name: Name, body: Seq) {
        self.funcs.insert(name, body);
    }
}
