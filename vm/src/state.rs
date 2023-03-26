use crate::inst::{Inst, Name, Seq};
use crate::memory::Memory;
use crate::reg::Registers;
use std::{collections::HashMap, io};
use util::Block;

#[derive(Default, Debug)]
pub struct State {
    regs: Registers,
    memory: Memory,
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
        &self.memory.blocks
    }
    pub fn issue(&mut self, inst: Inst) {
        let regs = &mut self.regs;
        match inst {
            Inst::Direct(data) => regs.direct(data),
            Inst::Insert(digit) => regs.insert(digit),
            Inst::Swap => regs.swap(),
            Inst::High => regs.high(),
            Inst::Low => regs.low(),
            Inst::Zero => regs.zero(),
            Inst::Delete => regs.delete(),
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
            Inst::Check => regs.check(),
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
            Inst::Load => self.memory.load(regs),
            Inst::Store => self.memory.store(regs),
            Inst::Save => self.memory.save(regs),
            Inst::Restore => self.memory.restore(regs),
            Inst::Put => self.put(),
            Inst::Get => self.get(),
            Inst::Quote(input) => self.memory.quote(regs, input.as_slice()),
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
    fn put(&mut self) {
        use io::Write;
        let buf = &[self.regs.data];
        match io::stdout().write(buf) {
            Ok(1) => (),
            _ => self.regs.error = true,
        }
    }
    fn get(&mut self) {
        use io::Read;
        let buf = &mut [self.regs.data];
        match io::stdin().read(buf) {
            Ok(1) => self.regs.data = buf[0],
            _ => self.regs.error = true,
        }
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
        self.funcs.entry(name).or_insert(body);
    }
}

#[cfg(test)]
mod state_tests {
    use super::{Inst, Registers, State};

    #[test]
    fn quote_test() {
        let mut state = make();
        state.run(&[Inst::Quote(vec![1, 2, 3, 4])]);
        assert_eq!(state.regs.coord, 3);
        state.run(&[Inst::Origin]);
        assert_eq!(state.regs.coord, 0);
        for i in 0..4 {
            assert_eq!(state.get_memory()[0][i], i + 1);
            state.run(&[Inst::Store, Inst::Right]);
            assert_eq!(state.get_memory()[0][i], 0);
        }
        assert_eq!(state.regs.coord, 4);
        state.run(&[Inst::Origin]);
        default_test(&state);
    }
    #[test]
    fn func_call_test() {
        let mut state = make();
        let to_vec = |name: &str| name.as_bytes().to_vec();
        let test = [
            Inst::Direct(4),
            Inst::Goto,
            Inst::Direct(3),
            Inst::Jump,
            Inst::Direct(2),
            Inst::Inc,
            Inst::Swap,
        ]
        .to_vec();
        let clear = [Inst::Origin, Inst::Start, Inst::Delete, Inst::Zero].to_vec();
        state.issue(Inst::Func(to_vec("test"), test));
        state.issue(Inst::Func(to_vec("clear"), clear));
        default_test(&state);
        state.issue(Inst::Call(to_vec("test")));
        assert_eq!(state.get_regs().data, 1);
        assert_eq!(state.get_regs().acc, 2);
        assert_eq!(state.get_regs().block, 3);
        assert_eq!(state.get_regs().coord, 4);
        state.issue(Inst::Call(to_vec("clear")));
        default_test(&state);
    }
    #[test]
    fn macro_exec_test() {
        let mut state = make();
        let record = [
            Inst::Direct(4),
            Inst::Goto,
            Inst::Direct(3),
            Inst::Jump,
            Inst::Direct(2),
            Inst::Inc,
            Inst::Swap,
        ]
        .to_vec();
        let clear = [Inst::Origin, Inst::Start, Inst::Delete, Inst::Zero].to_vec();
        state.run(&[Inst::Macro(b'a', record), Inst::Macro(b'c', clear)]);
        default_test(&state);
        state.issue(Inst::Exec(b'a'));
        assert_eq!(state.get_regs().data, 1);
        assert_eq!(state.get_regs().acc, 2);
        assert_eq!(state.get_regs().block, 3);
        assert_eq!(state.get_regs().coord, 4);
        state.issue(Inst::Exec(b'c'));
        default_test(&state);
    }
    #[test]
    fn macro_repeat_test() {
        let mut state = make();
        let record = [Inst::Add, Inst::High].to_vec();
        let clear = [Inst::Delete, Inst::Zero].to_vec();
        state.run(&[Inst::Macro(b'a', record), Inst::Macro(b'c', clear)]);
        default_test(&state);
        state.run(&[Inst::Direct(10), Inst::Swap]);
        assert_eq!(state.get_regs().data, 0);
        assert_eq!(state.get_regs().acc, 10);
        state.issue(Inst::Repeat(b'a'));
        assert_eq!(state.get_regs().data, 45);
        assert_eq!(state.get_regs().acc, 10);
        state.issue(Inst::Exec(b'c'));
        default_test(&state);
    }
    #[test]
    fn macro_eval_test() {
        let mut state = make();
        let record = [
            Inst::Exec(b'c'),
            Inst::Direct(4),
            Inst::Goto,
            Inst::Direct(3),
            Inst::Jump,
            Inst::Direct(2),
            Inst::Inc,
            Inst::Swap,
        ]
        .to_vec();
        let clear = [Inst::Origin, Inst::Start, Inst::Delete, Inst::Zero].to_vec();
        state.run(&[Inst::Macro(b'a', record), Inst::Macro(b'c', clear)]);
        state.run(&[Inst::Direct(b'a'), Inst::Eval]);
        assert_eq!(state.get_regs().data, 1);
        assert_eq!(state.get_regs().acc, 2);
        assert_eq!(state.get_regs().block, 3);
        assert_eq!(state.get_regs().coord, 4);
        state.run(&[Inst::Direct(b'c'), Inst::Eval]);
        default_test(&state);
    }
    fn make() -> State {
        let state = State::default();
        default_test(&state);
        state
    }
    fn default_regs_test(regs: &Registers) {
        assert_eq!(regs.data, 0);
        assert_eq!(regs.acc, 0);
        assert_eq!(regs.block, 0);
        assert_eq!(regs.coord, 0);
        assert!(!regs.error);
    }
    fn default_test(state: &State) {
        let regs = state.get_regs();
        let mem = state.get_memory();
        default_regs_test(regs);
        for block in mem.iter() {
            for cell in block.iter() {
                assert_eq!(*cell, 0);
            }
        }
    }
}
