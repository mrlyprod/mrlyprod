use super::Boundary;
use mrly_error::Result;
use mrly_math::cell::models::counting_dtype;
use mrly_tensor::Tensor;
use mrly_two::Cell2d;

pub fn next_grid(
    cell: &Cell2d,
    birth: &[usize],
    survive: &[usize],
    mask: &Tensor,
    boundary: Boundary,
) -> Result<Cell2d> {
    let types = cell.types();
    let neighbors = types.neighbors(mask, 1, boundary.wrap(), counting_dtype(mask))?;
    let side = mask.size() + 1;
    let mut rule = vec![0u8; 2 * side];
    for (row, counts) in [birth, survive].into_iter().enumerate() {
        for &n in counts.iter().filter(|&&n| n < side) {
            rule[row * side + n] = 1;
        }
    }
    let next = (0..types.size())
        .map(|i| rule[usize::from(types.at(i) == 1) * side + neighbors.at(i) as usize])
        .collect();
    Cell2d::new(Tensor::of(next, types.shape.clone())?)
}
