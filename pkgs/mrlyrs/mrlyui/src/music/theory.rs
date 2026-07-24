use mrlycore::rng::Rng;

pub const ROOT: i64 = 43;

pub const MAJOR: [i64; 7] = [0, 2, 4, 5, 7, 9, 11];

pub const NAMES: [&str; 12] = [
    "C", "C#", "D", "D#", "E", "F", "F#", "G", "G#", "A", "A#", "B",
];

const SEMITONES: [f32; 12] = [
    1.0,
    1.059_463_1,
    1.122_462,
    1.189_207_1,
    1.259_921_1,
    1.334_839_8,
    std::f32::consts::SQRT_2,
    1.498_307_1,
    1.587_401,
    1.681_792_9,
    1.781_797_4,
    1.887_748_6,
];

pub fn freq(midi: i64) -> f32 {
    let d = midi - 69;
    let mut octaves = d.div_euclid(12);
    let mut out = 440.0 * SEMITONES[d.rem_euclid(12) as usize];
    while octaves > 0 {
        out *= 2.0;
        octaves -= 1;
    }
    while octaves < 0 {
        out *= 0.5;
        octaves += 1;
    }
    out
}

pub fn name(midi: i64) -> String {
    format!(
        "{}{}",
        NAMES[midi.rem_euclid(12) as usize],
        midi.div_euclid(12) - 1
    )
}

pub fn class(name: &str) -> Option<i64> {
    NAMES.iter().position(|&n| n == name).map(|i| i as i64)
}

pub fn pick(rng: &mut Rng, root: i64, scale: &[i64], octaves: i64) -> i64 {
    let degree = *rng.choice(scale);
    root + 12 * rng.range(0, octaves) + degree
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn freq_lands_the_tuning_fork() {
        assert_eq!(freq(69), 440.0);
        assert_eq!(freq(57), 220.0);
        assert_eq!(freq(81), 880.0);
        assert!((freq(43) - 98.0).abs() < 0.01);
    }
    #[test]
    fn names_roundtrip_the_classes() {
        assert_eq!(name(43), "G2");
        assert_eq!(name(60), "C4");
        assert_eq!(name(69), "A4");
        assert_eq!(class("C"), Some(0));
        assert_eq!(class("G"), Some(7));
        assert_eq!(class("H"), None);
        for (i, n) in NAMES.iter().enumerate() {
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
}
