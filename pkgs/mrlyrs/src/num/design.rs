use crate::core::error::{shape_error, Result};
use crate::num::factor::mobius_sieve;
use crate::num::fft::fft;
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
/// assert_eq!(mrlyrs::num::design::digits_of(0b1011, 10), vec![0, 1, 3]);
/// ```
pub fn digits_of(mask: u32, base: u64) -> Vec<u64> {
    (0..base).filter(|&d| mask & (1u32 << d) != 0).collect()
}

/// Returns the elements of the digit design below the base raised to the depth, ascending: the whole numbers of at most that many base digits, every digit drawn from the set and the leading digit nonzero.
///
/// ```
/// assert_eq!(mrlyrs::num::design::elements(3, &[0, 1], 3), vec![1, 3, 4, 9, 10, 12, 13]);
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
/// assert_eq!(mrlyrs::num::design::size(&[0, 1], 20), 1048575);
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

/// Returns the running design Mobius meter, the partial sums of the Mobius values along the elements.
///
/// ```
/// assert_eq!(mrlyrs::num::design::meter(&[1, -1, -1, 0, -1]), vec![1, 0, -1, -1, -2]);
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
///
/// # Errors
///
/// Errs when the log grid holds fewer points than the series.
pub fn spectrum(log_x: &[f64], series: &[f64]) -> Result<(Vec<f64>, Vec<f64>)> {
    let n = series.len();
    if n < 2 || !n.is_power_of_two() {
        return Ok((Vec::new(), Vec::new()));
    }
    if log_x.len() < n {
        return shape_error(format!(
            "the grid holds {} and the series {n}.",
            log_x.len()
        ));
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
    fft(&mut re, &mut im, false)?;
    let range = n as f64 * step;
    let bins = n / 2 + 1;
    let gamma = (0..bins).map(|j| 2.0 * PI * j as f64 / range).collect();
    let power = (0..bins).map(|j| re[j] * re[j] + im[j] * im[j]).collect();
    Ok((gamma, power))
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
            let (_, value, _) = window.select_nth_unstable_by(mid, |a, b| a.total_cmp(b));
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
///
/// # Errors
///
/// Errs when the axis holds fewer bins than the score.
pub fn peaks(gamma: &[f64], score: &[f64], band: (f64, f64), threshold: f64) -> Result<Vec<usize>> {
    if gamma.len() < score.len() {
        return shape_error(format!(
            "the axis holds {} and the score {}.",
            gamma.len(),
            score.len()
        ));
    }
    let mut found: Vec<usize> = (1..score.len().saturating_sub(1))
        .filter(|&i| {
            gamma[i] > band.0
                && gamma[i] < band.1
                && score[i] > score[i - 1]
                && score[i] > score[i + 1]
                && score[i] > threshold
        })
        .collect();
    found.sort_by(|&a, &b| score[b].total_cmp(&score[a]));
    Ok(found)
}

/// Returns the design's pole lattice below the top, the ordinates 2 pi j over log q of the poles its Dirichlet series carries.
///
/// ```
/// let lines = mrlyrs::num::design::pole_lattice(3, 13.0);
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
    use crate::num::factor::mobius;

    const SAMPLES: usize = 4096;
    const FLOOR: usize = 101;
    const BAND: (f64, f64) = (4.0, 60.0);
    const THRESHOLD: f64 = 8.0;
    const TOP: usize = 10;
    const ECHO_CAP: u128 = 1 << 24;

    struct Echo {
        count: usize,
        last: i64,
        peak: i64,
        alpha: f64,
        span: f64,
        bin: f64,
        sieve: bool,
        logx: Vec<f64>,
        drawn: Vec<f64>,
        echo: Vec<f64>,
        rest: Vec<f64>,
        gamma: Vec<f64>,
        marks: Vec<f64>,
        head: Vec<usize>,
        found: usize,
        hits: usize,
        lines: usize,
        zeros: Vec<f64>,
        lattice: Vec<f64>,
        share: f64,
        residual: f64,
        chance: f64,
        chance_lines: f64,
    }

    fn read(base: u64, mask: u32, depth: usize, subtract: bool) -> Echo {
        let digits = digits_of(mask, base);
        let values = elements(base, &digits, depth);
        let mu: Vec<i8> = values.iter().map(|&v| mobius(v as usize)).collect();
        let running = meter(&mu);
        let alpha = (digits.len() as f64).ln() / (base as f64).ln();
        let exponent = alpha / 2.0;
        let logx = log_grid(&values, SAMPLES);
        let drawn = resample(&values, &running, exponent, &logx);
        let sieve = u128::from(base).pow(depth as u32) <= ECHO_CAP;
        let echo = if sieve {
            echo_series(&values, &logx, exponent)
        } else {
            Vec::new()
        };
        let rest: Vec<f64> = drawn
            .iter()
            .zip(echo.iter())
            .map(|(whole, part)| whole - part)
            .collect();
        let taken = if subtract && sieve { &rest } else { &drawn };
        let (gamma, power) = spectrum(&logx, taken).unwrap();
        let marks = score(&power, FLOOR);
        let found = peaks(&gamma, &marks, BAND, THRESHOLD).unwrap();
        let bin = gamma[1];
        let zeros: Vec<f64> = ZETA_ORDINATES
            .iter()
            .copied()
            .filter(|g| *g > BAND.0 && *g < BAND.1)
            .collect();
        let lattice: Vec<f64> = pole_lattice(base, BAND.1)
            .into_iter()
            .filter(|g| *g > BAND.0)
            .collect();
        let head: Vec<usize> = found.iter().copied().take(TOP).collect();
        let scale = upper_rms(&drawn);
        let width = BAND.1 - BAND.0;
        let chance = |list: &[f64]| (2.0 * bin * list.len() as f64 / width).min(1.0);
        Echo {
            count: values.len(),
            last: running.last().copied().unwrap(),
            peak: running.iter().map(|v| v.abs()).max().unwrap(),
            alpha,
            span: logx.last().unwrap() - logx.first().unwrap(),
            bin,
            sieve,
            hits: head
                .iter()
                .filter(|&&at| nearest(gamma[at], &zeros) <= bin)
                .count(),
            lines: head
                .iter()
                .filter(|&&at| nearest(gamma[at], &lattice) <= bin)
                .count(),
            found: found.len(),
            head,
            share: upper_rms(&echo) / scale,
            residual: upper_rms(&rest) / scale,
            chance: chance(&zeros),
            chance_lines: chance(&lattice),
            logx,
            drawn,
            echo,
            rest,
            gamma,
            marks,
            zeros,
            lattice,
        }
    }

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
    fn the_frequency_axis_is_two_pi_over_the_log_range() {
        let values = elements(3, &[0, 1], 10);
        let mu: Vec<i8> = values.iter().map(|&v| mobius(v as usize)).collect();
        let running = meter(&mu);
        let log_x = log_grid(&values, 1024);
        let series = resample(&values, &running, 0.5 * 2f64.ln() / 3f64.ln(), &log_x);
        let (gamma, power) = spectrum(&log_x, &series).unwrap();
        assert_eq!(gamma.len(), 513);
        assert_eq!(power.len(), 513);
        let step = (log_x[1023] - log_x[0]) / 1023.0;
        let range = 1024.0 * step;
        assert_eq!(gamma[0], 0.0);
        for j in [1usize, 7, 100, 512] {
            assert_eq!(gamma[j], 2.0 * PI * j as f64 / range);
        }
    }

    #[test]
    fn the_meter_reproduces_the_design_census() {
        let census = |base: u64, digits: &[u64], depth: usize| {
            let values = elements(base, digits, depth);
            let mu: Vec<i8> = values.iter().map(|&v| mobius(v as usize)).collect();
            let running = meter(&mu);
            (
                running.last().copied().unwrap(),
                running.iter().map(|value| value.abs()).max().unwrap(),
            )
        };
        let anchors = [
            (3u64, vec![0u64, 1], 14usize, (11i64, 105i64)),
            (3, vec![0, 1], 16, (149, 173)),
            (3, vec![0, 2], 14, (-10, 67)),
            (3, vec![1, 2], 8, (-31, 33)),
        ];
        for (base, digits, depth, want) in anchors {
            assert_eq!(census(base, &digits, depth), want, "base {base} {digits:?}");
        }
        assert_eq!(size(&[0, 1], 14), 16383);
        assert_eq!(size(&[0, 1], 20), 1048575);
    }

    #[test]
    fn the_full_digit_set_is_the_classical_mertens_control() {
        let full: Vec<u64> = (0..10).collect();
        let values = elements(10, &full, 6);
        let mu: Vec<i8> = values.iter().map(|&v| mobius(v as usize)).collect();
        assert_eq!(meter(&mu).last().copied().unwrap(), 212);
        let page = read(10, 0b11_1111_1111, 5, false);
        assert_eq!((page.count, page.last, page.peak), (99999, -48, 132));
        assert_eq!(format!("{:.6}", page.alpha), "1.000000");
    }

    #[test]
    fn the_full_digit_set_is_its_own_echo() {
        let page = read(10, 0b11_1111_1111, 5, false);
        assert_eq!(format!("{:.6}", page.share), "1.000000");
        assert_eq!(format!("{:.6}", page.residual), "0.000000");
        assert!(page.rest.iter().all(|v| v.abs() < 1e-9));
        assert!(page
            .echo
            .iter()
            .zip(&page.drawn)
            .all(|(part, whole)| (part - whole).abs() < 1e-9));
    }

    #[test]
    fn the_control_peaks_land_on_the_zeta_ordinates() {
        let page = read(10, 0b11_1111_1111, 5, false);
        assert_eq!((page.zeros.len(), page.lattice.len()), (13, 20));
        assert_eq!(format!("{:.6}", page.bin), "0.545618");
        assert_eq!((page.found, page.hits, page.lines), (13, 10, 6));
        assert_eq!(format!("{:.6}", page.chance), "0.253323");
        assert_eq!(format!("{:.6}", page.chance_lines), "0.389727");
        let leading: Vec<String> = page
            .head
            .iter()
            .take(5)
            .map(|&at| format!("{:.4}", page.gamma[at]))
            .collect();
        assert_eq!(leading.join(" "), "30.5546 32.7371 25.0984 21.2791 14.1861");
        let wide = read(2, 0b11, 18, false);
        assert_eq!((wide.found, wide.hits, wide.lines), (13, 10, 0));
        assert_eq!(wide.last, 24);
        assert_eq!(wide.lattice.len(), 6);
        assert_eq!(wide.gamma.len(), 2049);
    }

    #[test]
    fn the_design_carries_the_zeros_through_its_echo() {
        let page = read(10, 0b1_1111_1111, 5, false);
        assert_eq!((page.count, page.last, page.peak), (59048, 201, 268));
        assert_eq!(format!("{:.6}", page.alpha), "0.954243");
        assert_eq!(format!("{:.6}", page.span), "11.395132");
        assert_eq!(format!("{:.6}", page.bin), "0.551257");
        assert_eq!((page.found, page.hits, page.lines), (5, 5, 2));
        assert_eq!(format!("{:.6}", page.chance), "0.255941");
        assert_eq!(format!("{:.6}", page.chance_lines), "0.393755");
        let top = page.head[0];
        assert_eq!(format!("{:.4}", page.gamma[top]), "14.3327");
        assert_eq!(format!("{:.4}", page.marks[top]), "32.2266");
        assert_eq!(
            format!("{:.4}", nearest(page.gamma[top], &page.zeros)),
            "0.1980"
        );
        assert_eq!(format!("{:.6}", page.share), "0.354137");
        assert_eq!(format!("{:.6}", page.residual), "0.967810");
        assert_eq!(page.logx.len(), SAMPLES);
        let rest = read(10, 0b1_1111_1111, 5, true);
        assert_eq!((rest.found, rest.hits), (0, 0));
    }

    #[test]
    fn past_the_sieve_cap_the_echo_is_not_drawn() {
        let deep = read(3, 0b011, 17, false);
        assert!(!deep.sieve);
        assert!(deep.echo.is_empty());
        assert_eq!(deep.last, 157);
        let shallow = read(3, 0b011, 14, false);
        assert!(shallow.sieve);
        assert_eq!((shallow.last, shallow.peak), (11, 105));
        assert_eq!(shallow.echo.len(), SAMPLES);
    }

    #[test]
    fn refuses_an_axis_shorter_than_what_it_indexes() {
        assert!(peaks(&[1.0, 2.0], &[1.0, 2.0, 3.0], BAND, THRESHOLD).is_err());
        assert!(peaks(&[1.0, 2.0, 3.0], &[1.0, 2.0, 3.0], BAND, THRESHOLD).is_ok());
        assert!(spectrum(&[0.0, 1.0], &[1.0, 2.0, 3.0, 4.0]).is_err());
        assert!(spectrum(&[0.0, 1.0, 2.0, 3.0], &[1.0, 2.0, 3.0, 4.0]).is_ok());
        assert!(spectrum(&[], &[1.0, 2.0, 3.0]).is_ok());
    }
}
