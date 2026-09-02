use mrlycore::errors::{value_error, Result};

const CEILING: usize = 64;
const SCAN: usize = 8192;
const HALVINGS: usize = 200;
const SWEEPS: usize = 600;

fn overflow<T>() -> Result<T> {
    value_error("the carry arithmetic passes a hundred and twenty-eight bits.")
}

fn product(a: i128, b: i128) -> Result<i128> {
    match a.checked_mul(b) {
        Some(value) => Ok(value),
        None => overflow(),
    }
}

fn total(a: i128, b: i128) -> Result<i128> {
    match a.checked_add(b) {
        Some(value) => Ok(value),
        None => overflow(),
    }
}

fn middle(base: usize) -> Result<usize> {
    if base < 3 || base.is_multiple_of(2) {
        return value_error("the base must be odd and at least three.");
    }
    Ok((base - 1) / 2)
}

fn sized(dimension: usize) -> Result<()> {
    if !(2..=CEILING).contains(&dimension) {
        return value_error(format!("the dimension must be between 2 and {CEILING}."));
    }
    Ok(())
}

// POLYNOMIAL

fn convolve(left: &[i128], right: &[i128]) -> Result<Vec<i128>> {
    let mut out = vec![0i128; left.len() + right.len() - 1];
    for (i, &x) in left.iter().enumerate() {
        if x == 0 {
            continue;
        }
        for (j, &y) in right.iter().enumerate() {
            if y == 0 {
                continue;
            }
            out[i + j] = total(out[i + j], product(x, y)?)?;
        }
    }
    Ok(out)
}

/// The digit polynomial of the base-`q` middle-digit design in dimension `D`, lowest power first.
///
/// The design keeps the cells whose digit vector has at most one coordinate equal to the middle
/// digit, so its weight generating function is `A(t)^(D-1) (A(t) + D t^m)`, with `A` the sum of
/// every digit power but the middle one. At base three that factors as
/// `(1 + t^2)^(D-1) (1 + D t + t^2)`, and the sum of the coefficients is the fill.
///
/// ```
/// assert_eq!(mrlymath::dim::carry::digit_polynomial(3, 3).unwrap(), vec![1, 3, 3, 6, 3, 3, 1]);
/// ```
pub fn digit_polynomial(base: usize, dimension: usize) -> Result<Vec<i128>> {
    let centre = middle(base)?;
    sized(dimension)?;
    let alpha: Vec<i128> = (0..base).map(|digit| i128::from(digit != centre)).collect();
    let mut poly = vec![1i128];
    for _ in 1..dimension {
        poly = convolve(&poly, &alpha)?;
    }
    let mut second = alpha;
    second[centre] += dimension as i128;
    convolve(&poly, &second)
}

/// The count of level-one cells the design keeps, `f_D = (q - 1)^(D-1) (q - 1 + D)`.
///
/// ```
/// assert_eq!(mrlymath::dim::carry::fill(3, 3).unwrap(), 20);
/// assert_eq!(mrlymath::dim::carry::fill(5, 3).unwrap(), 112);
/// ```
pub fn fill(base: usize, dimension: usize) -> Result<i128> {
    middle(base)?;
    sized(dimension)?;
    let mut out = 1i128;
    for _ in 1..dimension {
        out = product(out, base as i128 - 1)?;
    }
    product(out, (base - 1 + dimension) as i128)
}

// MATRIX

fn coefficient(poly: &[i128], index: i128) -> i128 {
    if index < 0 || index as usize >= poly.len() {
        0
    } else {
        poly[index as usize]
    }
}

