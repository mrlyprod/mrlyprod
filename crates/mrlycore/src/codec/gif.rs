use crate::errors::{value_error, MrlyError, Result};
use crate::resample::block;
use gif::{DisposalMethod, Encoder, Frame, Repeat};
use std::borrow::Cow;

impl From<gif::EncodingError> for MrlyError {
    fn from(error: gif::EncodingError) -> MrlyError {
        MrlyError::Value(error.to_string())
    }
}

/// Encodes indexed frames as an animated gif89a, each source pixel a scale by scale block.
///
/// The frames index one shared palette, the delay is in hundredths of a second, the
/// animation loops forever, and the first fully transparent palette entry becomes the
/// frame's transparent color.
///
/// ```
/// let still = [0u8, 1, 1, 0];
/// let flip = [1u8, 0, 0, 1];
/// let palette = [[0, 0, 0, 255], [255, 255, 255, 255]];
/// let bytes = mrlycore::gif(&[&still[..], &flip[..]], &palette, 2, 2, 3, 8).unwrap();
/// assert_eq!(&bytes[0..6], b"GIF89a");
/// assert_eq!(bytes[bytes.len() - 1], 0x3b);
/// ```
pub fn gif(
    frames: &[&[u8]],
    palette: &[[u8; 4]],
    width: usize,
    height: usize,
    scale: usize,
    delay: usize,
) -> Result<Vec<u8>> {
    if scale < 1 {
        return value_error("scale must be at least 1.");
    }
    if width == 0 || height == 0 {
        return value_error("width and height must be at least 1.");
    }
    if frames.is_empty() {
        return value_error("gif needs at least one frame.");
    }
    if palette.is_empty() || palette.len() > 256 {
        return value_error("palette must hold 1 to 256 colors.");
    }
    let (out_w, out_h) = (width * scale, height * scale);
    if out_w > u16::MAX as usize || out_h > u16::MAX as usize {
        return value_error("gif size must fit in 16 bits.");
    }
    for frame in frames {
        if frame.len() != width * height {
            return value_error("every frame must hold width * height indices.");
        }
        if frame.iter().any(|&i| i as usize >= palette.len()) {
            return value_error("frame index out of palette range.");
        }
    }
    let table: Vec<u8> = palette
        .iter()
        .flat_map(|c| c[..3].iter().copied())
        .collect();
    let transparent = palette.iter().position(|c| c[3] == 0).map(|i| i as u8);
    let dispose = match transparent {
        Some(_) => DisposalMethod::Background,
        None => DisposalMethod::Keep,
    };
    let out = Vec::with_capacity(frames.len() * out_w * out_h / 2 + 1024);
    let mut encoder = Encoder::new(out, out_w as u16, out_h as u16, &table)?;
    encoder.set_repeat(Repeat::Infinite)?;
    for frame in frames {
        encoder.write_frame(&Frame {
            delay: delay.min(u16::MAX as usize) as u16,
            dispose,
            transparent,
            width: out_w as u16,
            height: out_h as u16,
            buffer: Cow::Owned(block(frame, width, height, scale)),
            ..Frame::default()
        })?;
    }
    Ok(encoder.into_inner()?)
}

