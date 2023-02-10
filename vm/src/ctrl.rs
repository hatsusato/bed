#[derive(Clone)]
pub enum Ctrl {
    Enter,
    Quote,
    Ignore,
    While,
    Direct,
    Call,
    Define,
    Run,
    Macro,
}
impl Default for Ctrl {
    fn default() -> Self {
        Self::Enter
    }
}
