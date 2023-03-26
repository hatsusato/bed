use std::{collections::HashMap, io};

#[derive(Debug)]
enum Kind {
    Null,
    Stdin,
    Stdout,
    Stderr,
}
impl Default for Kind {
    fn default() -> Self {
        Kind::Null
    }
}

#[derive(Default, Debug)]
pub struct Stream {
    kind: Kind,
}
impl Stream {
    pub fn stdin() -> Self {
        let kind = Kind::Stdin;
        Self { kind }
    }
    pub fn stdout() -> Self {
        let kind = Kind::Stdout;
        Self { kind }
    }
    pub fn stderr() -> Self {
        let kind = Kind::Stderr;
        Self { kind }
    }
    fn get(&mut self) -> Option<u8> {
        match self.kind {
            Kind::Stdin => read(&mut io::stdin()),
            _ => todo!(),
        }
    }
    fn put(&mut self, data: u8) -> Option<()> {
        match self.kind {
            Kind::Stdout => write(&mut io::stdout(), data),
            Kind::Stderr => write(&mut io::stderr(), data),
            _ => todo!(),
        }
    }
}
fn write(output: &mut dyn io::Write, data: u8) -> Option<()> {
    let buf = &mut [data];
    match output.write(buf) {
        Ok(1) => Some(()),
        _ => None,
    }
}
fn read(input: &mut dyn io::Read) -> Option<u8> {
    let buf = &mut [0];
    match input.read(buf) {
        Ok(1) => Some(buf[0]),
        _ => None,
    }
}

#[derive(Debug)]
pub struct Map {
    map: HashMap<u8, Stream>,
    input: u8,
    output: u8,
}
impl Default for Map {
    fn default() -> Self {
        let (input, output, error) = (0, 1, 2);
        let mut map = HashMap::new();
        map.insert(input, Stream::stdin());
        map.insert(output, Stream::stdout());
        map.insert(error, Stream::stderr());
        Self { map, input, output }
    }
}
impl Map {
    pub fn get(&mut self) -> Option<u8> {
        let get = Stream::get;
        self.map.get_mut(&self.input).and_then(get)
    }
    pub fn put(&mut self, data: u8) -> Option<()> {
        let put = |stream| Stream::put(stream, data);
        self.map.get_mut(&self.output).and_then(put)
    }
}
