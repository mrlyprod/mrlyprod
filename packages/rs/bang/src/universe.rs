use mrly_error::{Result, value_error};
use mrly_math::code::Code;
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;

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
        let mut image: u128 = 0;
        for (i, cell) in cells.iter().enumerate() {
            if (code.get() >> i) & 1 == 1 {
                image |= 1 << corner_index(&apply(g, cell));
            }
        }
        out.insert(Code(image));
    }
    out
}

pub fn anf(code: Code, dimension: usize) -> Vec<u8> {
    let cells = corners(dimension);
    let mut coeff: Vec<u8> = (0..cells.len())
        .map(|i| ((code.get() >> i) & 1) as u8)
        .collect();
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

pub fn total_exposure(code: Code, dimension: usize) -> bool {
    let cells = corners(dimension);
    let code = code.get();
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

pub fn touches_every_corner(code: Code, dimension: usize) -> bool {
    let all_even: Vec<u8> = vec![0; dimension];
    (code.get() >> corner_index(&all_even)) & 1 == 1
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

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Design {
    pub i: Code,
    pub dimension: usize,
    pub canonical: bool,
    pub class_rep: Code,
    pub orbit_size: usize,
}

impl Design {
    pub fn name(&self) -> Result<String> {
        mrly_name::Named::to_prose(&mrly_math::name::Bang::new(self.i.get(), self.dimension, 2))
    }
    pub fn rule(&self) -> Vec<Vec<u8>> {
        let cells = corners(self.dimension);
        let mut out: Vec<Vec<u8>> = cells
            .into_iter()
            .enumerate()
            .filter(|(i, _)| (self.i.get() >> i) & 1 == 1)
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

#[derive(Serialize, Deserialize)]
pub struct Universe {
    pub dimension: usize,
    pub total: usize,
    class_rep: Vec<Code>,
    orbit_size: Vec<usize>,
}

impl Universe {
    pub fn new(dimension: usize) -> Result<Universe> {
        if !(1..=4).contains(&dimension) {
            return value_error(format!(
                "a universe is enumerable only for dimensions 1 to 4, not {dimension}."
            ));
        }
        let total = 1usize << (1usize << dimension);
        let mut class_rep = Vec::with_capacity(total);
        let mut orbit_size = Vec::with_capacity(total);
        for code in 0..total {
            let orb = orbit(Code::from(code as u128), dimension);
            class_rep.push(*orb.iter().next().unwrap());
            orbit_size.push(orb.len());
        }
        Ok(Universe {
            dimension,
            total,
            class_rep,
            orbit_size,
        })
    }
    pub fn design(&self, code: Code) -> Design {
        let rep = self.class_rep[code.get() as usize];
        Design {
            i: code,
            dimension: self.dimension,
            canonical: rep == code,
            class_rep: rep,
            orbit_size: self.orbit_size[code.get() as usize],
        }
    }
    pub fn all(&self) -> Vec<Design> {
        (0..self.total)
            .map(|code| self.design(Code::from(code as u128)))
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

pub fn bang(dimension: usize) -> Result<Universe> {
    Universe::new(dimension)
}
