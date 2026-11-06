use super::models::CellHex;
use mrly_color::Color;
use mrly_error::Result;
use mrly_math::cell::base::Mode;
use mrly_math::cell::base::mapping;
use mrly_rand::Rng;
use std::collections::HashMap;

pub fn paint(
    mut cell: CellHex,
    custom: Option<&HashMap<u8, Vec<Color>>>,
    mode: Option<Mode>,
    rng: Option<&mut Rng>,
) -> Result<CellHex> {
    let defaults = mapping();
    let mapping = custom.unwrap_or(&defaults);
    cell.cell = cell.cell.paint(mapping, mode.unwrap_or(Mode::Type), rng)?;
    Ok(cell)
}
