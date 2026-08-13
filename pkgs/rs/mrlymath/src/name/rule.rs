use super::text;
use super::Named;
use crate::life::{Boundary, Config, Counts, Sequence};
use crate::two::Cell2d;
use mrlycore::errors::{value_error, Result};

const MAX_COUNT: usize = 9;

/// A life rule: the birth and survival counts and the edge policy.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Rule {
    /// The neighbor counts that create a cell.
    pub birth: Counts,
    /// The neighbor counts that keep a cell.
    pub survive: Counts,
    /// The edge policy.
    pub boundary: Boundary,
}

impl Rule {
    /// Builds a rule from its counts and edge policy, or an error for a listed count above 9.
    pub fn new(
        birth: impl Into<Counts>,
        survive: impl Into<Counts>,
        boundary: Boundary,
    ) -> Result<Rule> {
        let birth = birth.into();
        let survive = survive.into();
        nameable(&birth)?;
        nameable(&survive)?;
        Ok(Rule {
            birth,
            survive,
            boundary,
        })
    }
    /// Reads the rule out of a life config, or an error for a listed count above 9.
    pub fn of(config: &Config) -> Result<Rule> {
        Rule::new(
            config.birth.clone(),
            config.survive.clone(),
            config.boundary,
        )
    }
    /// Builds a life config running this rule over a neighborhood mask.
    pub fn config(&self, mask: Cell2d) -> Config {
        let mut config = Config::new(mask, self.birth.clone(), self.survive.clone());
        config.boundary = self.boundary;
        config
    }
    fn fold(counts: &[usize]) -> Vec<usize> {
        let mut out = counts.to_vec();
        out.sort_unstable();
        out.dedup();
        out
    }
    fn spell(counts: &Counts) -> String {
        match counts {
            Counts::List(list) => {
                let folded = Rule::fold(list);
                assert!(
                    folded.iter().all(|&n| n <= MAX_COUNT),
                    "a listed rule count leaves 0..=9; name it by its sequence"
                );
                folded.iter().map(usize::to_string).collect()
            }
            Counts::Drawn { seq, zeros, ones } => {
                let mut out = seq.name();
                if *zeros {
                    out.push('z');
                }
                if *ones {
                    out.push('o');
                }
                out
            }
        }
    }
    fn read(text: &str, tag: char) -> Result<(Counts, &str)> {
        let Some(body) = text.strip_prefix(tag) else {
            return value_error(format!("rule field {text:?} does not open with {tag:?}."));
        };
        if let Some((seq, tail)) = Sequence::read(body) {
            let (zeros, tail) = flag(tail, 'z');
            let (ones, tail) = flag(tail, 'o');
            return Ok((Counts::drawn(seq, zeros, ones), tail));
        }
        let end = body.bytes().take_while(u8::is_ascii_digit).count();
        let mut list: Vec<usize> = Vec::new();
        for b in body[..end].bytes() {
            let n = (b - b'0') as usize;
            if list.last().is_some_and(|&last| last >= n) {
                return value_error(format!("rule field {text:?} is not strictly ascending."));
            }
            list.push(n);
        }
        Ok((Counts::List(list), &body[end..]))
    }
}

fn flag(text: &str, letter: char) -> (bool, &str) {
    match text.strip_prefix(letter) {
        Some(rest) => (true, rest),
        None => (false, text),
    }
}

fn nameable(counts: &Counts) -> Result<()> {
    let Counts::List(list) = counts else {
        return Ok(());
    };
    match list.iter().find(|&&n| n > MAX_COUNT) {
        Some(n) => value_error(format!(
            "a listed rule count leaves 0..=9; {n} wants its sequence named instead."
        )),
        None => Ok(()),
    }
}

