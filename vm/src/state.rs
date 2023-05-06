use crate::inst::{Inst, Name, Seq};
use crate::memory::Memory;
use crate::reg::Registers;
use crate::streams::StreamMap;
use std::collections::HashMap;
use std::ops::{Index, IndexMut};
use util::{Block, Stream, BYTE_COUNT};

struct MacroRegistry {
    array: [Seq; BYTE_COUNT],
}
impl Default for MacroRegistry {
    fn default() -> Self {
        let array = [(); BYTE_COUNT].map(|_| Seq::default());
        Self { array }
    }
}
impl Index<u8> for MacroRegistry {
    type Output = Seq;
    fn index(&self, index: u8) -> &Self::Output {
        &self.array[usize::from(index)]
    }
}
impl IndexMut<u8> for MacroRegistry {
    fn index_mut(&mut self, index: u8) -> &mut Self::Output {
        &mut self.array[usize::from(index)]
    }
}

pub struct State {
    registers: Registers,
    memory: Memory,
    definition: HashMap<Name, Seq>,
    registry: MacroRegistry,
    streams: StreamMap,
}
impl Default for State {
    fn default() -> Self {
        Self {
            registers: Registers::default(),
            memory: Memory::default(),
            definition: HashMap::default(),
            registry: MacroRegistry::default(),
            streams: StreamMap::new(),
        }
    }
}
impl State {
    pub fn init(&mut self, input: Stream, output: Stream) {
        self.streams.init(input, output);
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
        let registers = &mut self.registers;
        let memory = &mut self.memory;
        let streams = &mut self.streams;
        match inst {
            Inst::Insert(digit) => registers.insert(digit),
            Inst::High => registers.high(),
            Inst::Low => registers.low(),
            Inst::Swap => registers.swap(),
            Inst::Zero => registers.zero(),
            Inst::Delete => registers.delete(),
            Inst::Right => registers.right(),
            Inst::Left => registers.left(),
            Inst::Down => registers.down(),
            Inst::Up => registers.up(),
            Inst::Goto => registers.goto(),
            Inst::Jump => registers.jump(),
            Inst::Coord => registers.coord(),
            Inst::Page => registers.page(),
            Inst::Origin => registers.origin(),
            Inst::Begin => registers.begin(),
            Inst::Add => registers.add(),
            Inst::Sub => registers.sub(),
            Inst::Mul => registers.mul(),
            Inst::Div => registers.div(),
            Inst::Inc => registers.inc(),
            Inst::Dec => registers.dec(),
            Inst::Shl => registers.shl(),
            Inst::Shr => registers.shr(),
            Inst::Rotl => registers.rotl(),
            Inst::Rotr => registers.rotr(),
            Inst::And => registers.and(),
            Inst::Or => registers.or(),
            Inst::Xor => registers.xor(),
            Inst::Not => registers.not(),
            Inst::Neg => registers.neg(),
            Inst::Bool => registers.bool(),
            Inst::Eq => registers.eq(),
            Inst::Lt => registers.lt(),
            Inst::Gt => registers.gt(),
            Inst::Check => registers.check(),
            Inst::Clear => registers.clear(),
            Inst::Load => memory.load(registers),
            Inst::Store => memory.store(registers),
            Inst::Restore => memory.restore(registers),
            Inst::Save => memory.save(registers),
            Inst::Getchar => streams.getchar(registers, memory),
            Inst::Putchar => streams.putchar(registers, memory),
            Inst::Stream => streams.stream(registers),
            Inst::Direct(data) => memory.direct(registers, data),
            Inst::Quote(seq) => memory.quote(registers, &seq),
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
        self.registry[key] = seq;
    }
    fn exec(&mut self, key: u8) {
        self.run(&self.registry[key].clone());
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
