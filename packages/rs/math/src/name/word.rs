use super::Bang;
use mrly_error::{Result, value_error};
use mrly_name::Named;
use mrly_name::kind;
use serde::{Deserialize, Serialize};

kind!("word");

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Word {
    pub kind: Kind,
    pub dim: usize,
    pub magic: Vec<u128>,
    pub side: Vec<usize>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub base: Option<Vec<usize>>,
}

impl Word {
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
    pub fn bases(&self) -> Vec<usize> {
        self.base
            .clone()
            .unwrap_or_else(|| vec![2; self.magic.len()])
    }
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
