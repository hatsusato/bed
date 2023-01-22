use super::State;

impl State {
    pub fn load(&mut self) {
        self.data = *self.current();
    }
    pub fn store(&mut self) {
        *self.current() = self.data;
    }
}
