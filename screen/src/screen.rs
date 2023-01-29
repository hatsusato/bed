use crossterm::{cursor, event, terminal, Command};
use event::{Event, KeyCode};

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
fn execute(cmd: impl Command) {
    use crossterm::ExecutableCommand;
    std::io::stdout().execute(cmd).unwrap();
}

pub fn getch() -> Option<char> {
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
