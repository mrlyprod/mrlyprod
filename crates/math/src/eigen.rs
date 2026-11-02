#![allow(clippy::needless_range_loop)]

use mrly_core::error::{Result, value_error};
use mrly_core::rng::Rng;

const SWEEPS: usize = 60;
const DENSE: usize = 200;
const BLOCK: usize = 4;
const SHIFT: f64 = 3e-3;
const RESIDUAL: f64 = 1e-12;
const KRYLOV: usize = 512;
const STRIDE: usize = 4 * BLOCK;
const DEFICIENT: f64 = 1e-13;
const SEED: u64 = 0x6c61_6e63_7a6f_7321;
const ROUNDS: usize = 32;

type Square = [[f64; BLOCK]; BLOCK];

pub(crate) type Pairs = (Vec<f64>, Vec<Vec<f64>>);

// DENSE

pub(crate) fn tridiagonalise(a: &mut [Vec<f64>], vectors: bool) -> (Vec<f64>, Vec<f64>) {
    let n = a.len();
    let mut d = vec![0.0; n];
    let mut e = vec![0.0; n];
    for i in (1..n).rev() {
        let l = i - 1;
        let mut h = 0.0;
        let mut scale = 0.0;
        if l > 0 {
            for k in 0..=l {
                scale += a[i][k].abs();
            }
        }
        if l == 0 || scale == 0.0 {
            e[i] = a[i][l];
            continue;
        }
        for k in 0..=l {
            a[i][k] /= scale;
            h += a[i][k] * a[i][k];
        }
        let mut f = a[i][l];
        let g = if f >= 0.0 { -h.sqrt() } else { h.sqrt() };
        e[i] = scale * g;
        h -= f * g;
        a[i][l] = f - g;
        f = 0.0;
        for j in 0..=l {
            if vectors {
                a[j][i] = a[i][j] / h;
            }
            let mut sum = 0.0;
            for k in 0..=j {
                sum += a[j][k] * a[i][k];
            }
            for k in (j + 1)..=l {
                sum += a[k][j] * a[i][k];
            }
            e[j] = sum / h;
            f += e[j] * a[i][j];
        }
        let hh = f / (h + h);
        for j in 0..=l {
            let f = a[i][j];
            let g = e[j] - hh * f;
            e[j] = g;
            for k in 0..=j {
                a[j][k] -= f * e[k] + g * a[i][k];
            }
        }
        d[i] = h;
    }
    e[0] = 0.0;
    if !vectors {
        for i in 0..n {
            d[i] = a[i][i];
        }
        return (d, e);
    }
    if n > 0 {
        d[0] = 0.0;
    }
    for i in 0..n {
        if d[i] != 0.0 {
            for j in 0..i {
                let mut g = 0.0;
                for k in 0..i {
                    g += a[i][k] * a[k][j];
                }
                for k in 0..i {
                    a[k][j] -= g * a[k][i];
                }
            }
        }
        d[i] = a[i][i];
        a[i][i] = 1.0;
        for j in 0..i {
            a[j][i] = 0.0;
            a[i][j] = 0.0;
        }
    }
    (d, e)
}

