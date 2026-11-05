use mrly_core::error::{Result, shape_error, value_error};
use std::f64::consts::PI;

pub fn fft(re: &mut [f64], im: &mut [f64], inverse: bool) -> Result<()> {
    let n = re.len();
    if n != im.len() {
        return shape_error(format!(
            "the real part holds {n} and the imaginary part {}.",
            im.len()
        ));
    }
    if n <= 1 {
        return Ok(());
    }
    if !n.is_power_of_two() {
        return value_error(format!("the length {n} is not a power of two."));
    }
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
    Ok(())
}

pub fn fft2(re: &mut [f64], im: &mut [f64], size: usize, inverse: bool) -> Result<()> {
    if re.len() != size * size {
        return shape_error(format!(
            "the buffer holds {} and the field is {size} by {size}.",
            re.len()
        ));
    }
    for r in 0..size {
        let s = r * size;
        fft(&mut re[s..s + size], &mut im[s..s + size], inverse)?;
    }
    let mut cre = vec![0.0; size];
    let mut cim = vec![0.0; size];
    for c in 0..size {
        for r in 0..size {
            cre[r] = re[r * size + c];
            cim[r] = im[r * size + c];
        }
        fft(&mut cre, &mut cim, inverse)?;
        for r in 0..size {
            re[r * size + c] = cre[r];
            im[r * size + c] = cim[r];
        }
    }
    Ok(())
}

pub fn magnitude_spectrum(field: &[f64], size: usize) -> Result<Vec<f64>> {
    let mut re = field.to_vec();
    let mut im = vec![0.0; field.len()];
    fft2(&mut re, &mut im, size, false)?;
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
    Ok(out)
}

pub fn transform(field: &[f64], size: usize) -> Result<(Vec<f64>, Vec<f64>)> {
    let mut re = field.to_vec();
    let mut im = vec![0.0; field.len()];
    fft2(&mut re, &mut im, size, false)?;
    Ok((re, im))
}

pub fn embed_kernel(mask: &[u8], side: usize, size: usize) -> Result<Vec<f64>> {
    if side.is_multiple_of(2) {
        return value_error(format!("the mask side {side} is not odd."));
    }
    if side > size {
        return value_error(format!("the mask side {side} overruns the field {size}."));
    }
    if mask.len() != side * side {
        return shape_error(format!(
            "the mask holds {} and its side is {side}.",
            mask.len()
        ));
    }
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
    Ok(out)
}

pub fn convolve_with(
    field: &[f64],
    kernel_re: &[f64],
    kernel_im: &[f64],
    size: usize,
) -> Result<Vec<f64>> {
    let n = size * size;
    if field.len() != n {
        return shape_error(format!(
            "the field holds {} and is {size} by {size}.",
            field.len()
        ));
    }
    if kernel_re.len() != n || kernel_im.len() != n {
        return shape_error(format!(
            "the kernel holds {} and {} against a {size} by {size} field.",
            kernel_re.len(),
            kernel_im.len()
        ));
    }
    let (mut re, mut im) = transform(field, size)?;
    for ((a, b), (&kr, &ki)) in re
        .iter_mut()
        .zip(im.iter_mut())
        .zip(kernel_re.iter().zip(kernel_im))
    {
        let (fr, fi) = (*a, *b);
        *a = fr * kr - fi * ki;
        *b = fr * ki + fi * kr;
    }
    fft2(&mut re, &mut im, size, true)?;
    let scale = 1.0 / n as f64;
    re.iter_mut().for_each(|v| *v *= scale);
    Ok(re)
}

pub fn convolve(field: &[f64], kernel: &[f64], size: usize) -> Result<Vec<f64>> {
    let (kernel_re, kernel_im) = transform(kernel, size)?;
    convolve_with(field, &kernel_re, &kernel_im, size)
}

pub fn log_spectrum(field: &[f64], size: usize) -> Result<Vec<f64>> {
    Ok(magnitude_spectrum(field, size)?
        .into_iter()
        .map(f64::ln_1p)
        .collect())
}

pub fn radial_profile(spectrum: &[f64], size: usize) -> Result<Vec<f64>> {
    if spectrum.len() != size * size {
        return shape_error(format!(
            "the spectrum holds {} and the field is {size} by {size}.",
            spectrum.len()
        ));
    }
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
    Ok(sums
        .iter()
        .zip(&counts)
        .map(|(&sum, &count)| if count == 0 { 0.0 } else { sum / count as f64 })
        .collect())
}

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

pub fn peak_wavelength(profile: &[f64], size: usize) -> f64 {
    match peak_ring(profile) {
        0 => 0.0,
        ring => size as f64 / ring as f64,
    }
}
