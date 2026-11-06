use crate::{BAR, BEAT, ChordType};
use mrly_audio::{Envelope, Timbre};
use mrly_name::named_enum;
use serde::{Deserialize, Serialize};

named_enum! {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
    #[serde(rename_all = "lowercase")]
    pub enum Movement {
        Repeat => "repeat",
        Random => "random",
        Up => "up",
        Down => "down",
        Pause => "pause",
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Voice {
    pub pool: Vec<i64>,
    pub movements: Vec<Movement>,
    pub chord: Option<ChordType>,
    pub counts: Vec<usize>,
    pub timbre: Timbre,
    pub envelope: Envelope,
}

impl Voice {
    pub fn new(pool: Vec<i64>, movements: Vec<Movement>) -> Voice {
        Voice {
            pool,
            movements,
            chord: None,
            counts: vec![1],
            timbre: Timbre::default(),
            envelope: Envelope::default(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Piece {
    pub progression: String,
    pub voices: Vec<Voice>,
    pub bar: usize,
    pub repeat: bool,
    pub beat: f64,
}

impl Piece {
    pub fn new(progression: &str, voices: Vec<Voice>) -> Piece {
        Piece {
            progression: progression.to_string(),
            voices,
            bar: BAR,
            repeat: false,
            beat: BEAT,
        }
    }
}
