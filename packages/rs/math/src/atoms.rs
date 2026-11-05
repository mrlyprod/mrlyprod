use mrly_core::rng::Rng;
use mrly_core::tensor::Tensor;

fn build(n: usize, rank: usize, mut rule: impl FnMut(&[usize]) -> bool) -> Tensor {
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

fn odd(at: &[usize]) -> usize {
    at.iter().filter(|c| *c % 2 == 1).count()
}

pub fn carpet_nd(n: usize, rank: usize) -> Tensor {
    build(n, rank, |at| odd(at) <= 1)
}

pub fn net_nd(n: usize, rank: usize) -> Tensor {
    build(n, rank, |at| odd(at) + 1 >= rank)
}

pub fn tree_nd(n: usize, rank: usize, axis: usize) -> Tensor {
    build(n, rank, |at| {
        at.iter().enumerate().all(|(i, c)| i == axis || c % 2 == 0)
    })
}

pub fn void_nd(n: usize, rank: usize) -> Tensor {
    build(n, rank, |at| at.iter().all(|c| c % 2 == at[0] % 2))
}

pub fn point_nd(n: usize, rank: usize) -> Tensor {
    build(n, rank, |at| odd(at) == rank)
}

pub fn dust_nd(n: usize, rank: usize) -> Tensor {
    build(n, rank, |at| odd(at) == 0)
}

pub fn line_nd(n: usize, rank: usize, axis: usize) -> Tensor {
    build(n, rank, |at| {
        at.iter().enumerate().all(|(i, c)| i == axis || c % 2 == 1)
    })
}

pub fn star_nd(n: usize, rank: usize) -> Tensor {
    build(n, rank, |at| odd(at) == 1)
}

pub fn zeros_2d(n: usize) -> Tensor {
    Tensor::new(vec![n, n])
}

pub fn zeros_3d(n: usize) -> Tensor {
    Tensor::new(vec![n, n, n])
}

pub fn ones_2d(n: usize) -> Tensor {
    Tensor::full(vec![n, n], 1)
}

pub fn ones_3d(n: usize) -> Tensor {
    Tensor::full(vec![n, n, n], 1)
}

pub fn noise_2d(n: usize, density: f64, rng: &mut Rng) -> Tensor {
    build(n, 2, |_| rng.chance(density))
}

pub fn noise_3d(n: usize, density: f64, rng: &mut Rng) -> Tensor {
    build(n, 3, |_| rng.chance(density))
}

pub fn carpet_2d(n: usize) -> Tensor {
    carpet_nd(n, 2)
}

pub fn carpet_3d(n: usize) -> Tensor {
    carpet_nd(n, 3)
}

pub fn net_2d(n: usize) -> Tensor {
    net_nd(n, 2)
}

pub fn net_3d(n: usize) -> Tensor {
    net_nd(n, 3)
}

pub fn htree_2d(n: usize) -> Tensor {
    tree_nd(n, 2, 1)
}

pub fn vtree_2d(n: usize) -> Tensor {
    tree_nd(n, 2, 0)
}

pub fn xtree_3d(n: usize) -> Tensor {
    tree_nd(n, 3, 0)
}

pub fn ytree_3d(n: usize) -> Tensor {
    tree_nd(n, 3, 1)
}

pub fn ztree_3d(n: usize) -> Tensor {
    tree_nd(n, 3, 2)
}

pub fn void_2d(n: usize) -> Tensor {
    void_nd(n, 2)
}

pub fn void_3d(n: usize) -> Tensor {
    void_nd(n, 3)
}

pub fn point_2d(n: usize) -> Tensor {
    point_nd(n, 2)
}

pub fn point_3d(n: usize) -> Tensor {
    point_nd(n, 3)
}

pub fn dust_2d(n: usize) -> Tensor {
    dust_nd(n, 2)
}

pub fn dust_3d(n: usize) -> Tensor {
    dust_nd(n, 3)
}

pub fn hline_2d(n: usize) -> Tensor {
    line_nd(n, 2, 1)
}

pub fn vline_2d(n: usize) -> Tensor {
    line_nd(n, 2, 0)
}

pub fn xline_3d(n: usize) -> Tensor {
    line_nd(n, 3, 0)
}

pub fn yline_3d(n: usize) -> Tensor {
    line_nd(n, 3, 1)
}

pub fn zline_3d(n: usize) -> Tensor {
    line_nd(n, 3, 2)
}

pub fn star_2d(n: usize) -> Tensor {
    star_nd(n, 2)
}

pub fn star_3d(n: usize) -> Tensor {
    star_nd(n, 3)
}
