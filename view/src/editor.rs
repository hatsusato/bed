use screen::Screen;
use util::{Stream, BLOCK_SIDE};
use vm::{Machine, State};

#[derive(Default)]
pub struct Editor {
    _screen: Screen,
    vm: Machine,
}
impl Editor {
    #[must_use]
    pub fn new(input: Stream, output: Stream) -> Self {
        Self {
            vm: Machine::new(input, output),
            ..Default::default()
        }
    }
    pub fn run(&mut self) {
        self.print_init();
        loop {
            match Screen::getch() {
                Some(input) => self.vm.execute(input),
                None => return,
            }
            self.print();
        }
    }
    pub fn print(&self) {
        self.print_header();
        self.print_body();
    }
    fn print_init(&self) {
        const ORIGIN: u16 = 0;
        const REG_WIDTH: u16 = 7;
        const REG_COUNT: u16 = 5;
        const KEY_OFFSET: u16 = 4;
        let offset = REG_WIDTH * REG_COUNT + KEY_OFFSET;
        self.print();
        Screen::move_cursor(offset, ORIGIN);
        Screen::print_display("    ", false);
        Screen::move_cursor(ORIGIN, ORIGIN);
    }
    fn print_header(&self) {
        const ORIGIN: u16 = 0;
        let state = self.vm.get_state();
        let last = self.vm.get_last();
        Screen::move_cursor(ORIGIN, ORIGIN);
        print_register(state, last);
    }
    fn print_body(&self) {
        let state = self.vm.get_state();
        for y in 0..BLOCK_SIDE {
            for x in 0..BLOCK_SIDE {
                move_cell(x, y);
                print_cell(state, x, y);
            }
        }
    }
}

fn move_cell(x: u8, y: u8) {
    const CELL_WIDTH: u16 = 3;
    const HEADER_OFFSET: u16 = 1;
    let x = u16::from(x) * CELL_WIDTH;
    let y = u16::from(y) + HEADER_OFFSET;
    Screen::move_cursor(x, y);
}
fn print_cell(state: &State, x: u8, y: u8) {
    let regs = state.get_regs();
    let block = &state.get_memory()[regs.block];
    let index = x + y * BLOCK_SIDE;
    let highlight = regs.coord == index;
    Screen::print_display(util::as_hex(block[index]), highlight);
}
fn print_register(state: &State, last: u8) {
    let regs = state.get_regs();
    let disp = format!(
        "D: {}, A: {}, B: {}, C: {}, E: {}, KEY: {:<4}",
        util::as_hex(regs.data),
        util::as_hex(regs.accum),
        util::as_hex(regs.block),
        util::as_hex(regs.coord),
        util::as_hex(regs.error),
        translate_ascii(last)
    );
    Screen::print_display(disp, false);
}
fn translate_ascii(key: u8) -> String {
    const NUL: u8 = 0x00;
    const BEL: u8 = 0x07;
    const BS: u8 = 0x08;
    const HT: u8 = 0x09;
    const LF: u8 = 0x0a;
    const VT: u8 = 0x0b;
    const FF: u8 = 0x0c;
    const CR: u8 = 0x0d;
    const SPACE: u8 = 0x20;
    if key.is_ascii_graphic() {
        return char::from(key).to_string();
    }
    match key {
        NUL => "\\0",
        BEL => "\\a",
        BS => "\\b",
        HT => "\\t",
        LF => "\\n",
        VT => "\\v",
        FF => "\\f",
        CR => "\\r",
        SPACE => "SPC",
        _ => return format!("\\x{}", util::as_hex(key)),
    }
    .to_string()
}
