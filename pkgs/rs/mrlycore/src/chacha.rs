use std::collections::HashSet;

const CONSTANTS: [u32; 4] = [0x6170_7865, 0x3320_646e, 0x7962_2d32, 0x6b20_6574];
const BUF_WORDS: usize = 64;
const BLOCK_WORDS: usize = 16;

/// An eight-round ChaCha keystream serving as a seekable source of random words.
#[derive(Clone)]
pub struct ChaCha8 {
    key: [u32; 8],
    block_pos: u64,
    buf: [u32; BUF_WORDS],
    index: usize,
}

impl ChaCha8 {
    /// Builds the generator from a 32-byte key.
    pub fn from_seed(seed: [u8; 32]) -> ChaCha8 {
        let mut key = [0u32; 8];
        for (word, chunk) in key.iter_mut().zip(seed.chunks_exact(4)) {
            *word = u32::from_le_bytes([chunk[0], chunk[1], chunk[2], chunk[3]]);
        }
        ChaCha8 {
            key,
            block_pos: 0,
            buf: [0; BUF_WORDS],
            index: BUF_WORDS,
        }
    }

    /// Builds the generator by stretching a word seed into a full key.
    pub fn from_u64(seed: u64) -> ChaCha8 {
        let mut state = seed;
        let mut bytes = [0u8; 32];
        for chunk in bytes.chunks_exact_mut(4) {
            chunk.copy_from_slice(&pcg32(&mut state));
        }
        ChaCha8::from_seed(bytes)
    }

    /// Returns the next 32-bit word of the keystream.
    pub fn next_u32(&mut self) -> u32 {
        if self.index >= BUF_WORDS {
            self.generate_and_set(0);
        }
        let value = self.buf[self.index];
        self.index += 1;
        value
    }

    /// Returns the next two keystream words joined into 64 bits, low word first.
    pub fn next_u64(&mut self) -> u64 {
        let index = self.index;
        if index < BUF_WORDS - 1 {
            self.index += 2;
            (u64::from(self.buf[index + 1]) << 32) | u64::from(self.buf[index])
        } else if index >= BUF_WORDS {
            self.generate_and_set(2);
            (u64::from(self.buf[1]) << 32) | u64::from(self.buf[0])
        } else {
            let x = u64::from(self.buf[BUF_WORDS - 1]);
            self.generate_and_set(1);
            let y = u64::from(self.buf[0]);
            (y << 32) | x
        }
    }

    /// Returns the absolute word position in the keystream.
    pub fn word_pos(&self) -> u128 {
        let buf_start_block = self.block_pos.wrapping_sub(4);
        let blocks_part = (self.index / BLOCK_WORDS) as u64;
        let words_part = (self.index % BLOCK_WORDS) as u64;
        let pos_block = buf_start_block.wrapping_add(blocks_part);
        u128::from(pos_block) * BLOCK_WORDS as u128 + u128::from(words_part)
    }

    /// Seeks the keystream to an absolute word position.
    pub fn set_word_pos(&mut self, word_offset: u128) {
        self.block_pos = (word_offset / BLOCK_WORDS as u128) as u64;
        self.generate_and_set((word_offset % BLOCK_WORDS as u128) as usize);
    }

    /// Returns a uniform double in the half-open unit interval.
    pub fn unit(&mut self) -> f64 {
        let scale = 1.0 / ((1u64 << 53) as f64);
        scale * ((self.next_u64() >> 11) as f64)
    }

    /// Returns a fair coin flip.
    pub fn boolean(&mut self) -> bool {
        (self.next_u32() as i32) < 0
    }

    /// Returns an unbiased uniform integer between low and high inclusive.
    pub fn range_i64(&mut self, low: i64, high: i64) -> i64 {
        assert!(low <= high, "range_i64: low > high");
        let range = high.wrapping_sub(low).wrapping_add(1) as u64;
        if range == 0 {
            return self.next_u64() as i64;
        }
        let zone = (range << range.leading_zeros()).wrapping_sub(1);
        loop {
            let (hi, lo) = wide_mul_u64(self.next_u64(), range);
            if lo <= zone {
                return low.wrapping_add(hi as i64);
            }
        }
    }

