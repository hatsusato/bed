use std::collections::VecDeque;
use std::io::{Read, Result, Write};
use std::path::Path;

pub enum Stream {
    Null,
    Stdin,
    Stdout,
    Stderr,
    File(std::fs::File),
    Queue(VecDeque<u8>),
    Stack(Vec<u8>),
}
impl Default for Stream {
    fn default() -> Self {
        Stream::Null
    }
}
impl Stream {
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
            Stream::Stdin => read(&mut std::io::stdin()),
            Stream::File(file) => read(file),
            Stream::Queue(queue) => queue.pop_front(),
            Stream::Stack(stack) => stack.pop(),
            _ => None,
        }
    }
    pub fn put(&mut self, data: u8) -> Option<()> {
        match self {
            Stream::Stdout => write(&mut std::io::stdout(), data),
            Stream::Stderr => write(&mut std::io::stderr(), data),
            Stream::File(file) => write(file, data),
            Stream::Queue(queue) => {
                queue.push_back(data);
                Some(())
            }
            Stream::Stack(stack) => {
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
