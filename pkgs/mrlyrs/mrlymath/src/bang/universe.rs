use std::collections::BTreeSet;

pub type Code = u128;

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

pub fn corners(dimension: usize) -> Vec<Vec<u8>> {
    (0..1usize << dimension)
        .map(|i| {
            (0..dimension)
                .map(|j| ((i >> (dimension - 1 - j)) & 1) as u8)
                .collect()
        })
        .collect()
}

pub fn corner_index(corner: &[u8]) -> usize {
    corner.iter().fold(0, |acc, &b| (acc << 1) | b as usize)
}

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

pub fn apply(element: &(Vec<usize>, Vec<u8>), corner: &[u8]) -> Vec<u8> {
    let (perm, flips) = element;
    (0..corner.len())
        .map(|i| corner[perm[i]] ^ flips[i])
        .collect()
}

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

pub fn index_width(dimension: usize) -> usize {
    let max: Code = (1 << (1 << dimension)) - 1;
    max.to_string().len()
}

#[derive(Clone, Debug)]
pub struct Design {
    pub i: Code,
    pub dimension: usize,
    pub canonical: bool,
    pub class_rep: Code,
    pub orbit_size: usize,
}

impl Design {
    pub fn name(&self) -> String {
        format!(
            "mrly_{:0width$}",
            self.i,
            width = index_width(self.dimension)
        )
    }
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
    pub fn degree(&self) -> i32 {
        degree(self.i, self.dimension)
    }
    pub fn anf(&self) -> String {
        anf_string(self.i, self.dimension)
    }
}

pub struct Universe {
    pub dimension: usize,
    pub total: usize,
    class_rep: Vec<Code>,
    orbit_size: Vec<usize>,
}

impl Universe {
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
    pub fn all(&self) -> Vec<Design> {
        (0..self.total)
            .map(|code| self.design(code as Code))
            .collect()
    }
    pub fn canonical(&self) -> Vec<Design> {
        self.all().into_iter().filter(|d| d.canonical).collect()
    }
    pub fn distinct(&self) -> usize {
        let mut reps: Vec<Code> = self.class_rep.clone();
        reps.sort();
        reps.dedup();
        reps.len()
    }
}

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
    fn names_and_anf() {
        let u = bang(2);
        assert_eq!(u.design(0).name(), "mrly_00");
        assert_eq!(u.design(0).anf(), "0");
        assert_eq!(u.design(1).anf(), "1+y+x+xy");
    }
}
