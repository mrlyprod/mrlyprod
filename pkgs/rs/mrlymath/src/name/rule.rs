use super::text;
use super::Named;
use crate::life::{Boundary, Config};
use crate::two::Cell2d;
use mrlycore::errors::{value_error, Result};

const MAX_COUNT: usize = 8;

/// A life rule: the birth and survival counts and the edge policy.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Rule {
    /// The neighbor counts that create a cell.
    pub birth: Vec<usize>,
    /// The neighbor counts that keep a cell.
    pub survive: Vec<usize>,
    /// The edge policy.
    pub boundary: Boundary,
}

impl Rule {
    /// Builds a rule from its counts and edge policy.
    pub fn new(birth: Vec<usize>, survive: Vec<usize>, boundary: Boundary) -> Rule {
        Rule {
            birth,
            survive,
            boundary,
        }
    }
    /// Reads the rule out of a life config.
    pub fn of(config: &Config) -> Rule {
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
        let mut out: Vec<usize> = counts.iter().copied().filter(|&n| n <= MAX_COUNT).collect();
        out.sort_unstable();
        out.dedup();
        out
    }
    fn digits(counts: &[usize]) -> String {
        Rule::fold(counts).iter().map(|n| n.to_string()).collect()
    }
    fn counts(field: &str, tag: char) -> Result<Vec<usize>> {
        let Some(digits) = field.strip_prefix(tag) else {
            return value_error(format!("field {field:?} does not open with {tag:?}."));
        };
        let mut out = Vec::new();
        for c in digits.chars() {
            let Some(n) = c.to_digit(10).map(|n| n as usize) else {
                return value_error(format!("field {field:?} holds a stray {c:?}."));
            };
            if n > MAX_COUNT {
                return value_error(format!("count {n} leaves 0..8."));
            }
            if out.last().is_some_and(|&last| last >= n) {
                return value_error(format!("field {field:?} is not strictly ascending."));
            }
            out.push(n);
        }
        Ok(out)
    }
}

impl Named for Rule {
    fn to_str(&self) -> String {
        let mut fields = vec![
            format!("b{}", Rule::digits(&self.birth)),
            format!("s{}", Rule::digits(&self.survive)),
        ];
        if self.boundary == Boundary::Wrap {
            fields.push("w".to_string());
        }
        text::compose("rule", &fields)
    }
    fn from_str(text: &str) -> Result<Rule> {
        let fields = text::split(text, "rule")?;
        if !(2..=3).contains(&fields.len()) {
            return value_error(format!(
                "rule name {text:?} wants fields b, s, then an optional w."
            ));
        }
        let birth = Rule::counts(fields[0], 'b')?;
        let survive = Rule::counts(fields[1], 's')?;
        let boundary = if fields.len() == 3 {
            if fields[2] != "w" {
                return value_error(format!("rule name {text:?} holds a stray field."));
            }
            Boundary::Wrap
        } else {
            Boundary::Constant
        };
        Ok(Rule::new(birth, survive, boundary))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mrlycore::rng::Rng;

    #[test]
    fn conway_is_b3_s23() {
        let conway = Rule::new(vec![3], vec![2, 3], Boundary::Constant);
        assert_eq!(conway.to_str(), "mrly_rule_b3_s23");
        assert_eq!(Rule::from_str("mrly_rule_b3_s23").unwrap(), conway);
        let wrapped = Rule::new(vec![3], vec![2, 3], Boundary::Wrap);
        assert_eq!(wrapped.to_str(), "mrly_rule_b3_s23_w");
    }
    #[test]
    fn to_str_folds_to_the_canonical_counts() {
        let messy = Rule::new(vec![3, 3, 1], vec![9, 2], Boundary::Constant);
        assert_eq!(messy.to_str(), "mrly_rule_b13_s2");
        let empty = Rule::new(Vec::new(), Vec::new(), Boundary::Constant);
        assert_eq!(empty.to_str(), "mrly_rule_b_s");
        assert_eq!(Rule::from_str("mrly_rule_b_s").unwrap(), empty);
    }
    #[test]
    fn only_the_canonical_form_parses() {
        for bad in [
            "mrly_rule_b33_s2",
            "mrly_rule_b31_s2",
            "mrly_rule_b9_s2",
            "mrly_rule_s23_b3",
            "mrly_rule_b3",
            "mrly_rule_b3_s23_x",
            "mrly_rule_b3_s23_w_w",
            "b3_s23",
        ] {
            assert!(Rule::from_str(bad).is_err(), "{bad}");
        }
    }
    #[test]
    fn config_round_trips_through_the_rule() {
        let mask = crate::two::designs::ones(3, 1).unwrap();
        let rule = Rule::new(vec![3, 6], vec![2, 3], Boundary::Wrap);
        let config = rule.config(mask);
        assert_eq!(Rule::of(&config), rule);
        assert_eq!(Rule::of(&config).to_str(), "mrly_rule_b36_s23_w");
    }
    #[test]
    fn seeded_values_round_trip() {
        let mut rng = Rng::new(5);
        for _ in 0..500 {
            let draw = |rng: &mut Rng| {
                let count = rng.below(5);
                (0..count).map(|_| rng.below(9)).collect::<Vec<usize>>()
            };
            let boundary = if rng.boolean() {
                Boundary::Wrap
            } else {
                Boundary::Constant
            };
            let rule = Rule::new(draw(&mut rng), draw(&mut rng), boundary);
            let name = rule.to_str();
            let back = Rule::from_str(&name).unwrap();
            assert_eq!(back.birth, Rule::fold(&rule.birth));
            assert_eq!(back.survive, Rule::fold(&rule.survive));
            assert_eq!(back.boundary, rule.boundary);
            assert_eq!(back.to_str(), name);
        }
    }
}
