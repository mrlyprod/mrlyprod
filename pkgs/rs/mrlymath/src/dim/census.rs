use super::models::CellNd;
use crate::census::{count, exposed};

/// Counts the filled sites of the cell.
pub fn fills<const N: usize>(cell: &CellNd<N>) -> usize {
    count(cell.types(), 1)
}

/// Counts the empty sites of the cell.
pub fn voids<const N: usize>(cell: &CellNd<N>) -> usize {
    count(cell.types(), 0)
}

/// Counts the faces of filled sites open to emptiness or the border.
pub fn exposure<const N: usize>(cell: &CellNd<N>) -> u128 {
    exposed(cell.types())
}
