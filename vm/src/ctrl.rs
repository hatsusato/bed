#[derive(Clone)]
pub enum DelayType {
    Immediate,
    Record,
    Macro,
    While,
}

#[derive(Clone)]
pub enum NameType {
    Define,
}

#[derive(Clone)]
pub enum Ctrl {
    Normal,
    Ignore,
    Delay(DelayType),
    Record(char),
    Name(NameType),
    Body,
    Quote,
}
impl Default for Ctrl {
    fn default() -> Self {
        Self::Normal
    }
}