impl Named for Rule {
    fn to_str(&self) -> String {
        let mut fields = vec![
            format!("b{}", Rule::spell(&self.birth)),
            format!("s{}", Rule::spell(&self.survive)),
        ];
        if self.boundary == Boundary::Wrap {
            fields.push("w".to_string());
        }
        text::compose("rule", &fields)
    }
    fn from_str(text: &str) -> Result<Rule> {
        let body = text::body(text, "rule")?;
        let (birth, rest) = Rule::read(body, 'b')?;
        let Some(rest) = rest.strip_prefix('_') else {
            return value_error(format!("rule name {text:?} wants an s field."));
        };
        let (survive, rest) = Rule::read(rest, 's')?;
        let boundary = match rest {
            "" => Boundary::Constant,
            "_w" => Boundary::Wrap,
            _ => return value_error(format!("rule name {text:?} holds a stray tail.")),
        };
        Rule::new(birth, survive, boundary)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::life::{moore, Story};
    use mrlycore::rng::Rng;
    use mrlycore::tensor::Tensor;

    fn wide_mask(side: usize) -> Cell2d {
        let mut mask = Tensor::full(vec![side, side], 1);
        mask.set(&[side / 2, side / 2], 0);
        Cell2d::new(mask)
    }

    #[test]
    fn conway_is_b3_s23() {
        let conway = Rule::new(vec![3], vec![2, 3], Boundary::Constant).unwrap();
        assert_eq!(conway.to_str(), "mrly_rule_b3_s23");
        assert_eq!(Rule::from_str("mrly_rule_b3_s23").unwrap(), conway);
        let wrapped = Rule::new(vec![3], vec![2, 3], Boundary::Wrap).unwrap();
        assert_eq!(wrapped.to_str(), "mrly_rule_b3_s23_w");
    }
    #[test]
    fn to_str_folds_to_the_canonical_counts() {
        let messy = Rule::new(vec![3, 3, 1], vec![9, 2], Boundary::Constant).unwrap();
        assert_eq!(messy.to_str(), "mrly_rule_b13_s29");
        let empty = Rule::new(Vec::new(), Vec::new(), Boundary::Constant).unwrap();
        assert_eq!(empty.to_str(), "mrly_rule_b_s");
        assert_eq!(Rule::from_str("mrly_rule_b_s").unwrap(), empty);
    }
    #[test]
    fn only_the_canonical_form_parses() {
        for bad in [
            "mrly_rule_b33_s2",
            "mrly_rule_b31_s2",
            "mrly_rule_s23_b3",
            "mrly_rule_b3",
            "mrly_rule_b3_s23_x",
            "mrly_rule_b3_s23_w_w",
            "mrly_rule_bfib_s3",
            "mrly_rule_brandom_s3",
            "mrly_rule_brandom_007_s3",
            "mrly_rule_bfibonacciozz_s3",
            "mrly_rule_bfibonacciq_s3",
            "b3_s23",
        ] {
            assert!(Rule::from_str(bad).is_err(), "{bad}");
        }
    }
    #[test]
    fn a_listed_count_above_nine_never_builds_a_rule() {
        assert!(Rule::new(vec![3, 12], vec![2, 3], Boundary::Constant).is_err());
        assert!(Rule::new(vec![3], vec![10], Boundary::Wrap).is_err());
        assert!(Rule::new(vec![3], vec![2, 3, 9], Boundary::Constant).is_ok());
        let mut config = Config::new(moore(), vec![3], vec![2, 3]);
        config.survive = Counts::List(vec![48]);
        assert!(Rule::of(&config).is_err());
    }
    #[test]
    fn config_round_trips_through_the_rule() {
        let mask = crate::two::designs::ones(3, 1).unwrap();
        let rule = Rule::new(vec![3, 6], vec![2, 3], Boundary::Wrap).unwrap();
        let config = rule.config(mask);
        assert_eq!(Rule::of(&config).unwrap(), rule);
        assert_eq!(Rule::of(&config).unwrap().to_str(), "mrly_rule_b36_s23_w");
    }
    #[test]
    fn a_drawn_rule_names_its_sequence() {
        let rule = Rule::new(
            Counts::drawn(Sequence::Fibonacci, false, true),
            Counts::drawn(Sequence::GridSquares, false, false),
            Boundary::Wrap,
        )
        .unwrap();
        assert_eq!(rule.to_str(), "mrly_rule_bfibonaccio_sgrid_squares_w");
        assert_eq!(Rule::from_str(&rule.to_str()).unwrap(), rule);
        let seeded = Rule::new(
            Counts::drawn(Sequence::Random(4848495), true, false),
            vec![3],
            Boundary::Constant,
        )
        .unwrap();
        assert_eq!(seeded.to_str(), "mrly_rule_brandom_4848495z_s3");
        assert_eq!(Rule::from_str(&seeded.to_str()).unwrap(), seeded);
    }
    #[test]
    fn a_wide_mask_run_replays_from_its_name() {
        let mask = wide_mask(7);
        let rule = Rule::new(
            Counts::drawn(Sequence::Fibonacci, false, false),
            Counts::drawn(Sequence::Primes, false, false),
            Boundary::Wrap,
        )
        .unwrap();
        let mut config = rule.config(mask.clone());
        config.max_generations = 12;
        assert_eq!(config.budget(), 48);
        let (birth, survive) = config.counts().unwrap();
        assert!(birth.iter().any(|&n| n > MAX_COUNT), "{birth:?}");
        assert!(survive.iter().any(|&n| n > MAX_COUNT), "{survive:?}");
        let mut seed = Tensor::new(vec![15, 15]);
        for (y, x) in [(6, 7), (7, 6), (7, 7), (7, 8), (8, 7)] {
            seed.set(&[y, x], 1);
        }
        let seed = Cell2d::new(seed);
        let mut story = Story::new();
        story.add(&seed, &config).unwrap();
        let back = Story::from_json(&story.to_json().unwrap()).unwrap();
        assert_eq!(Rule::of(&back.chapters[0].config).unwrap(), rule);
        assert_eq!(back.chapters[0].config.counts().unwrap(), (birth, survive));
        for (a, b) in story.grids().iter().zip(back.grids()) {
            assert_eq!(a.types(), b.types());
        }
        assert!(story.grids().len() > 1);
    }
    #[test]
    fn the_moore_budget_stays_in_the_digits() {
        let config = Rule::new(vec![3], vec![2, 3], Boundary::Constant)
            .unwrap()
            .config(moore());
        assert_eq!(config.budget(), 8);
    }
    #[test]
    fn seeded_values_round_trip() {
        let mut rng = Rng::new(5);
        for _ in 0..500 {
            let draw = |rng: &mut Rng| {
                let count = rng.below(5);
                (0..count).map(|_| rng.below(10)).collect::<Vec<usize>>()
            };
            let boundary = if rng.boolean() {
                Boundary::Wrap
            } else {
                Boundary::Constant
            };
            let rule = Rule::new(draw(&mut rng), draw(&mut rng), boundary).unwrap();
            let name = rule.to_str();
            let back = Rule::from_str(&name).unwrap();
            assert_eq!(back.birth.values(9).unwrap(), rule.birth.values(9).unwrap());
            assert_eq!(
                back.survive.values(9).unwrap(),
                rule.survive.values(9).unwrap()
            );
            assert_eq!(back.boundary, rule.boundary);
            assert_eq!(back.to_str(), name);
        }
    }
    #[test]
    fn seeded_sequences_round_trip() {
        let mut rng = Rng::new(11);
        let pool = Sequence::all();
        for _ in 0..200 {
            let pick = |rng: &mut Rng| match rng.below(3) {
                0 => Sequence::Random(rng.range(0, i64::MAX) as u64),
                1 => Sequence::CodeFills(rng.below(16) as u128),
                _ => *rng.choice(&pool),
            };
            let side = |rng: &mut Rng| Counts::drawn(pick(rng), rng.boolean(), rng.boolean());
            let rule = Rule::new(side(&mut rng), side(&mut rng), Boundary::Constant).unwrap();
            let name = rule.to_str();
            assert_eq!(Rule::from_str(&name).unwrap(), rule, "{name}");
        }
    }
}
