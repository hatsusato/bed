use crate::reg::Registers;
use std::collections::HashMap;
use util::{Flag, Stream};

const STDIN: u8 = 0;
const STDOUT: u8 = 1;
const STDERR: u8 = 2;
const NULL: u8 = u8::MAX;

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
    streams: StreamMap,
    input: u8,
    output: u8,
}
impl Maps {
    pub fn new(input: Stream, output: Stream) -> Self {
        Self {
            streams: StreamMap::new(input, output),
            input: STDIN,
            output: STDOUT,
        }
    }
    pub fn getchar(&mut self, regs: &mut Registers) {
        let stream = self.streams.get_stream(self.input);
        regs.getchar(|| stream.getchar());
    }
    pub fn putchar(&mut self, regs: &mut Registers) {
        let stream = self.streams.get_stream(self.output);
        regs.putchar(|data| stream.putchar(data));
    }
    pub fn stream(&mut self, regs: &mut Registers) {
        match regs.data {
            0 => regs.get_descriptor(|| self.input),
            1 => regs.get_descriptor(|| self.output),
            2 => regs.set_descriptor(|desc| self.input = desc),
            3 => regs.set_descriptor(|desc| self.output = desc),
            _ => unimplemented!(),
        }
    }
}
