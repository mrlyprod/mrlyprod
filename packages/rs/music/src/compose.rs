use crate::render::size;
use crate::scale::degrees;
use crate::{BLUES, ChordType, Movement, Piece, ROOT, Result, Scale, Voice, chord, pool};
use mrly_audio::{Timbre, Wave};
use mrly_rand::Rng;
use std::collections::HashMap;

pub fn compose(piece: &Piece, count: usize, seed: u64) -> Result<Vec<Vec<Vec<i64>>>> {
    let degrees = degrees(&piece.progression)?;
    if degrees.is_empty() {
        return Err("a progression needs at least one chord.".into());
    }
    if piece.bar == 0 {
        return Err("a bar needs at least one frame.".into());
    }
    if piece.voices.is_empty() {
        return Err("a piece needs at least one voice.".into());
    }
    for voice in &piece.voices {
        if voice.pool.is_empty() || voice.movements.is_empty() || voice.counts.is_empty() {
            return Err("every voice needs notes, movements and counts.".into());
        }
    }
    size(piece.beat, count as f64)?;
    size(piece.beat, piece.bar as f64 * degrees.len() as f64)?;
    let mut rng = Rng::new(seed);
    let mut bases: Vec<Vec<Vec<i64>>> = vec![Vec::new(); piece.voices.len()];
    let mut library: HashMap<(usize, usize), Vec<Vec<i64>>> = HashMap::new();
    for &degree in &degrees {
        for (at, voice) in piece.voices.iter().enumerate() {
            let bar = match library.get(&(at, degree)) {
                Some(bar) => bar.clone(),
                None => {
                    let bar = phrase(&mut rng, voice, degree, piece.bar);
                    if piece.repeat {
                        library.insert((at, degree), bar.clone());
                    }
                    bar
                }
            };
            bases[at].extend(bar);
        }
    }
    Ok(bases
        .iter()
        .map(|base| (0..count).map(|i| base[i % base.len()].clone()).collect())
        .collect())
}

pub fn frames(parts: &[Vec<Vec<i64>>]) -> Vec<Vec<i64>> {
    let longest = parts.iter().map(Vec::len).max().unwrap_or(0);
    (0..longest)
        .map(|i| {
            parts
                .iter()
                .filter_map(|part| part.get(i))
                .flatten()
                .copied()
                .collect()
        })
        .collect()
}

pub fn walk(pool: &[i64], target: &[i64], length: usize, step: i64) -> Result<Vec<Vec<i64>>> {
    let at: Vec<i64> = target
        .iter()
        .map(|note| {
            pool.iter()
                .position(|n| n == note)
                .map(|i| i as i64)
                .ok_or_else(|| format!("note {note} is not in the pool."))
        })
        .collect::<Result<_>>()?;
    let size = pool.len().max(1) as i64;
    let step = step.rem_euclid(size);
    Ok((0..length as i64)
        .map(|i| {
            let shift = (i - length as i64 + 1) * step;
            at.iter()
                .map(|&from| pool[(from + shift).rem_euclid(size) as usize])
                .collect()
        })
        .collect())
}

pub fn random(scale: Scale, seed: u64) -> Piece {
    let mut rng = Rng::new(seed);
    let progression = if rng.chance(0.25) {
        BLUES.to_string()
    } else {
        letters(&mut rng)
    };
    let octave = |from: i64, octaves: usize| pool(scale, ROOT + 12 * from, octaves);
    let timbre = Timbre::new(Wave::Triangle, [1, 3][rng.below(2)]);
    let kind = |rng: &mut Rng| Some([ChordType::Triad, ChordType::Seventh][rng.below(2)]);
    let mut bass = Voice::new(
        octave(0, 1 + rng.below(2)),
        vec![Movement::Repeat, Movement::Up, Movement::Down],
    );
    bass.chord = kind(&mut rng);
    let mut rhythm = Voice::new(
        octave(1, 1 + rng.below(2)),
        vec![Movement::Repeat, Movement::Random],
    );
    rhythm.chord = kind(&mut rng);
    rhythm.counts = vec![2, 3];
    let mut voices = vec![bass, rhythm];
    if rng.boolean() {
        voices.push(Voice::new(octave(2, 1), Movement::all().to_vec()));
    }
    for voice in &mut voices {
        voice.timbre = timbre;
    }
    Piece::new(&progression, voices)
}

fn phrase(rng: &mut Rng, voice: &Voice, degree: usize, length: usize) -> Vec<Vec<i64>> {
    let pool = match voice.chord {
        Some(kind) => chord(&voice.pool, degree, kind),
        None => voice.pool.clone(),
    };
    let first = draw(rng, &pool, &voice.counts);
    let mut last = first.clone();
    let mut frames = vec![first];
    for _ in 1..length {
        let notes = match voice.movements[rng.below(voice.movements.len())] {
            Movement::Repeat => last.clone(),
            Movement::Random => draw(rng, &pool, &voice.counts),
            Movement::Up => step(&pool, &last, 1),
            Movement::Down => step(&pool, &last, -1),
            Movement::Pause => Vec::new(),
        };
        if !notes.is_empty() {
            last.clone_from(&notes);
        }
        frames.push(notes);
    }
    frames
}

fn letters(rng: &mut Rng) -> String {
    let chords = 1 << rng.below(4);
    (0..chords)
        .flat_map(|_| {
            let letter = char::from(b"CDEFGAB"[rng.below(7)]);
            std::iter::repeat_n(letter, 8 / chords)
        })
        .collect()
}

fn draw(rng: &mut Rng, pool: &[i64], counts: &[usize]) -> Vec<i64> {
    let count = counts[rng.below(counts.len())];
    rng.sample_indices(pool.len(), count)
        .into_iter()
        .map(|i| pool[i])
        .collect()
}

fn step(pool: &[i64], notes: &[i64], by: i64) -> Vec<i64> {
    let size = pool.len() as i64;
    notes
        .iter()
        .map(|note| match pool.iter().position(|n| n == note) {
            Some(at) => pool[(at as i64 + by).rem_euclid(size) as usize],
            None => *note,
        })
        .collect()
}
