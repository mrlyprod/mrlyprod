use super::models::CellNd;
use mrlycore::cell::mapping;
use mrlycore::colors::Color;
use mrlycore::enums::Mode;
use std::collections::HashMap;

pub fn paint<const N: usize>(
    cell: CellNd<N>,
    custom: Option<&HashMap<u8, Vec<Color>>>,
    mode: Option<Mode>,
) -> CellNd<N> {
    let defaults = mapping();
    let mapping = custom.unwrap_or(&defaults);
    cell.paint(mapping, mode.unwrap_or(Mode::Type))
}
