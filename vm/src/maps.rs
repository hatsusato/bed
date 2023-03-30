use crate::inst::{Name, Seq};
use crate::reg::Registers;
use std::collections::HashMap;
use util::Stream;

pub struct StreamMap {
    map: HashMap<u8, Stream>,
    input: u8,
    output: u8,
}
impl StreamMap {
    pub fn new(input: Stream, output: Stream) -> Self {
        const DEFAULT_IN: u8 = 0;
        const DEFAULT_OUT: u8 = 1;
        let mut map = HashMap::new();
        map.insert(DEFAULT_IN, input);
        map.insert(DEFAULT_OUT, output);
        let (input, output) = (DEFAULT_IN, DEFAULT_OUT);
        Self { map, input, output }
    }
    pub fn get(&mut self) -> Option<u8> {
        let get = Stream::get;
        self.map.get_mut(&self.input).and_then(get)
    }
    pub fn put(&mut self, data: u8) -> Option<()> {
        let put = |stream| Stream::put(stream, data);
        self.map.get_mut(&self.output).and_then(put)
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
