use super::{kind, Named};
use crate::life::{Boundary, Config, Counts};
use crate::two::Cell2d;
use mrlycore::errors::Result;
use serde::{Deserialize, Serialize};

kind!("rule");

fn is_false(flag: &bool) -> bool {
    !flag
}

fn fold(counts: Counts) -> Counts {
    match counts {
        Counts::List(mut list) => {
            list.sort_unstable();
            list.dedup();
            Counts::List(list)
        }
        drawn => drawn,
    }
}

mod counts {
    use crate::life::{Counts, Sequence};
    use serde::de::{Error, SeqAccess, Visitor};
    use serde::ser::SerializeSeq;
    use serde::{Deserializer, Serializer};
    use std::fmt;

    pub fn spell(counts: &Counts) -> Option<String> {
        let Counts::Drawn { seq, zeros, ones } = counts else {
            return None;
        };
        let mut out = seq.name();
        if *zeros {
            out.push_str("_zeros");
        }
        if *ones {
            out.push_str("_ones");
        }
        Some(out)
    }

    pub fn read(text: &str) -> Option<Counts> {
        let (seq, tail) = Sequence::read(text)?;
        let (zeros, tail) = match tail.strip_prefix("_zeros") {
            Some(rest) => (true, rest),
            None => (false, tail),
        };
        let (ones, tail) = match tail.strip_prefix("_ones") {
            Some(rest) => (true, rest),
            None => (false, tail),
        };
        tail.is_empty().then(|| Counts::drawn(seq, zeros, ones))
    }

    pub fn serialize<S: Serializer>(counts: &Counts, serializer: S) -> Result<S::Ok, S::Error> {
        match counts {
            Counts::List(list) => {
                let folded = super::fold(Counts::List(list.clone()));
                let Counts::List(folded) = folded else {
                    unreachable!()
                };
                let mut seq = serializer.serialize_seq(Some(folded.len()))?;
                for n in folded {
                    seq.serialize_element(&n)?;
                }
                seq.end()
            }
            drawn => serializer.serialize_str(&spell(drawn).expect("a drawn side spells")),
        }
    }

    struct Side;

    impl<'de> Visitor<'de> for Side {
        type Value = Counts;
        fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
            f.write_str("a list of counts or a sequence word")
        }
        fn visit_seq<A: SeqAccess<'de>>(self, mut seq: A) -> Result<Counts, A::Error> {
            let mut list = Vec::new();
            while let Some(n) = seq.next_element::<usize>()? {
                list.push(n);
            }
            Ok(Counts::List(list))
        }
        fn visit_str<E: Error>(self, text: &str) -> Result<Counts, E> {
            read(text).ok_or_else(|| E::custom(format!("sequence {text:?} is not known.")))
        }
    }

    pub fn deserialize<'de, D: Deserializer<'de>>(deserializer: D) -> Result<Counts, D::Error> {
        deserializer.deserialize_any(Side)
    }
}

/// A life rule: the birth and survival counts and whether the edge wraps.
///
/// ```
/// use mrlymath::name::{Named, Rule};
/// let conway = Rule::new(vec![3], vec![2, 3], false);
/// assert_eq!(conway.to_json(), r#"{"kind":"rule","birth":[3],"survive":[2,3]}"#);
/// assert_eq!(Rule::from_json(&conway.to_json()).unwrap(), conway);
/// ```
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Rule {
    /// The kind word.
    pub kind: Kind,
    /// The neighbor counts that create a cell, listed or drawn from a sequence.
    #[serde(with = "counts")]
    pub birth: Counts,
    /// The neighbor counts that keep a cell, listed or drawn from a sequence.
    #[serde(with = "counts")]
    pub survive: Counts,
    /// Whether the edge wraps, false unless said.
    #[serde(default, skip_serializing_if = "is_false")]
    pub wrap: bool,
}

