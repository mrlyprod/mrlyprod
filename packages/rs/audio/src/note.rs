use crate::Result;
use std::f64::consts::SQRT_2;

pub const NOTES: [&str; 12] = [
    "C", "C#", "D", "D#", "E", "F", "F#", "G", "G#", "A", "A#", "B",
];

const SEMITONES: [f64; 12] = [
    1.0,
    1.059_463_094_359_295_3,
    1.122_462_048_309_373,
    1.189_207_115_002_721,
    1.259_921_049_894_873_2,
    1.334_839_854_170_034_4,
    SQRT_2,
    1.498_307_076_876_681_5,
    1.587_401_051_968_199_4,
    1.681_792_830_507_429,
    1.781_797_436_280_678_6,
    1.887_748_625_363_386_8,
];

pub fn hertz(midi: i64) -> f64 {
    let from = midi.clamp(0, 127) - 69;
    440.0 * SEMITONES[from.rem_euclid(12) as usize] * 2f64.powi(from.div_euclid(12) as i32)
}

pub fn name(midi: i64) -> String {
    let midi = midi.clamp(0, 127);
    format!("{}{}", NOTES[midi as usize % 12], midi / 12 - 1)
}

pub fn midi(name: &str) -> Result<i64> {
    let bad = || format!("note {name:?} is not a letter, an optional # or b, and an octave.");
    let mut chars = name.chars();
    let letter = chars.next().ok_or_else(bad)?;
    let class = match letter.to_ascii_uppercase() {
        'C' => 0,
        'D' => 2,
        'E' => 4,
        'F' => 5,
        'G' => 7,
        'A' => 9,
        'B' => 11,
        _ => return Err(bad()),
    };
    let rest = chars.as_str();
    let (shift, octave) = match rest.chars().next() {
        Some('#') => (1, &rest[1..]),
        Some('b') => (-1, &rest[1..]),
        _ => (0, rest),
    };
    let octave: i64 = octave.parse().map_err(|_| bad())?;
    let midi = 12 * (octave.clamp(-2, 11) + 1) + class + shift;
    if !(-1..=9).contains(&octave) || !(0..=127).contains(&midi) {
        return Err(format!("note {name:?} lies outside midi 0 to 127."));
    }
    Ok(midi)
}
