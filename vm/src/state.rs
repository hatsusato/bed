use crate::inst::{Inst, Name, Seq};
use crate::maps::Maps;
use crate::memory::Memory;
use crate::reg::Registers;
use std::collections::HashMap;
use util::{Block, Stream};

pub struct State {
    registers: Registers,
    memory: Memory,
    definition: HashMap<Name, Seq>,
    registry: HashMap<u8, Seq>,
    maps: Maps,
}
impl Default for State {
    fn default() -> Self {
        Self {
            registers: Registers::default(),
            memory: Memory::default(),
            definition: HashMap::new(),
            registry: HashMap::new(),
            maps: Maps::new(),
        }
    }
}
impl State {
    pub fn init(&mut self, input: Stream, output: Stream) {
        self.maps.init(input, output);
    }
    #[must_use]
    pub fn get_registers(&self) -> &Registers {
        &self.registers
    }
    #[must_use]
    pub fn get_memory(&self) -> &Block<Block<u8>> {
        self.memory.get_memory()
    }
    pub fn issue(&mut self, inst: Inst) {
        let regs = &mut self.registers;
        let mem = &mut self.memory;
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
            Inst::Getchar => maps.getchar(regs),
            Inst::Putchar => maps.putchar(regs),
            Inst::Stream => maps.stream(regs),
            Inst::Direct(data) => mem.direct(regs, data),
            Inst::Quote(seq) => mem.quote(regs, &seq),
            Inst::Register(key, val) => self.register(key, val),
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
    fn register(&mut self, key: u8, seq: Seq) {
        self.registry.insert(key, seq);
    }
    fn exec(&mut self, key: u8) {
        let seq = self.registry.get(&key).cloned().unwrap_or_default();
        self.run(&seq);
    }
    fn repeat(&mut self, key: u8) {
        let count = self.registers.accum;
        for i in 0..count {
            self.registers.accum = i;
            self.exec(key);
        }
        self.registers.accum = count;
    }
    fn eval(&mut self) {
        self.exec(self.registers.data);
    }
    fn define(&mut self, name: Name, body: Seq) {
        self.definition.entry(name).or_insert(body);
    }
    fn invoke(&mut self, name: &Name) {
        let body = self.definition.get(name).cloned().unwrap_or_default();
        self.run(&body);
    }
}

#[cfg(test)]
mod state_tests {
    use super::{Inst, Registers, State};

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
        assert_eq!(state.get_registers().data, 1);
        assert_eq!(state.get_registers().accum, 2);
        assert_eq!(state.get_registers().block, 3);
        assert_eq!(state.get_registers().cell, 4);
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
        state.run(&[Inst::Register(b'a', record), Inst::Register(b'c', clear)]);
        zero_test(&state);
        state.issue(Inst::Exec(b'a'));
        assert_eq!(state.get_registers().data, 1);
        assert_eq!(state.get_registers().accum, 2);
        assert_eq!(state.get_registers().block, 3);
        assert_eq!(state.get_registers().cell, 4);
        state.issue(Inst::Exec(b'c'));
        zero_test(&state);
    }
    #[test]
    fn macro_repeat_test() {
        let mut state = make();
        let record = [Inst::Add, Inst::High].to_vec();
        let clear = [Inst::Delete, Inst::Zero].to_vec();
        state.run(&[Inst::Register(b'a', record), Inst::Register(b'c', clear)]);
        zero_test(&state);
        state.run(&[Inst::Insert(10)]);
        assert_eq!(state.get_registers().data, 0);
        assert_eq!(state.get_registers().accum, 10);
        state.issue(Inst::Repeat(b'a'));
        assert_eq!(state.get_registers().data, 45);
        assert_eq!(state.get_registers().accum, 10);
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
        state.run(&[Inst::Register(b'a', record), Inst::Register(b'c', clear)]);
        state.run(&[Inst::Direct(b'a'), Inst::Load, Inst::Eval]);
        assert_eq!(state.get_registers().data, 1);
        assert_eq!(state.get_registers().accum, 2);
        assert_eq!(state.get_registers().block, 3);
        assert_eq!(state.get_registers().cell, 4);
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
        let state = State::default();
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
        zero_regs_test(&state.registers);
        for b in 0..u8::MAX {
            for c in 0..u8::MAX {
                assert_eq!(memory[b][c], 0);
            }
        }
    }
}
