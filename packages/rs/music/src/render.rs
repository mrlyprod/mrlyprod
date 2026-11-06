use crate::{Piece, Result, compose};
use mrly_audio::{MOST, RATE, hertz, mix, normalize, samples, tone};
use std::collections::HashMap;

pub fn render(piece: &Piece, parts: &[Vec<Vec<i64>>]) -> Result<Vec<f32>> {
    if parts.len() != piece.voices.len() {
        return Err(format!(
            "{} parts do not match {} voices.",
            parts.len(),
            piece.voices.len()
        ));
    }
    let longest = parts.iter().map(Vec::len).max().unwrap_or(0);
    let size = size(piece.beat, longest as f64)?;
    let mut out = vec![0.0f32; longest * size];
    for (voice, part) in piece.voices.iter().zip(parts) {
        let mut cache: HashMap<Vec<i64>, Vec<f32>> = HashMap::new();
        for (at, notes) in part.iter().enumerate() {
            if notes.is_empty() {
                continue;
            }
            if !cache.contains_key(notes) {
                let tones: Vec<Vec<f32>> = notes
                    .iter()
                    .map(|&midi| tone(hertz(midi), &voice.timbre, piece.beat))
                    .collect::<Result<_>>()?;
                cache.insert(notes.clone(), voice.envelope.apply(&mix(&tones)));
            }
            for (s, t) in out[at * size..].iter_mut().zip(&cache[notes]) {
                *s += t;
            }
        }
    }
    Ok(normalize(&out, 1.0))
}

pub fn wav(piece: &Piece, seconds: f64, seed: u64) -> Result<Vec<u8>> {
    let count = (positive("length", seconds)? / piece.beat).ceil();
    let parts = compose(piece, count as usize, seed)?;
    mrly_wav::encode(&render(piece, &parts)?, 1, RATE, 16)
}

const FRAMES: usize = 1 << 20;

pub(crate) fn size(beat: f64, frames: f64) -> Result<usize> {
    let size = samples(positive("beat", beat)?);
    if size == 0 {
        return Err(format!("beat {beat} is shorter than one sample."));
    }
    if frames > FRAMES as f64 || frames * size as f64 > MOST as f64 {
        return Err(format!(
            "a piece holds at most {FRAMES} frames and {MOST} samples."
        ));
    }
    Ok(size)
}

fn positive(what: &str, value: f64) -> Result<f64> {
    if value.is_finite() && value > 0.0 {
        Ok(value)
    } else {
        Err(format!(
            "{what} {value} is not a positive number of seconds."
        ))
    }
}
