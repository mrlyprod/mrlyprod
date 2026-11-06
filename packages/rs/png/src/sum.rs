const CRC: [[u32; 256]; 8] = tables();

const fn tables() -> [[u32; 256]; 8] {
    let mut out = [[0u32; 256]; 8];
    let mut n = 0;
    while n < 256 {
        let mut value = n as u32;
        let mut bit = 0;
        while bit < 8 {
            value = if value & 1 == 1 {
                0xEDB8_8320 ^ (value >> 1)
            } else {
                value >> 1
            };
            bit += 1;
        }
        out[0][n] = value;
        n += 1;
    }
    let mut n = 0;
    while n < 256 {
        let mut value = out[0][n];
        let mut table = 1;
        while table < 8 {
            value = out[0][(value & 0xFF) as usize] ^ (value >> 8);
            out[table][n] = value;
            table += 1;
        }
        n += 1;
    }
    out
}

pub fn crc32(bytes: &[u8]) -> u32 {
    let mut crc = !0u32;
    let mut words = bytes.chunks_exact(8);
    for word in &mut words {
        let low = u32::from_le_bytes([word[0], word[1], word[2], word[3]]) ^ crc;
        let high = u32::from_le_bytes([word[4], word[5], word[6], word[7]]);
        crc = CRC[7][(low & 0xFF) as usize]
            ^ CRC[6][(low >> 8 & 0xFF) as usize]
            ^ CRC[5][(low >> 16 & 0xFF) as usize]
            ^ CRC[4][(low >> 24) as usize]
            ^ CRC[3][(high & 0xFF) as usize]
            ^ CRC[2][(high >> 8 & 0xFF) as usize]
            ^ CRC[1][(high >> 16 & 0xFF) as usize]
            ^ CRC[0][(high >> 24) as usize];
    }
    for &byte in words.remainder() {
        crc = CRC[0][((crc ^ byte as u32) & 0xFF) as usize] ^ (crc >> 8);
    }
    !crc
}

pub fn adler32(bytes: &[u8]) -> u32 {
    let (mut a, mut b) = (1u32, 0u32);
    for chunk in bytes.chunks(5552) {
        let mut words = chunk.chunks_exact(16);
        for word in &mut words {
            let (mut sum, mut ramp) = (0u32, 0u32);
            for &byte in word {
                sum += byte as u32;
                ramp += sum;
            }
            b += 16 * a + ramp;
            a += sum;
        }
        for &byte in words.remainder() {
            a += byte as u32;
            b += a;
        }
        a %= 65521;
        b %= 65521;
    }
    b << 16 | a
}
