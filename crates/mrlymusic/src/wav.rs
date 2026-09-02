/// Encodes mono 16-bit pcm samples at a sample rate as a riff wave file.
///
/// ```
/// let bytes = mrlymusic::wav(&[0i16; 4], 44100);
/// assert_eq!(bytes.len(), 52);
/// assert_eq!(&bytes[0..4], b"RIFF");
/// ```
pub fn wav(samples: &[i16], rate: usize) -> Vec<u8> {
    let data = samples.len() * 2;
    let mut out = Vec::with_capacity(44 + data);
    out.extend_from_slice(b"RIFF");
    out.extend_from_slice(&(36 + data as u32).to_le_bytes());
    out.extend_from_slice(b"WAVE");
    out.extend_from_slice(b"fmt ");
    out.extend_from_slice(&16u32.to_le_bytes());
    out.extend_from_slice(&1u16.to_le_bytes());
    out.extend_from_slice(&1u16.to_le_bytes());
    out.extend_from_slice(&(rate as u32).to_le_bytes());
    out.extend_from_slice(&(rate as u32 * 2).to_le_bytes());
    out.extend_from_slice(&2u16.to_le_bytes());
    out.extend_from_slice(&16u16.to_le_bytes());
    out.extend_from_slice(b"data");
    out.extend_from_slice(&(data as u32).to_le_bytes());
    for &sample in samples {
        out.extend_from_slice(&sample.to_le_bytes());
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn wav_lays_out_the_header() {
        let bytes = wav(&[0, 1, -1], 44100);
        assert_eq!(bytes.len(), 50);
        assert_eq!(&bytes[0..4], b"RIFF");
        assert_eq!(&bytes[4..8], &42u32.to_le_bytes());
        assert_eq!(&bytes[8..12], b"WAVE");
        assert_eq!(&bytes[12..16], b"fmt ");
        assert_eq!(&bytes[16..20], &16u32.to_le_bytes());
        assert_eq!(&bytes[20..22], &1u16.to_le_bytes());
        assert_eq!(&bytes[22..24], &1u16.to_le_bytes());
        assert_eq!(&bytes[24..28], &44100u32.to_le_bytes());
        assert_eq!(&bytes[28..32], &88200u32.to_le_bytes());
        assert_eq!(&bytes[32..34], &2u16.to_le_bytes());
        assert_eq!(&bytes[34..36], &16u16.to_le_bytes());
        assert_eq!(&bytes[36..40], b"data");
        assert_eq!(&bytes[40..44], &6u32.to_le_bytes());
        assert_eq!(&bytes[44..], &[0, 0, 1, 0, 255, 255]);
    }
    #[test]
    fn wav_sizes_track_the_sample_count() {
        let empty = wav(&[], 8000);
        assert_eq!(empty.len(), 44);
        assert_eq!(&empty[4..8], &36u32.to_le_bytes());
        assert_eq!(&empty[40..44], &0u32.to_le_bytes());
        let hundred = wav(&[7; 100], 8000);
        assert_eq!(hundred.len(), 244);
        assert_eq!(&hundred[4..8], &236u32.to_le_bytes());
        assert_eq!(&hundred[24..28], &8000u32.to_le_bytes());
        assert_eq!(&hundred[28..32], &16000u32.to_le_bytes());
        assert_eq!(&hundred[40..44], &200u32.to_le_bytes());
    }
}