/// The carry matrix over the reachable carries `|c| <= (D-1)/2`, rows indexed by the carry out.
///
/// A level of the design adds one base-`q` digit per coordinate, so the height of the central
/// diagonal hyperplane moves by a digit sum `s` and a carry `c -> (c + mD - s)/q` with `m` the
/// middle digit; the map contracts onto this window from every start.
pub fn carry_matrix(base: usize, dimension: usize) -> Result<Vec<Vec<i128>>> {
    let poly = digit_polynomial(base, dimension)?;
    let shift = (middle(base)? * dimension) as i128;
    let q = base as i128;
    let half = ((dimension - 1) / 2) as i128;
    Ok((-half..=half)
        .map(|out| {
            (-half..=half)
                .map(|inside| coefficient(&poly, inside + shift - q * out))
                .collect()
        })
        .collect())
}

/// The reflection-even block of the carry matrix, of size `ceil(D/2)`.
///
/// The digit polynomial is palindromic, so the reflection `c -> -c` commutes with the carry map
/// and splits it; the even half carries the central count and its characteristic polynomial is
/// the recurrence the counts obey.
///
/// ```
/// assert_eq!(mrlymath::dim::carry::even_block(3, 3).unwrap(), vec![vec![6, 6], vec![1, 3]]);
/// ```
pub fn even_block(base: usize, dimension: usize) -> Result<Vec<Vec<i128>>> {
    let poly = digit_polynomial(base, dimension)?;
    let shift = (middle(base)? * dimension) as i128;
    let q = base as i128;
    let width = (dimension - 1) / 2 + 1;
    Ok((0..width)
        .map(|out| {
            (0..width)
                .map(|inside| {
                    let step = shift - q * out as i128;
                    let folded = if inside == 0 {
                        0
                    } else {
                        coefficient(&poly, step - inside as i128)
                    };
                    coefficient(&poly, step + inside as i128) + folded
                })
                .collect()
        })
        .collect())
}

/// The trace of a square integer matrix.
///
/// ```
/// let block = mrlymath::dim::carry::even_block(3, 3).unwrap();
/// assert_eq!(mrlymath::dim::carry::trace(&block), 9);
/// ```
pub fn trace(rows: &[Vec<i128>]) -> i128 {
    (0..rows.len()).map(|index| rows[index][index]).sum()
}

fn multiply(left: &[Vec<i128>], right: &[Vec<i128>]) -> Result<Vec<Vec<i128>>> {
    let n = left.len();
    let mut out = vec![vec![0i128; n]; n];
    for row in 0..n {
        for step in 0..n {
            let weight = left[row][step];
            if weight == 0 {
                continue;
            }
            for col in 0..n {
                out[row][col] = total(out[row][col], product(weight, right[step][col])?)?;
            }
        }
    }
    Ok(out)
}

/// The monic characteristic polynomial of a square integer matrix, highest power first.
///
/// Faddeev-LeVerrier in exact integers: every division lands whole, so no fraction is ever needed
/// and the answer is the recurrence's coefficients up to sign.
///
/// ```
/// let block = mrlymath::dim::carry::even_block(3, 3).unwrap();
/// assert_eq!(mrlymath::dim::carry::characteristic(&block).unwrap(), vec![1, -9, 12]);
/// ```
pub fn characteristic(rows: &[Vec<i128>]) -> Result<Vec<i128>> {
    let n = rows.len();
    let mut held: Vec<Vec<i128>> = (0..n)
        .map(|row| (0..n).map(|col| i128::from(row == col)).collect())
        .collect();
    let mut out = vec![1i128];
    for step in 1..=n {
        let walked = multiply(rows, &held)?;
        let sum = trace(&walked);
        if sum % step as i128 != 0 {
            return value_error("the characteristic polynomial left a remainder.");
        }
        let mark = -sum / step as i128;
        out.push(mark);
        held = walked;
        for (index, row) in held.iter_mut().enumerate().take(n) {
            row[index] = total(row[index], mark)?;
        }
    }
    Ok(out)
}

