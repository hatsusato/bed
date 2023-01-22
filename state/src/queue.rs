use super::State;

impl State {
    pub fn push(&mut self) {
        self.queue.push_back(self.data);
    }
    pub fn pop(&mut self) {
        match self.queue.pop_front() {
            Some(val) => self.data = val,
            None => self.raise(),
        }
    }
    pub fn len(&mut self) {
        self.set_len(self.queue.len());
    }
    pub fn argc(&mut self) {
        self.set_len(std::env::args().len());
    }
    pub fn argv(&mut self) {
        match std::env::args().nth(self.get_reg().into()) {
            Some(arg) => self.queue.extend(arg.as_bytes()),
            None => self.raise(),
        }
    }
    fn set_len(&mut self, len: usize) {
        const MAX_LEN: usize = u16::MAX as usize;
        self.set_reg(len.min(MAX_LEN) as u16);
        if MAX_LEN < len {
            self.raise();
        }
    }
}
