use crate::inst::{Inst, Name, Seq};
use crate::page::Page;
use crate::reg::Registers;
use std::collections::HashMap;
use util::Block;

#[derive(Default, Debug)]
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

#[cfg(test)]
mod state_tests {
    use super::{Inst, Registers, State};

    #[test]
    fn load_store_delete_test() {
        let mut state = make();
        default_test(&state);
        for i in 0..=u8::MAX {
            assert_eq!(state.get_regs().data, i);
            assert_eq!(state.get_regs().acc, i);
            state.run(&[Inst::Store, Inst::Inc, Inst::High, Inst::Goto]);
        }
        default_regs_test(state.get_regs());
        for i in 0..=u8::MAX {
            state.run(&[Inst::Load]);
            assert_eq!(state.get_regs().data, i);
            assert_eq!(state.get_regs().coord, i);
            state.run(&[Inst::Delete, Inst::Right]);
        }
        state.run(&[Inst::Load]);
        default_test(&state);
    }
    #[test]
    fn save_test() {
        let mut state = make();
        state.run(&[Inst::Inc, Inst::Goto, Inst::Inc, Inst::High, Inst::Jump]);
        state.run(&[Inst::Inc, Inst::High, Inst::Inc, Inst::Swap]);
        assert_eq!(state.get_regs().data, 4);
        assert_eq!(state.get_regs().acc, 3);
        assert_eq!(state.get_regs().block, 2);
        assert_eq!(state.get_regs().coord, 1);
        state.issue(Inst::Save);
        for i in 1..5 {
            assert_eq!(state.get_memory()[2][i], 5 - i);
            state.run(&[Inst::Delete, Inst::Right]);
        }
        state.run(&[Inst::Origin, Inst::Start, Inst::Zero, Inst::Direct(0)]);
        default_test(&state);
    }
    #[test]
    fn restore_test() {
        let mut state = make();
        for _i in 0..4 {
            state.run(&[Inst::Inc, Inst::High, Inst::Store, Inst::Goto]);
        }
        state.run(&[Inst::Origin, Inst::Restore]);
        assert_eq!(state.get_regs().data, 1);
        assert_eq!(state.get_regs().acc, 2);
        assert_eq!(state.get_regs().block, 3);
        assert_eq!(state.get_regs().coord, 4);
        state.run(&[
            Inst::Origin,
            Inst::Start,
            Inst::Zero,
            Inst::Direct(0),
            Inst::Save,
        ]);
        default_test(&state);
    }
    #[test]
    fn quote_test() {
        let mut state = make();
        state.issue(Inst::Quote(vec![1, 2, 3, 4]));
        for i in 0..4 {
            assert_eq!(state.get_memory()[0][i], i + 1);
            state.run(&[Inst::Delete, Inst::Right]);
        }
        state.issue(Inst::Origin);
        default_test(&state);
    }
    #[test]
    fn func_call_test() {
        let mut state = make();
        let to_vec = |name: &str| name.as_bytes().to_vec();
        let test = [
            Inst::Inc,
            Inst::Goto,
            Inst::Inc,
            Inst::High,
            Inst::Jump,
            Inst::Inc,
            Inst::High,
            Inst::Inc,
            Inst::Swap,
        ]
        .to_vec();
        let clear = [Inst::Origin, Inst::Start, Inst::Zero, Inst::Direct(0)].to_vec();
        state.issue(Inst::Func(to_vec("test"), test));
        state.issue(Inst::Func(to_vec("clear"), clear));
        default_test(&state);
        state.issue(Inst::Call(to_vec("test")));
        assert_eq!(state.get_regs().data, 4);
        assert_eq!(state.get_regs().acc, 3);
        assert_eq!(state.get_regs().block, 2);
        assert_eq!(state.get_regs().coord, 1);
        state.issue(Inst::Call(to_vec("clear")));
        default_test(&state);
    }
    #[test]
    fn macro_exec_test() {
        let mut state = make();
        let record = [
            Inst::Inc,
            Inst::Goto,
            Inst::Inc,
            Inst::High,
            Inst::Jump,
            Inst::Inc,
            Inst::High,
            Inst::Inc,
            Inst::Swap,
        ]
        .to_vec();
        let clear = [Inst::Origin, Inst::Start, Inst::Zero, Inst::Direct(0)].to_vec();
        state.run(&[Inst::Macro(b'a', record), Inst::Macro(b'c', clear)]);
        default_test(&state);
        state.issue(Inst::Exec(b'a'));
        assert_eq!(state.get_regs().data, 4);
        assert_eq!(state.get_regs().acc, 3);
        assert_eq!(state.get_regs().block, 2);
        assert_eq!(state.get_regs().coord, 1);
        state.issue(Inst::Exec(b'c'));
        default_test(&state);
    }
    #[test]
    fn macro_repeat_test() {
        let mut state = make();
        let record = [Inst::Add, Inst::High].to_vec();
        let clear = [Inst::Zero, Inst::Direct(0)].to_vec();
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
            Inst::Inc,
            Inst::Goto,
            Inst::Inc,
            Inst::High,
            Inst::Jump,
            Inst::Inc,
            Inst::High,
            Inst::Inc,
            Inst::Swap,
        ]
        .to_vec();
        let clear = [Inst::Origin, Inst::Start, Inst::Zero, Inst::Direct(0)].to_vec();
        state.run(&[Inst::Macro(b'a', record), Inst::Macro(b'c', clear)]);
        state.run(&[Inst::Direct(b'a'), Inst::Eval]);
        assert_eq!(state.get_regs().data, 4);
        assert_eq!(state.get_regs().acc, 3);
        assert_eq!(state.get_regs().block, 2);
        assert_eq!(state.get_regs().coord, 1);
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
