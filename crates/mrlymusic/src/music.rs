use crate::audio::{self, Timbre, Wave};
use mrlycore::state;
use std::collections::HashMap;

/// The ways a voice moves from one frame to the next.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Movement {
    /// Holds the previous frame's notes.
    Repeat,
    /// Draws fresh notes from the pool.
    Random,
    /// Steps every note one pool index up, wrapping at the top.
    Up,
    /// Steps every note one pool index down, wrapping at the bottom.
    Down,
    /// Plays an empty frame.
    Pause,
}

/// The chord stackings over a progression degree.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ChordType {
    /// Root, third and fifth.
    Triad,
    /// Root, third, fifth and seventh.
    Seventh,
}

impl ChordType {
    /// Returns the chord's intervals as pool index offsets from the root.
    pub fn intervals(&self) -> &'static [usize] {
        match self {
            ChordType::Triad => &[0, 2, 4],
            ChordType::Seventh => &[0, 2, 4, 6],
        }
    }
}

/// One melodic line: a pool of midi notes, movements to walk it, and a timbre.
#[derive(Clone, Debug, PartialEq)]
pub struct Voice {
    /// The midi notes the voice draws from.
    pub note_pool: Vec<i64>,
    /// The movements the voice may take between frames.
    pub movements: Vec<Movement>,
    /// The chord stacked on each progression degree, or None to play the whole pool.
    pub chord_type: Option<ChordType>,
    /// The counts of simultaneous notes the voice may draw.
    pub num_notes: Vec<usize>,
    /// The waveform each partial is drawn with.
    pub wave_type: Wave,
    /// The number of harmonics summed from the series.
    pub num_harmonics: usize,
    /// The wave whose recipe weights the harmonics.
    pub harmonic_wave: Wave,
}

impl Voice {
    /// Builds a voice over a note pool with its movements, the rest at their defaults.
    pub fn new(note_pool: Vec<i64>, movements: Vec<Movement>) -> Voice {
        Voice {
            note_pool,
            movements,
            chord_type: None,
            num_notes: vec![1],
            wave_type: Wave::Sine,
            num_harmonics: 1,
            harmonic_wave: Wave::Triangle,
        }
    }
    /// Returns the voice's timbre.
    pub fn timbre(&self) -> Timbre {
        Timbre::new(self.wave_type, self.harmonic_wave, self.num_harmonics)
    }
}

impl Default for Voice {
    fn default() -> Voice {
        Voice::new(Vec::new(), Vec::new())
    }
}

/// Returns a chord letter's diatonic degree, C through B as 0 through 6, or None for a stranger.
///
/// ```
/// assert_eq!(mrlymusic::music::degree('F'), Some(3));
/// ```
pub fn degree(letter: char) -> Option<i64> {
    "CDEFGAB"
        .chars()
        .position(|c| c == letter)
        .map(|i| i as i64)
}

/// Composes frames of midi notes by walking every voice through a chord progression.
///
/// Each letter earns every voice a bar of bar_length frames, letters beyond C through B
/// are skipped, and the base track loops until count frames are filled. With repeat on,
/// a voice replays its first bar for a letter every time that letter returns.
pub fn compose(
    progression: &str,
    voices: &[Voice],
    bar_length: usize,
    count: usize,
    repeat: bool,
) -> Vec<Vec<i64>> {
    let mut tracks: Vec<Vec<Vec<i64>>> = vec![Vec::new(); voices.len()];
    let mut library: HashMap<(usize, char), Vec<Vec<i64>>> = HashMap::new();
    for letter in progression.chars() {
        let Some(d) = degree(letter) else { continue };
        for (i, voice) in voices.iter().enumerate() {
            let bar = if repeat {
                library
                    .entry((i, letter))
                    .or_insert_with(|| voice_bar(voice, d as usize, bar_length))
                    .clone()
            } else {
                voice_bar(voice, d as usize, bar_length)
            };
            tracks[i].extend(bar);
        }
    }
    let base = concatenate(&tracks);
    if base.is_empty() {
        return Vec::new();
    }
    let mut frames = Vec::with_capacity(count);
    while frames.len() < count {
        for frame in &base {
            if frames.len() == count {
                break;
            }
            frames.push(frame.clone());
        }
    }
    frames
}

