use super::rng::Rng;
use super::trig;
use crate::{json, Json};

/// The home midi note, G2.
pub const ROOT: i64 = 43;

/// The major scale as semitone offsets from a root.
pub const MAJOR: [i64; 7] = [0, 2, 4, 5, 7, 9, 11];

/// The twelve pitch class names, C first.
pub const NOTES: [&str; 12] = [
    "C", "C#", "D", "D#", "E", "F", "F#", "G", "G#", "A", "A#", "B",
];

/// The frequency of every midi note in millihertz, note 69 at 440000.
pub const MILLIHERTZ: [i64; 128] = [
    8176, 8662, 9177, 9723, 10301, 10913, 11562, 12250, 12978, 13750, 14568, 15434, 16352, 17324,
    18354, 19445, 20602, 21827, 23125, 24500, 25957, 27500, 29135, 30868, 32703, 34648, 36708,
    38891, 41203, 43654, 46249, 48999, 51913, 55000, 58270, 61735, 65406, 69296, 73416, 77782,
    82407, 87307, 92499, 97999, 103826, 110000, 116541, 123471, 130813, 138591, 146832, 155563,
    164814, 174614, 184997, 195998, 207652, 220000, 233082, 246942, 261626, 277183, 293665, 311127,
    329628, 349228, 369994, 391995, 415305, 440000, 466164, 493883, 523251, 554365, 587330, 622254,
    659255, 698456, 739989, 783991, 830609, 880000, 932328, 987767, 1046502, 1108731, 1174659,
    1244508, 1318510, 1396913, 1479978, 1567982, 1661219, 1760000, 1864655, 1975533, 2093005,
    2217461, 2349318, 2489016, 2637020, 2793826, 2959955, 3135963, 3322438, 3520000, 3729310,
    3951066, 4186009, 4434922, 4698636, 4978032, 5274041, 5587652, 5919911, 6271927, 6644875,
    7040000, 7458620, 7902133, 8372018, 8869844, 9397273, 9956063, 10548082, 11175303, 11839822,
    12543854,
];

/// The four wave names.
pub const WAVES: [&str; 4] = ["sine", "triangle", "square", "sawtooth"];

/// The sample rate in hertz.
pub const RATE: usize = 44100;

/// The render volume as a percentage of full scale.
pub const VOLUME: i64 = 30;

/// The number of harmonics summed per wave.
pub const VOICES: usize = 16;

/// The sample count of a single-cycle wavetable.
pub const CYCLE: usize = 1024;

/// The fade length at each end of a note, in seconds.
pub const FADE: f32 = 1.0 / 64.0;

const PEAK: f32 = VOLUME as f32 / 100.0;

const MILLI: f32 = 1000.0;

/// The four waveform shapes.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Wave {
    /// The fundamental alone.
    Sine,
    /// Odd harmonics fading as one over n squared.
    Triangle,
    /// Odd harmonics fading as one over n.
    Square,
    /// All harmonics fading as one over n.
    Sawtooth,
}

impl Wave {
    /// Returns the wave one of the four names spells, or None for a stranger.
    pub fn parse(name: &str) -> Option<Wave> {
        match name {
            "sine" => Some(Wave::Sine),
            "triangle" => Some(Wave::Triangle),
            "square" => Some(Wave::Square),
            "sawtooth" => Some(Wave::Sawtooth),
            _ => None,
        }
    }
    /// Returns the wave's lowercase name.
    pub fn name(&self) -> &'static str {
        match self {
            Wave::Sine => "sine",
            Wave::Triangle => "triangle",
            Wave::Square => "square",
            Wave::Sawtooth => "sawtooth",
        }
    }
    /// Returns the wave's amplitude at a phase measured in turns, wrapping whole turns away.
    pub fn sample(&self, phase: f32) -> f32 {
        let t = phase - phase.floor();
        match self {
            Wave::Sine => ring(t),
            Wave::Triangle => 2.0 * (2.0 * (t - (t + 0.5).floor())).abs() - 1.0,
            Wave::Square => {
                let s = ring(t);
                if s > 0.0 {
                    1.0
                } else if s < 0.0 {
                    -1.0
                } else {
                    0.0
                }
            }
            Wave::Sawtooth => 2.0 * (t - (t + 0.5).floor()),
        }
    }
    /// Returns the wave's additive recipe as pairs of harmonic multiple and weight.
    pub fn recipe(&self, voices: usize) -> Vec<(f32, f32)> {
        match self {
            Wave::Sine => vec![(1.0, 1.0)],
            Wave::Square => odds(voices).map(|n| (n, 1.0 / n)).collect(),
            Wave::Triangle => odds(voices).map(|n| (n, 1.0 / (n * n))).collect(),
            Wave::Sawtooth => (1..=voices).map(|i| (i as f32, 1.0 / i as f32)).collect(),
        }
    }
}

