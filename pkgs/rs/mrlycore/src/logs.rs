use std::f64::consts::LN_2;

const MANTISSA: u64 = 0x000f_ffff_ffff_ffff;
const ONE_EXPONENT: u64 = 0x3ff0_0000_0000_0000;
const EXPONENT: u64 = 0x7ff;
const BIAS: i32 = 1023;
const SCALE: f64 = 4_503_599_627_370_496.0;
const TERMS: usize = 16;

/// Returns the natural logarithm of a number, from a series instead of libm.
pub fn ln(x: f64) -> f64 {
    if x.is_nan() || x < 0.0 {
        return f64::NAN;
    }
    if x == 0.0 {
        return f64::NEG_INFINITY;
    }
    if x.is_infinite() {
        return x;
    }
    let mut bits = x.to_bits();
    let mut e: i32 = 0;
    if (bits >> 52) & EXPONENT == 0 {
        bits = (x * SCALE).to_bits();
        e -= 52;
    }
    e += (((bits >> 52) & EXPONENT) as i32) - BIAS;
    let mut m = f64::from_bits((bits & MANTISSA) | ONE_EXPONENT);
    if m >= 1.5 {
        m *= 0.5;
        e += 1;
    }
    let s = (m - 1.0) / (m + 1.0);
    let square = s * s;
    let mut term = s;
    let mut sum = 0.0;
    for k in 0..TERMS {
        sum += term / (2 * k + 1) as f64;
        term *= square;
    }
    e as f64 * LN_2 + 2.0 * sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_and_two_are_exact() {
        assert_eq!(ln(1.0).to_bits(), 0.0f64.to_bits());
        assert_eq!(ln(2.0), LN_2);
    }

    #[test]
    fn the_edges_and_the_subnormals_hold() {
        assert!(ln(f64::NAN).is_nan());
        assert!(ln(-1.0).is_nan());
        assert_eq!(ln(0.0), f64::NEG_INFINITY);
        assert_eq!(ln(-0.0), f64::NEG_INFINITY);
        assert!(ln(f64::NEG_INFINITY).is_nan());
        assert_eq!(ln(f64::INFINITY), f64::INFINITY);
        assert_eq!(ln(f64::from_bits(1)), -1074.0 * LN_2);
    }

    #[test]
    fn the_integers_track_libm() {
        for i in 2..=100_000u32 {
            let x = i as f64;
            let want = x.ln();
            assert!(
                (ln(x) - want).abs() <= 4.0 * f64::EPSILON * want.abs(),
                "ln({x}) is {} not {want}",
                ln(x)
            );
        }
    }

    #[test]
    fn the_powers_of_two_track_libm() {
        for k in -300..=300i32 {
            let x = f64::from_bits(((BIAS + k) as u64) << 52);
            let want = x.ln();
            assert!(
                (ln(x) - want).abs() <= 4.0 * f64::EPSILON * want.abs(),
                "ln(2^{k}) is {} not {want}",
                ln(x)
            );
        }
    }
}
