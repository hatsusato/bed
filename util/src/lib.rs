mod block;
mod display;
mod stream;

pub use block::{Block, BLOCK_SIDE};
pub use display::as_hex;
pub use stream::{Flag, Select, Stream};

pub fn to_option<T, E: std::error::Error>(result: Result<T, E>) -> Option<T> {
    let p = |e| {
        eprintln!("{e}");
        None
    };
    result.map_or_else(p, Some)
}