/// One tone to render: a pitch, a shape, and a length.
pub struct Note {
    /// The midi note number.
    pub midi: i64,
    /// The waveform.
    pub wave: Wave,
    /// The duration in seconds.
    pub seconds: f32,
}

impl Note {
    /// Builds a note from pitch, wave, and duration.
    pub fn new(midi: i64, wave: Wave, seconds: f32) -> Note {
        Note {
            midi,
            wave,
            seconds,
        }
    }
}

/// A two-axis timbre: the shape each partial is drawn with, and the series that weights them.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Timbre {
    /// The waveform each partial is drawn with.
    pub shape: Wave,
    /// The wave whose recipe picks the partials and their weights.
    pub series: Wave,
    /// The number of harmonics summed from the series.
    pub harmonics: usize,
}

impl Timbre {
    /// Builds a timbre from shape, series, and harmonic count.
    pub fn new(shape: Wave, series: Wave, harmonics: usize) -> Timbre {
        Timbre {
            shape,
            series,
            harmonics,
        }
    }
}

/// Returns a midi note's frequency in millihertz, clamped to the keyboard.
///
/// ```
/// assert_eq!(mrlycore::audio::freq(69), 440_000);
/// ```
pub fn freq(midi: i64) -> i64 {
    MILLIHERTZ[midi.clamp(0, 127) as usize]
}

/// Returns a midi note's name, class then octave, like A4 for 69.
pub fn name(midi: i64) -> String {
    format!(
        "{}{}",
        NOTES[midi.rem_euclid(12) as usize],
        midi.div_euclid(12) - 1
    )
}

/// Returns the pitch class index of a note name, or None for a stranger.
pub fn class(name: &str) -> Option<i64> {
    NOTES.iter().position(|&n| n == name).map(|i| i as i64)
}

/// Draws a scale degree from the rng, lifted up to octaves above the root.
pub fn pick(rng: &mut Rng, root: i64, scale: &[i64], octaves: i64) -> i64 {
    let degree = *rng.choice(scale);
    root + 12 * rng.range(0, octaves) + degree
}

/// Renders a note to float samples, peaking at the volume and faded at both ends.
pub fn render(note: &Note) -> Vec<f32> {
    let mut out = partials(note.midi, Wave::Sine, note.wave, VOICES, note.seconds);
    let peak = out.iter().fold(0.0f32, |m, s| m.max(s.abs()));
    if peak > 0.0 {
        let k = PEAK / peak;
        for s in out.iter_mut() {
            *s *= k;
        }
    }
    let count = out.len();
    let ramp = ((FADE * RATE as f32) as usize).min(count / 2);
    for i in 0..ramp {
        let g = i as f32 / ramp as f32;
        out[i] *= g;
        out[count - 1 - i] *= g;
    }
    out
}

/// Renders a midi note through a timbre to unit-peak float samples, with no fades.
pub fn tone(midi: i64, timbre: &Timbre, seconds: f32) -> Vec<f32> {
    let mut out = partials(midi, timbre.shape, timbre.series, timbre.harmonics, seconds);
    let peak = out.iter().fold(0.0f32, |m, s| m.max(s.abs()));
    if peak > 0.0 {
        for s in out.iter_mut() {
            *s /= peak;
        }
    }
    out
}

/// Clamps float samples into 16-bit pcm.
pub fn pcm(samples: &[f32]) -> Vec<i16> {
    samples
        .iter()
        .map(|s| (s.clamp(-1.0, 1.0) * 32767.0) as i16)
        .collect()
}

/// Builds a unit-peak single-cycle wavetable of the wave at a pitch, muting harmonics above Nyquist.
pub fn cycle(wave: &Wave, hz: f32) -> Vec<f32> {
    let mut out = vec![0.0f32; CYCLE];
    for (mult, weight) in wave.recipe(VOICES) {
        if hz * mult * 2.0 >= RATE as f32 {
            continue;
        }
        for (i, s) in out.iter_mut().enumerate() {
            *s += weight * Wave::Sine.sample(mult * i as f32 / CYCLE as f32);
        }
    }
    let peak = out.iter().fold(0.0f32, |m, s| m.max(s.abs()));
    if peak > 0.0 {
        for s in out.iter_mut() {
            *s /= peak;
        }
    }
    out
}

/// Returns a named sound cue as a note op, falling back to the blip.
pub fn cue(name: &str) -> Json {
    let (offset, ms, gain) = match name {
        "good" => (31, 140, 30),
        "bad" => (13, 160, 30),
        "win" => (36, 320, 30),
        "lose" => (5, 380, 30),
        _ => (24, 90, 25),
    };
    json!({ "op": "note", "freq": freq(ROOT + offset), "ms": ms, "gain": gain })
}

