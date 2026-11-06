use crate::{Boundary, Config, Counts};
use mrly_error::Result;
use mrly_name::Named;
use mrly_name::kind;
use mrly_two::Cell2d;
use serde::{Deserialize, Serialize};

kind!("rule");

fn is_false(flag: &bool) -> bool {
    !flag
}

fn sorted(mut list: Vec<usize>) -> Vec<usize> {
    list.sort_unstable();
    list.dedup();
    list
}

fn fold(counts: Counts) -> Counts {
    match counts {
        Counts::List(list) => Counts::List(sorted(list)),
        drawn => drawn,
    }
}

mod counts {
    use crate::{Counts, Source};
    use serde::de::{Error, SeqAccess, Visitor};
    use serde::ser::SerializeSeq;
    use serde::{Deserializer, Serializer};
    use std::fmt;

    fn word(seq: Source, zeros: bool, ones: bool) -> String {
        let mut out = seq.name();
        if zeros {
            out.push_str("_zeros");
        }
        if ones {
            out.push_str("_ones");
        }
        out
    }

    pub(super) fn read(text: &str) -> Option<Counts> {
        let (seq, tail) = Source::read(text)?;
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

    pub(super) fn serialize<S: Serializer>(
        counts: &Counts,
        serializer: S,
    ) -> Result<S::Ok, S::Error> {
        match counts {
            Counts::List(list) => {
                let folded = super::sorted(list.clone());
                let mut seq = serializer.serialize_seq(Some(folded.len()))?;
                for n in folded {
                    seq.serialize_element(&n)?;
                }
                seq.end()
            }
            Counts::Drawn { seq, zeros, ones } => {
                serializer.serialize_str(&word(*seq, *zeros, *ones))
            }
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

    pub(super) fn deserialize<'de, D: Deserializer<'de>>(
        deserializer: D,
    ) -> Result<Counts, D::Error> {
        deserializer.deserialize_any(Side)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Rule {
    pub kind: Kind,
    #[serde(with = "counts")]
    pub birth: Counts,
    #[serde(with = "counts")]
    pub survive: Counts,
    #[serde(default, skip_serializing_if = "is_false")]
    pub wrap: bool,
}

impl Rule {
    pub fn new(birth: Counts, survive: Counts, wrap: bool) -> Rule {
        Rule {
            kind: Kind,
            birth: fold(birth),
            survive: fold(survive),
            wrap,
        }
    }
    pub fn of(config: &Config) -> Rule {
        Rule::new(
            config.birth.clone(),
            config.survive.clone(),
            config.boundary.wrap(),
        )
    }
    pub fn boundary(&self) -> Boundary {
        if self.wrap {
            Boundary::Wrap
        } else {
            Boundary::Constant
        }
    }
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
