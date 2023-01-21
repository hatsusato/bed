use crate::State;

impl State {
    pub fn right(&mut self) {
        (self.coord, _) = self.coord.overflowing_add(1);
    }
    pub fn left(&mut self) {
        (self.coord, _) = self.coord.overflowing_sub(1);
    }
    pub fn down(&mut self) {
        (self.coord, _) = self.coord.overflowing_add(16);
    }
    pub fn up(&mut self) {
        (self.coord, _) = self.coord.overflowing_sub(16);
    }
}