    /// Returns an unbiased uniform integer below n.
    pub fn below_u64(&mut self, n: u64) -> u64 {
        assert!(n > 0, "below_u64: empty range");
        let zone = (n << n.leading_zeros()).wrapping_sub(1);
        loop {
            let (hi, lo) = wide_mul_u64(self.next_u64(), n);
            if lo <= zone {
                return hi;
            }
        }
    }

    /// Shuffles the slice in place by Fisher-Yates.
    pub fn shuffle<T>(&mut self, seq: &mut [T]) {
        for i in (1..seq.len()).rev() {
            let j = self.index_below(i + 1);
            seq.swap(i, j);
        }
    }

    /// Draws amount distinct indices below length, choosing its strategy by the sizes.
    pub fn sample_indices(&mut self, length: usize, amount: usize) -> Vec<usize> {
        assert!(amount <= length, "sample_indices: amount > length");
        if length > u32::MAX as usize {
            return self.sample_rejection_u64(length as u64, amount as u64);
        }
        let length = length as u32;
        let amount = amount as u32;
        if amount < 163 {
            const C: [[f32; 2]; 2] = [[1.6, 8.0 / 45.0], [10.0, 70.0 / 9.0]];
            let j = if length < 500_000 { 0 } else { 1 };
            let amount_fp = amount as f32;
            let m4 = C[0][j] * amount_fp;
            if amount > 11 && (length as f32) < (C[1][j] + m4) * amount_fp {
                self.sample_inplace(length, amount)
            } else {
                self.sample_floyd(length, amount)
            }
        } else {
            const C: [f32; 2] = [270.0, 330.0 / 9.0];
            let j = if length < 500_000 { 0 } else { 1 };
            if (length as f32) < C[j] * (amount as f32) {
                self.sample_inplace(length, amount)
            } else {
                self.sample_rejection_u32(length, amount)
            }
        }
    }

    fn index_below(&mut self, ubound: usize) -> usize {
        if ubound <= u32::MAX as usize {
            self.range_u32(0, ubound as u32 - 1) as usize
        } else {
            self.below_u64(ubound as u64) as usize
        }
    }

    fn range_u32(&mut self, low: u32, high: u32) -> u32 {
        let range = high.wrapping_sub(low).wrapping_add(1);
        if range == 0 {
            return self.next_u32();
        }
        let zone = (range << range.leading_zeros()).wrapping_sub(1);
        loop {
            let (hi, lo) = wide_mul_u32(self.next_u32(), range);
            if lo <= zone {
                return low.wrapping_add(hi);
            }
        }
    }

    fn sample_floyd(&mut self, length: u32, amount: u32) -> Vec<usize> {
        let floyd_shuffle = amount < 50;
        let mut indices: Vec<u32> = Vec::with_capacity(amount as usize);
        for j in length - amount..length {
            let t = self.range_u32(0, j);
            if floyd_shuffle {
                if let Some(pos) = indices.iter().position(|&x| x == t) {
                    indices.insert(pos, j);
                    continue;
                }
            } else if indices.contains(&t) {
                indices.push(j);
                continue;
            }
            indices.push(t);
        }
        if !floyd_shuffle {
            for i in (1..amount).rev() {
                let j = self.range_u32(0, i);
                indices.swap(i as usize, j as usize);
            }
        }
        indices.into_iter().map(|i| i as usize).collect()
    }

    fn sample_inplace(&mut self, length: u32, amount: u32) -> Vec<usize> {
        let mut indices: Vec<u32> = (0..length).collect();
        for i in 0..amount {
            let j = self.range_u32(i, length - 1);
            indices.swap(i as usize, j as usize);
        }
        indices.truncate(amount as usize);
        indices.into_iter().map(|i| i as usize).collect()
    }

