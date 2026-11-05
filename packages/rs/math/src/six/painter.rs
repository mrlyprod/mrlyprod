use super::models::Cell6d;
use mrly_core::cell::Mode;
use mrly_core::cell::mapping;
use mrly_core::colors::Color;
use mrly_core::error::Result;
use mrly_core::rng::Rng;
use std::collections::HashMap;

pub fn paint(
    mut cell: Cell6d,
    custom: Option<&HashMap<u8, Vec<Color>>>,
    mode: Option<Mode>,
    rng: Option<&mut Rng>,
) -> Result<Cell6d> {
    let defaults = mapping();
    let mapping = custom.unwrap_or(&defaults);
    cell.cell = cell.cell.paint(mapping, mode.unwrap_or(Mode::Type), rng)?;
    Ok(cell)
}
