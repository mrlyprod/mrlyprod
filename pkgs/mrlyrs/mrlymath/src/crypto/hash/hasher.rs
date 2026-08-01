use super::config::Config;
use super::sponge::sponge_hash;
use mrlycore::errors::Result;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Digest {
    pub bits: Vec<u8>,
}

impl Digest {
    pub fn to_bytes(&self) -> Vec<u8> {
        self.bits
            .chunks(8)
            .map(|chunk| {
                let mut byte = 0u8;
                for (k, &b) in chunk.iter().enumerate() {
                    byte |= (b & 1) << (7 - k);
                }
                byte
            })
            .collect()
    }
    pub fn hex(&self) -> String {
        self.to_bytes().iter().map(|b| format!("{b:02x}")).collect()
    }
}

pub fn digest(message: &[u8], config: &Config) -> Result<Digest> {
    Ok(Digest {
        bits: sponge_hash(message, config)?,
    })
}

pub fn hexdigest(message: &[u8], config: &Config) -> Result<String> {
    Ok(digest(message, config)?.hex())
}

pub fn keyed_hexdigest(key: &[u8], message: &[u8], config: &Config) -> Result<String> {
    let mut buf = Vec::with_capacity(key.len() + message.len());
    buf.extend_from_slice(key);
    buf.extend_from_slice(message);
    hexdigest(&buf, config)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn hex_length_matches_digest_bits() {
        let cfg = Config::default();
        let h = hexdigest(b"hello", &cfg).unwrap();
        assert_eq!(h.len(), cfg.digest_bits / 4);
    }
    #[test]
    fn different_inputs_differ() {
        let cfg = Config::default();
        assert_ne!(
            hexdigest(b"hello", &cfg).unwrap(),
            hexdigest(b"hellp", &cfg).unwrap()
        );
    }
    #[test]
    fn keyed_depends_on_key() {
        let cfg = Config::default();
        let a = keyed_hexdigest(b"key1", b"msg", &cfg).unwrap();
        let b = keyed_hexdigest(b"key2", b"msg", &cfg).unwrap();
        assert_ne!(a, b);
    }
    #[test]
    fn bytes_round_trip_bits() {
        let d = Digest {
            bits: vec![1, 0, 1, 0, 0, 0, 0, 1, 1, 1, 1, 1, 0, 0, 0, 0],
        };
        assert_eq!(d.to_bytes(), vec![0b10100001, 0b11110000]);
        assert_eq!(d.hex(), "a1f0");
    }
}
