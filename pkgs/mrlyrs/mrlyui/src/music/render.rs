use super::theory;
use super::wave::Wave;

pub const RATE: usize = 44100;

pub const FADE: f32 = 1.0 / 64.0;

pub const VOLUME: f64 = 0.3;

pub const VOICES: usize = 16;

pub struct Note {
    pub midi: i64,
    pub wave: Wave,
    pub seconds: f32,
}

impl Note {
    pub fn new(midi: i64, wave: Wave, seconds: f32) -> Note {
        Note {
            midi,
            wave,
            seconds,
        }
    }
}

pub fn render(note: &Note) -> Vec<f32> {
    let base = theory::freq(note.midi);
    let count = (note.seconds * RATE as f32) as usize;
    let mut out = vec![0.0f32; count];
    for (mult, weight) in note.wave.recipe(VOICES) {
        let pitch = base * mult;
        if pitch * 2.0 >= RATE as f32 {
            continue;
        }
        let step = pitch / RATE as f32;
        let mut phase = 0.0f32;
        for s in out.iter_mut() {
            *s += weight * Wave::Sine.sample(phase);
            phase += step;
            if phase >= 1.0 {
                phase -= 1.0;
            }
        }
    }
    let peak = out.iter().fold(0.0f32, |m, s| m.max(s.abs()));
    if peak > 0.0 {
        let k = VOLUME as f32 / peak;
        for s in out.iter_mut() {
            *s *= k;
        }
    }
    let ramp = ((FADE * RATE as f32) as usize).min(count / 2);
    for i in 0..ramp {
        let g = i as f32 / ramp as f32;
        out[i] *= g;
        out[count - 1 - i] *= g;
    }
    out
}

pub fn pcm(samples: &[f32]) -> Vec<i16> {
    samples
        .iter()
        .map(|s| (s.clamp(-1.0, 1.0) * 32767.0) as i16)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn render_fills_the_duration() {
        let note = Note::new(69, Wave::Sine, 0.15);
        assert_eq!(render(&note).len(), (0.15 * RATE as f32) as usize);
    }
    #[test]
    fn render_peaks_at_the_volume() {
        for wave in [Wave::Sine, Wave::Triangle, Wave::Square, Wave::Sawtooth] {
            let samples = render(&Note::new(69, wave, 0.15));
            let peak = samples.iter().fold(0.0f32, |m, s| m.max(s.abs()));
            assert!(
                (peak - VOLUME as f32).abs() < 1e-4,
                "{} {peak}",
                wave.name()
            );
        }
    }
    #[test]
    fn render_fades_the_endpoints() {
        let samples = render(&Note::new(69, Wave::Square, 0.15));
        assert_eq!(samples[0], 0.0);
        assert_eq!(samples[samples.len() - 1], 0.0);
        let ramp = (FADE * RATE as f32) as usize;
        assert!(samples[..ramp].iter().all(|s| s.abs() <= VOLUME as f32));
    }
    #[test]
    fn pcm_clamps_to_i16() {
        assert_eq!(pcm(&[2.0, -2.0, 0.0, 1.0]), vec![32767, -32767, 0, 32767]);
    }
}
