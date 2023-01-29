use crossterm::{cursor, event, style, terminal, Command};
use std::fmt::Display;

struct AlternateScreen {}
impl Default for AlternateScreen {
    fn default() -> Self {
        execute(terminal::EnterAlternateScreen);
        Self {}
    }
}
impl Drop for AlternateScreen {
    fn drop(&mut self) {
        execute(terminal::LeaveAlternateScreen);
    }
}

struct HideCursor {}
impl Default for HideCursor {
    fn default() -> Self {
        execute(cursor::Hide);
        Self {}
    }
}
impl Drop for HideCursor {
    fn drop(&mut self) {
        execute(cursor::Show);
    }
}

struct RawMode {}
impl Default for RawMode {
    fn default() -> Self {
        terminal::enable_raw_mode().unwrap();
        Self {}
    }
}
impl Drop for RawMode {
    fn drop(&mut self) {
        terminal::disable_raw_mode().unwrap();
    }
}

#[derive(Default)]
pub struct Screen {
    _alternate_screen: AlternateScreen,
    _hide_cursor: HideCursor,
    _raw_mode: RawMode,
}
impl Screen {
    #[must_use]
    pub fn getch() -> Option<char> {
        use event::{Event, KeyCode};
        use Event::Key;
        use KeyCode::{Char, Enter, Tab};
        if let Ok(Key(key)) = event::read() {
            match key.code {
                Char(c) => Some(c),
                Enter => Some('\n'),
                Tab => Some('\t'),
                _ => None,
            }
        } else {
            None
        }
    }
    pub fn print_display(disp: impl Display) {
        use style::Print;
        queue(Print(disp));
        flush();
    }
    pub fn print_highlight(disp: impl Display) {
        use style::Attribute;
        Self::print_display(format!(
            "{}{}{}",
            Attribute::Reverse,
            disp,
            Attribute::NoReverse
        ));
    }
}

fn execute(cmd: impl Command) {
    use crossterm::ExecutableCommand;
    std::io::stdout().execute(cmd).unwrap();
}
fn queue(cmd: impl Command) {
    use crossterm::QueueableCommand;
    std::io::stdout().queue(cmd).unwrap();
}
fn flush() {
    use std::io::Write;
    std::io::stdout().flush().unwrap();
}
