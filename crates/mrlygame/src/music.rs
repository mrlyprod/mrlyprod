use crate::config::{FPS, FREEZE_US, HEATMAP_FPS, INTER_FREEZE_US};
use crate::Way;
use mrlycore::state::{boolean, choice};
use mrlymusic::audio::{Timbre, Wave, MAJOR, ROOT};
use mrlymusic::music::{self, ChordType, Movement, Voice};

/// The twelve-bar blues progression every Conway segment plays.
pub const BLUES: &str = "CCCCFFCCGFCG";

const BAR: usize = 4;
const PEAK: f32 = 1.0;

/// One segment's musical styling: progression, voices and timbre.
#[derive(Clone, Debug)]
pub struct Score {
    /// The chord letters, one beat each.
    pub progression: String,
    /// The voices walking the progression.
    pub voices: Vec<Voice>,
    /// The timbre every voice renders with.
    pub timbre: Timbre,
}

fn octave(offset: i64) -> Vec<i64> {
    MAJOR.iter().map(|i| ROOT + i + offset).collect()
}

fn build_voices() -> Vec<Voice> {
    let lows = octave(0);
    let mids = octave(12);
    let highs = octave(24);
    let mut bass = Voice::new(
        if boolean() {
            lows.clone()
        } else {
            [lows, mids.clone()].concat()
        },
        vec![Movement::Repeat, Movement::Up, Movement::Down],
    );
    bass.chord_type = Some(choice(&[ChordType::Triad, ChordType::Seventh]));
    let mut rhythm = Voice::new(
        if boolean() {
            mids.clone()
        } else {
            [mids, highs.clone()].concat()
        },
        vec![Movement::Repeat, Movement::Random],
    );
    rhythm.chord_type = Some(choice(&[ChordType::Triad, ChordType::Seventh]));
    rhythm.num_notes = vec![2, 3];
    let mut voices = vec![bass, rhythm];
    if boolean() {
        voices.push(Voice::new(
            highs,
            vec![
                Movement::Repeat,
                Movement::Random,
                Movement::Up,
                Movement::Down,
                Movement::Pause,
            ],
        ));
    }
    voices
}

fn build_progression(way: Way) -> String {
    if way == Way::Conway {
        return BLUES.to_string();
    }
    let rhythms = [vec![8], vec![4, 4], vec![2, 2, 2, 2], vec![1; 8]];
    let rhythm = choice(&rhythms);
    let letters = ['C', 'D', 'E', 'F', 'G', 'A', 'B'];
    let chords: Vec<char> = (0..rhythm.len()).map(|_| choice(&letters)).collect();
    let mut out = String::new();
    for (i, beats) in rhythm.iter().enumerate() {
        for _ in 0..*beats {
            out.push(chords[i]);
        }
    }
    out
}

/// Draws a segment's score under its way: the blues under Conway, a drawn rhythm otherwise.
pub fn score(way: Way) -> Score {
    let progression = build_progression(way);
    let voices = build_voices();
    let shape = choice(&[Wave::Sine, Wave::Triangle]);
    let harmonics = choice(&[1usize, 3]);
    Score {
        progression,
        voices,
        timbre: Timbre::new(shape, Wave::Triangle, harmonics),
    }
}

/// Times a chord track with one beat per chord and freezes on the held first and last chords.
pub fn assemble(track: &[Vec<i64>], beat: f32, open: f32, close: f32) -> Vec<(Vec<i64>, f32)> {
    let Some(first) = track.first() else {
        return Vec::new();
    };
    let mut timed = Vec::with_capacity(track.len() + 2);
    if open > 0.0 {
        timed.push((first.clone(), open));
    }
    for chord in track {
        timed.push((chord.clone(), beat));
    }
    if close > 0.0 {
        timed.push((track.last().expect("a first implies a last").clone(), close));
    }
    timed
}

/// Steps the opening chord through a three-octave pool and returns the reversed trail.
pub fn heatmap_track(first: &[i64], count: usize) -> Vec<Vec<i64>> {
    if count == 0 {
        return Vec::new();
    }
    let mut pool = Vec::new();
    for offset in [0, 12, 24] {
        pool.extend(octave(offset));
    }
    let step: i64 = if choice(&["up", "down"]) == "up" {
        -1
    } else {
        1
    };
    let shift = |notes: &[i64]| -> Vec<i64> {
        notes
            .iter()
            .map(|n| {
                let at = pool.iter().position(|p| p == n).unwrap_or(0) as i64;
                pool[(at + step).rem_euclid(pool.len() as i64) as usize]
            })
            .collect()
    };
    let mut current = shift(first);
    let mut track = vec![current.clone()];
    for _ in 1..count {
        current = shift(&current);
        track.push(current.clone());
    }
    track.reverse();
    track
}

/// Splits the halved frame budget across segments by cumulative halving, without drift.
pub fn halve(lengths: &[usize]) -> Vec<usize> {
    let mut out = Vec::with_capacity(lengths.len());
    let mut frames = 0;
    let mut halved = 0;
    for &length in lengths {
        frames += length;
        let expected = frames / 2;
        out.push(expected - halved);
        halved = expected;
    }
    out
}