    fn sample_rejection_u32(&mut self, length: u32, amount: u32) -> Vec<usize> {
        let ints_to_reject = (u32::MAX - length + 1) % length;
        let zone = u32::MAX - ints_to_reject;
        let mut cache = HashSet::with_capacity(amount as usize);
        let mut indices = Vec::with_capacity(amount as usize);
        for _ in 0..amount {
            let mut pos = self.distr_u32(length, zone);
            while !cache.insert(pos) {
                pos = self.distr_u32(length, zone);
            }
            indices.push(pos as usize);
        }
        indices
    }

    fn sample_rejection_u64(&mut self, length: u64, amount: u64) -> Vec<usize> {
        let ints_to_reject = (u64::MAX - length + 1) % length;
        let zone = u64::MAX - ints_to_reject;
        let mut cache = HashSet::with_capacity(amount as usize);
        let mut indices = Vec::with_capacity(amount as usize);
        for _ in 0..amount {
            let mut pos = self.distr_u64(length, zone);
            while !cache.insert(pos) {
                pos = self.distr_u64(length, zone);
            }
            indices.push(pos as usize);
        }
        indices
    }

    fn distr_u32(&mut self, range: u32, zone: u32) -> u32 {
        loop {
            let (hi, lo) = wide_mul_u32(self.next_u32(), range);
            if lo <= zone {
                return hi;
            }
        }
    }

    fn distr_u64(&mut self, range: u64, zone: u64) -> u64 {
        loop {
            let (hi, lo) = wide_mul_u64(self.next_u64(), range);
            if lo <= zone {
                return hi;
            }
        }
    }

    fn generate_and_set(&mut self, index: usize) {
        self.generate();
        self.index = index;
    }

    fn generate(&mut self) {
        for block in 0..4 {
            let counter = self.block_pos.wrapping_add(block as u64);
            let initial = [
                CONSTANTS[0],
                CONSTANTS[1],
                CONSTANTS[2],
                CONSTANTS[3],
                self.key[0],
                self.key[1],
                self.key[2],
                self.key[3],
                self.key[4],
                self.key[5],
                self.key[6],
                self.key[7],
                counter as u32,
                (counter >> 32) as u32,
                0,
                0,
            ];
            let mut state = initial;
            for _ in 0..4 {
                quarter(&mut state, 0, 4, 8, 12);
                quarter(&mut state, 1, 5, 9, 13);
                quarter(&mut state, 2, 6, 10, 14);
                quarter(&mut state, 3, 7, 11, 15);
                quarter(&mut state, 0, 5, 10, 15);
                quarter(&mut state, 1, 6, 11, 12);
                quarter(&mut state, 2, 7, 8, 13);
                quarter(&mut state, 3, 4, 9, 14);
            }
            let out = &mut self.buf[block * BLOCK_WORDS..(block + 1) * BLOCK_WORDS];
            for ((slot, word), start) in out.iter_mut().zip(state).zip(initial) {
                *slot = word.wrapping_add(start);
            }
        }
        self.block_pos = self.block_pos.wrapping_add(4);
    }
}

fn quarter(s: &mut [u32; 16], a: usize, b: usize, c: usize, d: usize) {
    s[a] = s[a].wrapping_add(s[b]);
    s[d] = (s[d] ^ s[a]).rotate_left(16);
    s[c] = s[c].wrapping_add(s[d]);
    s[b] = (s[b] ^ s[c]).rotate_left(12);
    s[a] = s[a].wrapping_add(s[b]);
    s[d] = (s[d] ^ s[a]).rotate_left(8);
    s[c] = s[c].wrapping_add(s[d]);
    s[b] = (s[b] ^ s[c]).rotate_left(7);
}

fn pcg32(state: &mut u64) -> [u8; 4] {
    const MUL: u64 = 6364136223846793005;
    const INC: u64 = 11634580027462260723;
    *state = state.wrapping_mul(MUL).wrapping_add(INC);
    let s = *state;
    let xorshifted = (((s >> 18) ^ s) >> 27) as u32;
    let rot = (s >> 59) as u32;
    xorshifted.rotate_right(rot).to_le_bytes()
}

