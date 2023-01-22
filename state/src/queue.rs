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
}