impl Rule {
    /// Builds a rule from its counts and edge policy, listed counts folded to a sorted set.
    pub fn new(birth: impl Into<Counts>, survive: impl Into<Counts>, wrap: bool) -> Rule {
        Rule {
            kind: Kind,
            birth: fold(birth.into()),
            survive: fold(survive.into()),
            wrap,
        }
    }
    /// Reads the rule out of a life config.
    pub fn of(config: &Config) -> Rule {
        Rule::new(
            config.birth.clone(),
            config.survive.clone(),
            config.boundary.wrap(),
        )
    }
    /// Returns the edge policy the rule runs under.
    pub fn boundary(&self) -> Boundary {
        if self.wrap {
            Boundary::Wrap
        } else {
            Boundary::Constant
        }
    }
    /// Builds a life config running this rule over a neighborhood mask.
    pub fn config(&self, mask: Cell2d) -> Config {
        let mut config = Config::new(mask, self.birth.clone(), self.survive.clone());
        config.boundary = self.boundary();
        config
    }
}

impl Named for Rule {
    const KIND: &'static str = "rule";
    const LISTS: &'static [&'static str] = &["birth", "survive"];
    fn checked(self) -> Result<Rule> {
        Ok(Rule::new(self.birth, self.survive, self.wrap))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::life::{animate, moore, Sequence};
    use mrlycore::rng::Rng;
    use mrlycore::tensor::Tensor;

    const CONWAY: &str = r#"{"kind":"rule","birth":[3],"survive":[2,3]}"#;

    fn wide_mask(side: usize) -> Cell2d {
        let mut mask = Tensor::full(vec![side, side], 1);
        mask.set(&[side / 2, side / 2], 0);
        Cell2d::new(mask)
    }

    #[test]
    fn conway_holds_through_every_view() {
        let conway = Rule::new(vec![3], vec![2, 3], false);
        assert_eq!(conway.to_json(), CONWAY);
        assert_eq!(Rule::from_json(CONWAY).unwrap(), conway);
        assert_eq!(conway.to_url(), "/rule?birth=3&survive=2,3");
        assert_eq!(conway.to_file(), "rule_birth=[3]_survive=[2,3]");
        assert_eq!(conway.to_mrly(), "rule birth [3], survive [2 3]");
        assert_eq!(Rule::from_url(&conway.to_url()).unwrap(), conway);
        assert_eq!(Rule::from_file(&conway.to_file()).unwrap(), conway);
        assert_eq!(conway.to_id().len(), 8);
        let wrapped = Rule::new(vec![3], vec![2, 3], true);
        assert_eq!(
            wrapped.to_json(),
            r#"{"kind":"rule","birth":[3],"survive":[2,3],"wrap":true}"#
        );
        assert_eq!(wrapped.to_mrly(), "rule birth [3], survive [2 3], wrap");
        assert_ne!(wrapped.to_id(), conway.to_id());
    }
    #[test]
    fn the_wide_row_holds() {
        let wide = Rule::new(
            vec![12, 13],
            Counts::drawn(Sequence::Fibonacci, false, false),
            true,
        );
        let text = r#"{"kind":"rule","birth":[12,13],"survive":"fibonacci","wrap":true}"#;
        assert_eq!(wide.to_json(), text);
        assert_eq!(Rule::from_json(text).unwrap(), wide);
        assert_eq!(
            wide.to_url(),
            "/rule?birth=12,13&survive=fibonacci&wrap=true"
        );
        assert_eq!(
            wide.to_file(),
            "rule_birth=[12,13]_survive=fibonacci_wrap=true"
        );
        assert_eq!(
            wide.to_mrly(),
            "rule birth [12 13], survive fibonacci, wrap"
        );
        assert_eq!(Rule::from_url(&wide.to_url()).unwrap(), wide);
        assert_eq!(Rule::from_file(&wide.to_file()).unwrap(), wide);
    }
    #[test]
    fn to_json_folds_to_the_canonical_counts() {
        let messy = Rule::new(vec![3, 3, 1], vec![9, 2], false);
        assert_eq!(
            messy.to_json(),
            r#"{"kind":"rule","birth":[1,3],"survive":[2,9]}"#
        );
        let empty = Rule::new(Vec::new(), Vec::new(), false);
        assert_eq!(
            empty.to_json(),
            r#"{"kind":"rule","birth":[],"survive":[]}"#
        );
        assert_eq!(Rule::from_json(&empty.to_json()).unwrap(), empty);
        assert_eq!(Rule::from_url("/rule?birth=&survive=").unwrap(), empty);
        assert_eq!(Rule::from_file("rule_birth=[]_survive=[]").unwrap(), empty);
        let spelt =
            Rule::from_json(r#"{"kind":"rule","survive":[3,2,3],"birth":[3],"wrap":false}"#)
                .unwrap();
        assert_eq!(spelt, Rule::new(vec![3], vec![2, 3], false));
        assert_eq!(spelt.to_json(), CONWAY);
    }
    #[test]
    fn only_a_rule_parses() {
        for bad in [
            r#"{"kind":"bang","birth":[3],"survive":[2,3]}"#,
            r#"{"birth":[3],"survive":[2,3]}"#,
            r#"{"kind":"rule","birth":[3]}"#,
            r#"{"kind":"rule","birth":[3],"survive":[2,3],"wrap":1}"#,
            r#"{"kind":"rule","birth":[3],"survive":[2,3],"mask":7}"#,
            r#"{"kind":"rule","birth":"fib","survive":[3]}"#,
            r#"{"kind":"rule","birth":"random","survive":[3]}"#,
            r#"{"kind":"rule","birth":"random_007","survive":[3]}"#,
            r#"{"kind":"rule","birth":"fibonacci_zeros_zeros","survive":[3]}"#,
            r#"{"kind":"rule","birth":"fibonacci_ones_zeros","survive":[3]}"#,
            r#"{"kind":"rule","birth":"fibonacciq","survive":[3]}"#,
            r#"{"kind":"rule","birth":[-1],"survive":[3]}"#,
            r#"{"kind":"rule","birth":3,"survive":[3]}"#,
            "rule birth [3], survive [2 3]",
            "rule_birth=[3]_survive=[2,3]",
        ] {
            assert!(Rule::from_json(bad).is_err(), "{bad}");
        }
    }
    #[test]
    fn a_listed_count_above_nine_has_a_name() {
        let rule = Rule::new(vec![3, 12], vec![2, 3, 48], true);
        assert_eq!(
            rule.to_json(),
            r#"{"kind":"rule","birth":[3,12],"survive":[2,3,48],"wrap":true}"#
        );
        assert_eq!(Rule::from_json(&rule.to_json()).unwrap(), rule);
        let mut config = Config::new(moore(), vec![3], vec![2, 3]);
        config.survive = Counts::List(vec![48]);
        assert_eq!(Rule::of(&config).survive, Counts::List(vec![48]));
    }
    #[test]
    fn config_round_trips_through_the_rule() {
        let mask = crate::two::designs::ones(3, 1).unwrap();
        let rule = Rule::new(vec![3, 6], vec![2, 3], true);
        let config = rule.config(mask);
        assert_eq!(config.boundary, Boundary::Wrap);
        assert_eq!(Rule::of(&config), rule);
        assert_eq!(
            Rule::of(&config).to_json(),
            r#"{"kind":"rule","birth":[3,6],"survive":[2,3],"wrap":true}"#
        );
    }
    #[test]
    fn a_drawn_rule_names_its_sequence() {
        let rule = Rule::new(
            Counts::drawn(Sequence::Fibonacci, false, true),
            Counts::drawn(Sequence::GridSquares, false, false),
            true,
        );
        assert_eq!(
            rule.to_json(),
            r#"{"kind":"rule","birth":"fibonacci_ones","survive":"grid_squares","wrap":true}"#
        );
        assert_eq!(Rule::from_json(&rule.to_json()).unwrap(), rule);
        assert_eq!(Rule::from_url(&rule.to_url()).unwrap(), rule);
        assert_eq!(Rule::from_file(&rule.to_file()).unwrap(), rule);
        let seeded = Rule::new(
            Counts::drawn(Sequence::Random(4848495), true, false),
            vec![3],
            false,
        );
        assert_eq!(
            seeded.to_json(),
            r#"{"kind":"rule","birth":"random_4848495_zeros","survive":[3]}"#
        );
        assert_eq!(Rule::from_json(&seeded.to_json()).unwrap(), seeded);
        assert_eq!(Rule::from_file(&seeded.to_file()).unwrap(), seeded);
        assert_eq!(
            seeded.to_file(),
            "rule_birth=random_4848495_zeros_survive=[3]"
        );
    }
    #[test]
    fn a_wide_mask_run_replays_from_its_name() {
        let mask = wide_mask(7);
        let rule = Rule::new(
            Counts::drawn(Sequence::Fibonacci, false, false),
            Counts::drawn(Sequence::Primes, false, false),
            true,
        );
        let mut config = rule.config(mask.clone());
        config.max_generations = 12;
        assert_eq!(config.budget(), 48);
        let (birth, survive) = config.counts().unwrap();
        assert!(birth.iter().any(|&n| n > 9), "{birth:?}");
        assert!(survive.iter().any(|&n| n > 9), "{survive:?}");
        let mut seed = Tensor::new(vec![15, 15]);
        for (y, x) in [(6, 7), (7, 6), (7, 7), (7, 8), (8, 7)] {
            seed.set(&[y, x], 1);
        }
        let seed = Cell2d::new(seed);
        let back = Rule::from_json(&Rule::of(&config).to_json()).unwrap();
        assert_eq!(back, rule);
        let mut replay = back.config(mask);
        replay.max_generations = 12;
        assert_eq!(replay.counts().unwrap(), (birth, survive));
        let run = animate(&seed, &config).unwrap();
        let again = animate(&seed, &replay).unwrap();
        for (a, b) in run.grids.iter().zip(&again.grids) {
            assert_eq!(a.types(), b.types());
        }
        assert!(run.grids.len() > 1);
    }
    #[test]
    fn the_moore_budget_stays_in_the_digits() {
        let config = Rule::new(vec![3], vec![2, 3], false).config(moore());
        assert_eq!(config.budget(), 8);
    }
    #[test]
    fn seeded_values_round_trip() {
        let mut rng = Rng::new(5);
        for _ in 0..500 {
            let draw = |rng: &mut Rng| {
                let count = rng.below(5);
                (0..count).map(|_| rng.below(50)).collect::<Vec<usize>>()
            };
            let rule = Rule::new(draw(&mut rng), draw(&mut rng), rng.boolean());
            let text = rule.to_json();
            let back = Rule::from_json(&text).unwrap();
            assert_eq!(
                back.birth.values(49).unwrap(),
                rule.birth.values(49).unwrap()
            );
            assert_eq!(
                back.survive.values(49).unwrap(),
                rule.survive.values(49).unwrap()
            );
            assert_eq!(back.wrap, rule.wrap);
            assert_eq!(back.to_json(), text);
            assert_eq!(Rule::from_url(&rule.to_url()).unwrap(), rule);
            assert_eq!(Rule::from_file(&rule.to_file()).unwrap(), rule);
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
            let rule = Rule::new(side(&mut rng), side(&mut rng), false);
            let text = rule.to_json();
            assert_eq!(Rule::from_json(&text).unwrap(), rule, "{text}");
            assert_eq!(Rule::from_url(&rule.to_url()).unwrap(), rule, "{text}");
            assert_eq!(Rule::from_file(&rule.to_file()).unwrap(), rule, "{text}");
        }
    }
}
