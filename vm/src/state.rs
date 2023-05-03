use crate::inst::{Inst, Name, Seq};
use crate::maps::Maps;
use crate::memory::Memory;
use crate::reg::Registers;
use std::collections::HashMap;
use util::{Block, Stream};

pub struct State {
    regs: Registers,
    mem: Memory,
    maps: Maps,
    defs: HashMap<Name, Seq>,
}
impl State {
    #[must_use]
    pub fn new(input: Stream, output: Stream) -> Self {
        Self {
            regs: Registers::default(),
            mem: Memory::default(),
            maps: Maps::new(input, output),
            defs: HashMap::new(),
        }
    }
    #[must_use]
    pub fn get_regs(&self) -> &Registers {
        &self.regs
    }
    #[must_use]
    pub fn get_memory(&self) -> &Block<Block<u8>> {
        self.mem.get_memory()
    }
    pub fn issue(&mut self, inst: Inst) {
        let regs = &mut self.regs;
        let mem = &mut self.mem;
        let maps = &mut self.maps;
        match inst {
            Inst::Insert(digit) => regs.insert(digit),
            Inst::High => regs.high(),
            Inst::Low => regs.low(),
            Inst::Swap => regs.swap(),
            Inst::Zero => regs.zero(),
            Inst::Delete => regs.delete(),
            Inst::Right => regs.right(),
            Inst::Left => regs.left(),
            Inst::Down => regs.down(),
            Inst::Up => regs.up(),
            Inst::Goto => regs.goto(),
            Inst::Jump => regs.jump(),
            Inst::Coord => regs.coord(),
            Inst::Page => regs.page(),
            Inst::Origin => regs.origin(),
            Inst::Begin => regs.begin(),
            Inst::Add => regs.add(),
            Inst::Sub => regs.sub(),
            Inst::Mul => regs.mul(),
            Inst::Div => regs.div(),
            Inst::Inc => regs.inc(),
            Inst::Dec => regs.dec(),
            Inst::Shl => regs.shl(),
            Inst::Shr => regs.shr(),
            Inst::Rotl => regs.rotl(),
            Inst::Rotr => regs.rotr(),
            Inst::And => regs.and(),
            Inst::Or => regs.or(),
            Inst::Xor => regs.xor(),
            Inst::Not => regs.not(),
            Inst::Neg => regs.neg(),
            Inst::Bool => regs.bool(),
            Inst::Eq => regs.eq(),
            Inst::Lt => regs.lt(),
            Inst::Gt => regs.gt(),
            Inst::Check => regs.check(),
            Inst::Clear => regs.clear(),
            Inst::Load => mem.load(regs),
            Inst::Store => mem.store(regs),
            Inst::Restore => mem.restore(regs),
            Inst::Save => mem.save(regs),
            Inst::Input => maps.input(regs),
            Inst::Output => maps.output(regs),
            Inst::Stream => maps.stream(regs),
            Inst::Direct(data) => mem.direct(regs, data),
            Inst::Quote(seq) => mem.quote(regs, &seq),
            Inst::Macro(key, val) => maps.register(key, val),
            Inst::Exec(key) => self.exec(key),
            Inst::Repeat(key) => self.repeat(key),
            Inst::Eval => self.eval(),
            Inst::Define(name, body) => self.define(name, body),
            Inst::Invoke(name) => self.invoke(&name),
            Inst::Nop | Inst::Skip => (),
        }
    }
    fn run(&mut self, seq: &[Inst]) {
        seq.iter().for_each(|inst| self.issue(inst.clone()));
    }
    fn exec(&mut self, key: u8) {
        self.run(&self.maps.get_macro(key));
    }
    fn repeat(&mut self, key: u8) {
        let count = self.regs.accum;
        for i in 0..count {
            self.regs.accum = i;
            self.exec(key);
        }
        self.regs.accum = count;
    }
    fn eval(&mut self) {
        self.exec(self.regs.data);
    }
    fn define(&mut self, name: Name, body: Seq) {
        self.defs.entry(name).or_insert(body);
    }
    fn invoke(&mut self, name: &Name) {
        let body = self.defs.get(name).cloned().unwrap_or_default();
        self.run(&body);
    }
}

#[cfg(test)]
mod state_tests {
    use super::{Inst, Registers, State, Stream};

