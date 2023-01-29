use crossterm::{cursor, Command};

#[derive(Default)]
pub struct Screen {
    _screen: screen::Screen,
}
impl Screen {
    pub fn move_cursor(x: u16, y: u16) {
        use cursor::MoveTo;
        Self::queue(MoveTo(x, y));
    }
    fn queue(cmd: impl Command) {
        use crossterm::QueueableCommand;
        std::io::stdout().queue(cmd).unwrap();
    }
}
