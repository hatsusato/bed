use std::io;

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
    pub fn get(&mut self) -> Option<u8> {
        match self.kind {
            Kind::Stdin => read(&mut io::stdin()),
            _ => None,
        }
    }
    pub fn put(&mut self, data: u8) -> Option<()> {
        match self.kind {
            Kind::Stdout => write(&mut io::stdout(), data),
            Kind::Stderr => write(&mut io::stderr(), data),
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
