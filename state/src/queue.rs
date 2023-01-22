use super::State;

impl State {
    pub fn push(&mut self) {
        self.queue.push(self.data);
    }
    pub fn pop(&mut self) {
        if self.queue.len() == 0 {
            self.raise();
        } else {
            self.queue.remove(0);
        }
    }
    pub fn argc(&mut self) {
        let argc = std::env::args().len();
        self.acc = argc.min(u8::MAX as usize) as u8;
    }
    pub fn argv(&mut self) {
        let index = self.data as usize;
        if let Some(arg) = std::env::args().nth(index) {
            self.queue.extend(arg.as_bytes());
        } else {
            self.raise();
        }
    }
}
