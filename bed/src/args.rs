use clap::Parser;
use std::path::PathBuf;
use util::{to_option, Flag, Select, Stream};

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
    pub fn open_default(&self, select: Select) -> Stream {
        match self.choose(select) {
            Some(path) => Stream::make_file(path, Flag::from(select)),
            None => Stream::make_default(self.is_interactive(), select),
        }
    }
    fn choose(&self, select: Select) -> Option<&PathBuf> {
        match select {
            Select::Input => self.input.as_ref(),
            Select::Output => self.output.as_ref(),
        }
    }
}
