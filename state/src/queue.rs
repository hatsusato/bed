use super::State;

impl State {
    pub fn push(&mut self) {
        self.queue.push_back(self.data);
    }
    pub fn pop(&mut self) {
        if let Some(val) = self.queue.pop_front() {
            self.data = val;
        } else {
            self.raise();
        }
    }
    pub fn argc(&mut self) {
        self.acc = std::env::args().len().min(u8::MAX as usize) as u8;
    }
    pub fn argv(&mut self) {
        if let Some(arg) = std::env::args().nth(self.data as usize) {
            self.queue.extend(arg.as_bytes());
        } else {
            self.raise();
        }
    }
}