pub(crate) fn implicit_ql(
    d: &mut [f64],
    e: &mut [f64],
    mut z: Option<&mut [Vec<f64>]>,
) -> Result<()> {
    let n = d.len();
    for i in 1..n {
        e[i - 1] = e[i];
    }
    if n > 0 {
        e[n - 1] = 0.0;
    }
    for l in 0..n {
        let mut sweeps = 0;
        loop {
            let mut m = l;
            while m + 1 < n {
                let dd = d[m].abs() + d[m + 1].abs();
                if e[m].abs() <= f64::EPSILON * dd {
                    break;
                }
                m += 1;
            }
            if m == l {
                break;
            }
            sweeps += 1;
            if sweeps > SWEEPS {
                return value_error("The QL iteration did not converge.");
            }
            let mut g = (d[l + 1] - d[l]) / (2.0 * e[l]);
            let mut r = g.hypot(1.0);
            g = d[m] - d[l] + e[l] / (g + if g >= 0.0 { r.abs() } else { -r.abs() });
            let mut s = 1.0;
            let mut c = 1.0;
            let mut p = 0.0;
            let mut underflow = false;
            let mut i = m;
            while i > l {
                i -= 1;
                let f = s * e[i];
                let b = c * e[i];
                r = f.hypot(g);
                e[i + 1] = r;
                if r == 0.0 {
                    d[i + 1] -= p;
                    e[m] = 0.0;
                    underflow = true;
                    break;
                }
                s = f / r;
                c = g / r;
                g = d[i + 1] - p;
                r = (d[i] - g) * s + 2.0 * c * b;
                p = s * r;
                d[i + 1] = g + p;
                g = c * r - b;
                if let Some(z) = z.as_deref_mut() {
                    let (low, high) = z.split_at_mut(i + 1);
                    for (u, w) in low[i].iter_mut().zip(high[0].iter_mut()) {
                        let f = *w;
                        *w = s * *u + c * f;
                        *u = c * *u - s * f;
                    }
                }
            }
            if underflow {
                continue;
            }
            d[l] -= p;
            e[l] = g;
            e[m] = 0.0;
        }
    }
    Ok(())
}

pub(crate) fn symmetric(matrix: &[Vec<f64>]) -> Result<Pairs> {
    let n = matrix.len();
    let mut a = matrix.to_vec();
    let (mut d, mut e) = tridiagonalise(&mut a, true);
    let mut z: Vec<Vec<f64>> = (0..n).map(|i| (0..n).map(|k| a[k][i]).collect()).collect();
    implicit_ql(&mut d, &mut e, Some(&mut z))?;
    if d.iter().any(|v| !v.is_finite()) {
        return value_error("The spectrum is not a number.");
    }
    let mut order: Vec<usize> = (0..n).collect();
    order.sort_by(|&x, &y| d[x].total_cmp(&d[y]));
    let values = order.iter().map(|&k| d[k]).collect();
    let vectors = order.iter().map(|&k| std::mem::take(&mut z[k])).collect();
    Ok((values, vectors))
}

// BAND

struct Band {
    n: usize,
    width: usize,
    cells: Vec<f64>,
}

impl Band {
    fn new(rows: &[Vec<(usize, f64)>], shift: f64) -> Band {
        let n = rows.len();
        let width = rows
            .iter()
            .enumerate()
            .flat_map(|(i, row)| row.iter().filter(move |e| e.0 < i).map(move |e| i - e.0))
            .max()
            .unwrap_or(0);
        let span = width + 1;
        let mut cells = vec![0.0; n * span];
        for (i, row) in rows.iter().enumerate() {
            for &(j, value) in row {
                if j <= i {
                    cells[i * span + width + j - i] += value;
                }
            }
            cells[i * span + width] -= shift;
        }
        Band { n, width, cells }
    }

    fn first(&self, i: usize) -> usize {
        i.saturating_sub(self.width)
    }

    fn at(&self, i: usize, j: usize) -> usize {
        i * (self.width + 1) + self.width + j - i
    }

    fn lower(&self, i: usize) -> &[f64] {
        &self.cells[self.at(i, self.first(i))..self.at(i, i)]
    }

    fn pivot(&self, i: usize) -> f64 {
        self.cells[self.at(i, i)]
    }
}

fn dot(a: &[f64], b: &[f64]) -> f64 {
    let mut acc = [0.0; 4];
    let (x, y) = (a.chunks_exact(4), b.chunks_exact(4));
    let tail: f64 = x
        .remainder()
        .iter()
        .zip(y.remainder())
        .map(|(p, q)| p * q)
        .sum();
    for (p, q) in x.zip(y) {
        for t in 0..4 {
            acc[t] += p[t] * q[t];
        }
    }
    (acc[0] + acc[1]) + (acc[2] + acc[3]) + tail
}

fn cholesky(mut band: Band) -> Result<Band> {
    for i in 0..band.n {
        let first = band.first(i);
        for j in first..=i {
            let (a, b) = (band.at(i, first), band.at(j, first));
            let sum = dot(&band.cells[a..a + j - first], &band.cells[b..b + j - first]);
            let at = band.at(i, j);
            let value = band.cells[at] - sum;
            if j < i {
                band.cells[at] = value / band.pivot(j);
            } else if value > 0.0 {
                band.cells[at] = value.sqrt();
            } else {
                return value_error("The shifted Laplacian is not positive definite.");
            }
        }
    }
    Ok(band)
}

