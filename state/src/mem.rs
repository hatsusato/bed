use crate::State;

impl State {
    pub fn load(&mut self) {
        self.data = *self.current();
    }
    pub fn store(&mut self) {
        *self.current() = self.data;
    }
    fn current(&mut self) -> &mut u8 {
        &mut self.memory[self.block][self.coord]
    }
}
