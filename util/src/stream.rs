use std::path::Path;
use std::{fs, io};

#[derive(Debug)]
enum Kind {
    Null,
    Stdin,
    Stdout,
    Stderr,
    File(fs::File),
}
impl Default for Kind {
    fn default() -> Self {
        Kind::Null
    }
}

#[derive(Debug, Default)]
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
    #[must_use]
    pub fn open<P: AsRef<Path>>(path: P) -> Self {
        let mut options = fs::File::options();
        options.read(true).write(true).create_new(true);
        match options.open(path) {
            Ok(file) => Self {
                kind: Kind::File(file),
            },
            _ => Self::default(),
        }
    }
    pub fn get(&mut self) -> Option<u8> {
        match &mut self.kind {
            Kind::Stdin => read(&mut io::stdin()),
            Kind::File(file) => read(file),
            _ => None,
        }
    }
    pub fn put(&mut self, data: u8) -> Option<()> {
        match &mut self.kind {
            Kind::Stdout => write(&mut io::stdout(), data),
            Kind::Stderr => write(&mut io::stderr(), data),
            Kind::File(file) => write(file, data),
            _ => None,
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
