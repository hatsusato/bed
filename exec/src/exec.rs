use inst::Inst;
use state::State;

pub struct Exec {}
impl Exec {
    pub fn new() -> Self {
        Self {}
    }
    pub fn exec(key: char, state: &mut State) {
        let inst = Inst::new(key);
        state.exec(inst);
        state.push(key);
    }
}
