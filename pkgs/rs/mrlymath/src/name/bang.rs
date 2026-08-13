use super::text;
use super::Named;
use mrlycore::errors::{value_error, Result};

/// A bang design code pinned to its dimension and numeral base.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Bang {
    /// The design's code.
    pub code: u128,
    /// The design's dimension.
    pub dimension: usize,
    /// The numeral base of the corners.
    pub base: usize,
}

impl Bang {
    /// Pins a code to its dimension and base.
    pub fn new(code: u128, dimension: usize, base: usize) -> Bang {
        Bang {
            code,
            dimension,
            base,
        }
    }
    fn cells(dimension: usize, base: usize) -> Result<u32> {
        if dimension < 1 {
            return value_error("dimension must be at least 1.");
        }
        if base < 2 {
            return value_error("base must be at least 2.");
        }
        match u32::try_from(base)
            .ok()
            .zip(u32::try_from(dimension).ok())
            .and_then(|(b, d)| b.checked_pow(d))
        {
            Some(cells) if cells < 128 => Ok(cells),
            _ => value_error(format!(
                "dimension {dimension} base {base} exceeds the u128 code space."
            )),
        }
    }
    fn fit(&self) -> Result<()> {
        let cells = Bang::cells(self.dimension, self.base)?;
        if self.code >> cells != 0 {
            return value_error(format!(
                "code {} out of range for dimension {} base {} (0..{}).",
                self.code,
                self.dimension,
                self.base,
                (1u128 << cells) - 1
            ));
        }
        Ok(())
    }
}

impl Named for Bang {
    fn to_str(&self) -> String {
        let mut fields = vec![text::run(&[('d', Some(self.dimension as u128))])];
        if self.base != 2 {
            fields.push(text::run(&[('q', Some(self.base as u128))]));
        }
        fields.push(self.code.to_string());
        text::compose("bang", &fields)
    }
    fn from_str(text: &str) -> Result<Bang> {
        let fields = text::split(text, "bang")?;
        if !(2..=3).contains(&fields.len()) {
            return value_error(format!(
                "bang name {text:?} wants fields d, optional q, then the code."
            ));
        }
        let dimension = match text::tags(fields[0])?.as_slice() {
            [('d', Some(d))] => text::small(*d)?,
            _ => return value_error(format!("bang name {text:?} opens without a d field.")),
        };
        let base = if fields.len() == 3 {
            match text::tags(fields[1])?.as_slice() {
                [('q', Some(2))] => {
                    return value_error(format!("bang name {text:?} spells the default base."))
                }
                [('q', Some(q))] => text::small(*q)?,
                _ => return value_error(format!("bang name {text:?} holds a stray field.")),
            }
        } else {
            2
        };
        let code = text::number(fields[fields.len() - 1])?;
        let bang = Bang::new(code, dimension, base);
        bang.fit()?;
        Ok(bang)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mrlycore::rng::Rng;

    #[test]
    fn base_two_elides() {
        assert_eq!(Bang::new(7, 2, 2).to_str(), "mrly_bang_d2_7");
        assert_eq!(Bang::new(23, 3, 2).to_str(), "mrly_bang_d3_23");
        assert_eq!(Bang::new(0, 2, 3).to_str(), "mrly_bang_d2_q3_0");
    }
    #[test]
    fn canonical_names_parse() {
        assert_eq!(
            Bang::from_str("mrly_bang_d2_7").unwrap(),
            Bang::new(7, 2, 2)
        );
        assert_eq!(
            Bang::from_str("mrly_bang_d2_q3_511").unwrap(),
            Bang::new(511, 2, 3)
        );
    }
    #[test]
    fn only_the_canonical_form_parses() {
        for bad in [
            "mrly_d2_b2_7",
            "mrly_07",
            "mrly_bang_7",
            "mrly_bang_d2_q2_7",
            "mrly_bang_d2_07",
            "mrly_bang_d2_16",
            "mrly_bang_d0_1",
            "mrly_bang_d2_7_",
            "MRLY_BANG_D2_7",
            "mrly_bang_d2_7x",
            "7",
        ] {
            assert!(Bang::from_str(bad).is_err(), "{bad}");
        }
    }
    #[test]
    fn seeded_values_round_trip() {
        let mut rng = Rng::new(11);
        for _ in 0..500 {
            let base = *rng.choice(&[2usize, 3]);
            let top = if base == 2 { 4 } else { 3 };
            let dimension = rng.range(1, top) as usize;
            let cells = (base as u32).pow(dimension as u32);
            let code = rng.below(1usize << cells) as u128;
            let bang = Bang::new(code, dimension, base);
            let name = bang.to_str();
            assert_eq!(Bang::from_str(&name).unwrap(), bang);
            assert_eq!(Bang::from_str(&name).unwrap().to_str(), name);
        }
    }
}
