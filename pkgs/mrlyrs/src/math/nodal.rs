use crate::core::error::{value_error, Result};
use crate::math::bang::factory::code_to_corners;
use crate::math::bang::Code;
use crate::math::counts::fill;
use crate::math::spectrum::clusters;
use crate::math::two::create;
use crate::num::eigen::{lowest, symmetric};
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

/// The Chladni figures of a design: the lowest modes of the plate cut as the design, each with its nodal domains, its Courant bound and its raster.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Figures {
    /// The side of the design's square raster.
    pub side: usize,
    /// The lowest eigenvalues of the plate's Laplacian `D - A`, ascending.
    pub values: Vec<f64>,
    /// The strong nodal domain count of each figure.
    pub domains: Vec<usize>,
    /// The Courant bound of each figure: the 1-based index of the last member of its eigenvalue's cluster.
    pub bound: Vec<usize>,
    /// The figures as row-major `side * side` rasters back to back, NaN off the plate, each scaled so its largest magnitude is 1.
    pub fields: Vec<f64>,
}

// THE PLATE

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

// THE BASIS

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

// THE DOORS

/// Answers whether a 2D base-2 design cuts a plate the figures can ring: one 4-connected piece of 16 to 10,000 cells on a side of at most 128. The cells are counted before the design is built.
///
/// # Errors
///
/// Errs when the code is not a 2D base-2 code, 0 to 15.
pub fn holds(code: u64, number: usize, level: usize) -> Result<bool> {
    Ok(cut(code, number, level)?.is_some())
}

