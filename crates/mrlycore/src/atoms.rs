use super::tensor::Tensor;

fn build(n: usize, rank: usize, rule: impl Fn(&[usize]) -> bool) -> Tensor {
    let mut out = Tensor::new(vec![n; rank]);
    let mut at = vec![0usize; rank];
    for flat in 0..out.size() {
        let mut rest = flat;
        for axis in (0..rank).rev() {
            at[axis] = rest % n;
            rest /= n;
        }
        out.bytes_mut()[flat] = rule(&at) as u8;
    }
    out
}

fn odd(at: &[usize]) -> usize {
    at.iter().filter(|c| *c % 2 == 1).count()
}

/// Builds a carpet of the given side at any rank, on where at most one coordinate is odd.
pub fn carpet_nd(n: usize, rank: usize) -> Tensor {
    build(n, rank, |at| odd(at) <= 1)
}

/// Builds a net of the given side at any rank, on where the odd coordinates plus one reach the rank.
pub fn net_nd(n: usize, rank: usize) -> Tensor {
    build(n, rank, |at| odd(at) + 1 >= rank)
}

/// Builds a tree of the given side at any rank, even on every axis but the free one; an axis past the rank frees none.
pub fn tree_nd(n: usize, rank: usize, axis: usize) -> Tensor {
    build(n, rank, |at| {
        at.iter().enumerate().all(|(i, c)| i == axis || c % 2 == 0)
    })
}

/// Builds a void of the given side at any rank, on where every coordinate shares one parity.
pub fn void_nd(n: usize, rank: usize) -> Tensor {
    build(n, rank, |at| at.iter().all(|c| c % 2 == at[0] % 2))
}

/// Builds a point of the given side at any rank, on where every coordinate is odd.
pub fn point_nd(n: usize, rank: usize) -> Tensor {
    build(n, rank, |at| odd(at) == rank)
}

/// Builds a dust of the given side at any rank, on where every coordinate is even.
pub fn dust_nd(n: usize, rank: usize) -> Tensor {
    build(n, rank, |at| odd(at) == 0)
}

/// Builds a line of the given side at any rank, odd on every axis but the free one; an axis past the rank frees none.
pub fn line_nd(n: usize, rank: usize, axis: usize) -> Tensor {
    build(n, rank, |at| {
        at.iter().enumerate().all(|(i, c)| i == axis || c % 2 == 1)
    })
}

/// Builds a star of the given side at any rank, on where exactly one coordinate is odd.
pub fn star_nd(n: usize, rank: usize) -> Tensor {
    build(n, rank, |at| odd(at) == 1)
}

/// Builds an n by n tensor of zeros.
pub fn zeros_2d(n: usize) -> Tensor {
    Tensor::new(vec![n, n])
}

/// Builds an n by n by n tensor of zeros.
pub fn zeros_3d(n: usize) -> Tensor {
    Tensor::new(vec![n, n, n])
}

/// Builds an n by n tensor of ones.
pub fn ones_2d(n: usize) -> Tensor {
    Tensor::full(vec![n, n], 1)
}

/// Builds an n by n by n tensor of ones.
pub fn ones_3d(n: usize) -> Tensor {
    Tensor::full(vec![n, n, n], 1)
}

/// Builds an n by n tensor where each cell turns on with probability density.
pub fn noise_2d(n: usize, density: f64) -> Tensor {
    build(n, 2, |_| super::state::random() < density)
}

/// Builds an n by n by n tensor where each cell turns on with probability density.
pub fn noise_3d(n: usize, density: f64) -> Tensor {
    build(n, 3, |_| super::state::random() < density)
}

/// Builds an n by n carpet, on where at most one coordinate is odd.
pub fn carpet_2d(n: usize) -> Tensor {
    carpet_nd(n, 2)
}

/// Builds an n by n by n carpet, on where at most one coordinate is odd.
pub fn carpet_3d(n: usize) -> Tensor {
    carpet_nd(n, 3)
}

/// Builds an n by n net, on where at least one coordinate is odd.
pub fn net_2d(n: usize) -> Tensor {
    net_nd(n, 2)
}

/// Builds an n by n by n net, on where at least two coordinates are odd.
pub fn net_3d(n: usize) -> Tensor {
    net_nd(n, 3)
}

