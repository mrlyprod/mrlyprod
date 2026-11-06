use crate::{FADE, samples};
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct Envelope {
    pub attack: f64,
    pub decay: f64,
    pub sustain: f64,
    pub release: f64,
}

impl Default for Envelope {
    fn default() -> Envelope {
        Envelope::fade()
    }
}

impl Envelope {
    pub fn new(attack: f64, decay: f64, sustain: f64, release: f64) -> Envelope {
        Envelope {
            attack,
            decay,
            sustain,
            release,
        }
    }
    pub fn fade() -> Envelope {
        Envelope::new(FADE, 0.0, 1.0, FADE)
    }
    pub fn gains(&self, count: usize) -> Vec<f32> {
        let (mut attack, mut release) = (samples(self.attack), samples(self.release));
        let whole = attack as f64 + release as f64;
        if whole > count as f64 {
            attack = (attack as f64 * count as f64 / whole) as usize;
            release = (release as f64 * count as f64 / whole) as usize;
        }
        let decay = samples(self.decay);
        let sustain = self.sustain.clamp(0.0, 1.0);
        (0..count)
            .map(|i| {
                let mut g = if i < attack {
                    i as f64 / attack as f64
                } else if i - attack < decay {
                    1.0 - (1.0 - sustain) * (i - attack) as f64 / decay as f64
                } else {
                    sustain
                };
                let left = count - 1 - i;
                if left < release {
                    g *= left as f64 / release as f64;
                }
                g as f32
            })
            .collect()
    }
    pub fn apply(&self, samples: &[f32]) -> Vec<f32> {
        samples
            .iter()
            .zip(self.gains(samples.len()))
            .map(|(s, g)| s * g)
            .collect()
    }
}