fn solve(factor: &Band, x: &mut [f64]) {
    for i in 0..factor.n {
        let first = factor.first(i);
        let (done, rest) = x.split_at_mut(i * BLOCK);
        let mut acc = [0.0; BLOCK];
        acc.copy_from_slice(&rest[..BLOCK]);
        for (g, xk) in factor
            .lower(i)
            .iter()
            .zip(done[first * BLOCK..].chunks_exact(BLOCK))
        {
            for a in 0..BLOCK {
                acc[a] -= g * xk[a];
            }
        }
        let pivot = factor.pivot(i);
        for a in 0..BLOCK {
            rest[a] = acc[a] / pivot;
        }
    }
    for i in (0..factor.n).rev() {
        let first = factor.first(i);
        let (head, rest) = x.split_at_mut(i * BLOCK);
        let pivot = factor.pivot(i);
        let mut own = [0.0; BLOCK];
        for (o, r) in own.iter_mut().zip(rest.iter_mut()) {
            *r /= pivot;
            *o = *r;
        }
        for (g, xk) in factor
            .lower(i)
            .iter()
            .zip(head[first * BLOCK..].chunks_exact_mut(BLOCK))
        {
            for a in 0..BLOCK {
                xk[a] -= g * own[a];
            }
        }
    }
}

pub(crate) fn below(rows: &[Vec<(usize, f64)>], shift: f64) -> usize {
    let mut band = Band::new(rows, shift);
    let mut pivots = vec![0.0; band.n];
    let mut scaled = vec![0.0; band.width];
    let mut negative = 0;
    for i in 0..band.n {
        let first = band.first(i);
        let row = band.at(i, first);
        for t in 0..i - first {
            let near = band.at(first + t, first);
            scaled[t] = band.cells[row + t] - dot(&scaled[..t], &band.cells[near..near + t]);
        }
        let mut pivot = band.pivot(i);
        for t in 0..i - first {
            let unit = scaled[t] / pivots[first + t];
            band.cells[row + t] = unit;
            pivot -= scaled[t] * unit;
        }
        if pivot == 0.0 {
            pivot = f64::EPSILON;
        }
        pivots[i] = pivot;
        negative += usize::from(pivot < 0.0);
    }
    negative
}

// BLOCKS

fn gram(q: &[f64], w: &[f64]) -> Square {
    let mut c = [[0.0; BLOCK]; BLOCK];
    for (qi, wi) in q.chunks_exact(BLOCK).zip(w.chunks_exact(BLOCK)) {
        for a in 0..BLOCK {
            for b in 0..BLOCK {
                c[a][b] += qi[a] * wi[b];
            }
        }
    }
    c
}

fn subtract(w: &mut [f64], q: &[f64], c: &Square) {
    for (wi, qi) in w.chunks_exact_mut(BLOCK).zip(q.chunks_exact(BLOCK)) {
        for a in 0..BLOCK {
            for b in 0..BLOCK {
                wi[b] -= qi[a] * c[a][b];
            }
        }
    }
}

fn transpose(c: &Square) -> Square {
    let mut t = [[0.0; BLOCK]; BLOCK];
    for a in 0..BLOCK {
        for b in 0..BLOCK {
            t[a][b] = c[b][a];
        }
    }
    t
}

fn lengths(w: &[f64]) -> [f64; BLOCK] {
    let mut out = [0.0; BLOCK];
    for wi in w.chunks_exact(BLOCK) {
        for a in 0..BLOCK {
            out[a] += wi[a] * wi[a];
        }
    }
    out.map(f64::sqrt)
}

fn deflate(w: &mut [f64], locked: &[Vec<f64>]) {
    for y in locked {
        let mut c = [0.0; BLOCK];
        for (wi, yi) in w.chunks_exact(BLOCK).zip(y) {
            for b in 0..BLOCK {
                c[b] += yi * wi[b];
            }
        }
        for (wi, yi) in w.chunks_exact_mut(BLOCK).zip(y) {
            for b in 0..BLOCK {
                wi[b] -= yi * c[b];
            }
        }
    }
}

