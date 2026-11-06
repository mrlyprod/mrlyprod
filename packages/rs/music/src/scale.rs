use crate::Result;
use mrly_name::named_enum;
use serde::{Deserialize, Serialize};

const MAJOR: [i64; 7] = [0, 2, 4, 5, 7, 9, 11];

named_enum! {
    #[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
    #[serde(rename_all = "lowercase")]
    pub enum Scale {
        #[default]
        Major => "major",
        Dorian => "dorian",
        Phrygian => "phrygian",
        Lydian => "lydian",
        Mixolydian => "mixolydian",
        Minor => "minor",
        Locrian => "locrian",
    }
}

impl Scale {
    pub fn steps(&self) -> [i64; 7] {
        let mode = *self as usize;
        std::array::from_fn(|i| (MAJOR[(mode + i) % 7] - MAJOR[mode]).rem_euclid(12))
    }
}

named_enum! {
    #[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
    #[serde(rename_all = "lowercase")]
    pub enum ChordType {
        #[default]
        Triad => "triad",
        Seventh => "seventh",
        Ninth => "ninth",
    }
}

impl ChordType {
    pub fn intervals(&self) -> Vec<usize> {
        match self {
            ChordType::Triad => vec![0, 2, 4],
            ChordType::Seventh => vec![0, 2, 4, 6],
            ChordType::Ninth => vec![0, 2, 4, 6, 8],
        }
    }
}

pub fn pool(scale: Scale, root: i64, octaves: usize) -> Vec<i64> {
    (0..octaves as i64)
        .flat_map(|octave| scale.steps().map(|step| root + 12 * octave + step))
        .collect()
}

pub fn chord(pool: &[i64], degree: usize, kind: ChordType) -> Vec<i64> {
    let mut notes = Vec::new();
    if pool.is_empty() {
        return notes;
    }
    for interval in kind.intervals() {
        let note = pool[(degree + interval) % pool.len()];
        if !notes.contains(&note) {
            notes.push(note);
        }
    }
    notes
}

pub fn chords(progression: &str, pool: &[i64], kind: ChordType) -> Result<Vec<Vec<i64>>> {
    Ok(degrees(progression)?
        .into_iter()
        .map(|degree| chord(pool, degree, kind))
        .collect())
}

pub(crate) fn degrees(progression: &str) -> Result<Vec<usize>> {
    progression
        .chars()
        .filter(|c| !c.is_whitespace())
        .map(|letter| {
            "CDEFGAB".find(letter).ok_or_else(|| {
                format!("chord {letter:?} in {progression:?} is not a letter from C to B.")
            })
        })
        .collect()
}
