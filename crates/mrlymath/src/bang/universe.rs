use std::collections::BTreeSet;

/// The bitmask of filled corners that names a design.
pub type Code = u128;

/// Returns every permutation of 0..n in sorted order.
pub fn permutations(n: usize) -> Vec<Vec<usize>> {
    if n == 0 {
        return vec![vec![]];
    }
    let mut out = Vec::new();
    let mut items: Vec<usize> = (0..n).collect();
    heap(&mut items, n, &mut out);
    out.sort();
    out
}

fn heap(items: &mut Vec<usize>, k: usize, out: &mut Vec<Vec<usize>>) {
    if k == 1 {
        out.push(items.clone());
        return;
    }
    for i in 0..k {
        heap(items, k - 1, out);
        if k.is_multiple_of(2) {
            items.swap(i, k - 1);
        } else {
            items.swap(0, k - 1);
        }
    }
}

/// Returns the binary corners of a dimension in code order.
pub fn corners(dimension: usize) -> Vec<Vec<u8>> {
    (0..1usize << dimension)
        .map(|i| {
            (0..dimension)
                .map(|j| ((i >> (dimension - 1 - j)) & 1) as u8)
                .collect()
        })
        .collect()
}

/// Returns the bit position a binary corner occupies in a code.
pub fn corner_index(corner: &[u8]) -> usize {
    corner.iter().fold(0, |acc, &b| (acc << 1) | b as usize)
}

/// Returns the full symmetry group as axis permutations paired with flip patterns.
pub fn symmetries(dimension: usize) -> Vec<(Vec<usize>, Vec<u8>)> {
    let mut out = Vec::new();
    for perm in permutations(dimension) {
        for f in 0..1usize << dimension {
            let flips: Vec<u8> = (0..dimension)
                .map(|j| ((f >> (dimension - 1 - j)) & 1) as u8)
                .collect();
            out.push((perm.clone(), flips));
        }
    }
    out
}

/// Applies a symmetry element to a corner.
pub fn apply(element: &(Vec<usize>, Vec<u8>), corner: &[u8]) -> Vec<u8> {
    let (perm, flips) = element;
    (0..corner.len())
        .map(|i| corner[perm[i]] ^ flips[i])
        .collect()
}

/// Returns every code a design reaches under the full symmetry group.
pub fn orbit(code: Code, dimension: usize) -> BTreeSet<Code> {
    let cells = corners(dimension);
    let group = symmetries(dimension);
    let mut out = BTreeSet::new();
    for g in &group {
        let mut image: Code = 0;
        for (i, cell) in cells.iter().enumerate() {
            if (code >> i) & 1 == 1 {
                image |= 1 << corner_index(&apply(g, cell));
            }
        }
        out.insert(image);
    }
    out
}

/// Returns the algebraic normal form coefficients of a code, one per corner.
pub fn anf(code: Code, dimension: usize) -> Vec<u8> {
    let cells = corners(dimension);
    let mut coeff: Vec<u8> = (0..cells.len()).map(|i| ((code >> i) & 1) as u8).collect();
    for axis in 0..dimension {
        for (i, cell) in cells.iter().enumerate() {
            if cell[axis] == 1 {
                let mut lower = cell.clone();
                lower[axis] = 0;
                coeff[i] ^= coeff[corner_index(&lower)];
            }
        }
    }
    coeff
}

/// Returns the algebraic degree of a code, or -1 for the zero design.
pub fn degree(code: Code, dimension: usize) -> i32 {
    let cells = corners(dimension);
    let coeff = anf(code, dimension);
    cells
        .iter()
        .enumerate()
        .filter(|(i, _)| coeff[*i] == 1)
        .map(|(_, c)| c.iter().map(|&b| b as i32).sum())
        .max()
        .unwrap_or(-1)
}

/// Returns whether no two filled corners of a code sit at Hamming distance one.
///
/// Such a design buries no face at any side and any level, so its surface is six per cell.
///
/// ```
/// assert!(mrlymath::bang::universe::total_exposure(129, 3));
/// assert!(!mrlymath::bang::universe::total_exposure(23, 3));
/// ```
pub fn total_exposure(code: Code, dimension: usize) -> bool {
    let cells = corners(dimension);
    for (i, cell) in cells.iter().enumerate() {
        if (code >> i) & 1 == 0 {
            continue;
        }
        for axis in 0..dimension {
            let mut neighbor = cell.clone();
            neighbor[axis] ^= 1;
            if (code >> corner_index(&neighbor)) & 1 == 1 {
                return false;
            }
        }
    }
    true
}

