use crossterm::event::{Event, KeyCode};
use crossterm::Command;

pub struct Screen {}
impl Screen {
    pub fn print_string(msg: String) {
        use crossterm::style::Print;
        Self::queue(Print(msg));
        Self::flush();
    }
    pub fn getch() -> Option<char> {
        if let Ok(Event::Key(key)) = crossterm::event::read() {
            Self::translate(key.code)
        } else {
            None
        }
    }
    fn translate(code: KeyCode) -> Option<char> {
        use KeyCode::*;
        Some(match code {
            Char(c) => c,
            Enter => '\n',
            Tab => '\t',
            _ => return None,
        })
    }
    fn queue(cmd: impl Command) {
        use crossterm::QueueableCommand;
        std::io::stdout().queue(cmd).unwrap();
    }
    fn execute(cmd: impl Command) {
        use crossterm::ExecutableCommand;
        std::io::stdout().execute(cmd).unwrap();
    }
    fn flush() {
        use std::io::Write;
        std::io::stdout().flush().unwrap();
    }
}
