use std::collections::VecDeque;
use std::io::{Read, Result, Write};
use std::path::Path;

enum Kind {
    Null,
    Stdin,
    Stdout,
    Stderr,
    File(std::fs::File),
    Queue(VecDeque<u8>),
    Stack(Vec<u8>),
}
impl Default for Kind {
    fn default() -> Self {
        Kind::Null
    }
}
impl Kind {
    pub fn make_argv(index: u8) -> Self {
        let argv = std::env::args().nth(index.into());
        let argv = argv.map(|argv| argv.as_bytes().iter().copied().collect());
        Self::Queue(argv.unwrap_or_default())
    }
    pub fn make_file<P: AsRef<Path>>(path: P, flag: &Flag) -> Result<Self> {
        let mut options = std::fs::File::options();
        options.read(flag.is_read()).write(flag.is_write());
        options.open(path).map(Self::File)
    }
    pub fn get(&mut self) -> Option<u8> {
        match self {
            Kind::Stdin => read(&mut std::io::stdin()),
            Kind::File(file) => read(file),
            Kind::Queue(queue) => queue.pop_front(),
            Kind::Stack(stack) => stack.pop(),
            _ => None,
        }
    }
    pub fn put(&mut self, data: u8) -> Option<()> {
        match self {
            Kind::Stdout => write(&mut std::io::stdout(), data),
            Kind::Stderr => write(&mut std::io::stderr(), data),
            Kind::File(file) => write(file, data),
            Kind::Queue(queue) => {
                queue.push_back(data);
                Some(())
            }
            Kind::Stack(stack) => {
                stack.push(data);
                Some(())
            }
            _ => None,
        }
    }
}

pub enum Flag {
    Read,
    Write,
    Both,
}
impl Flag {
    fn is_read(&self) -> bool {
        matches!(self, Flag::Read | Flag::Both)
    }
    fn is_write(&self) -> bool {
        matches!(self, Flag::Write | Flag::Both)
    }
}

#[derive(Default)]
pub struct Stream {
    kind: Kind,
}
impl Stream {
    #[must_use]
    pub fn stdin() -> Self {
        let kind = Kind::Stdin;
        Self { kind }
    }
    #[must_use]
    pub fn stdout() -> Self {
        let kind = Kind::Stdout;
        Self { kind }
    }
    #[must_use]
    pub fn stderr() -> Self {
        let kind = Kind::Stderr;
        Self { kind }
    }
    fn make_file(file: std::fs::File) -> Self {
        let kind = Kind::File(file);
        Self { kind }
    }
    pub fn make_queue() -> Self {
        let kind = Kind::Queue(VecDeque::new());
        Self { kind }
    }
    pub fn make_stack() -> Self {
        let kind = Kind::Stack(Vec::new());
        Self { kind }
    }
    pub fn make_argv(index: u8) -> Self {
        let kind = Kind::make_argv(index);
        Self { kind }
    }
    /// # Errors
    pub fn open<P: AsRef<Path>>(path: P, flag: &Flag) -> Result<Self> {
        let kind = Kind::make_file(path, flag)?;
        Ok(Self { kind })
    }
    pub fn get(&mut self) -> Option<u8> {
        self.kind.get()
    }
    pub fn put(&mut self, data: u8) -> Option<()> {
        self.kind.put(data)
    }
}
fn write(output: &mut dyn Write, data: u8) -> Option<()> {
    let buf = &mut [data];
    match output.write(buf) {
        Ok(1) => Some(()),
        _ => None,
    }
}
fn read(input: &mut dyn Read) -> Option<u8> {
    let buf = &mut [0];
    match input.read(buf) {
        Ok(1) => Some(buf[0]),
        _ => None,
    }
}