fn partials(midi: i64, shape: Wave, series: Wave, harmonics: usize, seconds: f32) -> Vec<f32> {
    let base = freq(midi) as f32 / MILLI;
    let count = (seconds * RATE as f32) as usize;
    let mut out = vec![0.0f32; count];
    for (mult, weight) in series.recipe(harmonics) {
        let pitch = base * mult;
        if pitch * 2.0 >= RATE as f32 {
            continue;
        }
        let step = pitch / RATE as f32;
        let mut phase = 0.0f32;
        for s in out.iter_mut() {
            *s += weight * shape.sample(phase);
            phase += step;
            if phase >= 1.0 {
                phase -= 1.0;
            }
        }
    }
    out
}

fn odds(voices: usize) -> impl Iterator<Item = f32> {
    (0..voices).map(|i| (2 * i + 1) as f32)
}

fn ring(t: f32) -> f32 {
    let x = t * trig::N as f32;
    let i = x.floor();
    let frac = x - i;
    let a = trig::SINE[(i as usize) % trig::N];
    let b = trig::SINE[(i as usize + 1) % trig::N];
    a + (b - a) * frac
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_roundtrips_the_names() {
        for name in WAVES {
            assert_eq!(Wave::parse(name).unwrap().name(), name);
        }
        assert_eq!(Wave::parse("noise"), None);
    }
    #[test]
    fn samples_hit_the_landmarks() {
        assert!(Wave::Sine.sample(0.0).abs() < 1e-6);
        assert!((Wave::Sine.sample(0.25) - 1.0).abs() < 1e-4);
        assert!((Wave::Sine.sample(0.75) + 1.0).abs() < 1e-4);
        assert_eq!(Wave::Triangle.sample(0.0), -1.0);
        assert_eq!(Wave::Triangle.sample(0.5), 1.0);
        assert_eq!(Wave::Square.sample(0.25), 1.0);
        assert_eq!(Wave::Square.sample(0.75), -1.0);
        assert_eq!(Wave::Sawtooth.sample(0.25), 0.5);
        assert_eq!(Wave::Sawtooth.sample(0.75), -0.5);
    }
    #[test]
    fn samples_wrap_whole_turns() {
        for wave in [Wave::Sine, Wave::Triangle, Wave::Square, Wave::Sawtooth] {
            assert_eq!(wave.sample(0.25), wave.sample(3.25));
        }
    }
    #[test]
    fn recipes_carry_the_classic_weights() {
        assert_eq!(Wave::Sine.recipe(8), vec![(1.0, 1.0)]);
        assert_eq!(
            Wave::Square.recipe(3),
            vec![(1.0, 1.0), (3.0, 1.0 / 3.0), (5.0, 0.2)]
        );
        assert_eq!(
            Wave::Triangle.recipe(3),
            vec![(1.0, 1.0), (3.0, 1.0 / 9.0), (5.0, 1.0 / 25.0)]
        );
        assert_eq!(
            Wave::Sawtooth.recipe(3),
            vec![(1.0, 1.0), (2.0, 0.5), (3.0, 1.0 / 3.0)]
        );
    }
    #[test]
    fn freq_lands_the_tuning_fork() {
        assert_eq!(freq(69), 440_000);
        assert_eq!(freq(57), 220_000);
        assert_eq!(freq(81), 880_000);
        assert_eq!(freq(67), 391_995);
        assert_eq!(freq(43), 97_999);
    }
    #[test]
    fn freq_clamps_outside_the_keyboard() {
        assert_eq!(freq(-4), MILLIHERTZ[0]);
        assert_eq!(freq(900), MILLIHERTZ[127]);
    }
    #[test]
    fn octaves_double_the_millihertz() {
        for midi in 0..116 {
            let low = freq(midi);
            let high = freq(midi + 12);
            assert!((high - 2 * low).abs() <= 1, "midi {midi}: {low} {high}");
        }
    }
    #[test]
    fn names_roundtrip_the_classes() {
        assert_eq!(name(43), "G2");
        assert_eq!(name(60), "C4");
        assert_eq!(name(69), "A4");
        assert_eq!(class("C"), Some(0));
        assert_eq!(class("G"), Some(7));
        assert_eq!(class("H"), None);
        for (i, n) in NOTES.iter().enumerate() {
            assert_eq!(class(n), Some(i as i64));
        }
    }
    #[test]
    fn pick_is_seeded_and_in_range() {
        let mut a = Rng::new(7);
        let mut b = Rng::new(7);
        for _ in 0..32 {
            let x = pick(&mut a, ROOT, &MAJOR, 1);
            assert_eq!(x, pick(&mut b, ROOT, &MAJOR, 1));
            assert!((ROOT..=ROOT + 12 + 11).contains(&x));
            assert!(MAJOR.contains(&((x - ROOT) % 12)));
        }
    }
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
            assert!((peak - PEAK).abs() < 1e-4, "{} {peak}", wave.name());
        }
    }
    #[test]
    fn render_fades_the_endpoints() {
        let samples = render(&Note::new(69, Wave::Square, 0.15));
        assert_eq!(samples[0], 0.0);
        assert_eq!(samples[samples.len() - 1], 0.0);
        let ramp = (FADE * RATE as f32) as usize;
        assert!(samples[..ramp].iter().all(|s| s.abs() <= PEAK));
    }
    #[test]
    fn tone_peaks_at_unity() {
        for wave in [Wave::Sine, Wave::Triangle, Wave::Square, Wave::Sawtooth] {
            let timbre = Timbre::new(Wave::Sine, wave, VOICES);
            let samples = tone(69, &timbre, 0.15);
            assert_eq!(samples.len(), (0.15 * RATE as f32) as usize);
            let peak = samples.iter().fold(0.0f32, |m, s| m.max(s.abs()));
            assert!((peak - 1.0).abs() < 1e-4, "{} {peak}", wave.name());
        }
    }
    #[test]
    fn tone_matches_render_inside_the_fades() {
        let rendered = render(&Note::new(69, Wave::Square, 0.15));
        let timbre = Timbre::new(Wave::Sine, Wave::Square, VOICES);
        let toned = tone(69, &timbre, 0.15);
        let ramp = (FADE * RATE as f32) as usize;
        for i in ramp..rendered.len() - ramp {
            assert!((rendered[i] - toned[i] * PEAK).abs() < 1e-4);
        }
    }
    #[test]
    fn tone_separates_the_axes() {
        let pure = tone(69, &Timbre::new(Wave::Sine, Wave::Sine, 1), 0.1);
        let bent = tone(69, &Timbre::new(Wave::Triangle, Wave::Sine, 1), 0.1);
        let rich = tone(69, &Timbre::new(Wave::Sine, Wave::Triangle, VOICES), 0.1);
        assert_ne!(pure, bent);
        assert_ne!(pure, rich);
        assert_ne!(bent, rich);
    }
    #[test]
    fn tone_thins_to_sine_near_nyquist() {
        assert_eq!(
            tone(127, &Timbre::new(Wave::Sine, Wave::Square, VOICES), 0.05),
            tone(127, &Timbre::new(Wave::Sine, Wave::Sine, 1), 0.05)
        );
    }
    #[test]
    fn pcm_clamps_to_i16() {
        assert_eq!(pcm(&[2.0, -2.0, 0.0, 1.0]), vec![32767, -32767, 0, 32767]);
    }
    #[test]
    fn cycle_peaks_at_unity() {
        for wave in [Wave::Sine, Wave::Triangle, Wave::Square, Wave::Sawtooth] {
            let table = cycle(&wave, 440.0);
            assert_eq!(table.len(), CYCLE);
            let peak = table.iter().fold(0.0f32, |m, s| m.max(s.abs()));
            assert!((peak - 1.0).abs() < 1e-4, "{} {peak}", wave.name());
        }
    }
    #[test]
    fn cycle_mutes_above_nyquist() {
        assert!(cycle(&Wave::Sine, 23000.0).iter().all(|s| *s == 0.0));
    }
    #[test]
    fn cycle_thins_to_sine_near_nyquist() {
        assert_eq!(cycle(&Wave::Square, 8000.0), cycle(&Wave::Sine, 8000.0));
    }
    #[test]
    fn cues_are_notes_without_wave() {
        for name in ["blip", "good", "bad", "win", "lose"] {
            let sound = cue(name);
            assert_eq!(sound["op"], "note");
            assert!(sound["freq"].as_i64().unwrap() > 0);
            assert!(sound["ms"].as_i64().unwrap() >= 90);
            assert!(sound["gain"].as_i64().unwrap() > 0);
            assert!(sound.get("wave").is_none());
        }
    }
    #[test]
    fn cues_land_their_offsets() {
        assert_eq!(cue("blip")["freq"], json!(391_995));
        assert_eq!(cue("good")["freq"], json!(freq(ROOT + 31)));
        assert_eq!(cue("bad")["freq"], json!(freq(ROOT + 13)));
        assert_eq!(cue("win")["freq"], json!(freq(ROOT + 36)));
        assert_eq!(cue("lose")["freq"], json!(freq(ROOT + 5)));
        assert_eq!(cue("mystery"), cue("blip"));
    }
    #[test]
    fn gains_are_centi_percent() {
        assert_eq!(cue("blip")["gain"], json!(25));
        assert_eq!(cue("win")["gain"], json!(30));
    }
}