#[cfg(test)]
mod tests {
    use super::*;
    use gif::{ColorOutput, DecodeOptions};
    struct Gif {
        width: usize,
        height: usize,
        palette: Vec<[u8; 3]>,
        repeat: Repeat,
        frames: Vec<Frame<'static>>,
    }
    fn ungif(bytes: &[u8]) -> Gif {
        assert_eq!(&bytes[0..6], b"GIF89a");
        assert_eq!(bytes[bytes.len() - 1], 0x3b);
        let mut options = DecodeOptions::new();
        options.set_color_output(ColorOutput::Indexed);
        let decoder = options.read_info(bytes).unwrap();
        let palette = decoder
            .global_palette()
            .unwrap()
            .chunks(3)
            .map(|c| [c[0], c[1], c[2]])
            .collect();
        Gif {
            width: decoder.width() as usize,
            height: decoder.height() as usize,
            palette,
            repeat: decoder.repeat(),
            frames: decoder.into_iter().map(|f| f.unwrap()).collect(),
        }
    }
    #[test]
    fn gif_round_trips_indexed_frames() {
        let first = [0u8, 1, 2, 1, 2, 0, 1, 0, 1, 1, 0, 2];
        let second = [2u8, 2, 0, 0, 1, 2, 1, 2, 0, 1, 1, 0];
        let palette = [[255, 0, 0, 255], [0, 255, 0, 255], [0, 0, 255, 0]];
        let bytes = gif(&[&first[..], &second[..]], &palette, 4, 3, 1, 5).unwrap();
        let out = ungif(&bytes);
        assert_eq!((out.width, out.height), (4, 3));
        assert_eq!(out.palette.len(), 4);
        assert_eq!(&out.palette[..3], &[[255, 0, 0], [0, 255, 0], [0, 0, 255]]);
        assert_eq!(out.frames.len(), 2);
        assert_eq!(&out.frames[0].buffer[..], &first);
        assert_eq!(&out.frames[1].buffer[..], &second);
    }
    #[test]
    fn gif_scales_every_pixel_into_a_block() {
        let frame = [0u8, 1, 1, 0];
        let palette = [[0, 0, 0, 255], [255, 255, 255, 255]];
        let bytes = gif(&[&frame[..]], &palette, 2, 2, 3, 0).unwrap();
        let out = ungif(&bytes);
        assert_eq!((out.width, out.height), (6, 6));
        assert_eq!(out.frames.len(), 1);
        assert_eq!(out.frames[0].buffer.len(), 36);
        for y in 0..6 {
            for x in 0..6 {
                let want = frame[(y / 3) * 2 + x / 3];
                assert_eq!(out.frames[0].buffer[y * 6 + x], want, "pixel {x},{y}");
            }
        }
    }
    #[test]
    fn gif_carries_delay_loop_and_transparency() {
        let frame = [0u8, 1];
        let palette = [[1, 2, 3, 255], [4, 5, 6, 0]];
        let bytes = gif(&[&frame[..], &frame[..]], &palette, 2, 1, 1, 7).unwrap();
        let out = ungif(&bytes);
        assert_eq!(out.palette, [[1, 2, 3], [4, 5, 6]]);
        assert_eq!(out.repeat, Repeat::Infinite);
        assert_eq!(out.frames.len(), 2);
        for frame in &out.frames {
            assert_eq!(frame.delay, 7);
            assert_eq!(frame.transparent, Some(1));
            assert_eq!(frame.dispose, DisposalMethod::Background);
            assert_eq!((frame.width, frame.height), (2, 1));
        }
        let opaque = gif(&[&frame[..]], &[[1, 2, 3, 255], [4, 5, 6, 255]], 2, 1, 1, 7).unwrap();
        let out = ungif(&opaque);
        assert_eq!(out.frames[0].transparent, None);
        assert_eq!(out.frames[0].dispose, DisposalMethod::Keep);
        let slow = gif(&[&frame[..]], &palette, 2, 1, 1, 1 << 20).unwrap();
        assert_eq!(ungif(&slow).frames[0].delay, u16::MAX);
    }
    #[test]
    fn gif_round_trips_a_full_palette() {
        let mut random = crate::chacha::ChaCha8::from_u64(11);
        let palette: Vec<[u8; 4]> = (0..256).map(|i| [i as u8, 9, 9, 255]).collect();
        let frame: Vec<u8> = (0..128 * 128).map(|_| random.next_u32() as u8).collect();
        let bytes = gif(&[&frame[..]], &palette, 128, 128, 1, 4).unwrap();
        let out = ungif(&bytes);
        assert_eq!((out.width, out.height), (128, 128));
        assert_eq!(out.palette.len(), 256);
        assert_eq!(out.palette[200], [200, 9, 9]);
        assert_eq!(&out.frames[0].buffer[..], &frame[..]);
    }
    #[test]
    fn gif_compresses_flat_frames() {
        let frame = vec![3u8; 256 * 256];
        let palette: Vec<[u8; 4]> = (0..8).map(|i| [i as u8 * 30, 0, 0, 255]).collect();
        let bytes = gif(&[&frame[..]], &palette, 256, 256, 1, 4).unwrap();
        assert!(bytes.len() < 2000, "flat frame took {} bytes", bytes.len());
        assert_eq!(&ungif(&bytes).frames[0].buffer[..], &frame[..]);
    }
    #[test]
    fn gif_rejects_bad_inputs() {
        let frame = [0u8, 1, 1, 0];
        let palette = [[0, 0, 0, 255], [255, 255, 255, 255]];
        assert!(gif(&[&frame[..]], &palette, 2, 2, 0, 5).is_err());
        assert!(gif(&[], &palette, 2, 2, 1, 5).is_err());
        assert!(gif(&[&frame[..]], &[], 2, 2, 1, 5).is_err());
        assert!(gif(&[&frame[..]], &palette, 3, 2, 1, 5).is_err());
        assert!(gif(&[&frame[..]], &palette, 2, 0, 1, 5).is_err());
        assert!(gif(&[&[0u8, 1, 2, 0][..]], &palette, 2, 2, 1, 5).is_err());
        assert!(gif(&[&frame[..]], &palette, 2, 2, 40000, 5).is_err());
    }
}
