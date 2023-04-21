use crate::inst::{Name, Seq};
use crate::reg::Registers;
use std::collections::HashMap;
use util::Stream;

const STDIN: u8 = 0;
const STDOUT: u8 = 1;
const STDERR: u8 = 2;

#[derive(Debug, PartialEq, Eq, Hash)]
enum Select {
    Input,
    Output,
}
impl Select {
    fn new(flag: u8) -> Self {
        match flag % 2 {
            0 => Self::Input,
            1 => Self::Output,
            _ => unreachable!(),
        }
    }
}
enum Action {
    SetIndex(u8),
    GetIndex,
    Open(u8),
    Nop,
}
impl Action {
    fn new(data: u8, accum: u8) -> Self {
        match data / 2 {
            0 => Self::SetIndex(accum),
            1 => Self::GetIndex,
            2 => Self::Open(accum),
            _ => Self::Nop,
        }
    }
}
struct StreamAction {
    select: Select,
    action: Action,
}
impl StreamAction {
    fn new(flags: u8, index: u8) -> Self {
        let select = Select::new(flags);
        let action = Action::new(flags, index);
        Self { select, action }
    }
}
struct StreamIndices {
    map: HashMap<Select, u8>,
}
impl Default for StreamIndices {
    fn default() -> Self {
        let mut map = HashMap::default();
        map.insert(Select::Input, STDIN);
        map.insert(Select::Output, STDOUT);
        Self { map }
    }
}
impl StreamIndices {
    fn get(&mut self, select: &Select) -> &mut u8 {
        self.map.get_mut(select).unwrap()
    }
}

struct StreamMap {
    map: HashMap<u8, Stream>,
    indices: StreamIndices,
}
impl StreamMap {
    fn new(input: Stream, output: Stream) -> Self {
        let mut map = HashMap::new();
        map.insert(STDIN, input);
        map.insert(STDOUT, output);
        map.insert(STDERR, Stream::Stderr);
        let indices = StreamIndices::default();
        Self { map, indices }
    }
    fn get(&mut self) -> Option<u8> {
        let get = Stream::get;
        let index = self.indices.get(&Select::Input);
        self.map.get_mut(index).and_then(get)
    }
    fn put(&mut self, data: u8) -> Option<()> {
        let put = |stream| Stream::put(stream, data);
        let index = self.indices.get(&Select::Output);
        self.map.get_mut(index).and_then(put)
    }
    fn action(&mut self, action: &StreamAction) -> Option<u8> {
        let index = self.indices.get(&action.select);
        match action.action {
            Action::SetIndex(set) => *index = set,
            Action::GetIndex => return Some(*index),
            _ => (),
        }
        None
    }
}

pub struct Maps {
    macros: HashMap<u8, Seq>,
    funcs: HashMap<Name, Seq>,
    streams: StreamMap,
}
impl Maps {
    pub fn new(input: Stream, output: Stream) -> Self {
        Self {
            macros: HashMap::default(),
            funcs: HashMap::default(),
            streams: StreamMap::new(input, output),
        }
    }
    pub fn get(&mut self, regs: &mut Registers) {
        match self.streams.get() {
            Some(data) => regs.data = data,
            None => regs.error = true,
        }
    }
    pub fn put(&mut self, regs: &mut Registers) {
        match self.streams.put(regs.data) {
            Some(()) => (),
            None => regs.error = true,
        }
    }
    pub fn action(&mut self, regs: &mut Registers) {
        let action = StreamAction::new(regs.data, regs.accum);
        if let Some(accum) = self.streams.action(&action) {
            regs.accum = accum;
        }
    }
    pub fn define(&mut self, name: Name, body: Seq) {
        self.funcs.entry(name).or_insert(body);
    }
    pub fn get_func(&self, name: &Name) -> Seq {
        self.funcs.get(name).cloned().unwrap_or_default()
    }
    pub fn register(&mut self, key: u8, val: Seq) {
        self.macros.insert(key, val);
    }
    pub fn get_macro(&self, key: u8) -> Seq {
        self.macros.get(&key).cloned().unwrap_or_default()
    }
}
