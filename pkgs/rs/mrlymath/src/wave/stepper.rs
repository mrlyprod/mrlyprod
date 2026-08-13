use super::medium::Medium;
use super::COURANT;
use std::f64::consts::PI;

/// The source envelope's centre in steps.
pub const T0: f64 = 40.0;
/// The source envelope's spread in steps.
pub const SPREAD: f64 = 12.0;

/// Runs the leapfrog stepper and records the receiver column's summed pressure at every step.
///
/// A Gaussian-modulated sine drives the source column, the side walls copy their
/// inner neighbour, and the top and bottom walls stay rigid.
pub fn run(medium: &Medium, steps: usize, src_col: usize, rec_col: usize, f_src: f64) -> Vec<f64> {
    let (ny, nx) = (medium.height, medium.width);
    let c2: Vec<f64> = medium
        .speed
        .iter()
        .map(|s| s * s * COURANT * COURANT)
        .collect();
    let mut p = vec![0.0f64; ny * nx];
    let mut p_prev = vec![0.0f64; ny * nx];
    let mut p_new = vec![0.0f64; ny * nx];
    let mut rec = Vec::with_capacity(steps);
    for t in 0..steps {
        for r in 1..ny - 1 {
            for c in 1..nx - 1 {
                let i = r * nx + c;
                let lap = p[i - nx] + p[i + nx] + p[i - 1] + p[i + 1] - 4.0 * p[i];
                p_new[i] = 2.0 * p[i] - p_prev[i] + c2[i] * lap;
            }
        }
        let t = t as f64;
        let drive =
            (-(t - T0) * (t - T0) / (2.0 * SPREAD * SPREAD)).exp() * (2.0 * PI * f_src * t).sin();
        for r in 1..ny - 1 {
            p_new[r * nx + src_col] += drive;
            p_new[r * nx] = p[r * nx + 1];
            p_new[r * nx + nx - 1] = p[r * nx + nx - 2];
        }
        for c in 0..nx {
            p_new[c] = 0.0;
            p_new[(ny - 1) * nx + c] = 0.0;
        }
        std::mem::swap(&mut p_prev, &mut p);
        std::mem::swap(&mut p, &mut p_new);
        rec.push((0..ny).map(|r| p[r * nx + rec_col]).sum());
    }
    rec
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_pulse_reaches_the_receiver() {
        let m = Medium::free(8, 40);
        let rec = run(&m, 128, 2, 20, 0.06);
        assert_eq!(rec.len(), 128);
        assert!(rec[..8].iter().all(|&v| v == 0.0));
        assert!(rec.iter().any(|&v| v.abs() > 1e-3));
    }

    #[test]
    fn two_runs_agree_bit_for_bit() {
        let m = Medium::free(6, 30);
        let a = run(&m, 64, 2, 25, 0.06);
        let b = run(&m, 64, 2, 25, 0.06);
        assert_eq!(a, b);
    }
}
