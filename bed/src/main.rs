mod args;

use crate::args::Args;
use clap::Parser;
use std::io::Result;
use view::Editor;
use vm::Machine;

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
