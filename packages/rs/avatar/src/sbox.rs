const SBOX: [u8; 16] = [3, 11, 15, 12, 4, 14, 2, 6, 10, 13, 7, 5, 0, 9, 8, 1];

pub(crate) fn apply(bits: &[u8]) -> Vec<u8> {
    let mut out = Vec::with_capacity(bits.len());
    for nibble in bits.chunks_exact(4) {
        let value = nibble
            .iter()
            .fold(0usize, |acc, &b| acc << 1 | usize::from(b));
        let mapped = SBOX[value];
        out.extend((0..4).map(|k| (mapped >> (3 - k)) & 1));
    }
    out
}