fn wide_mul_u64(a: u64, b: u64) -> (u64, u64) {
    let t = u128::from(a) * u128::from(b);
    ((t >> 64) as u64, t as u64)
}

fn wide_mul_u32(a: u32, b: u32) -> (u32, u32) {
    let t = u64::from(a) * u64::from(b);
    ((t >> 32) as u32, t as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn words_match_rand_chacha() {
        let mut c = ChaCha8::from_u64(7);
        let words: Vec<u32> = (0..8).map(|_| c.next_u32()).collect();
        assert_eq!(
            words,
            [
                601310139, 677729076, 781920570, 721508819, 1160025393, 3024842937, 155073251,
                3121330102
            ]
        );
        let doubles: Vec<u64> = (0..4).map(|_| c.next_u64()).collect();
        assert_eq!(
            doubles,
            [
                11091271176959810440,
                6629102542470643238,
                1532177833826564251,
                15666632647712377470
            ]
        );
        let mut z = ChaCha8::from_u64(0);
        let zwords: Vec<u32> = (0..8).map(|_| z.next_u32()).collect();
        assert_eq!(
            zwords,
            [
                2811902828, 3045455719, 3134767159, 2001118559, 2179114726, 3002797362, 2409334908,
                258433188
            ]
        );
    }

    #[test]
    fn distributions_match_rand() {
        let mut c = ChaCha8::from_u64(7);
        let units: Vec<u64> = (0..4).map(|_| c.unit().to_bits()).collect();
        assert_eq!(
            units,
            [
                4594853223840476064,
                4595220474943196564,
                4604518774960858258,
                4604721123211421639
            ]
        );
        let bools: Vec<bool> = (0..8).map(|_| c.boolean()).collect();
        assert_eq!(bools, [false, true, true, false, false, false, false, true]);
        let ranges: Vec<i64> = (0..8).map(|_| c.range_i64(-1000, 1000)).collect();
        assert_eq!(ranges, [-271, 980, -600, -232, 43, -486, -147, 53]);
        let belows: Vec<u64> = (0..8).map(|_| c.below_u64(97)).collect();
        assert_eq!(belows, [29, 7, 31, 21, 42, 79, 2, 7]);
    }

    #[test]
    fn sequences_match_rand() {
        let mut s = ChaCha8::from_u64(7);
        let mut perm: Vec<usize> = (0..16).collect();
        s.shuffle(&mut perm);
        assert_eq!(perm, [11, 4, 9, 8, 10, 1, 12, 6, 5, 7, 0, 3, 13, 14, 15, 2]);
        let floyd = ChaCha8::from_u64(7).sample_indices(100, 5);
        assert_eq!(floyd, [13, 15, 16, 26, 70]);
        let inplace = ChaCha8::from_u64(7).sample_indices(100, 20);
        assert_eq!(
            inplace,
            [14, 16, 18, 29, 71, 8, 74, 13, 63, 41, 17, 86, 43, 99, 53, 32, 48, 62, 91, 0]
        );
        let rejection = ChaCha8::from_u64(7).sample_indices(100_000, 163);
        assert_eq!(
            &rejection[..10],
            [14000, 15779, 18205, 16798, 27008, 70427, 3610, 72674, 7116, 60125]
        );
    }

    #[test]
    fn word_pos_seeks_the_stream() {
        let mut c = ChaCha8::from_u64(7);
        assert_eq!(c.word_pos(), 0);
        for _ in 0..70 {
            c.next_u32();
        }
        assert_eq!(c.word_pos(), 70);
        c.next_u64();
        assert_eq!(c.word_pos(), 72);
        c.set_word_pos(5);
        let a = c.next_u64();
        c.set_word_pos(5);
        assert_eq!(c.next_u64(), a);
    }
}
