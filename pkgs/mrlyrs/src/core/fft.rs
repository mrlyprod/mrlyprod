use std::f64::consts::PI;

pub fn fft(re: &mut [f64], im: &mut [f64], inverse: bool) {
    let n = re.len();
    assert_eq!(n, im.len(), "re and im must be equal length");
    if n <= 1 {
        return;
    }
    assert!(n.is_power_of_two(), "fft length must be a power of two");
    let mut j = 0usize;
    for i in 1..n {
        let mut bit = n >> 1;
        while j & bit != 0 {
            j ^= bit;
            bit >>= 1;
        }
        j |= bit;
        if i < j {
            re.swap(i, j);
            im.swap(i, j);
        }
    }
    let sign = if inverse { 1.0 } else { -1.0 };
    let mut len = 2;
    while len <= n {
        let ang = sign * 2.0 * PI / len as f64;
        let (wre, wim) = (ang.cos(), ang.sin());
        let half = len / 2;
        let mut start = 0;
        while start < n {
            let (mut cre, mut cim) = (1.0f64, 0.0f64);
            for k in 0..half {
                let i = start + k;
                let j = i + half;
                let (ure, uim) = (re[i], im[i]);
                let (vre, vim) = (re[j] * cre - im[j] * cim, re[j] * cim + im[j] * cre);
                re[i] = ure + vre;
                im[i] = uim + vim;
                re[j] = ure - vre;
                im[j] = uim - vim;
                let nre = cre * wre - cim * wim;
                cim = cre * wim + cim * wre;
                cre = nre;
            }
            start += len;
        }
        len <<= 1;
    }
}

pub fn fft2(re: &mut [f64], im: &mut [f64], size: usize, inverse: bool) {
    assert_eq!(re.len(), size * size, "buffer must be size*size");
    for r in 0..size {
        let s = r * size;
        fft(&mut re[s..s + size], &mut im[s..s + size], inverse);
    }
    let mut cre = vec![0.0; size];
    let mut cim = vec![0.0; size];
    for c in 0..size {
        for r in 0..size {
            cre[r] = re[r * size + c];
            cim[r] = im[r * size + c];
        }
        fft(&mut cre, &mut cim, inverse);
        for r in 0..size {
            re[r * size + c] = cre[r];
            im[r * size + c] = cim[r];
        }
    }
}

pub fn magnitude_spectrum(field: &[f64], size: usize) -> Vec<f64> {
    let mut re = field.to_vec();
    let mut im = vec![0.0; field.len()];
    fft2(&mut re, &mut im, size, false);
    let mut out = vec![0.0; size * size];
    let half = size / 2;
    for r in 0..size {
        for c in 0..size {
            let mag = (re[r * size + c].powi(2) + im[r * size + c].powi(2)).sqrt();
            let rr = (r + half) % size;
            let cc = (c + half) % size;
            out[rr * size + cc] = mag;
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn impulse_has_flat_spectrum() {
        let mut re = vec![0.0; 8];
        let mut im = vec![0.0; 8];
        re[0] = 1.0;
        fft(&mut re, &mut im, false);
        for k in 0..8 {
            let mag = (re[k].powi(2) + im[k].powi(2)).sqrt();
            assert!((mag - 1.0).abs() < 1e-9, "bin {k} mag {mag}");
        }
    }
    #[test]
    fn forward_then_inverse_round_trips() {
        let orig: Vec<f64> = (0..16).map(|i| (i as f64 * 0.7).sin()).collect();
        let mut re = orig.clone();
        let mut im = vec![0.0; 16];
        fft(&mut re, &mut im, false);
        fft(&mut re, &mut im, true);
        for (i, v) in orig.iter().enumerate() {
            assert!((re[i] / 16.0 - v).abs() < 1e-9, "index {i}");
        }
    }
    #[test]
    fn pure_sinusoid_has_two_symmetric_peaks() {
        let n = 16;
        let signal: Vec<f64> = (0..n)
            .map(|i| (2.0 * PI * 3.0 * i as f64 / n as f64).cos())
            .collect();
        let mut re = signal.clone();
        let mut im = vec![0.0; n];
        fft(&mut re, &mut im, false);
        let mag: Vec<f64> = (0..n)
            .map(|k| (re[k].powi(2) + im[k].powi(2)).sqrt())
            .collect();
        let peak = mag
            .iter()
            .enumerate()
            .max_by(|a, b| a.1.partial_cmp(b.1).unwrap())
            .unwrap()
            .0;
        assert!(peak == 3 || peak == n - 3, "peak at {peak}");
    }
    #[test]
    fn spectrum2d_centres_dc() {
        let size = 8;
        let field = vec![2.0; size * size];
        let spec = magnitude_spectrum(&field, size);
        let centre = (size / 2) * size + size / 2;
        let max = spec.iter().cloned().fold(0.0f64, f64::max);
        assert!((spec[centre] - max).abs() < 1e-9);
        assert!(spec[centre] > 0.0);
    }
}
