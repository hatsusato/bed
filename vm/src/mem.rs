use util::Block;

#[derive(Default, Clone, Copy)]
pub struct Page {
    page: Block<u8>,
}
impl Page {
    pub fn get(&self) -> &Block<u8> {
        &self.page
    }
    pub fn get_mut(&mut self) -> &mut Block<u8> {
        &mut self.page
    }
}
