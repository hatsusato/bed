use crate::inst::{Name, Seq};
use crate::reg::Registers;
use std::collections::HashMap;
use util::{Select, Stream};

const STDIN: u8 = 0;
const STDOUT: u8 = 1;
const STDERR: u8 = 2;
const NULL: u8 = u8::MAX;

enum Action {
    SetIndex,
    GetIndex,
    Open,
    Nop,
}
impl Action {
    fn new(flags: u8) -> Self {
        match flags / 2 {
            0 => Self::SetIndex,
            1 => Self::GetIndex,
            2 => Self::Open,
            _ => Self::Nop,
        }
    }
}
struct StreamAction {
    select: Select,
    action: Action,
    index: u8,
}
impl StreamAction {
    fn new(flags: u8, index: u8) -> Self {
        let select = Select::from(flags);
        let action = Action::new(flags);
        Self {
            select,
            action,
            index,
        }
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
    fn get(&mut self, select: Select) -> u8 {
        *self.map.get(&select).unwrap()
    }
    fn set(&mut self, select: Select, val: u8) {
        self.map.insert(select, val);
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
        map.insert(NULL, Stream::Null);
        let indices = StreamIndices::default();
        Self { map, indices }
    }
    fn get(&mut self, select: Select) -> &mut Stream {
        let index = self.indices.get(select);
        let contains = self.map.contains_key(&index);
        let key = if contains { index } else { NULL };
        self.map.get_mut(&key).unwrap()
    }
    fn set_index(&mut self, select: Select, val: u8) {
        self.indices.set(select, val);
    }
    fn get_index(&mut self, select: Select) -> u8 {
        self.indices.get(select)
    }
    fn action(&mut self, action: &StreamAction) -> Option<u8> {
        match action.action {
            Action::SetIndex => self.set_index(action.select, action.index),
            Action::GetIndex => return Some(self.get_index(action.select)),
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
        regs.get(self.streams.get(Select::Input));
    }
    pub fn put(&mut self, regs: &mut Registers) {
        regs.put(self.streams.get(Select::Output));
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
