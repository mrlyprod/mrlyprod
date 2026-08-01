pub fn walsh_spectrum(code: u128, n: usize) -> Vec<i64> {
    let size = 1usize << n;
    let mut t: Vec<i64> = (0..size)
        .map(|x| if (code >> x) & 1 == 1 { -1 } else { 1 })
        .collect();
    let mut len = 1;
    while len < size {
        let mut i = 0;
        while i < size {
            for j in i..i + len {
                let a = t[j];
                let b = t[j + len];
                t[j] = a + b;
                t[j + len] = a - b;
            }
            i += len << 1;
        }
        len <<= 1;
    }
    t
}

pub fn nonlinearity(code: u128, n: usize) -> i64 {
    if n == 0 {
        return 0;
    }
    let max_w = walsh_spectrum(code, n)
        .into_iter()
        .map(|w| w.abs())
        .max()
        .unwrap_or(0);
    (1i64 << (n - 1)) - max_w / 2
}

pub fn is_balanced(code: u128, n: usize) -> bool {
    let size = 1u32 << n;
    (code & ((1u128 << size) - 1)).count_ones() == size / 2
}

pub fn sac(code: u128, n: usize) -> f64 {
    if n == 0 {
        return 0.0;
    }
    let size = 1usize << n;
    let mut total = 0.0;
    for bit in 0..n {
        let mut flips = 0usize;
        for x in 0..size {
            let f = (code >> x) & 1;
            let g = (code >> (x ^ (1 << bit))) & 1;
            if f != g {
                flips += 1;
            }
        }
        total += flips as f64 / size as f64;
    }
    total / n as f64
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn linear_function_has_zero_nonlinearity() {
        let n = 3;
        let size = 1usize << n;
        let mut code: u128 = 0;
        for x in 0..size {
            if x & 1 == 1 {
                code |= 1 << x;
            }
        }
        assert_eq!(nonlinearity(code, n), 0);
    }
    #[test]
    fn n3_max_nonlinearity_is_two() {
        let n = 3;
        let max = (0..(1u128 << (1 << n)))
            .map(|c| nonlinearity(c, n))
            .max()
            .unwrap();
        assert_eq!(max, 2);
    }
    #[test]
    fn n4_reaches_bent_level_six() {
        let n = 4;
        let max = (0..(1u128 << (1 << n)))
            .map(|c| nonlinearity(c, n))
            .max()
            .unwrap();
        assert_eq!(max, 6);
    }
    #[test]
    fn balance_detects_half_ones() {
        assert!(is_balanced(0b0011, 2));
        assert!(!is_balanced(0b0111, 2));
    }
    #[test]
    fn sac_of_constant_is_zero() {
        assert_eq!(sac(0, 3), 0.0);
        let n = 3;
        let size = 1usize << n;
        let mut code: u128 = 0;
        for x in 0..size {
            if x & 1 == 1 {
                code |= 1 << x;
            }
        }
        assert!((sac(code, n) - 1.0 / 3.0).abs() < 1e-12);
    }
}
