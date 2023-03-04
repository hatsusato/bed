use crate::{Bank, Page};
use util::Block;

#[derive(Default)]
pub struct State {
    bank: Bank,
    memory: Block<Page>,
}
