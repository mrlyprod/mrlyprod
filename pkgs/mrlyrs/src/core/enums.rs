#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mode {
    Type,
    Tag,
    Index,
    Enumerate,
    Random,
    Row,
    Column,
    Depth,
}