/// The determinant of a square integer matrix, read off its characteristic polynomial.
///
/// ```
/// let block = mrlymath::dim::carry::even_block(3, 3).unwrap();
/// assert_eq!(mrlymath::dim::carry::determinant(&block).unwrap(), 12);
/// ```
pub fn determinant(rows: &[Vec<i128>]) -> Result<i128> {
    let poly = characteristic(rows)?;
    let last = poly[rows.len()];
    Ok(if rows.len().is_multiple_of(2) {
        last
    } else {
        -last
    })
}

// LADDER

/// The counts `a_D(L)` of level-`L` cells meeting the central diagonal hyperplane, from `L = 0`.
///
/// The count is the top-left entry of the `L`-th power of the even block. The walk stops early
/// when the next count would pass a hundred and twenty-eight bits, so the answer runs as far as
/// exact integers reach and no further.
///
/// ```
/// let terms = mrlymath::dim::carry::ladder(3, 3, 6).unwrap();
/// assert_eq!(terms, vec![1, 6, 42, 306, 2250, 16578, 122202]);
/// ```
pub fn ladder(base: usize, dimension: usize, levels: usize) -> Result<Vec<i128>> {
    let block = even_block(base, dimension)?;
    let n = block.len();
    let mut carried = vec![0i128; n];
    carried[0] = 1;
    let mut out = vec![1i128];
    for _ in 0..levels {
        let mut next = vec![0i128; n];
        for row in 0..n {
            let mut sum = 0i128;
            for col in 0..n {
                match block[row][col]
                    .checked_mul(carried[col])
                    .and_then(|part| sum.checked_add(part))
                {
                    Some(value) => sum = value,
                    None => return Ok(out),
                }
            }
            next[row] = sum;
        }
        out.push(next[0]);
        carried = next;
    }
    Ok(out)
}

// ROOT

/// The Perron root of a nonnegative square integer matrix.
///
/// The characteristic polynomial is rescaled by the largest row sum, which bounds the root above,
/// then the largest sign change on the unit interval is hunted on a grid and closed by bisection,
/// the same walk the recurrence growth reader takes.
///
/// ```
/// let block = mrlymath::dim::carry::even_block(3, 3).unwrap();
/// let root = mrlymath::dim::carry::perron(&block).unwrap();
/// assert!((root - (9.0 + 33f64.sqrt()) / 2.0).abs() < 1e-9);
/// ```
pub fn perron(rows: &[Vec<i128>]) -> Result<f64> {
    let poly = characteristic(rows)?;
    let bound = rows
        .iter()
        .map(|row| row.iter().sum::<i128>())
        .max()
        .unwrap_or(0) as f64;
    if bound <= 0.0 {
        return value_error("the block has no positive row sum.");
    }
    let mut scaled = Vec::with_capacity(poly.len());
    let mut power = 1.0f64;
    for &term in &poly {
        scaled.push(term as f64 / power);
        power *= bound;
    }
    let value = |y: f64| scaled.iter().fold(0.0f64, |acc, &term| acc * y + term);
    for step in (0..SCAN).rev() {
        let mut lo = step as f64 / SCAN as f64;
        let mut hi = (step + 1) as f64 / SCAN as f64;
        if value(lo) > 0.0 || value(hi) < 0.0 {
            continue;
        }
        for _ in 0..HALVINGS {
            let mid = (lo + hi) / 2.0;
            if value(mid) <= 0.0 {
                lo = mid;
            } else {
                hi = mid;
            }
        }
        return Ok(bound * (lo + hi) / 2.0);
    }
    value_error("the block shows no root inside its row-sum bound.")
}

// SIGN

