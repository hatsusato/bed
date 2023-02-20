#[derive(Clone)]
pub enum IssueType {
    Macro,
}

#[derive(Clone)]
pub enum NameType {
    Define,
}

#[derive(Clone)]
pub enum Ctrl {
    Normal,
    Ignore,
    Record(char),
    Issue(IssueType),
    Name(NameType),
    Body,
    Quote,
}
impl Default for Ctrl {
    fn default() -> Self {
        Self::Normal
    }
}
