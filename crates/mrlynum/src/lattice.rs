use crate::factor::coprime;

/// Sieves the Euler totients of zero through n.
///
/// ```
/// assert_eq!(mrlynum::lattice::totients(6), vec![0, 1, 1, 2, 2, 4, 2]);
/// ```
pub fn totients(n: usize) -> Vec<u64> {
    let mut phi: Vec<u64> = (0..=n as u64).collect();
    for p in 2..=n {
        if phi[p] == p as u64 {
            for m in (p..=n).step_by(p) {
                phi[m] -= phi[m] / p as u64;
            }
        }
    }
    phi
}

/// Counts the ordered pairs of coprime coordinates between one and n: twice the totient sum less one.
pub fn coprime_pairs(n: usize) -> u64 {
    if n == 0 {
        return 0;
    }
    2 * totients(n)[1..].iter().sum::<u64>() - 1
}

/// Estimates pi from visibility: the density of coprime pairs in the n-by-n window tends to six over pi squared.
pub fn pi_estimate(n: usize) -> f64 {
    let density = coprime_pairs(n) as f64 / (n as f64 * n as f64);
    (6.0 / density).sqrt()
}

/// A visible node: a reduced fraction and the brightness a stack of scales one through the window gives it.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Node {
    /// The numerator, coprime to the denominator.
    pub num: u64,
    /// The denominator.
    pub den: u64,
    /// The count of scales putting a line here: the floor of the window over the denominator.
    pub brightness: u64,
}

/// A grid crossing of two visible nodes, its brightness the separable product.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Node2d {
    /// The horizontal node.
    pub x: Node,
    /// The vertical node.
    pub y: Node,
    /// The product of the two axis brightnesses.
    pub brightness: u64,
}

/// Lists the visible nodes of a window in ascending value: every reduced fraction with denominator at most n.
pub fn nodes(n: usize) -> Vec<Node> {
    let mut out = Vec::new();
    for den in 1..=n {
        for num in 0..=den {
            if coprime(num, den) {
                out.push(Node {
                    num: num as u64,
                    den: den as u64,
                    brightness: (n / den) as u64,
                });
            }
        }
    }
    out.sort_by(|a, b| (a.num as u128 * b.den as u128).cmp(&(b.num as u128 * a.den as u128)));
    out
}

/// Walks the Farey sequence of the order by the Stern-Brocot mediant recurrence from zero over one to one over one, an independent route to the same nodes.
pub fn farey(order: usize) -> Vec<Node> {
    let mut out = Vec::new();
    if order == 0 {
        return out;
    }
    let n = order as u64;
    let (mut a, mut b, mut c, mut d) = (0u64, 1u64, 1u64, n);
    out.push(Node {
        num: a,
        den: b,
        brightness: n / b,
    });
    while c <= n {
        let k = (n + b) / d;
        (a, b, c, d) = (c, d, k * c - a, k * d - b);
        out.push(Node {
            num: a,
            den: b,
            brightness: n / b,
        });
    }
    out
}

/// Lists the grid crossings of a window's nodes, row-major over the ascending axis nodes.
pub fn grid(n: usize) -> Vec<Node2d> {
    let axis = nodes(n);
    let mut out = Vec::with_capacity(axis.len() * axis.len());
    for &y in &axis {
        for &x in &axis {
            out.push(Node2d {
                x,
                y,
                brightness: x.brightness * y.brightness,
            });
        }
    }
    out
}

/// Counts the nodes window n lights that window n minus one lacked: two at window one, phi of n after.
pub fn new_nodes(n: usize) -> u64 {
    if n == 0 {
        return 0;
    }
    (nodes(n).len() - nodes(n - 1).len()) as u64
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::f64::consts::PI;

    #[test]
    fn totients_match_hand_checked_values() {
        let phi = totients(12);
        assert_eq!(phi, vec![0, 1, 1, 2, 2, 4, 2, 6, 4, 6, 4, 10, 4]);
    }

    #[test]
    fn coprime_pairs_match_a_brute_count() {
        for n in [1usize, 2, 3, 10, 20] {
            let mut brute = 0;
            for a in 1..=n {
                for b in 1..=n {
                    if coprime(a, b) {
                        brute += 1;
                    }
                }
            }
            assert_eq!(coprime_pairs(n), brute, "window {n}");
        }
    }

    #[test]
    fn pi_estimate_converges_by_a_hundred_thousand() {
        assert!((pi_estimate(100_000) - PI).abs() < 1e-4);
    }

    #[test]
    fn node_counts_follow_the_farey_sequence() {
        for (n, count) in [(1, 2), (2, 3), (3, 5), (4, 7), (5, 11), (6, 13)] {
            assert_eq!(nodes(n).len(), count, "window {n}");
        }
    }

    #[test]
    fn nodes_are_reduced_bright_and_ascending() {
        let all = nodes(12);
        for pair in all.windows(2) {
            assert!(pair[0].num * pair[1].den < pair[1].num * pair[0].den);
        }
        for node in all {
            assert!(coprime(node.num as usize, node.den as usize));
            assert_eq!(node.brightness, 12 / node.den);
        }
    }

    #[test]
    fn the_farey_walk_lands_on_the_window_nodes_exactly() {
        assert!(farey(0).is_empty());
        for n in 1..=50 {
            assert_eq!(farey(n), nodes(n), "order {n}");
        }
        let ends = farey(7);
        assert_eq!((ends[0].num, ends[0].den), (0, 1));
        assert_eq!((ends.last().unwrap().num, ends.last().unwrap().den), (1, 1));
    }

    #[test]
    fn the_farey_length_is_one_past_the_totient_sum() {
        let phi = totients(50);
        for n in 1..=50usize {
            let want = 1 + phi[1..=n].iter().sum::<u64>() as usize;
            assert_eq!(farey(n).len(), want, "order {n}");
        }
    }

    #[test]
    fn grid_brightness_is_separable() {
        let flat = grid(3);
        assert_eq!(flat.len(), 25);
        for cross in flat {
            assert_eq!(cross.brightness, cross.x.brightness * cross.y.brightness);
        }
        let corner = &grid(3)[0];
        assert_eq!((corner.x.num, corner.y.num), (0, 0));
        assert_eq!(corner.brightness, 9);
    }

    #[test]
    fn new_nodes_is_the_totient() {
        let phi = totients(50);
        assert_eq!(new_nodes(1), 2);
        for (n, &expected) in phi.iter().enumerate().skip(2) {
            assert_eq!(new_nodes(n), expected, "window {n}");
        }
    }
}
