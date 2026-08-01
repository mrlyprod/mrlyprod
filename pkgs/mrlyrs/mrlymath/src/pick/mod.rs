use crate::two::{designs, Cell2d};
use mrlycore::tensor::Tensor;

pub const NAMES: [&str; 16] = [
    "blank", "dot", "dash", "slash", "tick", "bar", "grid", "comb", "node", "step", "weave",
    "mesh", "wave", "loom", "quilt", "solid",
];

pub fn vocab(count: usize) -> Vec<usize> {
    (1..NAMES.len() - 1).take(count).collect()
}

pub fn name(code: usize) -> &'static str {
    NAMES.get(code).copied().unwrap_or("blank")
}

pub fn tile(code: usize, size: usize, fg: [u8; 4], bg: [u8; 4]) -> Cell2d {
    let mask = designs::create(code as u128, size, 1, 0, 2)
        .map(|c| c.types().clone())
        .unwrap_or_else(|_| Tensor::full(vec![size, size], 1));
    let colors: Vec<[u8; 4]> = mask
        .bytes()
        .iter()
        .map(|&v| if v == 1 { fg } else { bg })
        .collect();
    let mut cell = Cell2d::new(mask);
    cell.cell.colors = Some(colors);
    cell
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn tiles_are_fixed_size_and_distinct() {
        let a = tile(3, 8, [255, 0, 0, 255], [0, 0, 0, 0]);
        let b = tile(7, 8, [255, 0, 0, 255], [0, 0, 0, 0]);
        assert_eq!((a.width(), a.height()), (8, 8));
        assert_eq!((b.width(), b.height()), (8, 8));
        assert_ne!(a.types(), b.types());
    }
    #[test]
    fn vocab_skips_trivial_codes() {
        let v = vocab(9);
        assert_eq!(v.len(), 9);
        assert!(!v.contains(&0) && !v.contains(&15));
    }
}
