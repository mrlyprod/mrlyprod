use super::models::CellNd;
use crate::core::census::{count, exposed};

pub fn fills<const N: usize>(cell: &CellNd<N>) -> usize {
    count(cell.types(), 1)
}

pub fn voids<const N: usize>(cell: &CellNd<N>) -> usize {
    count(cell.types(), 0)
}

pub fn exposure<const N: usize>(cell: &CellNd<N>) -> u128 {
    exposed(cell.types())
}