fn reorthogonalise(w: &mut [f64], basis: &[Vec<f64>], locked: &[Vec<f64>]) {
    for _ in 0..2 {
        let before = lengths(w);
        deflate(w, locked);
        for q in basis {
            let c = gram(q, w);
            subtract(w, q, &c);
        }
        let after = lengths(w);
        if (0..BLOCK).all(|a| after[a] > 0.5 * before[a]) {
            break;
        }
    }
}

fn column(w: &[f64], a: usize) -> Vec<f64> {
    w.iter().skip(a).step_by(BLOCK).copied().collect()
}

fn fresh(w: &[f64], a: usize, basis: &[Vec<f64>], locked: &[Vec<f64>], rng: &mut Rng) -> Vec<f64> {
    let n = w.len() / BLOCK;
    let mut v: Vec<f64> = (0..n).map(|_| rng.unit() - 0.5).collect();
    let remove = |v: &mut [f64], along: &[f64]| {
        let c = dot(along, v);
        for (vi, x) in v.iter_mut().zip(along) {
            *vi -= c * x;
        }
    };
    for _ in 0..2 {
        for y in locked {
            remove(&mut v, y);
        }
        for q in basis {
            for b in 0..BLOCK {
                remove(&mut v, &column(q, b));
            }
        }
        for b in 0..a {
            remove(&mut v, &column(w, b));
        }
    }
    let length = dot(&v, &v).sqrt();
    v.iter_mut().for_each(|x| *x /= length);
    v
}

struct Space<'a> {
    basis: Vec<Vec<f64>>,
    locked: &'a [Vec<f64>],
    rng: Rng,
}

impl Space<'_> {
    fn orthonormalise(&mut self, w: &mut [f64], reach: [f64; BLOCK]) -> Square {
        let mut r = [[0.0; BLOCK]; BLOCK];
        for a in 0..BLOCK {
            for _ in 0..2 {
                for b in 0..a {
                    let c: f64 = w.chunks_exact(BLOCK).map(|wi| wi[a] * wi[b]).sum();
                    r[b][a] += c;
                    for wi in w.chunks_exact_mut(BLOCK) {
                        wi[a] -= c * wi[b];
                    }
                }
            }
            let length = lengths(w)[a];
            if length > DEFICIENT * reach[a] {
                r[a][a] = length;
                for wi in w.chunks_exact_mut(BLOCK) {
                    wi[a] /= length;
                }
            } else {
                let v = fresh(w, a, &self.basis, self.locked, &mut self.rng);
                r[a][a] = w
                    .chunks_exact(BLOCK)
                    .zip(&v)
                    .map(|(wi, vi)| wi[a] * vi)
                    .sum();
                for (wi, vi) in w.chunks_exact_mut(BLOCK).zip(&v) {
                    wi[a] = *vi;
                }
            }
        }
        r
    }
}

// LANCZOS

fn apply(rows: &[Vec<(usize, f64)>], v: &[f64]) -> Vec<f64> {
    rows.iter()
        .map(|row| row.iter().map(|&(j, a)| a * v[j]).sum())
        .collect()
}

fn closed(values: &[f64], count: usize, gap: f64) -> usize {
    let mut cut = count;
    while cut < values.len() && values[cut] - values[cut - 1] <= gap {
        cut += 1;
    }
    cut
}

struct Krylov<'a> {
    rows: &'a [Vec<(usize, f64)>],
    factor: &'a Band,
    scale: f64,
    gap: f64,
}

