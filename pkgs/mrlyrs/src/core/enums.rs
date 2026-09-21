/// The ways paint picks a color within a type's palette.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mode {
    /// The first palette color, always.
    Type,
    /// The color each cell's tag indexes.
    Tag,
    /// The color the flat position indexes.
    Index,
    /// The colors cycled in encounter order.
    Enumerate,
    /// A random palette color per cell.
    Random,
    /// The color the row index picks.
    Row,
    /// The color the column index picks.
    Column,
    /// The color the depth index picks.
    Depth,
}
