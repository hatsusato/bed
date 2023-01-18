use crossterm::event::{Event, KeyCode};

pub struct Screen {}
impl Screen {
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
}
