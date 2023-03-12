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

struct DisplayReverse {}
impl Default for DisplayReverse {
    fn default() -> Self {
        print(style::Attribute::Reverse);
        Self {}
    }
}
impl Drop for DisplayReverse {
    fn drop(&mut self) {
        print(style::Attribute::NoReverse);
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
    pub fn getch() -> Option<u8> {
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
            .and_then(|c| u8::try_from(c).ok())
        } else {
            None
        }
    }
    pub fn print_display(disp: impl Display, highlight: bool) {
        if highlight {
            let _rev = DisplayReverse::default();
            print(disp);
        } else {
            print(disp);
        }
        flush();
    }
    pub fn move_cursor(x: u16, y: u16) {
        use cursor::MoveTo;
        queue(MoveTo(x, y));
    }
}

fn print(disp: impl Display) {
    use style::Print;
    queue(Print(disp));
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
