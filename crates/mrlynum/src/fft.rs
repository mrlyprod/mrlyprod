use std::f64::consts::PI;

/// Transforms parallel real and imaginary slices in place over a power-of-two length, unscaled in either direction.
///
/// ```
/// let (mut re, mut im) = (vec![1.0, 0.0, 0.0, 0.0], vec![0.0; 4]);
/// mrlynum::fft::fft(&mut re, &mut im, false);
/// assert_eq!(re, vec![1.0; 4]);
/// ```
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

/// Transforms a size-square field in place, rows first and then columns.
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

/// Returns the magnitudes of a square field's transform, shifted so zero frequency sits at the centre.
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

/// Transforms a real size-square field forward by fft2, returning the real and imaginary parts.
pub fn transform(field: &[f64], size: usize) -> (Vec<f64>, Vec<f64>) {
    let mut re = field.to_vec();
    let mut im = vec![0.0; field.len()];
    fft2(&mut re, &mut im, size, false);
    (re, im)
}

/// Lays an odd-side mask into a size-square kernel with the mask centre at index (0, 0) and negative offsets wrapped; the cell at offset (dr, dc) lands at (-dr, -dc) modulo size, so convolving a field by the kernel reads at every site the mask-weighted sum over its neighbours, the neighbour count the life step counts.
pub fn embed_kernel(mask: &[u8], side: usize, size: usize) -> Vec<f64> {
    assert!(side % 2 == 1, "the mask side must be odd");
    assert!(side <= size, "the mask must fit the field");
    assert_eq!(mask.len(), side * side, "mask must be side*side");
    let centre = side / 2;
    let mut out = vec![0.0; size * size];
    for r in 0..side {
        for c in 0..side {
            let value = mask[r * side + c];
            if value == 0 {
                continue;
            }
            let rr = (size + centre - r) % size;
            let cc = (size + centre - c) % size;
            out[rr * size + cc] = f64::from(value);
        }
    }
    out
}

/// Convolves a size-square field on the torus by a kernel already transformed by fft2, the inverse scaled back by size squared.
pub fn convolve_with(field: &[f64], kernel_re: &[f64], kernel_im: &[f64], size: usize) -> Vec<f64> {
    let n = size * size;
    assert_eq!(field.len(), n, "field must be size*size");
    assert_eq!(kernel_re.len(), n, "kernel must be size*size");
    assert_eq!(kernel_im.len(), n, "kernel must be size*size");
    let (mut re, mut im) = transform(field, size);
    for ((a, b), (&kr, &ki)) in re
        .iter_mut()
        .zip(im.iter_mut())
        .zip(kernel_re.iter().zip(kernel_im))
    {
        let (fr, fi) = (*a, *b);
        *a = fr * kr - fi * ki;
        *b = fr * ki + fi * kr;
    }
    fft2(&mut re, &mut im, size, true);
    let scale = 1.0 / n as f64;
    re.iter_mut().for_each(|v| *v *= scale);
    re
}

/// Circularly convolves a size-square field on the torus by a kernel of the same shape through fft2 both ways.
pub fn convolve(field: &[f64], kernel: &[f64], size: usize) -> Vec<f64> {
    let (kernel_re, kernel_im) = transform(kernel, size);
    convolve_with(field, &kernel_re, &kernel_im, size)
}

/// Returns the centred magnitude spectrum of a size-square field through log(1 + magnitude), the DC bin included at the centre.
pub fn log_spectrum(field: &[f64], size: usize) -> Vec<f64> {
    magnitude_spectrum(field, size)
        .into_iter()
        .map(f64::ln_1p)
        .collect()
}

/// Averages a centred size-square spectrum over rings of integer radius from the centre bin, a bin joining the ring its distance rounds to, rings 0 through size over two; ring k holds the frequencies near k cycles per field.
pub fn radial_profile(spectrum: &[f64], size: usize) -> Vec<f64> {
    assert_eq!(spectrum.len(), size * size, "spectrum must be size*size");
    let half = size / 2;
    let mut sums = vec![0.0; half + 1];
    let mut counts = vec![0usize; half + 1];
    for r in 0..size {
        for c in 0..size {
            let dr = r as f64 - half as f64;
            let dc = c as f64 - half as f64;
            let ring = dr.hypot(dc).round() as usize;
            if ring <= half {
                sums[ring] += spectrum[r * size + c];
                counts[ring] += 1;
            }
        }
    }
    sums.iter()
        .zip(&counts)
        .map(|(&sum, &count)| if count == 0 { 0.0 } else { sum / count as f64 })
        .collect()
}

