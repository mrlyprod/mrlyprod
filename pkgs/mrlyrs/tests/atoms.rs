mod tests {
    use mrlyrs::core::tensor::Tensor;
    use mrlyrs::math::atoms::*;
    use mrlyrs::math::bang::factory::{corners_to_code, create, residue_corners};
    use mrlyrs::math::bang::Code;
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
        let full = Code::from(15u64);
        assert_eq!(
            rule_code_2d(|x, y| x + y >= 1),
            Code::from(full.get() ^ rule_code_2d(|x, y| x + y == 0).get())
        );
        let full3 = Code::from(255u64);
        assert_eq!(
            rule_code_3d(|x, y, z| x + y + z >= 2),
            Code::from(full3.get() ^ rule_code_3d(|x, y, z| x + y + z <= 1).get())
        );
    }
    #[test]
    fn carpet_3d_is_menger() {
        assert_eq!(rule_code_3d(|x, y, z| x + y + z <= 1).get(), 23);
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
