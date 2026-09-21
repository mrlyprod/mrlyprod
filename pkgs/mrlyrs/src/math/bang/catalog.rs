use super::universe::bang;
use crate::core::named_enum;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::sync::{Mutex, OnceLock};

/// Returns the canonical design codes of a dimension, computed once and cached for the process.
pub fn universe_codes(dimension: usize) -> &'static [u128] {
    static CACHE: OnceLock<Mutex<BTreeMap<usize, &'static [u128]>>> = OnceLock::new();
    let cache = CACHE.get_or_init(|| Mutex::new(BTreeMap::new()));
    let mut guard = cache.lock().unwrap();
    if let Some(codes) = guard.get(&dimension) {
        return codes;
    }
    let codes: Vec<u128> = bang(dimension)
        .canonical()
        .into_iter()
        .map(|design| design.i)
        .collect();
    let leaked: &'static [u128] = Box::leak(codes.into_boxed_slice());
    guard.insert(dimension, leaked);
    leaked
}

/// Builds the tile sources a catalog names at a dimension.
pub fn sources(catalog: &Catalog, dimension: usize) -> Vec<Source> {
    match catalog {
        Catalog::Classics => classics(dimension)
            .into_iter()
            .map(Source::Classic)
            .collect(),
        Catalog::Universe => universe_codes(dimension)
            .iter()
            .map(|&code| Source::Code(code))
            .collect(),
        Catalog::Codes(list) => list.iter().map(|&code| Source::Code(code)).collect(),
    }
}

/// The pool of sources a tile may draw from.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Catalog {
    /// The classic designs only.
    Classics,
    /// The canonical codes, one per symmetry orbit.
    Universe,
    /// An explicit list of codes.
    Codes(Vec<u128>),
}

named_enum! {
    /// The named designs a source can point at: the four classics and their four antis.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub enum Design {
        /// The carpet with a lattice of holes.
        Carpet => "Carpet",
        /// The net of crossing lines.
        Net => "Net",
        /// The stripes along the even rows.
        Htree => "Htree",
        /// The stripes along the even columns.
        Vtree => "Vtree",
        /// The checkerboard lattice.
        Void => "Void",
        /// The beams along the x axis.
        Xtree => "Xtree",
        /// The beams along the y axis.
        Ytree => "Ytree",
        /// The beams along the z axis.
        Ztree => "Ztree",
        /// The points at the odd-odd sites.
        Point => "Point",
        /// The dust at the even-even sites.
        Dust => "Dust",
        /// The lines along the odd rows.
        Hline => "Hline",
        /// The lines along the odd columns.
        Vline => "Vline",
        /// The star of sites with exactly one odd coordinate.
        Star => "Star",
        /// The rods along the x axis.
        Xline => "Xline",
        /// The rods along the y axis.
        Yline => "Yline",
        /// The rods along the z axis.
        Zline => "Zline",
    }
}

/// The five classic designs of the plane.
pub const CLASSICS_2D: [Design; 5] = [
    Design::Carpet,
    Design::Net,
    Design::Htree,
    Design::Vtree,
    Design::Void,
];

/// The six classic designs of the cube.
pub const CLASSICS_3D: [Design; 6] = [
    Design::Carpet,
    Design::Net,
    Design::Xtree,
    Design::Ytree,
    Design::Ztree,
    Design::Void,
];

/// The five antis of the plane, the complements of the five classics in order.
pub const ANTIS_2D: [Design; 5] = [
    Design::Point,
    Design::Dust,
    Design::Hline,
    Design::Vline,
    Design::Star,
];

/// The six antis of the cube: point, dust, the three lines and the star.
pub const ANTIS_3D: [Design; 6] = [
    Design::Point,
    Design::Dust,
    Design::Xline,
    Design::Yline,
    Design::Zline,
    Design::Star,
];

/// The origin of one tile layer, a one-field json object.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Source {
    /// A classic named design.
    #[serde(rename = "design")]
    Classic(Design),
    /// A numbered rule code, spelled as a decimal string.
    #[serde(rename = "code")]
    Code(#[serde(with = "decimal")] u128),
}

mod decimal {
    use serde::{Deserialize, Deserializer, Serializer};

    pub fn serialize<S: Serializer>(code: &u128, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.collect_str(code)
    }

    pub fn deserialize<'de, D: Deserializer<'de>>(deserializer: D) -> Result<u128, D::Error> {
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

/// Returns the classic designs for a dimension.
pub fn classics(dimension: usize) -> Vec<Design> {
    match dimension {
        3 => CLASSICS_3D.to_vec(),
        _ => CLASSICS_2D.to_vec(),
    }
}

/// Returns the anti designs for a dimension.
pub fn antis(dimension: usize) -> Vec<Design> {
    match dimension {
        3 => ANTIS_3D.to_vec(),
        _ => ANTIS_2D.to_vec(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::json;
    #[test]
    fn names_parse_back() {
        for design in Design::all() {
            assert_eq!(design, design.name().parse().unwrap());
        }
    }
    #[test]
    fn catalog_classics_are_named_designs() {
        assert_eq!(
            sources(&Catalog::Classics, 2),
            CLASSICS_2D
                .into_iter()
                .map(Source::Classic)
                .collect::<Vec<_>>()
        );
        assert_eq!(
            sources(&Catalog::Classics, 3),
            CLASSICS_3D
                .into_iter()
                .map(Source::Classic)
                .collect::<Vec<_>>()
        );
    }
    #[test]
    fn catalog_universe_has_full_orbit_counts() {
        assert_eq!(sources(&Catalog::Universe, 2).len(), 6);
        assert_eq!(sources(&Catalog::Universe, 3).len(), 22);
    }
    #[test]
    fn source_json_round_trips() {
        for source in [Source::Classic(Design::Vtree), Source::Code(232)] {
            let json = serde_json::to_value(source).unwrap();
            let back: Source = serde_json::from_value(json).unwrap();
            assert_eq!(source, back);
        }
    }
    #[test]
    fn source_json_spells_codes_as_strings() {
        let wide = u128::MAX - 1;
        let json = serde_json::to_value(Source::Code(wide)).unwrap();
        assert_eq!(json, json!({ "code": wide.to_string() }));
        let back: Source = serde_json::from_value(json).unwrap();
        assert_eq!(back, Source::Code(wide));
    }
    #[test]
    fn source_json_reads_bare_int_codes() {
        let read = |value| serde_json::from_value::<Source>(value);
        assert_eq!(read(json!({ "code": 7 })).unwrap(), Source::Code(7));
        assert!(read(json!({ "code": "soup" })).is_err());
        assert!(read(json!({ "code": true })).is_err());
        assert!(read(json!({ "design": "Soup" })).is_err());
    }
}
