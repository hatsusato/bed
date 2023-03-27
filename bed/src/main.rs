use clap::Parser;
use std::fs::File;
use std::io::{BufReader, Read, Result};
use std::path::PathBuf;
use util::Stream;
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
    fn open_input(&self) -> Stream {
        let stdin = || {
            if self.is_interactive() {
                Stream::default()
            } else {
                Stream::stdin()
            }
        };
        self.input.as_ref().map_or_else(stdin, Stream::open)
    }
    fn open_output(&self) -> Stream {
        let stdout = || {
            if self.is_interactive() {
                Stream::default()
            } else {
                Stream::stdout()
            }
        };
        self.output.as_ref().map_or_else(stdout, Stream::open)
    }
}

fn main() -> Result<()> {
    let args = Args::parse();
    if args.source.is_some() {
        interpreter(&args)
    } else {
        interactive(&args);
        Ok(())
    }
}

fn interpreter(args: &Args) -> Result<()> {
    let code = args.open_code()?;
    let mut vm = Machine::new(args.open_input(), args.open_output());
    vm.run(&code);
    Ok(())
}
fn interactive(args: &Args) {
    let mut editor = Editor::new(args.open_input(), args.open_output());
    editor.run();
}
