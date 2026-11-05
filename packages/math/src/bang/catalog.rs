use super::universe::bang;
use mrly_core::error::Result;
use mrly_core::named_enum;
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

named_enum! {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub enum Design {
        Carpet => "Carpet",
        Net => "Net",
        Htree => "Htree",
        Vtree => "Vtree",
        Void => "Void",
        Xtree => "Xtree",
        Ytree => "Ytree",
        Ztree => "Ztree",
        Point => "Point",
        Dust => "Dust",
        Hline => "Hline",
        Vline => "Vline",
        Star => "Star",
        Xline => "Xline",
        Yline => "Yline",
        Zline => "Zline",
    }
}

pub const CLASSICS_2D: [Design; 5] = [
    Design::Carpet,
    Design::Net,
    Design::Htree,
    Design::Vtree,
    Design::Void,
];

pub const CLASSICS_3D: [Design; 6] = [
    Design::Carpet,
    Design::Net,
    Design::Xtree,
    Design::Ytree,
    Design::Ztree,
    Design::Void,
];

pub const ANTIS_2D: [Design; 5] = [
    Design::Point,
    Design::Dust,
    Design::Hline,
    Design::Vline,
    Design::Star,
];

pub const ANTIS_3D: [Design; 6] = [
    Design::Point,
    Design::Dust,
    Design::Xline,
    Design::Yline,
    Design::Zline,
    Design::Star,
];

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

pub fn classics(dimension: usize) -> Vec<Design> {
    match dimension {
        3 => CLASSICS_3D.to_vec(),
        _ => CLASSICS_2D.to_vec(),
    }
}

pub fn antis(dimension: usize) -> Vec<Design> {
    match dimension {
        3 => ANTIS_3D.to_vec(),
        _ => ANTIS_2D.to_vec(),
    }
}
