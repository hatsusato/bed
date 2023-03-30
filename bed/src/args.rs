use clap::Parser;
use std::fs::File;
use std::io::{BufReader, Read, Result};
use std::path::PathBuf;
use util::{Flag, Stream};

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
    pub fn open_code(&self) -> Result<Vec<u8>> {
        Ok(match self.source.as_ref() {
            Some(path) => {
                let file = File::open(path)?;
                let mut buf = String::new();
                BufReader::new(file).read_to_string(&mut buf)?;
                buf.into_bytes()
            }
            None => Vec::default(),
        })
    }
    pub fn open_input(&self) -> Result<Stream> {
        Ok(if let Some(path) = self.output.as_ref() {
            Stream::make_file(path, &Flag::Read)?
        } else if self.is_interactive() {
            Stream::default()
        } else {
            Stream::stdin()
        })
    }
    pub fn open_output(&self) -> Result<Stream> {
        Ok(if let Some(path) = self.output.as_ref() {
            Stream::make_file(path, &Flag::Write)?
        } else if self.is_interactive() {
            Stream::default()
        } else {
            Stream::stdout()
        })
    }
}
