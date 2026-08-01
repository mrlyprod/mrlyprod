use mrlycore::trig;

pub const NAMES: [&str; 4] = ["sine", "triangle", "square", "sawtooth"];

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Wave {
    Sine,
    Triangle,
    Square,
    Sawtooth,
}

impl Wave {
    pub fn parse(name: &str) -> Option<Wave> {
        match name {
            "sine" => Some(Wave::Sine),
            "triangle" => Some(Wave::Triangle),
            "square" => Some(Wave::Square),
            "sawtooth" => Some(Wave::Sawtooth),
            _ => None,
        }
    }
    pub fn name(&self) -> &'static str {
        match self {
            Wave::Sine => "sine",
            Wave::Triangle => "triangle",
            Wave::Square => "square",
            Wave::Sawtooth => "sawtooth",
        }
    }
    pub fn sample(&self, phase: f32) -> f32 {
        let t = phase - phase.floor();
        match self {
            Wave::Sine => sine(t),
            Wave::Triangle => 2.0 * (2.0 * (t - (t + 0.5).floor())).abs() - 1.0,
            Wave::Square => {
                let s = sine(t);
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
    pub fn recipe(&self, voices: usize) -> Vec<(f32, f32)> {
        match self {
            Wave::Sine => vec![(1.0, 1.0)],
            Wave::Square => odds(voices).map(|n| (n, 1.0 / n)).collect(),
            Wave::Triangle => odds(voices).map(|n| (n, 1.0 / (n * n))).collect(),
            Wave::Sawtooth => (1..=voices).map(|i| (i as f32, 1.0 / i as f32)).collect(),
        }
    }
}

fn odds(voices: usize) -> impl Iterator<Item = f32> {
    (0..voices).map(|i| (2 * i + 1) as f32)
}

fn sine(t: f32) -> f32 {
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
        for name in NAMES {
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
    fn recipes_carry_the_legacy_weights() {
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
}
