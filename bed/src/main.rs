use clap::Parser;
use std::fs::File;
use std::io::{BufReader, Read, Result};
use std::path::PathBuf;
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

fn main() -> Result<()> {
    let args = Args::parse();
    if args.source.is_some() {
        interpreter(&args)?;
    } else {
        interactive(&args);
    }
    Ok(())
}

fn interpreter(args: &Args) -> Result<()> {
    let f = File::open(args.source.as_ref().unwrap())?;
    let mut r = BufReader::new(f);
    let buf = &mut String::new();
    r.read_to_string(buf)?;
    let mut vm = Machine::default();
    vm.run(buf.as_bytes());
    Ok(())
}
fn interactive(_args: &Args) {
    let mut editor = Editor::default();
    editor.run();
}