    #[test]
    fn func_invoke_test() {
        let mut state = make();
        let to_vec = |name: &str| name.as_bytes().to_vec();
        let test = [
            Inst::Insert(4),
            Inst::High,
            Inst::Goto,
            Inst::Dec,
            Inst::High,
            Inst::Jump,
            Inst::Dec,
            Inst::High,
            Inst::Dec,
            Inst::Swap,
        ]
        .to_vec();
        let clear = [Inst::Origin, Inst::Begin, Inst::Delete, Inst::Zero].to_vec();
        state.issue(Inst::Define(to_vec("test"), test));
        state.issue(Inst::Define(to_vec("clear"), clear));
        zero_test(&state);
        state.issue(Inst::Invoke(to_vec("test")));
        assert_eq!(state.get_regs().data, 1);
        assert_eq!(state.get_regs().accum, 2);
        assert_eq!(state.get_regs().block, 3);
        assert_eq!(state.get_regs().cell, 4);
        state.issue(Inst::Invoke(to_vec("clear")));
        zero_test(&state);
    }
    #[test]
    fn macro_exec_test() {
        let mut state = make();
        let record = [
            Inst::Insert(4),
            Inst::High,
            Inst::Goto,
            Inst::Dec,
            Inst::High,
            Inst::Jump,
            Inst::Dec,
            Inst::High,
            Inst::Dec,
            Inst::Swap,
        ]
        .to_vec();
        let clear = [Inst::Origin, Inst::Begin, Inst::Delete, Inst::Zero].to_vec();
        state.run(&[Inst::Macro(b'a', record), Inst::Macro(b'c', clear)]);
        zero_test(&state);
        state.issue(Inst::Exec(b'a'));
        assert_eq!(state.get_regs().data, 1);
        assert_eq!(state.get_regs().accum, 2);
        assert_eq!(state.get_regs().block, 3);
        assert_eq!(state.get_regs().cell, 4);
        state.issue(Inst::Exec(b'c'));
        zero_test(&state);
    }
    #[test]
    fn macro_repeat_test() {
        let mut state = make();
        let record = [Inst::Add, Inst::High].to_vec();
        let clear = [Inst::Delete, Inst::Zero].to_vec();
        state.run(&[Inst::Macro(b'a', record), Inst::Macro(b'c', clear)]);
        zero_test(&state);
        state.run(&[Inst::Insert(10)]);
        assert_eq!(state.get_regs().data, 0);
        assert_eq!(state.get_regs().accum, 10);
        state.issue(Inst::Repeat(b'a'));
        assert_eq!(state.get_regs().data, 45);
        assert_eq!(state.get_regs().accum, 10);
        state.issue(Inst::Exec(b'c'));
        zero_test(&state);
    }
    #[test]
    fn macro_eval_test() {
        let mut state = make();
        let record = [
            Inst::Exec(b'c'),
            Inst::Insert(4),
            Inst::High,
            Inst::Goto,
            Inst::Dec,
            Inst::High,
            Inst::Jump,
            Inst::Dec,
            Inst::High,
            Inst::Dec,
            Inst::Swap,
        ]
        .to_vec();
        let clear = [Inst::Origin, Inst::Begin, Inst::Delete, Inst::Zero].to_vec();
        state.run(&[Inst::Macro(b'a', record), Inst::Macro(b'c', clear)]);
        state.run(&[Inst::Direct(b'a'), Inst::Load, Inst::Eval]);
        assert_eq!(state.get_regs().data, 1);
        assert_eq!(state.get_regs().accum, 2);
        assert_eq!(state.get_regs().block, 3);
        assert_eq!(state.get_regs().cell, 4);
        state.run(&[
            Inst::Direct(b'c'),
            Inst::Load,
            Inst::Low,
            Inst::Zero,
            Inst::Store,
            Inst::High,
            Inst::Eval,
            Inst::Store,
        ]);
        zero_test(&state);
    }
    fn make() -> State {
        let state = State::new(Stream::default(), Stream::default());
        zero_test(&state);
        state
    }
    fn zero_regs_test(regs: &Registers) {
        assert_eq!(regs.data, 0);
        assert_eq!(regs.accum, 0);
        assert_eq!(regs.block, 0);
        assert_eq!(regs.cell, 0);
        assert!(!regs.error);
    }
    fn zero_test(state: &State) {
        let memory = state.get_memory();
        zero_regs_test(&state.regs);
        for b in 0..u8::MAX {
            for c in 0..u8::MAX {
                assert_eq!(memory[b][c], 0);
            }
        }
    }
}
