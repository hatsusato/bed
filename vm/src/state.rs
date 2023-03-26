use crate::inst::{Inst, Name, Seq};
use crate::memory::Memory;
use crate::reg::Registers;
use std::collections::HashMap;
use util::{Block, Stream};

#[derive(Default, Debug)]
struct MacroMap {
    map: HashMap<u8, Seq>,
}
impl MacroMap {
    fn insert(&mut self, key: u8, val: Seq) {
        self.map.insert(key, val);
    }
    fn get(&self, key: u8) -> Seq {
        self.map.get(&key).cloned().unwrap_or_default()
    }
}

#[derive(Default, Debug)]
struct FuncMap {
    map: HashMap<Name, Seq>,
}
impl FuncMap {
    fn insert(&mut self, key: Name, val: Seq) {
        self.map.entry(key).or_insert(val);
    }
    fn get(&self, key: &Name) -> Seq {
        self.map.get(key).cloned().unwrap_or_default()
    }
}

#[derive(Debug)]
pub struct StreamMap {
    map: HashMap<u8, Stream>,
    input: u8,
    output: u8,
}
impl Default for StreamMap {
    fn default() -> Self {
        let (input, output, error) = (0, 1, 2);
        let mut map = HashMap::new();
        map.insert(input, Stream::stdin());
        map.insert(output, Stream::stdout());
        map.insert(error, Stream::stderr());
        Self { map, input, output }
    }
}
impl StreamMap {
    pub fn get(&mut self) -> Option<u8> {
        let get = Stream::get;
        self.map.get_mut(&self.input).and_then(get)
    }
    pub fn put(&mut self, data: u8) -> Option<()> {
        let put = |stream| Stream::put(stream, data);
        self.map.get_mut(&self.output).and_then(put)
    }
}

#[derive(Default, Debug)]
pub struct State {
    regs: Registers,
    memory: Memory,
    macros: MacroMap,
    funcs: FuncMap,
    streams: StreamMap,
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
        let mem = &mut self.memory;
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
            Inst::Get => self.get(),
            Inst::Put => self.put(),
            Inst::Quote(input) => self.quote(&input),
            Inst::Func(name, body) => self.define(name, body),
            Inst::Call(name) => self.call(&name),
            Inst::Macro(key, val) => self.register(key, val),
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
    fn get(&mut self) {
        match self.streams.get() {
            Some(data) => self.regs.data = data,
            None => self.regs.error = true,
        }
    }
    fn put(&mut self) {
        match self.streams.put(self.regs.data) {
            Some(_) => (),
            None => self.regs.error = true,
        }
    }
    fn quote(&mut self, input: &[u8]) {
        self.memory.quote(&mut self.regs, input);
    }
    fn define(&mut self, name: Name, body: Seq) {
        self.funcs.insert(name, body);
    }
    fn call(&mut self, name: &Name) {
        self.run(&self.funcs.get(name));
    }
    fn register(&mut self, key: u8, val: Seq) {
        self.macros.insert(key, val);
    }
    fn exec(&mut self, key: u8) {
        self.run(&self.macros.get(key));
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
    use super::{Inst, Registers, State};

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
        let state = State::default();
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
        zero_regs_test(&state.regs);
        for b in 0..u8::MAX {
            for c in 0..u8::MAX {
                assert_eq!(state.memory.blocks[b][c], 0);
            }
        }
    }
}
