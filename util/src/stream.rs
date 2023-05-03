use crate::to_option;
use std::collections::VecDeque;
use std::fs::File;
use std::io::{stderr, stdin, stdout, Read, Write};
use std::path::Path;

pub enum Stream {
    Null,
    Stdin,
    Stdout,
    Stderr,
    File(File),
    Queue(VecDeque<u8>),
}
impl Default for Stream {
    fn default() -> Self {
        Stream::Null
    }
}
impl Stream {
    #[must_use]
    pub fn make_default(interactive: bool, select: Select) -> Self {
        match (interactive, select) {
            (true, _) => Self::default(),
            (false, Select::Input) => Self::Stdin,
            (false, Select::Output) => Self::Stdout,
        }
    }
    pub fn make_argv(index: u8) -> Self {
        std::env::args()
            .nth(usize::from(index))
            .map(|argv| argv.as_bytes().to_vec())
            .map(VecDeque::from)
            .map(Stream::Queue)
            .unwrap_or_default()
    }
    pub fn make_file<P: AsRef<Path>>(path: P, flag: Flag) -> Self {
        let mut options = File::options();
        options.read(flag.is_read()).write(flag.is_write());
        to_option(options.open(path))
            .map(Self::File)
            .unwrap_or_default()
    }
    pub fn take_string(self) -> Option<String> {
        match self {
            Stream::Queue(queue) => Some(queue),
            _ => None,
        }
        .map(Vec::from)
        .map(String::from_utf8)
        .and_then(to_option)
    }
    pub fn getchar(&mut self) -> Option<u8> {
        match self {
            Stream::Stdin => read(&mut stdin()),
            Stream::File(file) => read(file),
            Stream::Queue(queue) => queue.pop_front(),
            _ => None,
        }
    }
    pub fn putchar(&mut self, data: u8) -> Option<()> {
        match self {
            Stream::Stdout => write(&mut stdout(), data),
            Stream::Stderr => write(&mut stderr(), data),
            Stream::File(file) => write(file, data),
            Stream::Queue(queue) => {
                queue.push_back(data);
                Some(())
            }
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Flag {
    Read,
    Write,
    Both,
}
impl Flag {
    fn is_read(self) -> bool {
        matches!(self, Flag::Read | Flag::Both)
    }
    fn is_write(self) -> bool {
        matches!(self, Flag::Write | Flag::Both)
    }
}
impl From<Select> for Flag {
    fn from(select: Select) -> Self {
        match select {
            Select::Input => Self::Read,
            Select::Output => Self::Write,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Select {
    Input,
    Output,
}
impl From<u8> for Select {
    fn from(value: u8) -> Self {
        match value % 2 {
            0 => Self::Input,
            1 => Self::Output,
            _ => unreachable!(),
        }
    }
}

fn write(output: &mut dyn Write, data: u8) -> Option<()> {
    let buf = &mut [data];
    match to_option(output.write(buf)) {
        Some(1) => Some(()),
        Some(0) | None => None,
        _ => unreachable!(),
    }
}
fn read(input: &mut dyn Read) -> Option<u8> {
    let buf = &mut [0];
    match to_option(input.read(buf)) {
        Some(1) => Some(buf[0]),
        Some(0) | None => None,
        _ => unreachable!(),
    }
}
