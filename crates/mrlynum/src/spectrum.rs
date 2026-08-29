#![allow(clippy::needless_range_loop)]

use crate::graph::models::Network;
use mrlycore::errors::{value_error, Result};

const SWEEPS: usize = 60;
const ZERO: f64 = 1e-12;

// SOLVER

fn tridiagonalise(a: &mut [Vec<f64>]) -> (Vec<f64>, Vec<f64>) {
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
    }
    e[0] = 0.0;
    for i in 0..n {
        d[i] = a[i][i];
    }
    (d, e)
}

fn implicit_ql(d: &mut [f64], e: &mut [f64]) -> Result<()> {
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

/// Returns the eigenvalues of a dense real symmetric matrix in ascending order.
///
/// The matrix is reduced to tridiagonal form by Householder reflections and then
/// diagonalised by implicit QL with Wilkinson shifts. Errors on a matrix that is
/// not square or whose entries disagree across the diagonal by more than `1e-12`.
///
/// ```
/// let m = vec![vec![2.0, 1.0], vec![1.0, 2.0]];
/// let values = mrlynum::spectrum::symmetric_eigenvalues(&m).unwrap();
/// assert!((values[0] - 1.0).abs() < 1e-12 && (values[1] - 3.0).abs() < 1e-12);
/// ```
pub fn symmetric_eigenvalues(matrix: &[Vec<f64>]) -> Result<Vec<f64>> {
    let n = matrix.len();
    for row in matrix {
        if row.len() != n {
            return value_error("The matrix is not square.");
        }
    }
    for i in 0..n {
        for j in (i + 1)..n {
            if (matrix[i][j] - matrix[j][i]).abs() > ZERO {
                return value_error(format!("The matrix is not symmetric at {i}, {j}."));
            }
        }
    }
    let mut work: Vec<Vec<f64>> = matrix.to_vec();
    let (mut d, mut e) = tridiagonalise(&mut work);
    implicit_ql(&mut d, &mut e)?;
    d.sort_by(|a, b| a.partial_cmp(b).unwrap());
    Ok(d)
}

// LAPLACIAN

/// Builds the Laplacian of a network, the combinatorial `D - A` or the normalised `I - D^-1/2 A D^-1/2`.
///
/// Self-loops are ignored and the degrees are the adjacency row sums. Errors when a
/// node carries no branch and the normalised form is asked for.
pub fn laplacian(network: &Network, normalised: bool) -> Result<Vec<Vec<f64>>> {
    let n = network.nodes.len();
    let mut matrix = vec![vec![0.0; n]; n];
    for branch in &network.branches {
        if branch.parent == branch.child {
            continue;
        }
        matrix[branch.parent][branch.child] += 1.0;
        matrix[branch.child][branch.parent] += 1.0;
    }
    let degree: Vec<f64> = matrix.iter().map(|row| row.iter().sum()).collect();
    for (index, &d) in degree.iter().enumerate() {
        if normalised && d == 0.0 {
            return value_error(format!("Node {index} carries no branch."));
        }
    }
    for i in 0..n {
        for j in 0..n {
            matrix[i][j] = if normalised {
                -matrix[i][j] / (degree[i] * degree[j]).sqrt()
            } else {
                -matrix[i][j]
            };
        }
        matrix[i][i] = if normalised { 1.0 } else { degree[i] };
    }
    Ok(matrix)
}

/// Returns the ascending Laplacian spectrum of a network, combinatorial or normalised.
///
/// ```
/// let mut net = mrlynum::graph::Network::new(1);
/// net.add_node(vec![0.0]).unwrap();
/// net.add_node(vec![1.0]).unwrap();
/// net.add_branch(0, 1, 1.0).unwrap();
/// let values = mrlynum::spectrum::laplacian_spectrum(&net, true).unwrap();
/// assert!(values[0].abs() < 1e-12 && (values[1] - 2.0).abs() < 1e-12);
/// ```
pub fn laplacian_spectrum(network: &Network, normalised: bool) -> Result<Vec<f64>> {
    symmetric_eigenvalues(&laplacian(network, normalised)?)
}

// READINGS

/// Groups eigenvalues into runs split by consecutive gaps above the tolerance, each run its mean and its size.
///
/// ```
/// let groups = mrlynum::spectrum::clusters(&[0.0, 1e-15, 2.0], 1e-9);
/// assert_eq!(groups.len(), 2);
/// assert_eq!(groups[0].1, 2);
/// ```
pub fn clusters(eigenvalues: &[f64], tolerance: f64) -> Vec<(f64, usize)> {
    let mut values = eigenvalues.to_vec();
    values.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let mut groups = Vec::new();
    let mut start = 0;
    for index in 1..=values.len() {
        if index == values.len() || values[index] - values[index - 1] > tolerance {
            let run = &values[start..index];
            groups.push((run.iter().sum::<f64>() / run.len() as f64, run.len()));
            start = index;
        }
    }
    groups
}

/// Counts the eigenvalues within the tolerance of a value.
pub fn multiplicity(eigenvalues: &[f64], value: f64, tolerance: f64) -> usize {
    eigenvalues
        .iter()
        .filter(|v| (**v - value).abs() <= tolerance)
        .count()
}

/// Builds the integrated density of states as points, each an eigenvalue and its rank fraction.
///
/// The eigenvalues are clamped at zero and sorted, exactly equal neighbours collapse to one
/// point at the run's last index over the total count, and points at or below `1e-12` drop.
///
/// ```
/// let points = mrlynum::spectrum::spectral_points(&[0.0, 0.5, 0.5, 2.0]);
/// assert_eq!(points.len(), 2);
/// assert!((points[0].1 - 0.75).abs() < 1e-12);
/// ```
pub fn spectral_points(eigenvalues: &[f64]) -> Vec<(f64, f64)> {
    let mut values: Vec<f64> = eigenvalues.iter().map(|&v| v.max(0.0)).collect();
    values.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let total = values.len();
    let mut points = Vec::new();
    let mut index = 0;
    while index < total {
        let mut last = index;
        while last + 1 < total && values[last + 1] == values[index] {
            last += 1;
        }
        if values[last] > ZERO {
            points.push((values[last], (last + 1) as f64 / total as f64));
        }
        index = last + 1;
    }
    points
}

/// Fits the low window of the integrated density of states in log-log: the intercept, the slope and the fitted count.
///
/// The points come from `spectral_points` and the first `max(floor(window * total), 3)` of
/// them are fitted by least squares. Returns `None` when fewer than two points remain.
pub fn spectral_fit(eigenvalues: &[f64], window: f64) -> Option<(f64, f64, usize)> {
    let points = spectral_points(eigenvalues);
    let xs: Vec<f64> = points.iter().map(|p| p.0.ln()).collect();
    let ys: Vec<f64> = points.iter().map(|p| p.1.ln()).collect();
    let top = ((window * eigenvalues.len() as f64).floor() as usize)
        .max(3)
        .min(xs.len());
    if top < 2 {
        return None;
    }
    let count = top as f64;
    let mean_x: f64 = xs[..top].iter().sum::<f64>() / count;
    let mean_y: f64 = ys[..top].iter().sum::<f64>() / count;
    let covariance: f64 = xs[..top]
        .iter()
        .zip(&ys[..top])
        .map(|(x, y)| (x - mean_x) * (y - mean_y))
        .sum();
    let variance: f64 = xs[..top].iter().map(|x| (x - mean_x) * (x - mean_x)).sum();
    if variance == 0.0 {
        return None;
    }
    let slope = covariance / variance;
    Some((mean_y - slope * mean_x, slope, top))
}

/// Reads the spectral exponent: twice the log-log slope of the integrated density of states over its low window.
pub fn spectral_exponent(eigenvalues: &[f64], window: f64) -> Option<f64> {
    spectral_fit(eigenvalues, window).map(|(_, slope, _)| 2.0 * slope)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::f64::consts::PI;

    fn path(n: usize) -> Network {
        let mut net = Network::new(1);
        for i in 0..n {
            net.add_node(vec![i as f64]).unwrap();
        }
        for i in 1..n {
            net.add_branch(i - 1, i, 1.0).unwrap();
        }
        net
    }

    fn complete(n: usize) -> Network {
        let mut net = Network::new(1);
        for i in 0..n {
            net.add_node(vec![i as f64]).unwrap();
        }
        for i in 0..n {
            for j in (i + 1)..n {
                net.add_branch(i, j, 1.0).unwrap();
            }
        }
        net
    }

    fn cycle(n: usize) -> Network {
        let mut net = path(n);
        net.add_branch(n - 1, 0, 1.0).unwrap();
        net
    }

    #[test]
    fn the_path_laplacian_reads_its_closed_form() {
        for n in 2..12usize {
            let values = laplacian_spectrum(&path(n), false).unwrap();
            let mut want: Vec<f64> = (0..n)
                .map(|k| 2.0 - 2.0 * (PI * k as f64 / n as f64).cos())
                .collect();
            want.sort_by(|a, b| a.partial_cmp(b).unwrap());
            for (got, expected) in values.iter().zip(&want) {
                assert!((got - expected).abs() < 1e-10, "n={n} {got} {expected}");
            }
        }
    }

    #[test]
    fn the_complete_laplacian_is_zero_once_and_n_the_rest() {
        for n in 2..10usize {
            let values = laplacian_spectrum(&complete(n), false).unwrap();
            assert!(values[0].abs() < 1e-10, "n={n}");
            for value in &values[1..] {
                assert!((value - n as f64).abs() < 1e-10, "n={n} {value}");
            }
        }
    }

    #[test]
    fn the_cycle_normalised_laplacian_reads_its_closed_form() {
        for n in 3..12usize {
            let values = laplacian_spectrum(&cycle(n), true).unwrap();
            let mut want: Vec<f64> = (0..n)
                .map(|k| 1.0 - (2.0 * PI * k as f64 / n as f64).cos())
                .collect();
            want.sort_by(|a, b| a.partial_cmp(b).unwrap());
            for (got, expected) in values.iter().zip(&want) {
                assert!((got - expected).abs() < 1e-10, "n={n} {got} {expected}");
            }
        }
    }

    #[test]
    fn a_random_symmetric_matrix_keeps_its_trace_and_frobenius_norm() {
        let n = 40;
        let mut seed = 0x2545f4914f6cdd1du64;
        let mut next = || {
            seed ^= seed << 13;
            seed ^= seed >> 7;
            seed ^= seed << 17;
            (seed >> 11) as f64 / (1u64 << 53) as f64 - 0.5
        };
        let mut m = vec![vec![0.0; n]; n];
        for i in 0..n {
            for j in i..n {
                let v = next();
                m[i][j] = v;
                m[j][i] = v;
            }
        }
        let trace: f64 = (0..n).map(|i| m[i][i]).sum();
        let frobenius: f64 = m.iter().flatten().map(|v| v * v).sum();
        let values = symmetric_eigenvalues(&m).unwrap();
        assert_eq!(values.len(), n);
        assert!((values.iter().sum::<f64>() - trace).abs() < 1e-9);
        assert!((values.iter().map(|v| v * v).sum::<f64>() - frobenius).abs() < 1e-9);
        assert!(values.windows(2).all(|w| w[0] <= w[1]));
    }

    #[test]
    fn a_three_by_three_matches_its_known_roots() {
        let m = vec![
            vec![2.0, 1.0, 0.0],
            vec![1.0, 2.0, 1.0],
            vec![0.0, 1.0, 2.0],
        ];
        let values = symmetric_eigenvalues(&m).unwrap();
        let root = 2.0f64.sqrt();
        for (got, want) in values.iter().zip([2.0 - root, 2.0, 2.0 + root]) {
            assert!((got - want).abs() < 1e-12, "{got} {want}");
        }
    }

    #[test]
    fn a_crooked_matrix_is_refused() {
        assert!(symmetric_eigenvalues(&[vec![1.0, 2.0]]).is_err());
        assert!(symmetric_eigenvalues(&[vec![1.0, 2.0], vec![3.0, 1.0]]).is_err());
        let mut lonely = Network::new(1);
        lonely.add_node(vec![0.0]).unwrap();
        assert!(laplacian(&lonely, true).is_err());
        assert!(laplacian(&lonely, false).is_ok());
    }

    #[test]
    fn the_clusters_and_multiplicities_split_a_hand_made_list() {
        let values = [0.0, 1e-15, 2e-15, 1.0, 1.0 + 1e-13, 1.0 - 1e-13, 2.0];
        let groups = clusters(&values, 1e-9);
        assert_eq!(
            groups.iter().map(|g| g.1).collect::<Vec<usize>>(),
            [3, 3, 1]
        );
        assert_eq!(groups.len(), 3);
        assert!(groups[0].0.abs() < 1e-14);
        assert!((groups[1].0 - 1.0).abs() < 1e-14);
        assert_eq!(multiplicity(&values, 1.0, 1e-12), 3);
        assert_eq!(multiplicity(&values, 1.0, 1e-14), 1);
        assert_eq!(multiplicity(&values, 0.0, 1e-9), 3);
        assert_eq!(clusters(&[], 1e-9).len(), 0);
        assert_eq!(spectral_exponent(&[0.0, 0.0], 0.1), None);
        assert_eq!(spectral_fit(&[0.0, 0.0], 0.1), None);
    }

    #[test]
    fn a_power_law_staircase_returns_its_slope_and_intercept() {
        let total = 200;
        let power = 0.75;
        let values: Vec<f64> = (0..total)
            .map(|j| ((j + 1) as f64 / total as f64).powf(1.0 / power))
            .collect();
        let (intercept, slope, fitted) = spectral_fit(&values, 0.1).unwrap();
        assert!((slope - power).abs() < 1e-9);
        assert!(intercept.abs() < 1e-9);
        assert_eq!(fitted, 20);
        assert_eq!(spectral_points(&values).len(), total);
        assert!((spectral_exponent(&values, 0.1).unwrap() - 2.0 * power).abs() < 1e-9);
    }
}
