use mrlycore::state::{boolean, choice, randint, sample};
use mrlymath::life::{Counts, Sequence};

/// One drawn rule: the sequences behind it and the parity rolls.
#[derive(Clone, Debug)]
pub struct Rulebook {
    /// The sequence behind the birth counts.
    pub birth_sequence: Sequence,
    /// The sequence behind the survive counts.
    pub survive_sequence: Sequence,
    /// Whether one sequence serves both sides.
    pub reflect: bool,
    /// Whether zero stays in the counts.
    pub zeros: bool,
    /// Whether one stays in the counts.
    pub ones: bool,
}

impl Rulebook {
    /// Returns the counts that create a cell.
    pub fn birth(&self) -> Counts {
        Counts::drawn(self.birth_sequence, self.zeros, self.ones)
    }
    /// Returns the counts that keep a cell.
    pub fn survive(&self) -> Counts {
        Counts::drawn(self.survive_sequence, self.zeros, self.ones)
    }
}

fn options(simple: bool) -> Vec<Sequence> {
    if simple {
        return Sequence::numbers().to_vec();
    }
    let mut both = Sequence::numbers().to_vec();
    both.extend(Sequence::designs());
    both.retain(|s| !s.is_random());
    both
}

fn sown(seq: Sequence) -> Sequence {
    if seq.is_random() {
        Sequence::Random(randint(0, i64::MAX) as u64)
    } else {
        seq
    }
}

/// Draws a rulebook, number sequences only on the simple path, random draws given a seed.
pub fn rulebook(simple: bool) -> Rulebook {
    let pool = options(simple);
    let reflect = boolean();
    let (birth_sequence, survive_sequence) = if reflect {
        let one = sown(choice(&pool));
        (one, one)
    } else {
        let pair = sample(&pool, 2);
        (sown(pair[0]), sown(pair[1]))
    };
    Rulebook {
        birth_sequence,
        survive_sequence,
        reflect,
        zeros: boolean(),
        ones: boolean(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mrlycore::state::{guard, seed};
    #[test]
    fn the_simple_tier_holds_the_six_numbers() {
        let pool = options(true);
        assert_eq!(pool.len(), 6);
        assert!(pool.iter().any(|s| s.is_random()));
    }
    #[test]
    fn the_full_tier_drops_random() {
        let pool = options(false);
        assert_eq!(pool.len(), 14);
        assert!(!pool.iter().any(|s| s.is_random()));
        assert!(pool.contains(&Sequence::GridSquares));
    }
    #[test]
    fn reflection_serves_one_sequence_to_both_sides() {
        let _g = guard();
        let mut mirrored = false;
        let mut split = false;
        for s in 0..50 {
            seed(s);
            let rule = rulebook(true);
            if rule.reflect {
                assert_eq!(rule.birth_sequence, rule.survive_sequence);
                mirrored = true;
            } else {
                assert_ne!(rule.birth_sequence, rule.survive_sequence);
                split = true;
            }
        }
        assert!(mirrored && split);
    }
    #[test]
    fn counts_respect_the_budget_and_the_rolls() {
        let _g = guard();
        for s in 0..50 {
            seed(s);
            let rule = rulebook(false);
            let birth = rule.birth().values(8).unwrap();
            let survive = rule.survive().values(8).unwrap();
            for n in birth.iter().chain(&survive) {
                assert!(*n <= 8);
                if *n == 0 {
                    assert!(rule.zeros);
                }
                if *n == 1 {
                    assert!(rule.ones);
                }
            }
        }
    }
    #[test]
    fn a_random_draw_carries_its_own_seed() {
        let _g = guard();
        for s in 0..200 {
            seed(s);
            let rule = rulebook(true);
            if let Sequence::Random(inner) = rule.birth_sequence {
                assert_ne!(inner, 0, "the listed placeholder seed leaked");
                assert_eq!(
                    rule.birth().values(8).unwrap(),
                    Counts::drawn(Sequence::Random(inner), rule.zeros, rule.ones)
                        .values(8)
                        .unwrap()
                );
                return;
            }
        }
        panic!("no seed drew the random sequence");
    }
    #[test]
    fn the_draw_replays_from_a_seed() {
        let _g = guard();
        seed(42);
        let a = rulebook(true);
        seed(42);
        let b = rulebook(true);
        assert_eq!(a.birth_sequence, b.birth_sequence);
        assert_eq!(a.survive_sequence, b.survive_sequence);
        assert_eq!(a.birth().values(8).unwrap(), b.birth().values(8).unwrap());
        assert_eq!(a.reflect, b.reflect);
    }
}
