use super::frames::writing;
use super::{COLS, ROWS};
use mrlycore::tensor::Tensor;
use mrlymath::two::{self, Cell2d};

pub fn cell(frame: &[usize]) -> Cell2d {
    let mut data = vec![0u8; ROWS * COLS];
    for &i in frame {
        data[i] = 1;
    }
    Cell2d::new(Tensor::of(data, vec![ROWS, COLS]))
}

pub fn logo() -> Cell2d {
    two::carpet(5, 1).expect("carpet(5, 1) is always valid")
}

pub fn grid(width: usize, height: usize) -> Cell2d {
    logo().tile(width, height)
}

pub fn wordmark() -> Cell2d {
    cell(writing().last().expect("writing is never empty"))
}