/// Returns whether a code fills the all-even corner, the rule that touches every grid corner at odd side.
///
/// ```
/// assert!(mrlymath::bang::universe::touches_every_corner(23, 3));
/// assert!(!mrlymath::bang::universe::touches_every_corner(232, 3));
/// ```
pub fn touches_every_corner(code: Code, dimension: usize) -> bool {
    let all_even: Vec<u8> = vec![0; dimension];
    (code >> corner_index(&all_even)) & 1 == 1
}

/// Formats the algebraic normal form of a code as a sum of monomials.
pub fn anf_string(code: Code, dimension: usize) -> String {
    const NAMES: [char; 6] = ['x', 'y', 'z', 'w', 'v', 'u'];
    let cells = corners(dimension);
    let coeff = anf(code, dimension);
    let mut order: Vec<usize> = (0..cells.len()).collect();
    order.sort_by_key(|&i| {
        (
            cells[i].iter().map(|&b| b as usize).sum::<usize>(),
            cells[i].clone(),
        )
    });
    let mut terms = Vec::new();
    for i in order {
        if coeff[i] == 1 {
            let popcount: usize = cells[i].iter().map(|&b| b as usize).sum();
            if popcount == 0 {
                terms.push("1".to_string());
            } else {
                terms.push(
                    (0..dimension)
                        .filter(|&j| cells[i][j] == 1)
                        .map(|j| NAMES[j])
                        .collect(),
                );
            }
        }
    }
    if terms.is_empty() {
        "0".to_string()
    } else {
        terms.join("+")
    }
}

/// A single design with its place in the orbit structure.
#[derive(Clone, Debug)]
pub struct Design {
    /// The design's code.
    pub i: Code,
    /// The design's dimension.
    pub dimension: usize,
    /// Whether this code is the smallest in its orbit.
    pub canonical: bool,
    /// The smallest code in the orbit.
    pub class_rep: Code,
    /// The number of codes in the orbit.
    pub orbit_size: usize,
}

impl Design {
    /// Returns the design's canonical mrly name.
    pub fn name(&self) -> String {
        crate::name::Named::to_str(&crate::name::Bang::new(self.i, self.dimension, 2))
    }
    /// Returns the design's filled corners in sorted order.
    pub fn rule(&self) -> Vec<Vec<u8>> {
        let cells = corners(self.dimension);
        let mut out: Vec<Vec<u8>> = cells
            .into_iter()
            .enumerate()
            .filter(|(i, _)| (self.i >> i) & 1 == 1)
            .map(|(_, c)| c)
            .collect();
        out.sort();
        out
    }
    /// Returns the design's algebraic degree, or -1 for the zero design.
    pub fn degree(&self) -> i32 {
        degree(self.i, self.dimension)
    }
    /// Returns the design's algebraic normal form as a string.
    pub fn anf(&self) -> String {
        anf_string(self.i, self.dimension)
    }
}

/// The complete enumeration of one dimension's designs and orbits.
pub struct Universe {
    /// The universe's dimension.
    pub dimension: usize,
    /// The number of codes in the universe.
    pub total: usize,
    class_rep: Vec<Code>,
    orbit_size: Vec<usize>,
}

impl Universe {
    /// Enumerates every orbit of a dimension from 1 to 4.
    pub fn new(dimension: usize) -> Self {
        assert!(
            (1..=4).contains(&dimension),
            "bang is enumerable only for dimensions 1-4"
        );
        let total = 1usize << (1usize << dimension);
        let mut class_rep = Vec::with_capacity(total);
        let mut orbit_size = Vec::with_capacity(total);
        for code in 0..total {
            let orb = orbit(code as Code, dimension);
            class_rep.push(*orb.iter().next().unwrap());
            orbit_size.push(orb.len());
        }
        Universe {
            dimension,
            total,
            class_rep,
            orbit_size,
        }
    }
    /// Returns the design at a code with its precomputed orbit facts.
    pub fn design(&self, code: Code) -> Design {
        let rep = self.class_rep[code as usize];
        Design {
            i: code,
            dimension: self.dimension,
            canonical: rep == code,
            class_rep: rep,
            orbit_size: self.orbit_size[code as usize],
        }
    }
    /// Returns every design in code order.
    pub fn all(&self) -> Vec<Design> {
        (0..self.total)
            .map(|code| self.design(code as Code))
            .collect()
    }
    /// Returns the designs whose codes lead their orbits.
    pub fn canonical(&self) -> Vec<Design> {
        self.all().into_iter().filter(|d| d.canonical).collect()
    }
    /// Returns the number of distinct orbits.
    pub fn distinct(&self) -> usize {
        let mut reps: Vec<Code> = self.class_rep.clone();
        reps.sort();
        reps.dedup();
        reps.len()
    }
}

