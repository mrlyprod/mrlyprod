use super::{kind, Bang, Named};
use crate::core::error::{value_error, Result};
use serde::{Deserialize, Serialize};

kind!("word");

/// A magic word: an ordered list of design letters, first letter outermost, each at its own side.
///
/// ```
/// use mrlyrs::math::name::{Named, Word};
/// let word = Word::new(2, &[(7, 3), (14, 7), (9, 5)]).unwrap();
/// assert_eq!(word.to_json(), r#"{"kind":"word","dim":2,"magic":[7,14,9],"side":[3,7,5]}"#);
/// assert_eq!(Word::from_json(&word.to_json()).unwrap(), word);
/// ```
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Word {
    /// The kind word.
    pub kind: Kind,
    /// The number of axes every letter shares.
    pub dim: usize,
    /// The codes of the letters in order.
    pub magic: Vec<u128>,
    /// The side each letter renders at.
    pub side: Vec<usize>,
    /// The base of each letter, absent when every letter is base 2.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub base: Option<Vec<usize>>,
}

impl Word {
    /// Pins an ordered letter list at base 2, or an error below two letters or outside the code range.
    pub fn new(dim: usize, letters: &[(u128, usize)]) -> Result<Word> {
        Word {
            kind: Kind,
            dim,
            magic: letters.iter().map(|&(code, _)| code).collect(),
            side: letters.iter().map(|&(_, side)| side).collect(),
            base: None,
        }
        .checked()
    }
    /// Returns the base of every letter, 2 where the name says nothing.
    pub fn bases(&self) -> Vec<usize> {
        self.base
            .clone()
            .unwrap_or_else(|| vec![2; self.magic.len()])
    }
    /// Returns every letter as a design pinned to the word's dimension and its own base.
    pub fn letters(&self) -> Vec<Bang> {
        self.magic
            .iter()
            .zip(self.bases())
            .map(|(&code, base)| Bang::new(code, self.dim, base))
            .collect()
    }
}

impl Named for Word {
    const KIND: &'static str = "word";
    const LISTS: &'static [&'static str] = &["magic", "side", "base"];
    fn checked(mut self) -> Result<Word> {
        if self.magic.len() < 2 {
            return value_error("a word needs at least two letters.");
        }
        if self.side.len() != self.magic.len() {
            return value_error(format!(
                "a word of {} letters wants {} sides, not {}.",
                self.magic.len(),
                self.magic.len(),
                self.side.len()
            ));
        }
        if let Some(base) = &self.base {
            if base.len() != self.magic.len() {
                return value_error(format!(
                    "a word of {} letters wants {} bases, not {}.",
                    self.magic.len(),
                    self.magic.len(),
                    base.len()
                ));
            }
            if base.iter().all(|&b| b == 2) {
                self.base = None;
            }
        }
        for letter in self.letters() {
            if letter.code == 0 {
                return value_error("a letter code of 0 draws nothing.");
            }
            letter.checked()?;
        }
        if let Some(side) = self.side.iter().find(|&&side| side < 2) {
            return value_error(format!("letter side {side} is below two."));
        }
        Ok(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_word_holds_through_every_view() {
        let word = Word::new(2, &[(7, 3), (14, 7), (9, 5)]).unwrap();
        assert_eq!(
            word.to_json(),
            r#"{"kind":"word","dim":2,"magic":[7,14,9],"side":[3,7,5]}"#
        );
        assert_eq!(
            word.to_url().unwrap(),
            "/word?dim=2&magic=7,14,9&side=3,7,5"
        );
        assert_eq!(
            word.to_file().unwrap(),
            "word_dim=2_magic=[7,14,9]_side=[3,7,5]"
        );
        assert_eq!(
            word.to_mrly().unwrap(),
            "word dim 2, magic [7 14 9], side [3 7 5]"
        );
        assert_eq!(Word::from_json(&word.to_json()).unwrap(), word);
        assert_eq!(Word::from_url(&word.to_url().unwrap()).unwrap(), word);
        assert_eq!(Word::from_file(&word.to_file().unwrap()).unwrap(), word);
        assert_eq!(word.to_id().len(), 8);
    }
    #[test]
    fn the_word_name_round_trips() {
        for letters in [vec![(3u128, 2usize), (6, 2)], vec![(7, 3), (14, 7), (9, 5)]] {
            let word = Word::new(2, &letters).unwrap();
            assert_eq!(Word::from_json(&word.to_json()).unwrap(), word);
        }
    }
    #[test]
    fn order_shows_in_the_name() {
        let one = Word::new(2, &[(3, 2), (6, 2)]).unwrap().to_json();
        let other = Word::new(2, &[(6, 2), (3, 2)]).unwrap().to_json();
        assert_ne!(one, other);
    }
    #[test]
    fn a_solid_or_mixed_base_word_has_a_name() {
        let solid = Word::new(3, &[(23, 3), (3, 3)]).unwrap();
        assert_eq!(
            solid.to_json(),
            r#"{"kind":"word","dim":3,"magic":[23,3],"side":[3,3]}"#
        );
        let mixed = Word {
            base: Some(vec![2, 3]),
            ..Word::new(2, &[(7, 3), (14, 3)]).unwrap()
        };
        let mixed = Word {
            magic: vec![7, 98],
            ..mixed
        }
        .checked()
        .unwrap();
        assert_eq!(
            mixed.to_json(),
            r#"{"kind":"word","dim":2,"magic":[7,98],"side":[3,3],"base":[2,3]}"#
        );
        assert_eq!(Word::from_json(&mixed.to_json()).unwrap(), mixed);
        assert_eq!(mixed.letters()[1], Bang::new(98, 2, 3));
        let mut plain = Word::new(2, &[(7, 3), (14, 3)]).unwrap();
        plain.base = Some(vec![2, 2]);
        assert_eq!(plain.checked().unwrap().base, None);
    }
    #[test]
    fn refuses_every_word_the_grammar_cannot_draw() {
        assert!(Word::new(2, &[(7, 3)]).is_err());
        assert!(Word::new(2, &[(273, 3), (9, 2)]).is_err());
        assert!(Word::new(2, &[(0, 3), (9, 2)]).is_err());
        assert!(Word::new(2, &[(7, 1), (9, 2)]).is_err());
        assert!(Word::new(0, &[(1, 2), (1, 2)]).is_err());
        for bad in [
            r#"{"kind":"word","dim":2,"magic":[7,14],"side":[3]}"#,
            r#"{"kind":"word","dim":2,"magic":[7,14],"side":[3,3],"base":[3]}"#,
            r#"{"kind":"word","dim":2,"magic":[7,98],"side":[3,3]}"#,
            r#"{"kind":"word","magic":[7,14],"side":[3,3]}"#,
            r#"{"kind":"word","dim":2,"magic":[7,14],"side":[3,3],"level":2}"#,
            r#"{"kind":"bang","dim":2,"code":7}"#,
            "word dim 2, magic [7 9], side [3 5]",
            "word_dim=2_magic=[7,9]_side=[3,5]",
        ] {
            assert!(Word::from_json(bad).is_err(), "{bad}");
        }
    }
}