/// Mixes one frame's notes through a timbre into a single buffer, one unit-peak tone each.
pub fn mix(notes: &[i64], timbre: &Timbre, seconds: f32) -> Vec<f32> {
    let count = (seconds * audio::RATE as f32) as usize;
    let mut out = vec![0.0f32; count];
    for &midi in notes {
        let tone = audio::tone(midi, timbre, seconds);
        for (s, t) in out.iter_mut().zip(&tone) {
            *s += t;
        }
    }
    out
}

/// Renders timed frames through a timbre into one track: each frame is mixed and faded,
/// then the whole track is normalized once to the target peak.
pub fn track(frames: &[(Vec<i64>, f32)], timbre: &Timbre, peak: f32) -> Vec<f32> {
    let mut out = Vec::new();
    for (notes, seconds) in frames {
        let mut frame = mix(notes, timbre, *seconds);
        fade(&mut frame);
        out.extend_from_slice(&frame);
    }
    let top = out.iter().fold(0.0f32, |m, s| m.max(s.abs()));
    if top > 0.0 {
        let k = peak / top;
        for s in out.iter_mut() {
            *s *= k;
        }
    }
    out
}

fn voice_bar(voice: &Voice, degree: usize, bar_length: usize) -> Vec<Vec<i64>> {
    let pool = chord_pool(voice, degree);
    bar(bar_length, &voice.movements, &pool, &voice.num_notes)
}

fn chord_pool(voice: &Voice, degree: usize) -> Vec<i64> {
    match voice.chord_type {
        None => voice.note_pool.clone(),
        Some(kind) => chord(&voice.note_pool, degree, kind),
    }
}

fn chord(pool: &[i64], degree: usize, chord_type: ChordType) -> Vec<i64> {
    let mut notes = Vec::new();
    if pool.is_empty() {
        return notes;
    }
    let root = degree % pool.len();
    for interval in chord_type.intervals() {
        let note = pool[(root + interval) % pool.len()];
        if !notes.contains(&note) {
            notes.push(note);
        }
    }
    notes
}

fn bar(count: usize, movements: &[Movement], pool: &[i64], num_notes: &[usize]) -> Vec<Vec<i64>> {
    let mut frames = Vec::with_capacity(count.max(1));
    frames.push(state::sample(pool, state::choice(num_notes)));
    for _ in 1..count {
        let movement = if movements.len() > 1 {
            state::choice(movements)
        } else {
            movements[0]
        };
        let previous = frames.last().unwrap();
        let notes = match movement {
            Movement::Repeat => previous.clone(),
            Movement::Random => state::sample(pool, state::choice(num_notes)),
            Movement::Up => step(pool, previous, 1),
            Movement::Down => step(pool, previous, -1),
            Movement::Pause => Vec::new(),
        };
        frames.push(notes);
    }
    frames
}

fn step(pool: &[i64], previous: &[i64], by: i64) -> Vec<i64> {
    previous
        .iter()
        .map(|n| {
            let at = pool.iter().position(|p| p == n).unwrap() as i64;
            pool[(at + by).rem_euclid(pool.len() as i64) as usize]
        })
        .collect()
}

fn concatenate(tracks: &[Vec<Vec<i64>>]) -> Vec<Vec<i64>> {
    let beats = tracks.iter().find(|t| !t.is_empty()).map_or(0, |t| t.len());
    let mut out = Vec::with_capacity(beats);
    for beat in 0..beats {
        let mut frame = Vec::new();
        for track in tracks {
            if !track.is_empty() {
                frame.extend_from_slice(&track[beat]);
            }
        }
        out.push(frame);
    }
    out
}

