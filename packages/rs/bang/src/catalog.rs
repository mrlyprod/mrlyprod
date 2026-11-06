use super::universe::bang;
use mrly_error::Result;
use mrly_math::catalog::{Design, classics};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::sync::{Mutex, OnceLock};

pub fn universe_codes(dimension: usize) -> Result<Vec<u128>> {
    static CACHE: OnceLock<Mutex<BTreeMap<usize, Vec<u128>>>> = OnceLock::new();
    let cache = CACHE.get_or_init(|| Mutex::new(BTreeMap::new()));
    let mut guard = match cache.lock() {
        Ok(guard) => guard,
        Err(poisoned) => poisoned.into_inner(),
    };
    if let Some(codes) = guard.get(&dimension) {
        return Ok(codes.clone());
    }
    let codes: Vec<u128> = bang(dimension)?
        .canonical()
        .into_iter()
        .map(|design| design.i.get())
        .collect();
    guard.insert(dimension, codes.clone());
    Ok(codes)
}

pub fn sources(catalog: &Catalog, dimension: usize) -> Result<Vec<Source>> {
    Ok(match catalog {
        Catalog::Classics => classics(dimension)
            .into_iter()
            .map(Source::Classic)
            .collect(),
        Catalog::Universe => universe_codes(dimension)?
            .iter()
            .map(|&code| Source::Code(code))
            .collect(),
        Catalog::Codes(list) => list.iter().map(|&code| Source::Code(code)).collect(),
        Catalog::Designs(list) => list.iter().map(|&design| Source::Classic(design)).collect(),
    })
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Catalog {
    Classics,
    Universe,
    Codes(Vec<u128>),
    Designs(Vec<Design>),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Source {
    #[serde(rename = "design")]
    Classic(Design),
    #[serde(rename = "code")]
    Code(#[serde(with = "decimal")] u128),
}

mod decimal {
    use serde::{Deserialize, Deserializer, Serializer};

    pub(super) fn serialize<S: Serializer>(code: &u128, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.collect_str(code)
    }

    pub(super) fn deserialize<'de, D: Deserializer<'de>>(
        deserializer: D,
    ) -> Result<u128, D::Error> {
        #[derive(Deserialize)]
        #[serde(untagged)]
        enum Code {
            Text(String),
            Number(u64),
        }
        match Code::deserialize(deserializer)? {
            Code::Text(text) => text.parse().map_err(serde::de::Error::custom),
            Code::Number(code) => Ok(code.into()),
        }
    }
}
