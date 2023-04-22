use crate::inst::{Name, Seq};
use crate::reg::Registers;
use std::collections::HashMap;
use util::{Flag, Select, Stream};

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
    fn set_index(&self, indices: &mut StreamIndices) {
        indices.set(self.select, self.index);
    }
    fn get_index(&self, indices: &StreamIndices) -> u8 {
        indices.get(self.select)
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
    fn get(&self, select: Select) -> u8 {
        *self.map.get(&select).unwrap()
    }
    fn set(&mut self, select: Select, val: u8) {
        self.map.insert(select, val);
    }
}

struct StreamMap {
    map: HashMap<u8, Stream>,
}
impl StreamMap {
    fn new(input: Stream, output: Stream) -> Self {
        let mut map = HashMap::new();
        map.insert(STDIN, input);
        map.insert(STDOUT, output);
        map.insert(STDERR, Stream::Stderr);
        map.insert(NULL, Stream::Null);
        Self { map }
    }
    fn get_stream(&mut self, index: u8) -> &mut Stream {
        let contains = self.map.contains_key(&index);
        let key = if contains { index } else { NULL };
        self.map.get_mut(&key).unwrap()
    }
    fn open(&mut self, index: u8, regs: &mut Registers) {
        if index != NULL {
            let stream = self.get_stream(index);
            if matches!(stream, Stream::Queue(_)) {
                let stream = std::mem::take(stream);
                if let Some(path) = stream.take_string() {
                    let stream = Stream::make_file(path, Flag::Both);
                    self.map.insert(index, stream);
                    return;
                }
            }
        }
        regs.error = true;
    }
}

pub struct Maps {
    macros: HashMap<u8, Seq>,
    funcs: HashMap<Name, Seq>,
    streams: StreamMap,
    indices: StreamIndices,
}
impl Maps {
    pub fn new(input: Stream, output: Stream) -> Self {
        Self {
            macros: HashMap::default(),
            funcs: HashMap::default(),
            streams: StreamMap::new(input, output),
            indices: StreamIndices::default(),
        }
    }
    pub fn get(&mut self, regs: &mut Registers) {
        let index = self.indices.get(Select::Input);
        let stream = self.streams.get_stream(index);
        regs.get(|| stream.get());
    }
    pub fn put(&mut self, regs: &mut Registers) {
        let index = self.indices.get(Select::Output);
        let stream = self.streams.get_stream(index);
        regs.put(|data| stream.put(data));
    }
    pub fn action(&mut self, regs: &mut Registers) {
        let action = StreamAction::new(regs.data, regs.accum);
        match &action.action {
            Action::SetIndex => action.set_index(&mut self.indices),
            Action::GetIndex => regs.accum = action.get_index(&self.indices),
            Action::Open => self.streams.open(action.index, regs),
            _ => unimplemented!(),
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