fn fade(samples: &mut [f32]) {
    let count = samples.len();
    let ramp = ((audio::FADE * audio::RATE as f32) as usize).min(count / 2);
    for i in 0..ramp {
        let g = i as f32 / ramp as f32;
        samples[i] *= g;
        samples[count - 1 - i] *= g;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn pool() -> Vec<i64> {
        vec![43, 45, 47, 48, 50, 52, 54]
    }
    fn voices() -> Vec<Voice> {
        let mut bass = Voice::new(pool(), vec![Movement::Repeat, Movement::Up, Movement::Down]);
        bass.chord_type = Some(ChordType::Triad);
        let mut rhythm = Voice::new(
            vec![55, 57, 59, 60, 62, 64, 66],
            vec![Movement::Repeat, Movement::Random],
        );
        rhythm.chord_type = Some(ChordType::Seventh);
        rhythm.num_notes = vec![3, 4];
        let lead = Voice::new(
            vec![67, 69, 71, 72, 74, 76, 78],
            vec![
                Movement::Repeat,
                Movement::Random,
                Movement::Up,
                Movement::Down,
                Movement::Pause,
            ],
        );
        vec![bass, rhythm, lead]
    }
    #[test]
    fn degrees_are_diatonic_not_chromatic() {
        assert_eq!(degree('F'), Some(3));
        assert_eq!(audio::class("F"), Some(5));
        for (i, letter) in "CDEFGAB".chars().enumerate() {
            assert_eq!(degree(letter), Some(i as i64));
        }
        assert_eq!(degree('H'), None);
        assert_eq!(degree('c'), None);
    }
    #[test]
    fn intervals_stack_thirds() {
        assert_eq!(ChordType::Triad.intervals(), [0, 2, 4]);
        assert_eq!(ChordType::Seventh.intervals(), [0, 2, 4, 6]);
    }
    #[test]
    fn chords_wrap_and_dedup() {
        assert_eq!(chord(&pool(), 0, ChordType::Triad), vec![43, 47, 50]);
        assert_eq!(chord(&pool(), 3, ChordType::Triad), vec![48, 52, 43]);
        assert_eq!(
            chord(&[60, 62, 64], 0, ChordType::Seventh),
            vec![60, 64, 62]
        );
        assert_eq!(chord(&[60], 4, ChordType::Seventh), vec![60]);
        assert_eq!(chord(&[], 2, ChordType::Triad), Vec::<i64>::new());
    }
    #[test]
    fn voices_carry_their_defaults() {
        let voice = Voice::new(vec![60], vec![Movement::Repeat]);
        assert_eq!(voice.chord_type, None);
        assert_eq!(voice.num_notes, vec![1]);
        assert_eq!(voice.wave_type, Wave::Sine);
        assert_eq!(voice.num_harmonics, 1);
        assert_eq!(voice.harmonic_wave, Wave::Triangle);
        assert_eq!(voice.timbre(), Timbre::new(Wave::Sine, Wave::Triangle, 1));
        assert_eq!(Voice::default(), Voice::new(Vec::new(), Vec::new()));
    }
    #[test]
    fn movements_wrap_the_pool() {
        let _g = state::guard();
        state::seed(7);
        let p = vec![1, 2, 3];
        for pair in bar(4, &[Movement::Up], &p, &[1]).windows(2) {
            let from = p.iter().position(|n| *n == pair[0][0]).unwrap();
            assert_eq!(pair[1][0], p[(from + 1) % p.len()]);
        }
        for pair in bar(4, &[Movement::Down], &p, &[1]).windows(2) {
            let from = p.iter().position(|n| *n == pair[0][0]).unwrap() as i64;
            assert_eq!(
                pair[1][0],
                p[(from - 1).rem_euclid(p.len() as i64) as usize]
            );
        }
    }
    #[test]
    fn repeat_holds_and_pause_empties() {
        let _g = state::guard();
        state::seed(3);
        let held = bar(4, &[Movement::Repeat], &pool(), &[2]);
        assert_eq!(held[0].len(), 2);
        for frame in &held {
            assert_eq!(frame, &held[0]);
        }
        let paused = bar(3, &[Movement::Pause], &pool(), &[2]);
        assert_eq!(paused[0].len(), 2);
        assert!(paused[1].is_empty());
        assert!(paused[2].is_empty());
        assert!(step(&pool(), &[], 1).is_empty());
    }
    #[test]
    fn random_draws_distinct_pool_notes() {
        let _g = state::guard();
        state::seed(9);
        let p = pool();
        for frame in bar(8, &[Movement::Random], &p, &[3]) {
            assert_eq!(frame.len(), 3);
            for n in &frame {
                assert!(p.contains(n));
            }
            let mut seen = frame.clone();
            seen.sort();
            seen.dedup();
            assert_eq!(seen.len(), 3);
        }
    }
    #[test]
    fn compose_replays_from_a_seed() {
        let _g = state::guard();
        state::seed(11);
        let a = compose("CFCFGFCG", &voices(), 4, 64, false);
        state::seed(11);
        let b = compose("CFCFGFCG", &voices(), 4, 64, false);
        assert_eq!(a, b);
        assert_eq!(a.len(), 64);
    }
    #[test]
    fn compose_loops_the_base_track() {
        let _g = state::guard();
        state::seed(5);
        let frames = compose("C", &voices(), 4, 6, false);
        assert_eq!(frames.len(), 6);
        assert_eq!(frames[4], frames[0]);
        assert_eq!(frames[5], frames[1]);
    }
    #[test]
    fn repeat_reuses_bars_across_letters() {
        let _g = state::guard();
        state::seed(2);
        let frames = compose("CC", &voices(), 4, 8, true);
        assert_eq!(&frames[..4], &frames[4..]);
    }
    #[test]
    fn compose_skips_stranger_letters() {
        let _g = state::guard();
        state::seed(4);
        let frames = compose("C-C", &voices(), 4, 8, true);
        state::seed(4);
        let again = compose("CC", &voices(), 4, 8, true);
        assert_eq!(frames, again);
    }
    #[test]
    fn compose_handles_an_empty_progression() {
        assert!(compose("", &voices(), 4, 64, false).is_empty());
        assert!(compose("C", &[], 4, 64, false).is_empty());
    }
    #[test]
    fn mix_sums_unit_tones() {
        let timbre = Timbre::new(Wave::Sine, Wave::Triangle, 3);
        let one = mix(&[69], &timbre, 0.05);
        assert_eq!(one, audio::tone(69, &timbre, 0.05));
        let two = mix(&[69, 69], &timbre, 0.05);
        let doubled: Vec<f32> = one.iter().map(|s| s * 2.0).collect();
        assert_eq!(two, doubled);
        assert!(mix(&[], &timbre, 0.05).iter().all(|s| *s == 0.0));
    }
    #[test]
    fn track_normalizes_once_globally() {
        let timbre = Timbre::new(Wave::Sine, Wave::Sine, 1);
        let frames = vec![(vec![69, 69, 69], 0.1f32), (vec![69], 0.1f32)];
        let out = track(&frames, &timbre, 0.9);
        let peak = out.iter().fold(0.0f32, |m, s| m.max(s.abs()));
        assert!((peak - 0.9).abs() < 1e-4);
        let quiet = out[out.len() / 2..]
            .iter()
            .fold(0.0f32, |m, s| m.max(s.abs()));
        assert!(quiet < 0.4);
    }
    #[test]
    fn track_fades_each_frame() {
        let timbre = Timbre::new(Wave::Sine, Wave::Sine, 1);
        let frames = vec![(vec![60], 0.1f32), (vec![64], 0.1f32)];
        let out = track(&frames, &timbre, 1.0);
        let half = out.len() / 2;
        assert_eq!(out[0], 0.0);
        assert_eq!(out[half - 1], 0.0);
        assert_eq!(out[half], 0.0);
        assert_eq!(out[out.len() - 1], 0.0);
    }
}
