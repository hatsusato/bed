use clap::Parser;
use std::fs::File;
use std::io::{BufReader, Read, Result};
use std::path::PathBuf;
use util::{Flag, Stream};
use view::Editor;
use vm::Machine;

#[derive(Debug, Parser)]
#[command(version, about)]
struct Args {
    #[arg(help = "target source file\ninvoke interactive mode if omitted")]
    source: Option<PathBuf>,
    #[arg(short = 'i', long, help = "stdin source file")]
    input: Option<PathBuf>,
    #[arg(short = 'o', long, help = "stdout destination file")]
    output: Option<PathBuf>,
}
impl Args {
    fn is_interactive(&self) -> bool {
        self.source.is_none()
    }
    fn open_code(&self) -> Result<Vec<u8>> {
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
    fn open_input(&self) -> Result<Stream> {
        Ok(if let Some(path) = self.output.as_ref() {
            Stream::open(path, &Flag::Read)?
        } else if self.is_interactive() {
            Stream::default()
        } else {
            Stream::stdin()
        })
    }
    fn open_output(&self) -> Result<Stream> {
        Ok(if let Some(path) = self.output.as_ref() {
            Stream::open(path, &Flag::Write)?
        } else if self.is_interactive() {
            Stream::default()
        } else {
            Stream::stdout()
        })
    }
}

fn main() {
    let args = Args::parse();
    let result = if args.is_interactive() {
        interactive(&args)
    } else {
        interpreter(&args)
    };
    result.map_err(|e| println!("{e}")).unwrap_or_default();
}
fn interpreter(args: &Args) -> Result<()> {
    let code = args.open_code()?;
    let input = args.open_input()?;
    let output = args.open_output()?;
    Machine::new(input, output).run(&code);
    Ok(())
}
fn interactive(args: &Args) -> Result<()> {
    let input = args.open_input()?;
    let output = args.open_output()?;
    Editor::new(input, output).run();
    Ok(())
}
