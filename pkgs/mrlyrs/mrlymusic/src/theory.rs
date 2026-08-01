use mrlycore::rng::Rng;

pub const ROOT: i64 = 43;

pub const MAJOR: [i64; 7] = [0, 2, 4, 5, 7, 9, 11];

pub const NAMES: [&str; 12] = [
    "C", "C#", "D", "D#", "E", "F", "F#", "G", "G#", "A", "A#", "B",
];

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

pub fn freq(midi: i64) -> i64 {
    MILLIHERTZ[midi.clamp(0, 127) as usize]
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
