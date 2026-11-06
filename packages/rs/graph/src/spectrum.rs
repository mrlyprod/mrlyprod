#![allow(clippy::needless_range_loop)]

use crate::models::Network;
use mrly_error::{Result, value_error};
use mrly_linalg::{implicit_ql, tridiagonalise};
use std::cmp::Ordering;

const ZERO: f64 = 1e-12;

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
    let (mut d, mut e) = tridiagonalise(&mut work, false);
    implicit_ql(&mut d, &mut e, None)?;
    if d.iter().any(|v| v.is_nan()) {
        return value_error("The spectrum is not a number; the matrix carries a NaN or overflows.");
    }
    d.sort_by(|a, b| a.partial_cmp(b).unwrap_or(Ordering::Equal));
    Ok(d)
}

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

pub fn laplacian_spectrum(network: &Network, normalised: bool) -> Result<Vec<f64>> {
    symmetric_eigenvalues(&laplacian(network, normalised)?)
}

pub fn clusters(eigenvalues: &[f64], tolerance: f64) -> Result<Vec<(f64, usize)>> {
    if eigenvalues.iter().any(|v| v.is_nan()) {
        return value_error("The eigenvalues hold a NaN.");
    }
    let mut values = eigenvalues.to_vec();
    values.sort_by(|a, b| a.partial_cmp(b).unwrap_or(Ordering::Equal));
    let mut groups = Vec::new();
    let mut start = 0;
    for index in 1..=values.len() {
        if index == values.len() || values[index] - values[index - 1] > tolerance {
            let run = &values[start..index];
            groups.push((run.iter().sum::<f64>() / run.len() as f64, run.len()));
            start = index;
        }
    }
    Ok(groups)
}

pub fn multiplicity(eigenvalues: &[f64], value: f64, tolerance: f64) -> usize {
    eigenvalues
        .iter()
        .filter(|v| (**v - value).abs() <= tolerance)
        .count()
}

pub fn spectral_points(eigenvalues: &[f64]) -> Vec<(f64, f64)> {
    let mut values: Vec<f64> = eigenvalues.iter().map(|&v| v.max(0.0)).collect();
    values.sort_by(|a, b| a.partial_cmp(b).unwrap_or(Ordering::Equal));
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

pub fn spectral_exponent(eigenvalues: &[f64], window: f64) -> Option<f64> {
    spectral_fit(eigenvalues, window).map(|(_, slope, _)| 2.0 * slope)
}
