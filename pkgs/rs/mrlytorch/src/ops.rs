use crate::math;

/// The elementwise transforms a map kernel can apply.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Map {
    /// Multiplies every value by a factor.
    Scale(f32),
    /// Adds an offset to every value.
    Shift(f32),
    /// Clamps negatives to zero.
    Relu,
    /// Marks positives as one and the rest as zero.
    Step,
    /// Squashes every value through the hyperbolic tangent.
    Tanh,
    /// Raises e to every value.
    Exp,
}

/// The pairwise combines a zip kernel can apply.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Zip {
    /// Adds the pair.
    Add,
    /// Subtracts the second from the first.
    Sub,
    /// Multiplies the pair.
    Mul,
}

/// The folds a reduce kernel can apply.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Reduce {
    /// Sums every value in a fixed left-to-right order.
    Sum,
    /// Takes the largest value, negative infinity for an empty buffer.
    Max,
}

/// Multiplies an m by k matrix with a k by n matrix into out, or panics when a buffer is short.
pub fn gemm(a: &[f32], b: &[f32], m: usize, k: usize, n: usize, out: &mut [f32]) {
    for i in 0..m {
        for j in 0..n {
            let mut acc = 0.0f32;
            for p in 0..k {
                acc += a[i * k + p] * b[p * n + j];
            }
            out[i * n + j] = acc;
        }
    }
}

/// Applies one transform to every value of x into out, or panics when out is shorter than x.
pub fn map(op: Map, x: &[f32], out: &mut [f32]) {
    for (o, &v) in out.iter_mut().zip(x) {
        *o = match op {
            Map::Scale(factor) => v * factor,
            Map::Shift(offset) => v + offset,
            Map::Relu => {
                if v > 0.0 {
                    v
                } else {
                    0.0
                }
            }
            Map::Step => {
                if v > 0.0 {
                    1.0
                } else {
                    0.0
                }
            }
            Map::Tanh => math::tanh(v as f64) as f32,
            Map::Exp => math::exp(v as f64) as f32,
        };
    }
}

/// Combines a and b pairwise into out, or panics when out is shorter than the pair.
pub fn zip(op: Zip, a: &[f32], b: &[f32], out: &mut [f32]) {
    for ((o, &x), &y) in out.iter_mut().zip(a).zip(b) {
        *o = match op {
            Zip::Add => x + y,
            Zip::Sub => x - y,
            Zip::Mul => x * y,
        };
    }
}

/// Folds a whole buffer into one value in a fixed order.
pub fn reduce(op: Reduce, x: &[f32]) -> f32 {
    match op {
        Reduce::Sum => {
            let mut acc = 0.0f32;
            for &v in x {
                acc += v;
            }
            acc
        }
        Reduce::Max => {
            let mut best = f32::NEG_INFINITY;
            for &v in x {
                if v > best {
                    best = v;
                }
            }
            best
        }
    }
}

/// Accumulates alpha times x into y, or panics when y is shorter than x.
pub fn axpy(alpha: f32, x: &[f32], y: &mut [f32]) {
    for (o, &v) in y.iter_mut().zip(x) {
        *o += alpha * v;
    }
}

/// Slides a kernel over a zero-padded grid into out, or panics when a buffer is short.
pub fn conv(
    x: &[f32],
    xs: [usize; 2],
    kernel: &[f32],
    ks: [usize; 2],
    pad: [usize; 2],
    out: &mut [f32],
) {
    let [h, w] = xs;
    let [kh, kw] = ks;
    let [ph, pw] = pad;
    let oh = h + 2 * ph + 1 - kh;
    let ow = w + 2 * pw + 1 - kw;
    for oy in 0..oh {
        for ox in 0..ow {
            let mut acc = 0.0f32;
            for ky in 0..kh {
                for kx in 0..kw {
                    let iy = oy + ky;
                    let ix = ox + kx;
                    if iy >= ph && ix >= pw && iy - ph < h && ix - pw < w {
                        acc += x[(iy - ph) * w + (ix - pw)] * kernel[ky * kw + kx];
                    }
                }
            }
            out[oy * ow + ox] = acc;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gemm_multiplies_by_hand() {
        let a = [1.0, 2.0, 3.0, 4.0, 5.0, 6.0];
        let b = [7.0, 8.0, 9.0, 10.0, 11.0, 12.0];
        let mut out = [0.0f32; 4];
        gemm(&a, &b, 2, 3, 2, &mut out);
        assert_eq!(out, [58.0, 64.0, 139.0, 154.0]);
    }

    #[test]
    fn map_transforms_by_hand() {
        let x = [-2.0, 0.0, 3.0];
        let mut out = [0.0f32; 3];
        map(Map::Relu, &x, &mut out);
        assert_eq!(out, [0.0, 0.0, 3.0]);
        map(Map::Step, &x, &mut out);
        assert_eq!(out, [0.0, 0.0, 1.0]);
        map(Map::Scale(2.0), &x, &mut out);
        assert_eq!(out, [-4.0, 0.0, 6.0]);
        map(Map::Shift(1.0), &x, &mut out);
        assert_eq!(out, [-1.0, 1.0, 4.0]);
        map(Map::Exp, &[0.0], &mut out[..1]);
        assert_eq!(out[0], 1.0);
        map(Map::Tanh, &[0.0], &mut out[..1]);
        assert_eq!(out[0], 0.0);
    }

    #[test]
    fn zip_reduce_and_axpy_by_hand() {
        let a = [1.0, 2.0, 3.0];
        let b = [4.0, 5.0, 6.0];
        let mut out = [0.0f32; 3];
        zip(Zip::Add, &a, &b, &mut out);
        assert_eq!(out, [5.0, 7.0, 9.0]);
        zip(Zip::Sub, &a, &b, &mut out);
        assert_eq!(out, [-3.0, -3.0, -3.0]);
        zip(Zip::Mul, &a, &b, &mut out);
        assert_eq!(out, [4.0, 10.0, 18.0]);
        assert_eq!(reduce(Reduce::Sum, &a), 6.0);
        assert_eq!(reduce(Reduce::Max, &b), 6.0);
        assert_eq!(reduce(Reduce::Max, &[]), f32::NEG_INFINITY);
        let mut y = [1.0, 1.0, 1.0];
        axpy(2.0, &a, &mut y);
        assert_eq!(y, [3.0, 5.0, 7.0]);
    }

    #[test]
    fn conv_pads_and_slides_by_hand() {
        let x = [1.0, 2.0, 3.0, 4.0];
        let kernel = [1.0, 0.0, 0.0, 1.0];
        let mut valid = [0.0f32; 1];
        conv(&x, [2, 2], &kernel, [2, 2], [0, 0], &mut valid);
        assert_eq!(valid, [5.0]);
        let ones = [1.0f32; 9];
        let mut same = [0.0f32; 4];
        conv(&x, [2, 2], &ones, [3, 3], [1, 1], &mut same);
        assert_eq!(same, [10.0, 10.0, 10.0, 10.0]);
    }
}
