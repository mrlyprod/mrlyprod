use super::models::Cell6d;
use crate::core::cell::mapping;
use crate::core::cell::Mode;
use crate::core::colors::Color;
use crate::core::error::Result;
use crate::core::rng::Rng;
use std::collections::HashMap;

/// Colors each triangle by its type through the custom or default mapping in the given or type mode.
///
/// # Errors
///
/// Errors when the Random mode arrives without a stream to draw from.
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