/// Builds the universe of a dimension.
///
/// ```
/// let u = mrlymath::bang::bang(2);
/// assert_eq!(u.total, 16);
/// assert_eq!(u.distinct(), 6);
/// ```
pub fn bang(dimension: usize) -> Universe {
    Universe::new(dimension)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn total_and_distinct_counts() {
        assert_eq!(bang(1).distinct(), 3);
        assert_eq!(bang(2).distinct(), 6);
        assert_eq!(bang(3).distinct(), 22);
        assert_eq!(bang(1).total, 4);
        assert_eq!(bang(2).total, 16);
        assert_eq!(bang(3).total, 256);
    }
    #[test]
    fn prefix_codes_canonical() {
        for d in 1..=3 {
            let u = bang(d);
            for k in 0..=(1usize << d) {
                let code = (1u128 << k) - 1;
                if (code as usize) < u.total {
                    assert!(u.design(code).canonical);
                }
            }
        }
    }
    #[test]
    fn anti_closure_3d() {
        let u = bang(3);
        let full: Code = (1 << (1 << 3)) - 1;
        let reps: Vec<Code> = u.canonical().iter().map(|d| d.class_rep).collect();
        for d in u.canonical() {
            let anti = full ^ d.i;
            assert!(reps.contains(&u.design(anti).class_rep));
        }
    }
    #[test]
    fn orbit_sizes_partition() {
        for d in 2..=3usize {
            let u = bang(d);
            let order = (1usize << d) * (1..=d).product::<usize>();
            let total: usize = u.canonical().iter().map(|x| x.orbit_size).sum();
            assert_eq!(total, u.total);
            for x in u.canonical() {
                assert_eq!(order % x.orbit_size, 0);
            }
        }
    }
    #[test]
    fn degree_histogram_3d() {
        let u = bang(3);
        let mut hist = std::collections::HashMap::new();
        for d in u.canonical() {
            *hist.entry(d.degree()).or_insert(0) += 1;
        }
        let expected: std::collections::HashMap<i32, i32> =
            [(-1, 1), (0, 1), (1, 3), (2, 9), (3, 8)]
                .into_iter()
                .collect();
        assert_eq!(hist, expected);
    }
    #[test]
    fn total_exposure_names_the_independent_corner_sets() {
        let exposed: Vec<Code> = (0..256).filter(|&c| total_exposure(c, 3)).collect();
        assert_eq!(exposed.len(), 35);
        let classes: BTreeSet<Code> = exposed
            .iter()
            .map(|&c| *orbit(c, 3).iter().next().unwrap())
            .collect();
        assert_eq!(
            classes.into_iter().collect::<Vec<Code>>(),
            [0, 1, 6, 22, 24, 105]
        );
        assert!(total_exposure(129, 3));
        assert!(!total_exposure(23, 3));
    }

    #[test]
    fn half_the_rules_hold_the_all_even_corner() {
        assert_eq!(
            (0..256).filter(|&c| touches_every_corner(c, 3)).count(),
            128
        );
        for code in [23u128, 3, 129] {
            assert!(touches_every_corner(code, 3), "code={code}");
        }
        assert!(!touches_every_corner(232, 3));
    }

    #[test]
    fn names_and_anf() {
        let u = bang(2);
        assert_eq!(u.design(0).name(), "mrly_bang_d2_0");
        assert_eq!(u.design(7).name(), "mrly_bang_d2_7");
        assert_eq!(u.design(0).anf(), "0");
        assert_eq!(u.design(1).anf(), "1+y+x+xy");
    }
}
