mod block;
mod display;
mod stream;

pub use block::Block;
pub use display::as_hex;
pub use stream::{Flag, Select, Stream};

pub const BYTE_BITS: u32 = u8::BITS;
pub const BYTE_COUNT: usize = 1 << BYTE_BITS;
pub const NIBBLE_BITS: u32 = BYTE_BITS / 2;
pub const NIBBLE_COUNT: u8 = 1 << NIBBLE_BITS;
pub const NIBBLE_MAX: u8 = NIBBLE_COUNT - 1;

pub fn to_option<T, E: std::error::Error>(result: Result<T, E>) -> Option<T> {
    let p = |e| {
        eprintln!("{e}");
        None
    };
    result.map_or_else(p, Some)
}
