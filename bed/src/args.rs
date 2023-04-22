use clap::Parser;
use std::path::PathBuf;
use util::{to_option, Flag, Stream};

#[derive(Debug, Parser)]
#[command(version, about)]
pub struct Args {
    #[arg(help = "target source file\ninvoke interactive mode if omitted")]
    source: Option<PathBuf>,
    #[arg(short = 'i', long, help = "stdin source file")]
    input: Option<PathBuf>,
    #[arg(short = 'o', long, help = "stdout destination file")]
    output: Option<PathBuf>,
}
impl Args {
    pub fn is_interactive(&self) -> bool {
        self.source.is_none()
    }
    pub fn open_code(&self) -> Option<Vec<u8>> {
        use std::fs::File;
        use std::io::{BufReader, Read};
        let mut buf = Vec::new();
        self.source
            .as_ref()
            .map(File::open)
            .and_then(to_option)
            .map(BufReader::new)
            .map(|mut reader| reader.read_to_end(&mut buf))
            .and_then(to_option)
            .map(|_| buf)
    }
    pub fn open_input(&self) -> Stream {
        if let Some(path) = self.output.as_ref() {
            Stream::make_file(path, &Flag::Read)
        } else if self.is_interactive() {
            Stream::Null
        } else {
            Stream::Stdin
        }
    }
    pub fn open_output(&self) -> Stream {
        if let Some(path) = self.output.as_ref() {
            Stream::make_file(path, &Flag::Write)
        } else if self.is_interactive() {
            Stream::Null
        } else {
            Stream::Stdout
        }
    }
}
