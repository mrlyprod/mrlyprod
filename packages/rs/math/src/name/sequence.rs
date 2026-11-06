use super::Bang;
use mrly_error::{Result, value_error};
use mrly_name::Named;
use mrly_name::kind;
use serde::{Deserialize, Serialize};

kind!("sequence");

const MEASURES: [&str; 15] = [
    "fills",
    "voids",
    "surface",
    "peak",
    "heights",
    "vertices",
    "edges",
    "faces",
    "euler",
    "triangles",
    "holes",
    "pieces",
    "gaps",
    "loops",
    "windows",
];

const AXES: [&str; 3] = ["level", "side", "window"];

fn two() -> usize {
    2
}

fn is_two(base: &usize) -> bool {
    *base == 2
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Sequence {
    pub kind: Kind,
    pub dim: usize,
    #[serde(default = "two", skip_serializing_if = "is_two")]
    pub base: usize,
    pub code: u128,
    pub measure: String,
    pub axis: String,
}

impl Sequence {
    pub fn new(code: u128, dim: usize, base: usize, measure: &str, axis: &str) -> Sequence {
        Sequence {
            kind: Kind,
            dim,
            base,
            code,
            measure: measure.to_string(),
            axis: axis.to_string(),
        }
    }
    pub fn design(&self) -> Bang {
        Bang::new(self.code, self.dim, self.base)
    }
}

impl Named for Sequence {
    const KIND: &'static str = "sequence";
    const BARE: &'static [&'static str] = &["measure", "axis"];
    fn checked(self) -> Result<Sequence> {
        self.design().checked()?;
        if !MEASURES.contains(&self.measure.as_str()) {
            return value_error(format!("unknown measure {:?}.", self.measure));
        }
        if !AXES.contains(&self.axis.as_str()) {
            return value_error(format!("unknown axis {:?}.", self.axis));
        }
        Ok(self)
    }
}
