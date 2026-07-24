use mrlycore::tensor::Tensor;

pub fn count(grid: &Tensor, value: u8) -> usize {
    grid.bytes().iter().filter(|&&v| v == value).count()
}

pub fn exposed(grid: &Tensor) -> u128 {
    let dims = grid.shape.clone();
    let rank = dims.len();
    let mut total: u128 = 0;
    for flat in 0..grid.size() {
        if grid.bytes()[flat] == 0 {
            continue;
        }
        let mut rem = flat;
        let mut coord = Vec::with_capacity(rank);
        for axis in 0..rank {
            let stride: usize = dims[(axis + 1)..].iter().product();
            coord.push(rem / stride);
            rem %= stride;
        }
        for axis in 0..rank {
            if coord[axis] == 0 || {
                let mut lo = coord.clone();
                lo[axis] -= 1;
                grid.get(&lo) == 0
            } {
                total += 1;
            }
            if coord[axis] + 1 == dims[axis] || {
                let mut hi = coord.clone();
                hi[axis] += 1;
                grid.get(&hi) == 0
            } {
                total += 1;
            }
        }
    }
    total
}

#[cfg(test)]
mod tests {
    use super::*;
    use mrlycore::atoms;
    #[test]
    fn exposed_is_perimeter_in_2d() {
        assert_eq!(exposed(&atoms::ones_2d(1)), 4);
        assert_eq!(exposed(&atoms::ones_2d(3)), 12);
        assert_eq!(exposed(&atoms::carpet_2d(3)), 16);
    }
    #[test]
    fn exposed_is_surface_in_3d() {
        assert_eq!(exposed(&atoms::ones_3d(1)), 6);
        assert_eq!(exposed(&atoms::ones_3d(2)), 24);
    }
    #[test]
    fn counts() {
        let c = atoms::carpet_2d(3);
        assert_eq!(count(&c, 1), 8);
        assert_eq!(count(&c, 0), 1);
    }
}
