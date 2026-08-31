use super::text;
use super::Named;
use mrlycore::errors::{value_error, Result};

/// A magic word name: an ordered list of plane letters at base two, first letter outermost.
///
/// The spelling is `mrly_word_d2_c<code>n<side>_c<code>n<side>` and it covers the plane at base
/// two alone, since the tile grammar caps codes at the corner range; a solid or base-q word still
/// has no canonical name.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Word {
    /// The letters in order, each a code and the side it renders at.
    pub letters: Vec<(u128, usize)>,
}

impl Word {
    /// Pins an ordered letter list, or an error below two letters or outside the plane corner range.
    ///
    /// ```
    /// use mrlymath::name::{Named, Word};
    /// let word = Word::new(&[(7, 3), (14, 7), (9, 5)]).unwrap();
    /// assert_eq!(word.to_str(), "mrly_word_d2_c7n3_c14n7_c9n5");
    /// assert_eq!(Word::from_str("mrly_word_d2_c7n3_c14n7_c9n5").unwrap(), word);
    /// ```
    pub fn new(letters: &[(u128, usize)]) -> Result<Word> {
        if letters.len() < 2 {
            return value_error("a word name needs at least two letters.");
        }
        for (code, side) in letters {
            if *code == 0 || *code > 15 {
                return value_error(format!(
                    "letter code {code} lies outside the plane corner range 1..15."
                ));
            }
            if *side < 2 {
                return value_error(format!("letter side {side} is below two."));
            }
        }
        Ok(Word {
            letters: letters.to_vec(),
        })
    }
}

impl Named for Word {
    fn to_str(&self) -> String {
        let mut fields = vec![text::run(&[('d', Some(2))])];
        for (code, side) in &self.letters {
            fields.push(text::run(&[('c', Some(*code)), ('n', Some(*side as u128))]));
        }
        text::compose("word", &fields)
    }
    fn from_str(text: &str) -> Result<Word> {
        let fields = super::text::split(text, "word")?;
        if fields.len() < 3 {
            return value_error(format!(
                "word name {text:?} wants a d field and two or more letters."
            ));
        }
        match super::text::tags(fields[0])?.as_slice() {
            [('d', Some(2))] => {}
            _ => {
                return value_error(format!(
                    "word name {text:?} opens outside the plane, which has no name kind yet."
                ))
            }
        }
        let mut letters = Vec::new();
        for field in &fields[1..] {
            match super::text::tags(field)?.as_slice() {
                [('c', Some(code)), ('n', Some(side))] => {
                    letters.push((*code, super::text::small(*side)?))
                }
                _ => {
                    return value_error(format!(
                        "letter {field:?} wants a c field then an n field."
                    ))
                }
            }
        }
        Word::new(&letters)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_word_name_round_trips() {
        for letters in [vec![(3u128, 2usize), (6, 2)], vec![(7, 3), (14, 7), (9, 5)]] {
            let word = Word::new(&letters).unwrap();
            assert_eq!(Word::from_str(&word.to_str()).unwrap(), word);
        }
    }

    #[test]
    fn order_shows_in_the_name() {
        let one = Word::new(&[(3, 2), (6, 2)]).unwrap().to_str();
        let other = Word::new(&[(6, 2), (3, 2)]).unwrap().to_str();
        assert_ne!(one, other);
    }

    #[test]
    fn the_grammar_refuses_what_it_cannot_spell() {
        assert!(Word::new(&[(7, 3)]).is_err());
        assert!(Word::new(&[(273, 3), (9, 2)]).is_err());
        assert!(Word::from_str("mrly_word_d3_c7n3_c9n5").is_err());
        assert!(Word::from_str("mrly_word_d2_c7n3").is_err());
        assert!(Word::from_str("mrly_bang_d2_7").is_err());
    }
}
