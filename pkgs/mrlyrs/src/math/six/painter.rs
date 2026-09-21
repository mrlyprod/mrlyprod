use super::models::Cell6d;
use crate::core::cell::mapping;
use crate::core::cell::Mode;
use crate::core::colors::Color;
use std::collections::HashMap;

/// Colors each triangle by its type through the custom or default mapping in the given or type mode.
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