/// The sign of `log_q rho_D - (log_q f_D - 1)`, the slice sign law's reading, in exact integers.
///
/// The characteristic polynomial is evaluated at `f_D/q` with the denominators cleared, so the
/// answer is a comparison of whole numbers and never a rounding. The law reads `(-1)^(D+1)`: the
/// slice exponent stands above the solid's dimension less one at odd `D` and below it at even `D`.
///
/// ```
/// assert_eq!(mrlymath::dim::carry::sign(3, 3).unwrap(), 1);
/// assert_eq!(mrlymath::dim::carry::sign(3, 4).unwrap(), -1);
/// ```
pub fn sign(base: usize, dimension: usize) -> Result<i32> {
    let block = even_block(base, dimension)?;
    let poly = characteristic(&block)?;
    let full = fill(base, dimension)?;
    let q = base as i128;
    let mut acc = 0i128;
    let mut weight = 1i128;
    for &term in &poly {
        acc = total(product(acc, full)?, product(term, weight)?)?;
        weight = product(weight, q)?;
    }
    Ok(match acc {
        acc if acc > 0 => -1,
        acc if acc < 0 => 1,
        _ => 0,
    })
}

/// The widest dimension the exact carry arithmetic reaches at the base.
///
/// The sign is the heaviest reading: it walks the characteristic polynomial at `f_D/q` with the
/// denominators cleared, so its running value climbs like `f_D^(D/2)` and a hundred and twenty-eight
/// bits run out at dimension fifteen in base three and dimension eleven in base five.
///
/// ```
/// assert_eq!(mrlymath::dim::carry::cap(3).unwrap(), 15);
/// assert_eq!(mrlymath::dim::carry::cap(5).unwrap(), 11);
/// ```
pub fn cap(base: usize) -> Result<usize> {
    middle(base)?;
    let mut top = 0;
    for dimension in 2..=CEILING {
        if sign(base, dimension).is_err() {
            break;
        }
        top = dimension;
    }
    if top < 2 {
        return value_error(format!(
            "base {base} reaches no dimension in exact integers."
        ));
    }
    Ok(top)
}

// SPECTRUM

fn ahead(walk: &[Vec<f64>], vector: &[f64]) -> Vec<f64> {
    walk.iter()
        .map(|row| row.iter().zip(vector).map(|(&a, &x)| a * x).sum())
        .collect()
}

fn behind(walk: &[Vec<f64>], vector: &[f64]) -> Vec<f64> {
    (0..vector.len())
        .map(|col| {
            (0..vector.len())
                .map(|row| walk[row][col] * vector[row])
                .sum()
        })
        .collect()
}

fn top(vector: &[f64]) -> f64 {
    vector.iter().fold(0.0f64, |peak, &x| peak.max(x.abs()))
}

fn settle(walk: &[Vec<f64>], left: bool) -> Option<Vec<f64>> {
    let mut vector = vec![1.0f64; walk.len()];
    for _ in 0..SWEEPS {
        let next = if left {
            behind(walk, &vector)
        } else {
            ahead(walk, &vector)
        };
        let peak = top(&next);
        if peak == 0.0 {
            return None;
        }
        vector = next.iter().map(|&x| x / peak).collect();
    }
    Some(vector)
}