fn seconds(us: u64) -> f32 {
    us as f32 / 1_000_000.0
}

/// Renders the quest's two-part audio: the frames half, then the heatmap half.
pub fn soundtrack(scores: &[Score], lengths: &[usize]) -> Vec<f32> {
    let heat_lengths = halve(lengths);
    let last = scores.len().saturating_sub(1);
    let mut frames_half = Vec::new();
    let mut heat_half = Vec::new();
    for (i, (score, &count)) in scores.iter().zip(lengths).enumerate() {
        let track = music::compose(&score.progression, &score.voices, BAR, count, false);
        let open = if i == 0 { seconds(FREEZE_US) } else { 0.0 };
        let close = if i == last {
            seconds(FREEZE_US)
        } else {
            seconds(INTER_FREEZE_US)
        };
        let timed = assemble(&track, 1.0 / FPS as f32, open, close);
        frames_half.extend(music::track(&timed, &score.timbre, PEAK));
        let first = track.first().cloned().unwrap_or_default();
        let heat = heatmap_track(&first, heat_lengths[i]);
        let heat_timed = assemble(&heat, 2.0 / HEATMAP_FPS as f32, open, close);
        heat_half.extend(music::track(&heat_timed, &score.timbre, PEAK));
    }
    frames_half.extend(heat_half);
    frames_half
}

#[cfg(test)]
mod tests {
    use super::*;
    use mrlycore::state::{guard, seed};
    #[test]
    fn conway_plays_the_blues() {
        let _g = guard();
        seed(1);
        assert_eq!(score(Way::Conway).progression, BLUES);
    }
    #[test]
    fn mrly_draws_an_eight_beat_progression() {
        let _g = guard();
        for s in 0..20 {
            seed(s);
            let drawn = score(Way::Mrly);
            assert_eq!(drawn.progression.len(), 8);
            assert!(drawn.progression.chars().all(|c| "CDEFGAB".contains(c)));
        }
    }
    #[test]
    fn conway_voices_hold_the_chord_and_note_shapes() {
        let _g = guard();
        for s in 0..20 {
            seed(s);
            let drawn = score(Way::Conway);
            assert!(drawn.voices.len() == 2 || drawn.voices.len() == 3);
            assert!(drawn.voices[0].chord_type.is_some());
            assert_eq!(drawn.voices[1].num_notes, vec![2, 3]);
            assert!(drawn.timbre.harmonics == 1 || drawn.timbre.harmonics == 3);
        }
    }
    #[test]
    fn assembly_freezes_the_held_ends() {
        let track = vec![vec![60], vec![62], vec![64]];
        let timed = assemble(&track, 0.125, 0.5, 0.5);
        assert_eq!(timed.len(), 5);
        assert_eq!(timed[0], (vec![60], 0.5));
        assert_eq!(timed[4], (vec![64], 0.5));
        let bare = assemble(&track, 0.125, 0.0, 0.5);
        assert_eq!(bare.len(), 4);
        assert!(assemble(&[], 0.125, 0.5, 0.5).is_empty());
    }
    #[test]
    fn the_heatmap_trail_reverses_a_steady_walk() {
        let _g = guard();
        seed(5);
        let trail = heatmap_track(&[ROOT, ROOT + 12], 6);
        assert_eq!(trail.len(), 6);
        let pool: Vec<i64> = [0, 12, 24]
            .iter()
            .flat_map(|&o| MAJOR.iter().map(move |i| ROOT + i + o))
            .collect();
        let places: Vec<usize> = trail
            .iter()
            .map(|chord| pool.iter().position(|p| *p == chord[0]).unwrap())
            .collect();
        let len = pool.len();
        for pair in places.windows(2) {
            let up = (pair[0] + 1) % len == pair[1];
            let down = (pair[1] + 1) % len == pair[0];
            assert!(up || down);
        }
        assert!(heatmap_track(&[ROOT], 0).is_empty());
    }
    #[test]
    fn halving_splits_without_drift() {
        assert_eq!(halve(&[8, 8, 8]), vec![4, 4, 4]);
        assert_eq!(halve(&[5, 5]), vec![2, 3]);
        assert_eq!(halve(&[1, 1, 1]), vec![0, 1, 0]);
        assert_eq!(halve(&[]), Vec::<usize>::new());
    }
    #[test]
    fn the_soundtrack_replays_and_carries_sound() {
        let _g = guard();
        seed(9);
        let scores = vec![score(Way::Conway), score(Way::Mrly)];
        seed(11);
        let a = soundtrack(&scores, &[4, 4]);
        seed(11);
        let b = soundtrack(&scores, &[4, 4]);
        assert_eq!(a, b);
        assert!(!a.is_empty());
        assert!(a.iter().any(|s| s.abs() > 0.1));
        assert!(a.iter().all(|s| s.abs() <= 1.0));
    }
}
