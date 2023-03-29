use crate::inst::{Name, Seq};
use crate::reg::Registers;
use std::collections::HashMap;
use util::Stream;

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
    in_id: u8,
    out_id: u8,
}
impl StreamMap {
    pub fn new(input: Stream, output: Stream) -> Self {
        let (in_id, out_id) = (0, 1);
        let mut map = HashMap::new();
        map.insert(in_id, input);
        map.insert(out_id, output);
        Self { map, in_id, out_id }
    }
    pub fn get(&mut self) -> Option<u8> {
        let get = Stream::get;
        self.map.get_mut(&self.in_id).and_then(get)
    }
    pub fn put(&mut self, data: u8) -> Option<()> {
        let put = |stream| Stream::put(stream, data);
        self.map.get_mut(&self.out_id).and_then(put)
    }
}

#[derive(Debug)]
pub struct Maps {
    macros: MacroMap,
    funcs: FuncMap,
    streams: StreamMap,
}
impl Maps {
    pub fn new(input: Stream, output: Stream) -> Self {
        Self {
            macros: MacroMap::default(),
            funcs: FuncMap::default(),
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
            Some(_) => (),
            None => regs.error = true,
        }
    }
    pub fn define(&mut self, name: Name, body: Seq) {
        self.funcs.insert(name, body);
    }
    pub fn get_func(&self, name: &Name) -> Seq {
        self.funcs.get(name)
    }
    pub fn register(&mut self, key: u8, val: Seq) {
        self.macros.insert(key, val);
    }
    pub fn get_macro(&self, key: u8) -> Seq {
        self.macros.get(key)
    }
}
