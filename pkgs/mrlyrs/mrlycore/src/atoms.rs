use super::tensor::Tensor;

fn build_2d(n: usize, rule: impl Fn(usize, usize) -> bool) -> Tensor {
    let mut out = Tensor::new(vec![n, n]);
    for x in 0..n {
        for y in 0..n {
            out.bytes_mut()[x * n + y] = rule(x, y) as u8;
        }
    }
    out
}

fn build_3d(n: usize, rule: impl Fn(usize, usize, usize) -> bool) -> Tensor {
    let mut out = Tensor::new(vec![n, n, n]);
    for x in 0..n {
        for y in 0..n {
            for z in 0..n {
                out.bytes_mut()[(x * n + y) * n + z] = rule(x, y, z) as u8;
            }
        }
    }
    out
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

pub fn noise_2d(n: usize, density: f64) -> Tensor {
    build_2d(n, |_, _| super::state::random() < density)
}

pub fn noise_3d(n: usize, density: f64) -> Tensor {
    build_3d(n, |_, _, _| super::state::random() < density)
}

pub fn carpet_2d(n: usize) -> Tensor {
    build_2d(n, |x, y| x % 2 + y % 2 <= 1)
}

pub fn carpet_3d(n: usize) -> Tensor {
    build_3d(n, |x, y, z| x % 2 + y % 2 + z % 2 <= 1)
}

pub fn net_2d(n: usize) -> Tensor {
    build_2d(n, |x, y| x % 2 + y % 2 >= 1)
}

pub fn net_3d(n: usize) -> Tensor {
    build_3d(n, |x, y, z| x % 2 + y % 2 + z % 2 >= 2)
}

pub fn htree_2d(n: usize) -> Tensor {
    build_2d(n, |x, _| x % 2 == 0)
}

pub fn vtree_2d(n: usize) -> Tensor {
    build_2d(n, |_, y| y % 2 == 0)
}

pub fn xtree_3d(n: usize) -> Tensor {
    build_3d(n, |_, y, z| y % 2 == 0 && z % 2 == 0)
}

pub fn ytree_3d(n: usize) -> Tensor {
    build_3d(n, |x, _, z| x % 2 == 0 && z % 2 == 0)
}

pub fn ztree_3d(n: usize) -> Tensor {
    build_3d(n, |x, y, _| x % 2 == 0 && y % 2 == 0)
}

pub fn void_2d(n: usize) -> Tensor {
    build_2d(n, |x, y| (x + y) % 2 == 0)
}

pub fn void_3d(n: usize) -> Tensor {
    build_3d(n, |x, y, z| x % 2 == y % 2 && y % 2 == z % 2)
}
