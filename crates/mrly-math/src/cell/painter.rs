use super::models::CellNd;
use mrly_core::cell::Mode;
use mrly_core::cell::mapping;
use mrly_core::colors::Color;
use mrly_core::error::Result;
use mrly_core::rng::Rng;
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
