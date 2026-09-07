use crate::factor::mobius_sieve;
use crate::fft::fft;
use std::f64::consts::PI;

/// The ordinates of the first fourteen nontrivial zeros of the Riemann zeta function, the imaginary parts of the zeros on the critical line in ascending order.
pub const ZETA_ORDINATES: [f64; 14] = [
    14.134725141734693,
    21.022039638771556,
    25.01085758014569,
    30.424876125859512,
    32.93506158773919,
    37.58617815882567,
    40.9187190121475,
    43.327073280915,
    48.00515088116716,
    49.7738324776723,
    52.97032147771446,
    56.44624769706339,
    59.34704400260235,
    60.83177852460981,
];

/// Returns the digits a bitmask names inside the base, ascending.
///
/// ```
/// assert_eq!(mrlynum::design::digits_of(0b1011, 10), vec![0, 1, 3]);
/// ```
pub fn digits_of(mask: u32, base: u64) -> Vec<u64> {
    (0..base).filter(|&d| mask & (1u32 << d) != 0).collect()
}

/// Returns the elements of the digit design below the base raised to the depth, ascending: the whole numbers of at most that many base digits, every digit drawn from the set and the leading digit nonzero.
///
/// ```
/// assert_eq!(mrlynum::design::elements(3, &[0, 1], 3), vec![1, 3, 4, 9, 10, 12, 13]);
/// ```
pub fn elements(base: u64, digits: &[u64], depth: usize) -> Vec<u64> {
    let lead: Vec<u64> = digits.iter().copied().filter(|&d| d > 0).collect();
    if depth == 0 || lead.is_empty() {
        return Vec::new();
    }
    let mut out = lead.clone();
    let mut level = lead;
    for _ in 1..depth {
        let mut next = Vec::with_capacity(level.len() * digits.len());
        for value in &level {
            for digit in digits {
                next.push(value * base + digit);
            }
        }
        out.extend_from_slice(&next);
        level = next;
    }
    out
}

/// Returns the count of elements the design holds at the depth, the length [`elements`] returns without building them.
///
/// ```
/// assert_eq!(mrlynum::design::size(&[0, 1], 20), 1048575);
/// ```
pub fn size(digits: &[u64], depth: usize) -> u128 {
    let lead = digits.iter().filter(|&&d| d > 0).count() as u128;
    let k = digits.len() as u128;
    if depth == 0 || lead == 0 {
        return 0;
    }
    let mut run = 1u128;
    let mut total = 0u128;
    for _ in 0..depth {
        total += lead * run;
        run *= k;
    }
    total
}

/// Returns the Mobius value of every number by trial division over the primes below the square root of the largest.
///
/// ```
/// assert_eq!(mrlynum::design::mobius_of(&[1, 2, 3, 4, 5, 6]), vec![1, -1, -1, 0, -1, 1]);
/// ```
pub fn mobius_of(values: &[u64]) -> Vec<i8> {
    let top = values.iter().copied().max().unwrap_or(0);
    let root = (top as f64).sqrt() as usize + 2;
    let primes: Vec<u64> = crate::classics::primes(root)
        .into_iter()
        .map(|p| p as u64)
        .collect();
    values
        .iter()
        .map(|&value| {
            let mut rest = value;
            let mut sign = 1i8;
            for &p in &primes {
                if p * p > rest {
                    break;
                }
                if rest % p == 0 {
                    rest /= p;
                    if rest % p == 0 {
                        return 0;
                    }
                    sign = -sign;
                }
            }
            if rest > 1 {
                sign = -sign;
            }
            sign
        })
        .collect()
}

/// Returns the running design Mobius meter, the partial sums of the Mobius values along the elements.
///
/// ```
/// assert_eq!(mrlynum::design::meter(&[1, -1, -1, 0, -1]), vec![1, 0, -1, -1, -2]);
/// ```
pub fn meter(mu: &[i8]) -> Vec<i64> {
    let mut total = 0i64;
    mu.iter()
        .map(|&value| {
            total += i64::from(value);
            total
        })
        .collect()
}