/// The Perron root over the modulus of the second eigenvalue, or none where the block is one wide.
///
/// Power iteration finds the leading pair, Hotelling deflation takes it out and a second walk,
/// reprojected every sweep so rounding cannot bring the leader back, reads the runner up. At base
/// three the ratio falls to `(D + 2)/(D - 2)`, so no fixed spectral gap survives the dimensions.
pub fn spectral_ratio(base: usize, dimension: usize) -> Result<Option<f64>> {
    let block = even_block(base, dimension)?;
    let n = block.len();
    if n < 2 {
        return Ok(None);
    }
    let bound = block
        .iter()
        .map(|row| row.iter().sum::<i128>())
        .max()
        .unwrap_or(1) as f64;
    let walk: Vec<Vec<f64>> = block
        .iter()
        .map(|row| row.iter().map(|&x| x as f64 / bound).collect())
        .collect();
    let (right, left) = match (settle(&walk, false), settle(&walk, true)) {
        (Some(right), Some(left)) => (right, left),
        _ => return Ok(None),
    };
    let lead = top(&ahead(&walk, &right));
    let pair: f64 = left.iter().zip(&right).map(|(&u, &v)| u * v).sum();
    if pair == 0.0 || lead == 0.0 {
        return Ok(None);
    }
    let mut trail: Vec<f64> = (0..n).map(|i| 1.0 + 0.37 * ((i * 7) % 5) as f64).collect();
    let mut second = 0.0f64;
    for _ in 0..SWEEPS {
        let share: f64 = left.iter().zip(&trail).map(|(&u, &x)| u * x).sum::<f64>() / pair;
        let cleared: Vec<f64> = trail
            .iter()
            .zip(&right)
            .map(|(&x, &v)| x - share * v)
            .collect();
        let next = ahead(&walk, &cleared);
        second = top(&next);
        if second == 0.0 {
            return Ok(None);
        }
        trail = next.iter().map(|&x| x / second).collect();
    }
    Ok(Some(lead / second))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn brute(base: usize, dimension: usize) -> Vec<i128> {
        let centre = (base - 1) / 2;
        let mut out = vec![0i128; (base - 1) * dimension + 1];
        for mut code in 0..base.pow(dimension as u32) {
            let mut sum = 0;
            let mut middles = 0;
            for _ in 0..dimension {
                let digit = code % base;
                code /= base;
                sum += digit;
                middles += usize::from(digit == centre);
            }
            if middles <= 1 {
                out[sum] += 1;
            }
        }
        out
    }

    fn central(base: usize, dimension: usize, levels: usize) -> Vec<i128> {
        let poly = digit_polynomial(base, dimension).unwrap();
        let mut out = vec![1i128];
        let mut span = vec![1i128];
        let mut step = 1usize;
        for _ in 0..levels {
            let mut next = vec![0i128; span.len() + (base - 1) * dimension * step];
            for (at, &count) in span.iter().enumerate() {
                if count == 0 {
                    continue;
                }
                for (weight, &many) in poly.iter().enumerate() {
                    next[at + weight * step] += count * many;
                }
            }
            span = next;
            step *= base;
            out.push(span[dimension * (step - 1) / 2]);
        }
        out
    }

    #[test]
    fn the_digit_polynomial_is_the_enumeration_of_the_kept_cells() {
        for base in [3usize, 5] {
            for dimension in 2..=6 {
                let got = digit_polynomial(base, dimension).unwrap();
                assert_eq!(
                    got,
                    brute(base, dimension),
                    "base {base} dimension {dimension}"
                );
                assert_eq!(
                    got.iter().sum::<i128>(),
                    fill(base, dimension).unwrap(),
                    "base {base} dimension {dimension}"
                );
            }
        }
        assert!(digit_polynomial(4, 3).is_err());
        assert!(digit_polynomial(3, 1).is_err());
    }

    #[test]
    fn the_three_dimensional_block_is_the_published_anchor() {
        let block = even_block(3, 3).unwrap();
        assert_eq!(block, vec![vec![6, 6], vec![1, 3]]);
        assert_eq!(trace(&block), 9);
        assert_eq!(determinant(&block).unwrap(), 12);
        assert_eq!(characteristic(&block).unwrap(), vec![1, -9, 12]);
        assert_eq!(
            ladder(3, 3, 6).unwrap(),
            vec![1, 6, 42, 306, 2250, 16578, 122202]
        );
        let root = perron(&block).unwrap();
        assert!((root - (9.0 + 33f64.sqrt()) / 2.0).abs() < 1e-9);
        assert!((root.log(3.0) - 1.818_410).abs() < 1e-6);
        assert!((20f64.log(3.0) - 1.0 - 1.726_833).abs() < 1e-6);
    }

    #[test]
    fn the_traces_read_the_two_closed_forms() {
        let want = [2i128, 9, 11, 60, 47, 336];
        for (index, &term) in want.iter().enumerate() {
            let dimension = index + 2;
            assert_eq!(trace(&even_block(3, dimension).unwrap()), term);
        }
        for dimension in 2..=cap(3).unwrap() {
            let got = trace(&even_block(3, dimension).unwrap());
            let want = if dimension % 2 == 0 {
                3 * 2i128.pow(dimension as u32 - 2) - 1
            } else {
                3 * dimension as i128 * 2i128.pow(dimension as u32 - 3)
            };
            assert_eq!(got, want, "dimension {dimension}");
        }
    }

    #[test]
    fn the_three_generators_of_the_ladder_agree() {
        for base in [3usize, 5] {
            for dimension in 2..=6 {
                let block = ladder(base, dimension, 4).unwrap();
                assert_eq!(
                    block,
                    central(base, dimension, 4),
                    "base {base} dimension {dimension}"
                );
                let full = carry_matrix(base, dimension).unwrap();
                let half = full.len() / 2;
                let mut walked: Vec<Vec<i128>> = (0..full.len())
                    .map(|row| (0..full.len()).map(|col| i128::from(row == col)).collect())
                    .collect();
                let mut got = vec![1i128];
                for _ in 0..4 {
                    walked = multiply(&full, &walked).unwrap();
                    got.push(walked[half][half]);
                }
                assert_eq!(got, block, "base {base} dimension {dimension}");
            }
        }
        assert_eq!(
            ladder(3, 4, 6).unwrap(),
            vec![1, 6, 132, 1848, 29040, 441408, 6772128]
        );
        assert_eq!(ladder(3, 5, 4).unwrap(), vec![1, 30, 1000, 35700, 1321600]);
        assert_eq!(
            ladder(3, 6, 4).unwrap(),
            vec![1, 20, 4030, 242300, 24642700]
        );
        assert_eq!(ladder(5, 3, 4).unwrap(), vec![1, 18, 414, 9702, 227646]);
    }

    #[test]
    fn the_ladder_stops_where_the_exact_integers_do() {
        let terms = ladder(3, 14, 40).unwrap();
        assert_eq!(&terms[..3], &[1i128, 3432, 922_926_862]);
        assert_eq!(terms.len(), 9);
        assert_eq!(ladder(3, 10, 40).unwrap().len(), 12);
        assert_eq!(
            ladder(3, 2, 8).unwrap(),
            vec![1, 2, 4, 8, 16, 32, 64, 128, 256]
        );
    }

    #[test]
    fn the_sign_alternates_to_the_cap_at_both_bases() {
        for base in [3usize, 5] {
            let top = cap(base).unwrap();
            for dimension in 2..=top {
                let got = sign(base, dimension).unwrap();
                let want = if dimension % 2 == 0 { -1 } else { 1 };
                assert_eq!(got, want, "base {base} dimension {dimension}");
                let block = even_block(base, dimension).unwrap();
                let root = perron(&block).unwrap();
                let edge = fill(base, dimension).unwrap() as f64 / base as f64;
                assert_eq!(
                    got,
                    if root > edge { 1 } else { -1 },
                    "base {base} dimension {dimension}"
                );
            }
            assert!(sign(base, top + 1).is_err());
        }
        assert_eq!(cap(3).unwrap(), 15);
        assert_eq!(cap(5).unwrap(), 11);
    }

    #[test]
    fn the_spectral_ratio_falls_to_the_free_bound() {
        assert_eq!(spectral_ratio(3, 2).unwrap(), None);
        let mut last = f64::INFINITY;
        for dimension in [6usize, 10, 20, 30, 50] {
            let got = spectral_ratio(3, dimension).unwrap().unwrap();
            let free = (dimension as f64 + 2.0) / (dimension as f64 - 2.0);
            assert!(got > 1.0 && got < last, "dimension {dimension} ratio {got}");
            assert!(
                (got - free).abs() < 0.05,
                "dimension {dimension} ratio {got}"
            );
            last = got;
        }
        let fifty = spectral_ratio(3, 50).unwrap().unwrap();
        assert!((fifty - 13.0 / 12.0).abs() < 1e-9, "ratio {fifty}");
    }
}