impl Krylov<'_> {
    fn ritz(
        &self,
        basis: &[Vec<f64>],
        diagonal: &[Square],
        coupling: &[Square],
        next: Option<&Square>,
        count: usize,
    ) -> Result<Option<(Pairs, f64)>> {
        let size = basis.len() * BLOCK;
        let mut t = vec![vec![0.0; size]; size];
        for (j, a) in diagonal.iter().enumerate() {
            for x in 0..BLOCK {
                for y in 0..BLOCK {
                    t[j * BLOCK + x][j * BLOCK + y] = a[x][y];
                }
            }
        }
        for (j, b) in coupling.iter().enumerate() {
            for x in 0..BLOCK {
                for y in 0..BLOCK {
                    t[(j + 1) * BLOCK + x][j * BLOCK + y] = b[x][y];
                    t[j * BLOCK + y][(j + 1) * BLOCK + x] = b[x][y];
                }
            }
        }
        let (thetas, vectors) = symmetric(&t)?;
        let order: Vec<usize> = (0..size).rev().collect();
        let values: Vec<f64> = order.iter().map(|&k| 1.0 / thetas[k] - SHIFT).collect();
        let cut = closed(&values, count, self.gap);
        if cut >= size {
            return Ok(None);
        }
        let tolerance = RESIDUAL * self.scale;
        for (rank, &k) in order.iter().enumerate().take(cut + 1) {
            let tail = &vectors[k][size - BLOCK..];
            let spill = next.map_or(0.0, |b| {
                b.iter()
                    .map(|row| dot(row, tail).powi(2))
                    .sum::<f64>()
                    .sqrt()
            });
            if spill * (self.scale + SHIFT) * (values[rank] + SHIFT) > tolerance {
                return Ok(None);
            }
        }
        let n = self.rows.len();
        let mut weights = vec![0.0; size * cut];
        for (rank, &k) in order.iter().enumerate().take(cut) {
            for (at, s) in vectors[k].iter().enumerate() {
                weights[at * cut + rank] = *s;
            }
        }
        let mut lifted = vec![0.0; n * cut];
        for (j, q) in basis.iter().enumerate() {
            for (out, qi) in lifted.chunks_exact_mut(cut).zip(q.chunks_exact(BLOCK)) {
                for a in 0..BLOCK {
                    let weight = &weights[(j * BLOCK + a) * cut..(j * BLOCK + a + 1) * cut];
                    for (o, s) in out.iter_mut().zip(weight) {
                        *o += qi[a] * s;
                    }
                }
            }
        }
        let mut found = Vec::with_capacity(cut);
        for rank in 0..cut {
            let mut v: Vec<f64> = lifted.iter().skip(rank).step_by(cut).copied().collect();
            let length = dot(&v, &v).sqrt();
            v.iter_mut().for_each(|x| *x /= length);
            let image = apply(self.rows, &v);
            let value = dot(&v, &image);
            let residual = image
                .iter()
                .zip(&v)
                .map(|(lv, x)| (lv - value * x).powi(2))
                .sum::<f64>()
                .sqrt();
            if residual > tolerance {
                return Ok(None);
            }
            found.push((value, v));
        }
        found.sort_by(|x, y| x.0.total_cmp(&y.0));
        Ok(Some((found.into_iter().unzip(), values[cut])))
    }

    fn run(&self, count: usize, locked: &[Vec<f64>], seed: u64) -> Result<(Pairs, f64)> {
        let n = self.rows.len();
        let cap = KRYLOV.min(n - locked.len()) / BLOCK * BLOCK;
        if cap < BLOCK {
            return value_error("The Krylov space has no room left.");
        }
        let mut space = Space {
            basis: Vec::new(),
            locked,
            rng: Rng::new(seed),
        };
        let mut start: Vec<f64> = (0..n * BLOCK).map(|_| space.rng.unit() - 0.5).collect();
        let reach = lengths(&start);
        reorthogonalise(&mut start, &[], locked);
        space.orthonormalise(&mut start, reach);
        space.basis.push(start);
        let mut diagonal: Vec<Square> = Vec::new();
        let mut coupling: Vec<Square> = Vec::new();
        let mut check = (2 * count + STRIDE).min(cap);
        loop {
            let j = space.basis.len() - 1;
            let mut w = space.basis[j].clone();
            solve(self.factor, &mut w);
            let reach = lengths(&w);
            let mut a = gram(&space.basis[j], &w);
            for x in 0..BLOCK {
                for y in 0..x {
                    let mean = (a[x][y] + a[y][x]) / 2.0;
                    a[x][y] = mean;
                    a[y][x] = mean;
                }
            }
            subtract(&mut w, &space.basis[j], &a);
            if j > 0 {
                subtract(&mut w, &space.basis[j - 1], &transpose(&coupling[j - 1]));
            }
            diagonal.push(a);
            reorthogonalise(&mut w, &space.basis, locked);
            let size = space.basis.len() * BLOCK;
            let last = size + BLOCK > cap;
            let next = (!last).then(|| space.orthonormalise(&mut w, reach));
            if last || size >= check {
                if let Some(found) =
                    self.ritz(&space.basis, &diagonal, &coupling, next.as_ref(), count)?
                {
                    return Ok(found);
                }
                if last {
                    return value_error(format!(
                        "The lowest {count} pairs did not settle in a Krylov space of {cap}."
                    ));
                }
                check = size + STRIDE;
            }
            if let Some(b) = next {
                coupling.push(b);
                space.basis.push(w);
            }
        }
    }
}