/// Returns the log grid uniform over the span of the elements, from the log of the first to the log of the last.
pub fn log_grid(values: &[u64], samples: usize) -> Vec<f64> {
    if values.is_empty() || samples == 0 {
        return Vec::new();
    }
    let lo = (values[0] as f64).ln();
    let hi = (values[values.len() - 1] as f64).ln();
    let step = if samples > 1 {
        (hi - lo) / (samples - 1) as f64
    } else {
        0.0
    };
    (0..samples).map(|i| lo + step * i as f64).collect()
}

/// Reads the running meter at every point of the log grid and divides by x to the exponent.
pub fn resample(values: &[u64], running: &[i64], exponent: f64, log_x: &[f64]) -> Vec<f64> {
    log_x
        .iter()
        .map(|&t| {
            let slot = values.partition_point(|&v| (v as f64).ln() <= t);
            let held = if slot == 0 {
                0.0
            } else {
                running[slot - 1] as f64
            };
            held / (exponent * t).exp()
        })
        .collect()
}

/// Returns the density echo, the sum of mu(n) A_F(n)/n over the whole numbers up to each grid point divided by x to the exponent, sieving the Mobius values to the largest element.
pub fn echo_series(values: &[u64], log_x: &[f64], exponent: f64) -> Vec<f64> {
    let top = match values.last() {
        Some(&value) => value,
        None => return vec![0.0; log_x.len()],
    };
    let mu = mobius_sieve(top as usize);
    let mut out = Vec::with_capacity(log_x.len());
    let mut total = 0.0f64;
    let mut seen = 0u64;
    let mut at = 0usize;
    let mut n = 1u64;
    for &t in log_x {
        let mut bound = t.exp().floor() as u64;
        while bound < top && ((bound + 1) as f64).ln() <= t {
            bound += 1;
        }
        while bound > 0 && (bound as f64).ln() > t {
            bound -= 1;
        }
        while n <= bound {
            if at < values.len() && values[at] == n {
                seen += 1;
                at += 1;
            }
            total += f64::from(mu[n as usize]) * seen as f64 / n as f64;
            n += 1;
        }
        out.push(total / (exponent * t).exp());
    }
    out
}

/// Returns the frequency axis and the power spectrum of the series: the mean removed, a Hann window laid on, a real transform taken, and bin j read as the ordinate 2 pi j over the log range.
pub fn spectrum(log_x: &[f64], series: &[f64]) -> (Vec<f64>, Vec<f64>) {
    let n = series.len();
    if n < 2 || !n.is_power_of_two() {
        return (Vec::new(), Vec::new());
    }
    let step = (log_x[n - 1] - log_x[0]) / (n - 1) as f64;
    let mean = series.iter().sum::<f64>() / n as f64;
    let mut re: Vec<f64> = series
        .iter()
        .enumerate()
        .map(|(i, &value)| {
            let window = 0.5 - 0.5 * (2.0 * PI * i as f64 / (n - 1) as f64).cos();
            (value - mean) * window
        })
        .collect();
    let mut im = vec![0.0f64; n];
    fft(&mut re, &mut im, false);
    let range = n as f64 * step;
    let bins = n / 2 + 1;
    let gamma = (0..bins).map(|j| 2.0 * PI * j as f64 / range).collect();
    let power = (0..bins).map(|j| re[j] * re[j] + im[j] * im[j]).collect();
    (gamma, power)
}

/// Returns the running median of the power over a window of the given width, the window clamped at the ends.
pub fn median_floor(power: &[f64], width: usize) -> Vec<f64> {
    let n = power.len();
    if n == 0 || width == 0 {
        return vec![0.0; n];
    }
    let half = width / 2;
    let mut window = vec![0.0f64; width];
    (0..n)
        .map(|i| {
            for (slot, cell) in window.iter_mut().enumerate() {
                let at = (i + slot).saturating_sub(half).min(n - 1);
                *cell = power[at];
            }
            let mid = width / 2;
            let (_, value, _) =
                window.select_nth_unstable_by(mid, |a, b| a.partial_cmp(b).unwrap());
            *value
        })
        .collect()
}