/// Finds the ring past the centre where a radial profile peaks, a tie broken at the smaller ring; zero when the profile holds no ring past ring 0.
pub fn peak_ring(profile: &[f64]) -> usize {
    let mut best = 0usize;
    let mut top = f64::NEG_INFINITY;
    for (ring, &value) in profile.iter().enumerate().skip(1) {
        if value > top {
            top = value;
            best = ring;
        }
    }
    best
}

/// Reads the wavelength in cells at a radial profile's peak, size over the peak ring with a tie broken at the smaller ring; zero when the profile holds no ring past ring 0.
pub fn peak_wavelength(profile: &[f64], size: usize) -> f64 {
    match peak_ring(profile) {
        0 => 0.0,
        ring => size as f64 / ring as f64,
    }
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
    fn moore() -> Vec<u8> {
        vec![1, 1, 1, 1, 0, 1, 1, 1, 1]
    }
    #[test]
    fn the_embedded_kernel_wraps_the_mask_about_the_origin() {
        let kernel = embed_kernel(&moore(), 3, 8);
        assert_eq!(kernel.iter().sum::<f64>(), 8.0);
        assert_eq!(kernel[0], 0.0);
        for (r, c) in [
            (0, 1),
            (0, 7),
            (1, 0),
            (7, 0),
            (1, 1),
            (1, 7),
            (7, 1),
            (7, 7),
        ] {
            assert_eq!(kernel[r * 8 + c], 1.0, "({r},{c})");
        }
    }
    #[test]
    fn an_off_centre_mask_cell_reads_the_neighbour_the_life_step_reads() {
        let kernel = embed_kernel(&[0, 1, 0, 0, 0, 0, 0, 0, 0], 3, 8);
        let mut field = vec![0.0; 64];
        field[3 * 8 + 3] = 1.0;
        let read = convolve(&field, &kernel, 8);
        for (i, v) in read.iter().enumerate() {
            let want = if i == 4 * 8 + 3 { 1.0 } else { 0.0 };
            assert!((v - want).abs() < 1e-9, "index {i} read {v}");
        }
    }
    #[test]
    fn the_moore_count_of_a_full_torus_is_eight_everywhere() {
        let kernel = embed_kernel(&moore(), 3, 16);
        let field = vec![1.0; 256];
        let counts = convolve(&field, &kernel, 16);
        assert!(counts.iter().all(|v| (v - 8.0).abs() < 1e-9));
        let (re, im) = transform(&kernel, 16);
        assert_eq!(convolve_with(&field, &re, &im, 16), counts);
    }
    #[test]
    fn the_log_spectrum_of_a_flat_field_is_one_centred_bin() {
        let spec = log_spectrum(&[2.0; 16], 4);
        assert!((spec[2 * 4 + 2] - 32f64.ln_1p()).abs() < 1e-9);
        assert!(spec
            .iter()
            .enumerate()
            .all(|(i, v)| i == 10 || v.abs() < 1e-9));
    }
    #[test]
    fn the_radial_profile_averages_rounded_rings_up_to_the_half_size() {
        let mut spec = vec![1.0; 16];
        spec[2 * 4 + 2] = 5.0;
        assert_eq!(radial_profile(&spec, 4), vec![5.0, 1.0, 1.0]);
    }
    #[test]
    fn the_peak_ring_skips_the_centre_and_breaks_ties_low() {
        assert_eq!(peak_ring(&[9.0, 1.0, 3.0, 3.0]), 2);
        assert_eq!(peak_wavelength(&[9.0, 1.0, 3.0, 3.0], 16), 8.0);
        assert_eq!(peak_ring(&[9.0]), 0);
        assert_eq!(peak_wavelength(&[9.0], 16), 0.0);
        assert_eq!(peak_ring(&[]), 0);
    }
}
