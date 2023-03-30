use crate::inst::{Inst, Name};
use crate::maps::Maps;
use crate::memory::Memory;
use crate::reg::Registers;
use util::{Block, Stream};

pub struct State {
    regs: Registers,
    memory: Memory,
    maps: Maps,
}
impl State {
    #[must_use]
    pub fn new(input: Stream, output: Stream) -> Self {
        Self {
            regs: Registers::default(),
            memory: Memory::default(),
            maps: Maps::new(input, output),
        }
    }
    #[must_use]
    pub fn get_regs(&self) -> &Registers {
        &self.regs
    }
    #[must_use]
    pub fn get_memory(&self) -> &Block<Block<u8>> {
        self.memory.get_memory()
    }
    pub fn issue(&mut self, inst: Inst) {
        let regs = &mut self.regs;
        let mem = &mut self.memory;
        let maps = &mut self.maps;
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
            Inst::Load => mem.load(regs),
            Inst::Store => mem.store(regs),
            Inst::Save => mem.save(regs),
            Inst::Restore => mem.restore(regs),
            Inst::Get => maps.get(regs),
            Inst::Put => maps.put(regs),
            Inst::Action => maps.action(regs),
            Inst::Quote(input) => mem.quote(regs, &input),
            Inst::Func(name, body) => maps.define(name, body),
            Inst::Call(name) => self.call(&name),
            Inst::Macro(key, val) => maps.register(key, val),
            Inst::Exec(key) => self.exec(key),
            Inst::Repeat(key) => self.repeat(key),
            Inst::Eval => self.eval(),
            Inst::Nop | Inst::Skip => (),
        }
    }
    fn run(&mut self, seq: &[Inst]) {
        for inst in seq {
            self.issue(inst.clone());
        }
    }
    fn call(&mut self, name: &Name) {
        self.run(&self.maps.get_func(name));
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
}

#[cfg(test)]
mod state_tests {
    use super::{Inst, Registers, State, Stream};

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
        zero_test(&state);
        state.issue(Inst::Call(to_vec("test")));
        assert_eq!(state.get_regs().data, 1);
        assert_eq!(state.get_regs().accum, 2);
        assert_eq!(state.get_regs().block, 3);
        assert_eq!(state.get_regs().coord, 4);
        state.issue(Inst::Call(to_vec("clear")));
        zero_test(&state);
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
        zero_test(&state);
        state.issue(Inst::Exec(b'a'));
        assert_eq!(state.get_regs().data, 1);
        assert_eq!(state.get_regs().accum, 2);
        assert_eq!(state.get_regs().block, 3);
        assert_eq!(state.get_regs().coord, 4);
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
        state.run(&[Inst::Direct(10), Inst::Swap]);
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
        assert_eq!(state.get_regs().accum, 2);
        assert_eq!(state.get_regs().block, 3);
        assert_eq!(state.get_regs().coord, 4);
        state.run(&[Inst::Direct(b'c'), Inst::Eval]);
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
        assert_eq!(regs.coord, 0);
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