/// Returns the power over its local median floor, the score a peak is read against.
pub fn score(power: &[f64], width: usize) -> Vec<f64> {
    let floor = median_floor(power, width);
    power
        .iter()
        .zip(floor.iter())
        .map(|(&value, &base)| value / base.max(f64::MIN_POSITIVE))
        .collect()
}

/// Returns the bins inside the band that rise above both neighbours and clear the score threshold, strongest first.
pub fn peaks(gamma: &[f64], score: &[f64], band: (f64, f64), threshold: f64) -> Vec<usize> {
    let mut found: Vec<usize> = (1..score.len().saturating_sub(1))
        .filter(|&i| {
            gamma[i] > band.0
                && gamma[i] < band.1
                && score[i] > score[i - 1]
                && score[i] > score[i + 1]
                && score[i] > threshold
        })
        .collect();
    found.sort_by(|&a, &b| score[b].partial_cmp(&score[a]).unwrap());
    found
}

/// Returns the design's pole lattice below the top, the ordinates 2 pi j over log q of the poles its Dirichlet series carries.
///
/// ```
/// let lines = mrlynum::design::pole_lattice(3, 13.0);
/// assert_eq!(format!("{:.4} {:.4} {}", lines[0], lines[1], lines.len()), "5.7192 11.4384 2");
/// ```
pub fn pole_lattice(base: u64, top: f64) -> Vec<f64> {
    let step = 2.0 * PI / (base as f64).ln();
    let mut out = Vec::new();
    let mut j = 1;
    while step * (j as f64) < top {
        out.push(step * j as f64);
        j += 1;
    }
    out
}

/// Returns the distance from the ordinate to the nearest entry of the list, infinite when the list is empty.
pub fn nearest(value: f64, list: &[f64]) -> f64 {
    list.iter()
        .map(|&entry| (entry - value).abs())
        .fold(f64::INFINITY, f64::min)
}

/// Returns the root mean square of the upper half of the series, the size the echo and the meter are compared at.
pub fn upper_rms(series: &[f64]) -> f64 {
    let half = series.len() / 2;
    let tail = &series[half..];
    if tail.is_empty() {
        return 0.0;
    }
    (tail.iter().map(|value| value * value).sum::<f64>() / tail.len() as f64).sqrt()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::factor::mobius;

    #[test]
    fn the_design_is_the_digit_strings_in_order() {
        assert_eq!(elements(10, &[0, 1], 2), vec![1, 10, 11]);
        assert_eq!(elements(3, &[1, 2], 2), vec![1, 2, 4, 5, 7, 8]);
        for depth in 1..=8 {
            let built = elements(3, &[0, 1], depth);
            assert_eq!(built.len() as u128, size(&[0, 1], depth));
            assert!(built.windows(2).all(|pair| pair[0] < pair[1]));
        }
        let full = elements(10, &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9], 4);
        assert_eq!(full, (1..10000).collect::<Vec<u64>>());
    }

    #[test]
    fn the_two_mobius_paths_agree() {
        let values = elements(3, &[0, 1], 12);
        let quick = mobius_of(&values);
        let slow: Vec<i8> = values.iter().map(|&v| mobius(v as usize)).collect();
        assert_eq!(quick, slow);
    }

    #[test]
    fn the_frequency_axis_is_two_pi_over_the_log_range() {
        let values = elements(3, &[0, 1], 10);
        let running = meter(&mobius_of(&values));
        let log_x = log_grid(&values, 1024);
        let series = resample(&values, &running, 0.5 * 2f64.ln() / 3f64.ln(), &log_x);
        let (gamma, power) = spectrum(&log_x, &series);
        assert_eq!(gamma.len(), 513);
        assert_eq!(power.len(), 513);
        let step = (log_x[1023] - log_x[0]) / 1023.0;
        let range = 1024.0 * step;
        assert_eq!(gamma[0], 0.0);
        for j in [1usize, 7, 100, 512] {
            assert_eq!(gamma[j], 2.0 * PI * j as f64 / range);
        }
    }
}