/// Builds an n by n tree, free on axis 1, on along the even rows.
pub fn htree_2d(n: usize) -> Tensor {
    tree_nd(n, 2, 1)
}

/// Builds an n by n tree, free on axis 0, on along the even columns.
pub fn vtree_2d(n: usize) -> Tensor {
    tree_nd(n, 2, 0)
}

/// Builds an n by n by n tree, free on axis 0, its beams running along x.
pub fn xtree_3d(n: usize) -> Tensor {
    tree_nd(n, 3, 0)
}

/// Builds an n by n by n tree, free on axis 1, its beams running along y.
pub fn ytree_3d(n: usize) -> Tensor {
    tree_nd(n, 3, 1)
}

/// Builds an n by n by n tree, free on axis 2, its beams running along z.
pub fn ztree_3d(n: usize) -> Tensor {
    tree_nd(n, 3, 2)
}

/// Builds an n by n void, on where both coordinates share one parity.
pub fn void_2d(n: usize) -> Tensor {
    void_nd(n, 2)
}

/// Builds an n by n by n void, on where all three coordinates share one parity.
pub fn void_3d(n: usize) -> Tensor {
    void_nd(n, 3)
}

/// Builds an n by n point, on where both coordinates are odd.
pub fn point_2d(n: usize) -> Tensor {
    point_nd(n, 2)
}

/// Builds an n by n by n point, on where all three coordinates are odd.
pub fn point_3d(n: usize) -> Tensor {
    point_nd(n, 3)
}

/// Builds an n by n dust, on where both coordinates are even.
pub fn dust_2d(n: usize) -> Tensor {
    dust_nd(n, 2)
}

/// Builds an n by n by n dust, on where all three coordinates are even.
pub fn dust_3d(n: usize) -> Tensor {
    dust_nd(n, 3)
}

/// Builds an n by n line, free on axis 1, on along the odd rows.
pub fn hline_2d(n: usize) -> Tensor {
    line_nd(n, 2, 1)
}

/// Builds an n by n line, free on axis 0, on along the odd columns.
pub fn vline_2d(n: usize) -> Tensor {
    line_nd(n, 2, 0)
}

/// Builds an n by n by n line, free on axis 0, its rods running along x.
pub fn xline_3d(n: usize) -> Tensor {
    line_nd(n, 3, 0)
}

/// Builds an n by n by n line, free on axis 1, its rods running along y.
pub fn yline_3d(n: usize) -> Tensor {
    line_nd(n, 3, 1)
}

/// Builds an n by n by n line, free on axis 2, its rods running along z.
pub fn zline_3d(n: usize) -> Tensor {
    line_nd(n, 3, 2)
}

/// Builds an n by n star, on where exactly one coordinate is odd.
pub fn star_2d(n: usize) -> Tensor {
    star_nd(n, 2)
}

/// Builds an n by n by n star, on where exactly one coordinate is odd.
pub fn star_3d(n: usize) -> Tensor {
    star_nd(n, 3)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state;
    #[test]
    fn noise_draws_in_flat_order() {
        let _g = state::guard();
        state::seed(11);
        let drawn = noise_3d(3, 0.5);
        state::seed(11);
        let mut want = Tensor::new(vec![3, 3, 3]);
        for flat in 0..27 {
            want.bytes_mut()[flat] = (state::random() < 0.5) as u8;
        }
        assert_eq!(drawn, want);
    }
    #[test]
    fn the_antis_complement_the_classics() {
        for n in [3, 5] {
            for (classic, anti) in [
                (carpet_2d(n), point_2d(n)),
                (net_2d(n), dust_2d(n)),
                (htree_2d(n), hline_2d(n)),
                (vtree_2d(n), vline_2d(n)),
                (void_2d(n), star_2d(n)),
            ] {
                let both = classic.bytes().iter().zip(anti.bytes());
                assert!(both.map(|(a, b)| a + b).all(|v| v == 1));
            }
        }
    }
    #[test]
    fn the_sponge_is_dust_plus_star() {
        for n in [3, 5, 7] {
            let sum: Vec<u8> = dust_3d(n)
                .bytes()
                .iter()
                .zip(star_3d(n).bytes())
                .map(|(a, b)| a + b)
                .collect();
            assert_eq!(sum, carpet_3d(n).bytes());
        }
    }
}
