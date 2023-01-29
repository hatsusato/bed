use crossterm::{cursor, terminal, Command};

struct AlternateScreen {}
impl Default for AlternateScreen {
    fn default() -> Self {
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
impl Default for HideCursor {
    fn default() -> Self {
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
    fn execute(cmd: impl Command) {
        use crossterm::ExecutableCommand;
        std::io::stdout().execute(cmd).unwrap();
    }
}
