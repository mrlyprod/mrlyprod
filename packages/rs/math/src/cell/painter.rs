use super::models::CellNd;
use crate::cell::base::Mode;
use crate::cell::base::mapping;
use mrly_color::Color;
use mrly_error::Result;
use mrly_rand::Rng;
use std::collections::HashMap;

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
