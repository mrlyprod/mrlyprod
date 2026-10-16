use crate::tensor::Tensor;

fn build(n: usize, rank: usize, rule: impl Fn(&[usize]) -> bool) -> Tensor {
    let mut out = Tensor::new(vec![n; rank]);
    let mut at = vec![0usize; rank];
    for flat in 0..out.size() {
        let mut rest = flat;
        for axis in (0..rank).rev() {
            at[axis] = rest % n;
            rest /= n;
        }
        out.put(flat, i64::from(rule(&at)));
    }
    out
}

fn carpet(at: &[usize]) -> bool {
    at.iter().filter(|c| *c % 2 == 1).count() <= 1
}

pub(crate) fn zeros_2d(n: usize) -> Tensor {
    Tensor::new(vec![n, n])
}

pub(crate) fn ones_2d(n: usize) -> Tensor {
    Tensor::full(vec![n, n], 1)
}

pub(crate) fn ones_3d(n: usize) -> Tensor {
    Tensor::full(vec![n, n, n], 1)
}

pub(crate) fn carpet_2d(n: usize) -> Tensor {
    build(n, 2, carpet)
}

pub(crate) fn carpet_3d(n: usize) -> Tensor {
    build(n, 3, carpet)
}

pub(crate) fn xtree_3d(n: usize) -> Tensor {
    build(n, 3, |at| at.iter().skip(1).all(|c| c % 2 == 0))
}
