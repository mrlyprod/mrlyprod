use mrly_error::{Result, value_error};
use mrly_graph::spectrum::clusters;
use mrly_linalg::{lowest, symmetric};
use mrly_math::code::Code;
use mrly_math::factory::code_to_corners;
use mrly_sequences::counts::fill;
use mrly_two::create;
use serde::{Deserialize, Serialize};

const LEAST: u128 = 16;
const MOST: u128 = 10_000;
const SIDE: usize = 128;
const CAP: usize = 64;
const GAP: f64 = 1e-8;
const ZERO: f64 = 1e-9;
const PEAK: f64 = 1e-9;
const SPLIT: f64 = 0.5;
const EMPTY: usize = usize::MAX;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Figures {
    pub side: usize,
    pub values: Vec<f64>,
    pub domains: Vec<usize>,
    pub bound: Vec<usize>,
    pub fields: Vec<f64>,
}

struct Plate {
    side: usize,
    cells: Vec<usize>,
    index: Vec<usize>,
}

impl Plate {
    fn neighbours(&self, cell: usize) -> impl Iterator<Item = usize> + '_ {
        let side = self.side;
        let at = self.cells[cell];
        let (row, col) = (at / side, at % side);
        [
            (row > 0).then(|| at - side),
            (row + 1 < side).then(|| at + side),
            (col > 0).then(|| at - 1),
            (col + 1 < side).then(|| at + 1),
        ]
        .into_iter()
        .flatten()
        .map(|near| self.index[near])
        .filter(|&near| near != EMPTY)
    }

    fn connected(&self) -> bool {
        let mut seen = vec![false; self.cells.len()];
        let mut stack = vec![0];
        seen[0] = true;
        let mut reached = 1;
        while let Some(cell) = stack.pop() {
            for near in self.neighbours(cell) {
                if !seen[near] {
                    seen[near] = true;
                    reached += 1;
                    stack.push(near);
                }
            }
        }
        reached == self.cells.len()
    }

    fn rows(&self) -> Vec<Vec<(usize, f64)>> {
        (0..self.cells.len())
            .map(|cell| {
                let mut row: Vec<(usize, f64)> =
                    self.neighbours(cell).map(|near| (near, -1.0)).collect();
                row.push((cell, row.len() as f64));
                row
            })
            .collect()
    }

    fn symmetries(&self) -> Vec<Vec<usize>> {
        let last = self.side - 1;
        let maps: [&dyn Fn(usize, usize) -> (usize, usize); 3] = [
            &|row, col| (col, row),
            &|row, col| (row, last - col),
            &|row, col| (last - row, col),
        ];
        maps.iter()
            .filter_map(|map| {
                self.cells
                    .iter()
                    .map(|&at| {
                        let (row, col) = map(at / self.side, at % self.side);
                        let image = self.index[row * self.side + col];
                        (image != EMPTY).then_some(image)
                    })
                    .collect::<Option<Vec<usize>>>()
            })
            .collect()
    }

    fn diagonal(&self) -> Vec<f64> {
        let side = self.side as f64;
        self.cells
            .iter()
            .map(|&at| {
                let x = ((at % self.side) as f64 + 0.5) / side;
                let y = ((at / self.side) as f64 + 0.5) / side;
                x + std::f64::consts::SQRT_2 * y
            })
            .collect()
    }

    fn domains(&self, v: &[f64]) -> usize {
        let floor = ZERO * peak(v);
        let sign: Vec<i8> = v
            .iter()
            .map(|&x| i8::from(x > floor) - i8::from(x < -floor))
            .collect();
        let mut parent: Vec<usize> = (0..v.len()).collect();
        let root = |parent: &mut Vec<usize>, mut at: usize| {
            while parent[at] != at {
                parent[at] = parent[parent[at]];
                at = parent[at];
            }
            at
        };
        for cell in 0..v.len() {
            if sign[cell] == 0 {
                continue;
            }
            for near in self.neighbours(cell) {
                if near > cell && sign[near] == sign[cell] {
                    let (a, b) = (root(&mut parent, cell), root(&mut parent, near));
                    parent[a.max(b)] = a.min(b);
                }
            }
        }
        (0..v.len())
            .filter(|&cell| sign[cell] != 0 && root(&mut parent, cell) == cell)
            .count()
    }

    fn raster(&self, v: &[f64]) -> Vec<f64> {
        let top = peak(v);
        let mut field = vec![f64::NAN; self.side * self.side];
        for (&at, &x) in self.cells.iter().zip(v) {
            field[at] = if top > 0.0 { x / top } else { x };
        }
        field
    }
}

fn cut(code: u64, number: usize, level: usize) -> Result<Option<Plate>> {
    let code = Code::from(code);
    code_to_corners(code, 2, 2)?;
    if number < 2 || level == 0 {
        return Ok(None);
    }
    let mut side = 1usize;
    for _ in 0..level {
        side = side.saturating_mul(number);
        if side > SIDE {
            return Ok(None);
        }
    }
    let count = fill(code, number, 2, level as u32, 2)?;
    if !(LEAST..=MOST).contains(&count) {
        return Ok(None);
    }
    let design = create(code, number, level, 0, 2)?;
    let types = design.types();
    if types.shape != [side, side] {
        return value_error(format!(
            "The design has the shape {:?}, not {side} by {side}.",
            types.shape
        ));
    }
    let mut index = vec![EMPTY; side * side];
    let mut cells = Vec::new();
    for (at, slot) in index.iter_mut().enumerate() {
        if types.at(at) != 0 {
            *slot = cells.len();
            cells.push(at);
        }
    }
    let plate = Plate { side, cells, index };
    Ok(plate.connected().then_some(plate))
}

