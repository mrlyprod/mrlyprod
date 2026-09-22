use super::models::CellNd;
use crate::core::cell::mapping;
use crate::core::cell::Mode;
use crate::core::colors::Color;
use crate::core::error::Result;
use crate::core::rng::Rng;
use std::collections::HashMap;

/// Colors the cell through the given mapping and mode, defaulting to the standard palette by type.
///
/// # Errors
///
/// Errors when the Random mode arrives without a stream to draw from.
pub fn paint<const N: usize>(
    cell: CellNd<N>,
    custom: Option<&HashMap<u8, Vec<Color>>>,
    mode: Option<Mode>,
    rng: Option<&mut Rng>,
) -> Result<CellNd<N>> {
    let defaults = mapping();
    let mapping = custom.unwrap_or(&defaults);
    cell.paint(mapping, mode.unwrap_or(Mode::Type), rng)
}