/// Rings the plate cut as the design `math::two::create(code, number, level, 0, 2)`: its cells joined to their four neighbours, free at the rim and at every hole, the combinatorial Laplacian `D - A`. Returns the lowest `count` modes, `count` clamped to the cells and to 64, each in a canonical basis: eigenvalues within `1e-8` form a cluster, split by the design's transpose, left-right and up-down mirrors in that order and then by the diagonal `x + sqrt(2) y`, each figure projected exactly onto its mirror parities and signed so its first largest cell is positive. A figure's domains are the 4-connected runs of one strict sign, cells within `1e-9` of its peak left out; its bound is the 1-based index of the last member of its cluster, the last cluster closed before the cut.
///
/// # Errors
///
/// Errs on what `holds` refuses, or when the solver does not settle.
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::math::spectrum::{laplacian, laplacian_spectrum};
    use crate::math::two::core_graph;
    use crate::num::chladni::plate;
    use crate::num::eigen::below;

    fn plate_of(code: u64, number: usize, level: usize) -> Plate {
        cut(code, number, level).unwrap().unwrap()
    }

    fn unit(figures: &Figures, plate: &Plate, k: usize) -> Vec<f64> {
        let size = figures.side * figures.side;
        let field = &figures.fields[k * size..(k + 1) * size];
        let v: Vec<f64> = plate.cells.iter().map(|&at| field[at]).collect();
        let length = v.iter().map(|x| x * x).sum::<f64>().sqrt();
        v.iter().map(|x| x / length).collect()
    }

    fn dot(a: &[f64], b: &[f64]) -> f64 {
        a.iter().zip(b).map(|(x, y)| x * y).sum()
    }

    #[test]
    fn level_three_matches_the_dense_spectrum_and_its_projectors() {
        let figures = figures(7, 3, 3, 48).unwrap();
        let plate = plate_of(7, 3, 3);
        let graph = core_graph(&create(Code::from(7u64), 3, 3, 0, 2).unwrap()).unwrap();
        let spectrum = laplacian_spectrum(&graph, false).unwrap();
        for (got, want) in figures.values.iter().zip(&spectrum) {
            assert!((got - want).abs() < 1e-9, "{got} {want}");
        }
        let (_, dense) = symmetric(&laplacian(&graph, false).unwrap()).unwrap();
        let mine: Vec<Vec<f64>> = (0..48).map(|k| unit(&figures, &plate, k)).collect();
        let mut start = 0;
        while start < 48 && figures.bound[start] <= 48 {
            let end = figures.bound[start];
            let mut worst = 0.0f64;
            for i in 0..plate.cells.len() {
                for j in 0..plate.cells.len() {
                    let p: f64 = mine[start..end].iter().map(|v| v[i] * v[j]).sum();
                    let q: f64 = dense[start..end].iter().map(|v| v[i] * v[j]).sum();
                    worst = worst.max((p - q).abs());
                }
            }
            assert!(worst < 1e-9, "cluster {start}..{end} {worst}");
            start = end;
        }
        assert!(start >= 47);
    }

    #[test]
    fn level_three_counts_the_lab_pins() {
        let figures = figures(7, 3, 3, 12).unwrap();
        let counts: Vec<usize> = [1, 4, 5, 8]
            .iter()
            .map(|k| figures.domains[k - 1])
            .collect();
        assert_eq!(counts, [1, 4, 4, 5]);
    }

    #[test]
    fn courant_holds_at_every_k() {
        for (code, number, level) in [(7, 3, 3), (7, 3, 4)] {
            let figures = figures(code, number, level, 48).unwrap();
            assert_eq!(figures.values.len(), 48);
            for (k, (d, b)) in figures.domains.iter().zip(&figures.bound).enumerate() {
                assert!(
                    d <= b,
                    "({code}, {number}, {level}) k {} domains {d} bound {b}",
                    k + 1
                );
            }
        }
    }

    #[test]
    fn the_figures_are_orthonormal_eigenvectors() {
        let figures = figures(7, 3, 3, 48).unwrap();
        let plate = plate_of(7, 3, 3);
        let rows = plate.rows();
        let vectors: Vec<Vec<f64>> = (0..48).map(|k| unit(&figures, &plate, k)).collect();
        for (k, v) in vectors.iter().enumerate() {
            for (j, u) in vectors.iter().enumerate() {
                let want = if j == k { 1.0 } else { 0.0 };
                assert!((dot(v, u) - want).abs() < 1e-10, "{k} {j}");
            }
            let value = figures.values[k];
            let residual = rows
                .iter()
                .enumerate()
                .map(|(i, row)| {
                    let lv: f64 = row.iter().map(|&(j, a)| a * v[j]).sum();
                    (lv - value * v[i]).powi(2)
                })
                .sum::<f64>()
                .sqrt();
            assert!(residual < 8e-12, "k {} residual {residual}", k + 1);
        }
    }

    #[test]
    fn a_cell_near_zero_but_past_the_tolerance_keeps_its_sign() {
        let mut index = vec![EMPTY; 9];
        index[..3].copy_from_slice(&[0, 1, 2]);
        let path = Plate {
            side: 3,
            cells: vec![0, 1, 2],
            index,
        };
        assert_eq!(path.domains(&[1.0, 1e-7, 1.0]), 1);
        assert_eq!(path.domains(&[1.0, 1e-10, 1.0]), 2);
    }

    #[test]
    fn the_inertia_counts_the_dense_spectrum_below_a_shift() {
        let rows = plate_of(7, 3, 3).rows();
        let graph = core_graph(&create(Code::from(7u64), 3, 3, 0, 2).unwrap()).unwrap();
        let spectrum = laplacian_spectrum(&graph, false).unwrap();
        let mut shifts = vec![-1.0, spectrum[spectrum.len() - 1] + 1.0];
        for k in (0..spectrum.len() - 1).step_by(31) {
            if spectrum[k + 1] - spectrum[k] > 1e-6 {
                shifts.push((spectrum[k] + spectrum[k + 1]) / 2.0);
            }
        }
        assert!(shifts.len() > 8);
        for shift in shifts {
            let want = spectrum.iter().filter(|&&v| v < shift).count();
            assert_eq!(below(&rows, shift), want, "{shift}");
        }
    }

    #[test]
    fn a_cluster_wider_than_the_block_turns_up_whole() {
        let figures = figures(7, 2, 5, 48).unwrap();
        let graph = core_graph(&create(Code::from(7u64), 2, 5, 0, 2).unwrap()).unwrap();
        let spectrum = laplacian_spectrum(&graph, false).unwrap();
        let widest = clusters(&spectrum[..48], GAP)
            .unwrap()
            .iter()
            .map(|g| g.1)
            .max();
        assert_eq!(widest, Some(10));
        for (got, want) in figures.values.iter().zip(&spectrum) {
            assert!((got - want).abs() < 1e-9, "{got} {want}");
        }
    }

    #[test]
    fn two_calls_agree() {
        let bits = |f: &Figures| -> Vec<u64> {
            f.values
                .iter()
                .chain(&f.fields)
                .map(|x| x.to_bits())
                .collect()
        };
        let (a, b) = (figures(7, 3, 3, 48).unwrap(), figures(7, 3, 3, 48).unwrap());
        assert_eq!(bits(&a), bits(&b));
        assert_eq!((a.domains, a.bound), (b.domains, b.bound));
    }

    #[test]
    fn the_full_grid_pair_is_the_plate() {
        let figures = figures(15, 3, 3, 8).unwrap();
        let plate_cells = plate_of(15, 3, 3);
        assert_eq!(figures.bound[6..8], [8, 8]);
        for (k, sign) in [(6, 1), (7, -1)] {
            let want = plate(1, 2, sign, 27).unwrap();
            let length = want.iter().map(|x| x * x).sum::<f64>().sqrt();
            let want: Vec<f64> = want.iter().map(|x| x / length).collect();
            let got = unit(&figures, &plate_cells, k);
            assert!((dot(&got, &want).abs() - 1.0).abs() < 1e-9, "k {}", k + 1);
        }
    }

    #[test]
    fn the_count_clamps_to_the_cells_and_the_cap() {
        let small = figures(15, 2, 2, 48).unwrap();
        assert_eq!((small.values.len(), small.fields.len()), (16, 16 * 16));
        let large = figures(15, 3, 2, 100).unwrap();
        assert_eq!((large.values.len(), large.fields.len()), (64, 64 * 81));
    }

    #[test]
    fn holds_refuses_and_figures_with_it() {
        assert!(holds(7, 3, 4).unwrap());
        for (code, number, level) in [(7, 3, 1), (7, 3, 5), (1, 3, 3), (7, 1, 9), (7, 3, 0)] {
            assert!(
                !holds(code, number, level).unwrap(),
                "{code} {number} {level}"
            );
            assert!(figures(code, number, level, 8).is_err());
        }
        assert!(holds(16, 3, 3).is_err());
        assert!(figures(16, 3, 3, 8).is_err());
    }
}
