use std::fmt::{Display, Formatter, Result};

pub struct HexData<T> {
    val: T,
}
pub fn as_hex<T>(val: T) -> HexData<T> {
    HexData { val }
}

impl Display for HexData<bool> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{:1x}", u8::from(self.val))
    }
}
impl Display for HexData<u8> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{:02x}", self.val)
    }
}
