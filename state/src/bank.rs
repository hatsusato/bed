use util::Page;

#[derive(Clone)]
pub struct Bank {
    pub acc: u8,
    pub block: u8,
    pub coord: u8,
    pub data: u8,
    pub error: bool,
    pub page: Option<Page>,
}
impl Bank {
    pub fn update_acc(&self, acc: u8) -> Self {
        let mut bank = self.clone();
        bank.acc = acc;
        bank
    }
    pub fn update_block(&self, block: u8) -> Self {
        let mut bank = self.clone();
        bank.block = block;
        bank
    }
    pub fn update_coord(&self, coord: u8) -> Self {
        let mut bank = self.clone();
        bank.coord = coord;
        bank
    }
    pub fn update_data(&self, data: u8) -> Self {
        let mut bank = self.clone();
        bank.data = data;
        bank
    }
    pub fn update_error(&self, error: bool) -> Self {
        let mut bank = self.clone();
        bank.error = error;
        bank
    }
    pub fn update_page(&self, page: Page) -> Self {
        let mut bank = self.clone();
        bank.page = Some(page);
        bank
    }
}
