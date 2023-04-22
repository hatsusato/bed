mod args;

use crate::args::Args;
use clap::Parser;
use view::Editor;
use vm::Machine;

fn main() {
    let args = Args::parse();
    if args.is_interactive() {
        interactive(&args);
    } else {
        interpreter(&args);
    }
}
fn interpreter(args: &Args) {
    let code = args.open_code().unwrap_or_default();
    let input = args.open_input();
    let output = args.open_output();
    Machine::new(input, output).run(&code);
}
fn interactive(args: &Args) {
    let input = args.open_input();
    let output = args.open_output();
    Editor::new(input, output).run();
}
