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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::math::bang::factory::{corners_to_code, create, residue_corners};
    use crate::math::bang::universe::Code;
    type Atom = fn(usize) -> Tensor;
    fn rule_code_2d(rule: impl Fn(usize, usize) -> bool) -> Code {
        let filled: Vec<Vec<u8>> = residue_corners(2, 2)
            .into_iter()
            .filter(|c| rule(c[0] as usize, c[1] as usize))
            .collect();
        corners_to_code(&filled, 2, 2)
    }
    fn rule_code_3d(rule: impl Fn(usize, usize, usize) -> bool) -> Code {
        let filled: Vec<Vec<u8>> = residue_corners(3, 2)
            .into_iter()
            .filter(|c| rule(c[0] as usize, c[1] as usize, c[2] as usize))
            .collect();
        corners_to_code(&filled, 3, 2)
    }
    #[test]
    fn atoms_2d_match_factory() {
        let pairs: Vec<(Atom, Code)> = vec![
            (carpet_2d, rule_code_2d(|x, y| x + y <= 1)),
            (net_2d, rule_code_2d(|x, y| x + y >= 1)),
            (htree_2d, rule_code_2d(|x, _| x == 0)),
            (vtree_2d, rule_code_2d(|_, y| y == 0)),
            (void_2d, rule_code_2d(|x, y| (x + y) % 2 == 0)),
        ];
        for (atom, code) in pairs {
            for n in 1..8 {
                assert_eq!(atom(n), create(code, n, 2, 2, 1).unwrap());
            }
        }
    }
    #[test]
    fn atoms_3d_match_factory() {
        let pairs: Vec<(Atom, Code)> = vec![
            (carpet_3d, rule_code_3d(|x, y, z| x + y + z <= 1)),
            (net_3d, rule_code_3d(|x, y, z| x + y + z >= 2)),
            (xtree_3d, rule_code_3d(|_, y, z| y == 0 && z == 0)),
            (ytree_3d, rule_code_3d(|x, _, z| x == 0 && z == 0)),
            (ztree_3d, rule_code_3d(|x, y, _| x == 0 && y == 0)),
            (void_3d, rule_code_3d(|x, y, z| x == y && y == z)),
        ];
        for (atom, code) in pairs {
            for n in 1..6 {
                assert_eq!(atom(n), create(code, n, 3, 2, 1).unwrap());
            }
        }
    }
    #[test]
    fn net_complements() {
        let full: Code = 15;
        assert_eq!(
            rule_code_2d(|x, y| x + y >= 1),
            full ^ rule_code_2d(|x, y| x + y == 0)
        );
        let full3: Code = 255;
        assert_eq!(
            rule_code_3d(|x, y, z| x + y + z >= 2),
            full3 ^ rule_code_3d(|x, y, z| x + y + z <= 1)
        );
    }
    #[test]
    fn carpet_3d_is_menger() {
        assert_eq!(rule_code_3d(|x, y, z| x + y + z <= 1), 23);
        assert_eq!(carpet_3d(3).sum(), 20);
    }
    #[test]
    fn trees_are_rotations() {
        for n in 1..6 {
            let h = htree_2d(n);
            let v = vtree_2d(n);
            assert_eq!(h.sum(), v.sum());
            for x in 0..n {
                for y in 0..n {
                    assert_eq!(h.get(&[x, y]), v.get(&[y, x]));
                }
            }
        }
    }
}
