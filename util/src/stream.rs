use std::collections::VecDeque;
use std::fs::File;
use std::path::Path;

pub enum Stream {
    Empty,
    Stdin,
    Stdout,
    Stderr,
    File(File),
    Queue(VecDeque<u8>),
}
impl Default for Stream {
    fn default() -> Self {
        Stream::Empty
    }
}
impl Stream {
    pub fn make_file<P: AsRef<Path>>(path: P, flag: Flag) -> Self {
        crate::to_option(flag.to_option().open(path))
            .map(Self::File)
            .unwrap_or_default()
    }
    #[must_use]
    pub fn make_queue() -> Self {
        Self::Queue(VecDeque::default())
    }
    #[must_use]
    pub fn make_standard(index: u8) -> Option<Self> {
        Some(match index {
            0 => Stream::Stdin,
            1 => Stream::Stdout,
            2 => Stream::Stderr,
            255 => Stream::Empty,
            _ => return None,
        })
    }
    pub fn take_string(&mut self) -> Option<String> {
        match self {
            Stream::Queue(queue) => Some(std::mem::take(queue)),
            _ => None,
        }
        .map(Vec::from)
        .map(String::from_utf8)
        .and_then(crate::to_option)
    }
    pub fn getchar(&mut self) -> Option<u8> {
        let buf = &mut [0; 1];
        match self.read(buf) {
            Some(1) => Some(buf[0]),
            Some(0) | None => None,
            _ => unreachable!(),
        }
    }
    pub fn putchar(&mut self, data: u8) -> Option<()> {
        let buf = &[data; 1];
        match self.write(buf) {
            Some(1) => Some(()),
            Some(0) | None => None,
            _ => unreachable!(),
        }
    }
    pub fn read(&mut self, buf: &mut [u8]) -> Option<usize> {
        use std::io::{stdin, Read};
        crate::to_option(match self {
            Stream::Stdin => stdin().read(buf),
            Stream::File(file) => file.read(buf),
            Stream::Queue(queue) => queue.read(buf),
            Stream::Empty | Stream::Stdout | Stream::Stderr => return None,
        })
    }
    pub fn write(&mut self, buf: &[u8]) -> Option<usize> {
        use std::io::{stderr, stdout, Write};
        crate::to_option(match self {
            Stream::Stdout => stdout().write(buf),
            Stream::Stderr => stderr().write(buf),
            Stream::File(file) => file.write(buf),
            Stream::Queue(queue) => queue.write(buf),
            Stream::Empty | Stream::Stdin => return None,
        })
    }
}
impl From<Select> for Stream {
    fn from(value: Select) -> Self {
        match value {
            Select::Input => Stream::Stdin,
            Select::Output => Stream::Stdout,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Flag {
    bits: u8,
}
impl Flag {
    const READ: Self = Self { bits: 1 << 0 };
    const WRITE: Self = Self { bits: 1 << 1 };
    const APPEND: Self = Self { bits: 1 << 2 };
    const TRUNCATE: Self = Self { bits: 1 << 3 };
    const CREATE: Self = Self { bits: 1 << 4 };
    const CREATE_NEW: Self = Self { bits: 1 << 5 };
    #[must_use]
    pub fn new(bits: u8) -> Self {
        Self { bits }
    }
    fn to_option(self) -> std::fs::OpenOptions {
        let mut options = File::options();
        options
            .read(self.check(Flag::READ))
            .write(self.check(Flag::WRITE))
            .append(self.check(Flag::APPEND))
            .truncate(self.check(Flag::TRUNCATE))
            .create(self.check(Flag::CREATE))
            .create_new(self.check(Flag::CREATE_NEW));
        options
    }
    fn check(self, flag: Self) -> bool {
        self.bits & flag.bits != 0
    }
}
impl From<Select> for Flag {
    fn from(value: Select) -> Self {
        match value {
            Select::Input => Self::READ,
            Select::Output => Self::WRITE,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Select {
    Input,
    Output,
}
