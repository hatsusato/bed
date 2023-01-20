use crossterm::{cursor, event, style, terminal, Command};

struct AlternateScreen {}
impl AlternateScreen {
    fn new() -> Self {
        Screen::execute(terminal::EnterAlternateScreen);
        Self {}
    }
}
impl Drop for AlternateScreen {
    fn drop(&mut self) {
        Screen::execute(terminal::LeaveAlternateScreen);
    }
}

struct HideCursor {}
impl HideCursor {
    fn new() -> Self {
        Screen::execute(cursor::Hide);
        Self {}
    }
}
impl Drop for HideCursor {
    fn drop(&mut self) {
        Screen::execute(cursor::Show);
    }
}

struct RawMode {}
impl RawMode {
    fn new() -> Self {
        terminal::enable_raw_mode().unwrap();
        Self {}
    }
}
impl Drop for RawMode {
    fn drop(&mut self) {
        terminal::disable_raw_mode().unwrap();
    }
}

pub struct Screen {
    _alternate_screen: AlternateScreen,
    _hide_cursor: HideCursor,
    _raw_mode: RawMode,
}
impl Screen {
    pub fn new() -> Self {
        Self {
            _alternate_screen: AlternateScreen::new(),
            _hide_cursor: HideCursor::new(),
            _raw_mode: RawMode::new(),
        }
    }
    pub fn print_string(msg: String) {
        use style::Print;
        Self::queue(Print(msg));
        Self::flush();
    }
    pub fn move_cursor(x: u16, y: u16) {
        use cursor::MoveTo;
        Self::queue(MoveTo(x, y));
    }
    pub fn getch() -> Option<char> {
        use event::{Event::Key, KeyCode::*};
        if let Ok(Key(key)) = event::read() {
            Some(match key.code {
                Char(c) => c,
                Enter => '\n',
                Tab => '\t',
                _ => return None,
            })
        } else {
            None
        }
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