fn peak(v: &[f64]) -> f64 {
    v.iter().fold(0.0f64, |top, x| top.max(x.abs()))
}

fn turn(tie: &[Vec<f64>], form: impl Fn(&[f64], &[f64]) -> f64) -> Result<Vec<(f64, Vec<f64>)>> {
    let size = tie.len();
    let mut matrix = vec![vec![0.0; size]; size];
    for a in 0..size {
        for b in 0..=a {
            let x = (form(&tie[a], &tie[b]) + form(&tie[b], &tie[a])) / 2.0;
            matrix[a][b] = x;
            matrix[b][a] = x;
        }
    }
    let (values, weights) = symmetric(&matrix)?;
    Ok(values
        .into_iter()
        .zip(weights)
        .map(|(value, weight)| {
            let mut v = vec![0.0; tie[0].len()];
            for (c, u) in weight.iter().zip(tie) {
                for (x, y) in v.iter_mut().zip(u) {
                    *x += c * y;
                }
            }
            (value, v)
        })
        .collect())
}

fn canonical(
    block: &[Vec<f64>],
    symmetries: &[Vec<usize>],
    diagonal: &[f64],
) -> Result<Vec<Vec<f64>>> {
    let mut ties = vec![block.to_vec()];
    for map in symmetries {
        let mut next = Vec::new();
        for tie in ties {
            if tie.len() < 2 {
                next.push(tie);
                continue;
            }
            let turned = turn(&tie, |a, b| a.iter().zip(map).map(|(x, &j)| x * b[j]).sum())?;
            let mut last = f64::INFINITY;
            for (value, v) in turned.into_iter().rev() {
                if last - value > SPLIT {
                    next.push(Vec::new());
                }
                if let Some(group) = next.last_mut() {
                    group.push(v);
                }
                last = value;
            }
        }
        ties = next;
    }
    let mut out = Vec::new();
    for tie in ties {
        if tie.len() < 2 {
            out.extend(tie);
            continue;
        }
        let turned = turn(&tie, |a, b| {
            a.iter()
                .zip(b)
                .zip(diagonal)
                .map(|((x, y), d)| x * y * d)
                .sum()
        })?;
        out.extend(turned.into_iter().map(|(_, v)| v));
    }
    Ok(out)
}

fn project(v: &[f64], symmetries: &[Vec<usize>]) -> Vec<f64> {
    let n = v.len();
    let norm: f64 = v.iter().map(|x| x * x).sum();
    let mut group: Vec<(Vec<usize>, f64)> = vec![((0..n).collect(), 1.0)];
    let generators: Vec<(&Vec<usize>, f64)> = symmetries
        .iter()
        .filter_map(|map| {
            let parity = v.iter().zip(map).map(|(x, &j)| x * v[j]).sum::<f64>() / norm;
            if parity > SPLIT {
                Some((map, 1.0))
            } else if parity < -SPLIT {
                Some((map, -1.0))
            } else {
                None
            }
        })
        .collect();
    let mut at = 0;
    while at < group.len() {
        for &(map, sign) in &generators {
            let image: Vec<usize> = group[at].0.iter().map(|&i| map[i]).collect();
            let character = group[at].1 * sign;
            if !group.iter().any(|(other, _)| *other == image) {
                group.push((image, character));
            }
        }
        at += 1;
    }
    let mut out = vec![0.0; n];
    let mut done = vec![false; n];
    for cell in 0..n {
        if done[cell] {
            continue;
        }
        let clash = group.iter().any(|(p, chi)| {
            group
                .iter()
                .any(|(q, psi)| p[cell] == q[cell] && chi != psi)
        });
        let mean = if clash {
            0.0
        } else {
            group.iter().map(|(p, chi)| chi * v[p[cell]]).sum::<f64>() / group.len() as f64
        };
        for (p, chi) in &group {
            out[p[cell]] = chi * mean;
            done[p[cell]] = true;
        }
    }
    let top = peak(&out);
    let first = out.iter().position(|x| x.abs() >= top - PEAK * top);
    if first.is_some_and(|cell| out[cell] < 0.0) {
        out.iter_mut().for_each(|x| *x = -*x);
    }
    out
}

pub fn holds(code: u64, number: usize, level: usize) -> Result<bool> {
    Ok(cut(code, number, level)?.is_some())
}

pub fn figures(code: u64, number: usize, level: usize, count: usize) -> Result<Figures> {
    let Some(plate) = cut(code, number, level)? else {
        return value_error(format!(
            "The design ({code}, {number}, {level}) is not one connected plate of {LEAST} to {MOST} cells on a side of at most {SIDE}."
        ));
    };
    let count = count.min(CAP);
    let (mut values, vectors) = lowest(&plate.rows(), count, GAP)?;
    let symmetries = plate.symmetries();
    let diagonal = plate.diagonal();
    let mut modes = Vec::with_capacity(values.len());
    let mut bound = Vec::with_capacity(values.len());
    let mut start = 0;
    for (_, size) in clusters(&values, GAP)? {
        for v in canonical(&vectors[start..start + size], &symmetries, &diagonal)? {
            modes.push(project(&v, &symmetries));
            bound.push(start + size);
        }
        start += size;
    }
    values.truncate(count);
    modes.truncate(count);
    bound.truncate(count);
    Ok(Figures {
        side: plate.side,
        values,
        domains: modes.iter().map(|v| plate.domains(v)).collect(),
        bound,
        fields: modes.iter().flat_map(|v| plate.raster(v)).collect(),
    })
}
