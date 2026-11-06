use super::models::CellNd;

pub fn fills<const N: usize>(cell: &CellNd<N>) -> usize {
    cell.types().count(1)
}

pub fn voids<const N: usize>(cell: &CellNd<N>) -> usize {
    cell.types().count(0)
}

pub fn exposure<const N: usize>(cell: &CellNd<N>) -> u128 {
    cell.types().exposed()
}
