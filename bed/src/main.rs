mod args;

use crate::args::Args;
use clap::Parser;
use util::Select;
use view::Editor;
use vm::Machine;

fn main() {
    let args = Args::parse();
    let input = args.open_default(Select::Input);
    let output = args.open_default(Select::Output);
    if args.is_interactive() {
        Editor::run(input, output);
    } else {
        let code = args.open_code().unwrap_or_default();
        Machine::run(&code, input, output);
    }
}
