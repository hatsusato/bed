use crossterm::{cursor, style, Command};

#[derive(Default)]
pub struct Screen {
    _screen: screen::Screen,
}
impl Screen {
    pub fn print_string(msg: String) {
        use style::Print;
        Self::queue(Print(msg));
        Self::flush();
    }
    pub fn print_highlight(msg: String) {
        use style::{Attribute, Print};
        Self::queue(Print(Attribute::Reverse));
        Self::queue(Print(msg));
        Self::queue(Print(Attribute::NoReverse));
    }
    pub fn move_cursor(x: u16, y: u16) {
        use cursor::MoveTo;
        Self::queue(MoveTo(x, y));
    }
    fn queue(cmd: impl Command) {
        use crossterm::QueueableCommand;
        std::io::stdout().queue(cmd).unwrap();
    }
    fn flush() {
        use std::io::Write;
        std::io::stdout().flush().unwrap();
    }
}