fn shift(top: f64, above: impl Iterator<Item = f64>, gap: f64) -> f64 {
    let next = above
        .filter(|&value| value > top + gap)
        .fold(f64::INFINITY, f64::min);
    if next.is_finite() {
        (top + next) / 2.0
    } else {
        top + gap
    }
}

fn sliced(rows: &[Vec<(usize, f64)>], count: usize, gap: f64) -> Result<Pairs> {
    let scale = rows
        .iter()
        .map(|row| row.iter().map(|e| e.1.abs()).sum::<f64>())
        .fold(0.0, f64::max);
    let factor = cholesky(Band::new(rows, -SHIFT))?;
    let krylov = Krylov {
        rows,
        factor: &factor,
        scale,
        gap,
    };
    let mut values: Vec<f64> = Vec::new();
    let mut vectors: Vec<Vec<f64>> = Vec::new();
    let mut want = count;
    let mut ceiling = f64::INFINITY;
    for round in 0..ROUNDS {
        let ((more, along), next) = krylov.run(want, &vectors, SEED.wrapping_add(round as u64))?;
        if round > 0 && !more.iter().any(|&v| v <= ceiling) {
            break;
        }
        let mut pairs: Vec<(f64, Vec<f64>)> = values.into_iter().zip(vectors).collect();
        pairs.extend(more.into_iter().zip(along));
        pairs.sort_by(|x, y| x.0.total_cmp(&y.0));
        (values, vectors) = pairs.into_iter().unzip();
        let cut = closed(&values, count, gap);
        ceiling = shift(
            values[cut - 1],
            values.get(cut).copied().into_iter().chain([next]),
            gap,
        );
        let have = values.iter().filter(|&&v| v <= ceiling).count();
        let truth = below(rows, ceiling);
        if truth == have {
            values.truncate(cut);
            vectors.truncate(cut);
            return Ok((values, vectors));
        }
        if truth < have {
            break;
        }
        want = (truth - have).min(2 * BLOCK);
    }
    value_error(format!(
        "The lowest {count} pairs did not all turn up: a degenerate cluster outran the rounds."
    ))
}

pub(crate) fn lowest(rows: &[Vec<(usize, f64)>], count: usize, gap: f64) -> Result<Pairs> {
    let n = rows.len();
    let count = count.min(n);
    if count == 0 {
        return Ok((Vec::new(), Vec::new()));
    }
    if n >= DENSE {
        return sliced(rows, count, gap);
    }
    let mut dense = vec![vec![0.0; n]; n];
    for (i, row) in rows.iter().enumerate() {
        for &(j, value) in row {
            dense[i][j] += value;
        }
    }
    let (mut values, mut vectors) = symmetric(&dense)?;
    let cut = closed(&values, count, gap);
    values.truncate(cut);
    vectors.truncate(cut);
    Ok((values, vectors))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_inertia_shift_sits_midway_to_the_next_value_past_the_gap() {
        assert_eq!(shift(1.0, [1.5, 3.0].into_iter(), 1e-8), 1.25);
        assert_eq!(shift(1.0, [1.0 + 2e-8].into_iter(), 1e-8), 1.0 + 1e-8);
        assert_eq!(shift(1.0, [1.0 + 5e-9].into_iter(), 1e-8), 1.0 + 1e-8);
    }
}
