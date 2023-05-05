use clap::Parser;
use std::path::PathBuf;
use util::{to_option, Flag, Select, Stream};

#[derive(Debug, Parser)]
#[command(version, about)]
pub struct Args {
    #[arg(help = "target source file")]
    source: PathBuf,
    #[arg(short = 'i', long, help = "stdin source file")]
    input: Option<PathBuf>,
    #[arg(short = 'o', long, help = "stdout destination file")]
    output: Option<PathBuf>,
    #[arg(long, help = "launch in a interactive mode")]
    interactive: bool,
}
impl Args {
    pub fn is_interactive(&self) -> bool {
        self.interactive
    }
    pub fn open_code(&self) -> Option<Vec<u8>> {
        use std::fs::File;
        use std::io::{BufReader, Read};
        let mut buf = Vec::new();
        Some(&self.source)
            .map(File::open)
            .and_then(to_option)
            .map(BufReader::new)
            .map(|mut reader| reader.read_to_end(&mut buf))
            .and_then(to_option)
            .map(|_len| buf)
    }
    pub fn open_default(&self, select: Select) -> Stream {
        if let Some(path) = self.choose(select) {
            Stream::make_file(path, Flag::from(select))
        } else if self.is_interactive() {
            Stream::default()
        } else {
            Stream::from(select)
        }
    }
    fn choose(&self, select: Select) -> Option<&PathBuf> {
        match select {
            Select::Input => self.input.as_ref(),
            Select::Output => self.output.as_ref(),
        }
    }
}
