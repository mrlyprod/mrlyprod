use super::models::Cell6d;
use mrlycore::cell::mapping;
use mrlycore::colors::Color;
use mrlycore::enums::Mode;
use std::collections::HashMap;

pub fn paint(
    mut cell: Cell6d,
    custom: Option<&HashMap<u8, Vec<Color>>>,
    mode: Option<Mode>,
) -> Cell6d {
    let defaults = mapping();
    let mapping = custom.unwrap_or(&defaults);
    cell.cell = cell.cell.paint(mapping, mode.unwrap_or(Mode::Type));
    cell
}
