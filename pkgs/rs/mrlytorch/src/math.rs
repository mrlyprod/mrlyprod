const LN2: f64 = core::f64::consts::LN_2;

const SQRT2: f64 = core::f64::consts::SQRT_2;

const HALF_PI: f64 = core::f64::consts::FRAC_PI_2;

fn pow2(k: i64) -> f64 {
    if k < -1022 {
        0.0
    } else if k > 1023 {
        f64::INFINITY
    } else {
        f64::from_bits(((k + 1023) as u64) << 52)
    }
}

pub fn exp(x: f64) -> f64 {
    if x > 709.0 {
        return f64::INFINITY;
    }
    if x < -709.0 {
        return 0.0;
    }
    let k = (x / LN2).round();
    let r = x - k * LN2;
    let mut term = 1.0;
    let mut sum = 1.0;
    for i in 1..18 {
        term *= r / i as f64;
        sum += term;
    }
    sum * pow2(k as i64)
}

pub fn ln(x: f64) -> f64 {
    if x <= 0.0 {
        return f64::NEG_INFINITY;
    }
    let bits = x.to_bits();
    let mut e = ((bits >> 52) & 0x7FF) as i64 - 1023;
    let mut m = f64::from_bits((bits & 0x000F_FFFF_FFFF_FFFF) | (1023u64 << 52));
    if m > SQRT2 {
        m *= 0.5;
        e += 1;
    }
    let t = (m - 1.0) / (m + 1.0);
    let t2 = t * t;
    let mut term = t;
    let mut sum = 0.0;
    for i in 0..14 {
        sum += term / (2 * i + 1) as f64;
        term *= t2;
    }
    2.0 * sum + e as f64 * LN2
}

fn cos_kernel(r: f64) -> f64 {
    let r2 = r * r;
    let mut term = 1.0;
    let mut sum = 1.0;
    for i in 1..10 {
        term *= -r2 / ((2 * i - 1) as f64 * (2 * i) as f64);
        sum += term;
    }
    sum
}

fn sin_kernel(r: f64) -> f64 {
    let r2 = r * r;
    let mut term = r;
    let mut sum = r;
    for i in 1..10 {
        term *= -r2 / ((2 * i) as f64 * (2 * i + 1) as f64);
        sum += term;
    }
    sum
}

pub fn cos(x: f64) -> f64 {
    let q = (x / HALF_PI).round();
    let r = x - q * HALF_PI;
    match (q as i64).rem_euclid(4) {
        0 => cos_kernel(r),
        1 => -sin_kernel(r),
        2 => -cos_kernel(r),
        _ => sin_kernel(r),
    }
}

pub fn tanh(x: f64) -> f64 {
    if x > 20.0 {
        return 1.0;
    }
    if x < -20.0 {
        return -1.0;
    }
    let t = exp(-2.0 * x.abs());
    let y = (1.0 - t) / (1.0 + t);
    if x < 0.0 {
        -y
    } else {
        y
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn close(a: f64, b: f64) -> bool {
        (a - b).abs() <= 1e-12 * (1.0 + a.abs().max(b.abs()))
    }

    #[test]
    fn exp_matches_the_standard_library() {
        for x in [-40.0, -3.5, -1.0, 0.0, 0.25, 1.0, 2.71, 10.0, 80.0] {
            assert!(close(exp(x), f64::exp(x)), "exp({x})");
        }
    }

    #[test]
    fn ln_matches_the_standard_library() {
        for x in [1e-40, 1e-6, 0.1, 0.5, 1.0, 1.5, 2.0, 3.0, 1e12] {
            assert!(close(ln(x), f64::ln(x)), "ln({x})");
        }
        assert_eq!(ln(0.0), f64::NEG_INFINITY);
    }

    #[test]
    fn cos_matches_the_standard_library() {
        for i in 0..64 {
            let x = i as f64 * core::f64::consts::TAU / 64.0;
            assert!(close(cos(x), f64::cos(x)), "cos({x})");
        }
    }

    #[test]
    fn tanh_matches_the_standard_library() {
        for x in [-25.0, -4.0, -1.0, -0.01, 0.0, 0.5, 2.0, 19.9, 30.0] {
            assert!(close(tanh(x), f64::tanh(x)), "tanh({x})");
        }
    }

    #[test]
    fn ln_inverts_exp() {
        for x in [-5.0, -0.5, 0.0, 1.0, 4.2] {
            assert!(close(ln(exp(x)), x), "ln(exp({x}))");
        }
    }
}
