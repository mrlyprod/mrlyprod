pub const COORD_CODES: [u128; 4] = [25390, 3772, 1511, 44551];

pub const SBOX: [u8; 16] = [3, 11, 15, 12, 4, 14, 2, 6, 10, 13, 7, 5, 0, 9, 8, 1];

pub const INV_SBOX: [u8; 16] = [12, 15, 6, 0, 4, 11, 7, 10, 14, 13, 8, 1, 3, 9, 5, 2];

#[inline]
pub fn apply(nibble: u8) -> u8 {
    SBOX[(nibble & 0x0F) as usize]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::boolean::{is_balanced, nonlinearity};
    #[test]
    fn sbox_is_a_bijection() {
        let mut seen = [false; 16];
        for &v in &SBOX {
            assert!(!seen[v as usize], "duplicate output {v}");
            seen[v as usize] = true;
        }
    }
    #[test]
    fn inverse_is_correct() {
        for x in 0..16u8 {
            assert_eq!(INV_SBOX[SBOX[x as usize] as usize], x);
        }
    }
    #[test]
    fn table_is_derived_from_mrly_codes() {
        for (x, &s) in SBOX.iter().enumerate() {
            let mut v = 0u8;
            for (b, &code) in COORD_CODES.iter().enumerate() {
                let bit = ((code >> x) & 1) as u8;
                v |= bit << (3 - b);
            }
            assert_eq!(v, s, "mismatch at {x}");
        }
    }
    #[test]
    fn coordinates_have_measured_quality() {
        for &code in &COORD_CODES {
            assert!(is_balanced(code, 4), "code {code} not balanced");
            assert_eq!(nonlinearity(code, 4), 4, "code {code} NL != 4");
        }
    }
}
