use crate::screen::Screen;
use util::Block;

impl Screen {
    pub fn print_block(block: &Block<u8>) {
        (0..16).for_each(|y| {
            let line: Vec<String> = (0..16).map(|x| format!("{:02x}", block[x])).collect();
            Self::print_string(line.join(" "));
            Self::move_cursor(0, y);
        });
    }
}
