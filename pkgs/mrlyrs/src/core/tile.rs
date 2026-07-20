use super::errors::{value_error, MrlyError, Result};
use serde_json::{json, Value};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Group {
    General,
    Fractal,
    Magic,
    Special,
    Mosaic,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Parity {
    Evens,
    Odds,
    Both,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Base {
    Two,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Catalog {
    Classics,
    Universe,
    Codes(Vec<u128>),
}

impl Group {
    pub fn name(self) -> &'static str {
        match self {
            Group::General => "General",
            Group::Fractal => "Fractal",
            Group::Magic => "Magic",
            Group::Special => "Special",
            Group::Mosaic => "Mosaic",
        }
    }
    pub fn parse(name: &str) -> Result<Group> {
        match name {
            "General" => Ok(Group::General),
            "Fractal" => Ok(Group::Fractal),
            "Magic" => Ok(Group::Magic),
            "Special" => Ok(Group::Special),
            "Mosaic" => Ok(Group::Mosaic),
            other => value_error(format!("unknown group {other:?}.")),
        }
    }
    pub fn all() -> [Group; 5] {
        [
            Group::General,
            Group::Fractal,
            Group::Magic,
            Group::Special,
            Group::Mosaic,
        ]
    }
}

impl Parity {
    pub fn keep(self, n: usize) -> bool {
        match self {
            Parity::Evens => n.is_multiple_of(2),
            Parity::Odds => !n.is_multiple_of(2),
            Parity::Both => true,
        }
    }
    pub fn name(self) -> &'static str {
        match self {
            Parity::Evens => "Evens",
            Parity::Odds => "Odds",
            Parity::Both => "Both",
        }
    }
    pub fn parse(name: &str) -> Result<Parity> {
        match name {
            "Evens" => Ok(Parity::Evens),
            "Odds" => Ok(Parity::Odds),
            "Both" => Ok(Parity::Both),
            other => value_error(format!("unknown parity {other:?}.")),
        }
    }
}

impl Base {
    pub fn value(self) -> usize {
        match self {
            Base::Two => 2,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Design {
    Carpet,
    Net,
    Htree,
    Vtree,
    Void,
    Xtree,
    Ytree,
    Ztree,
}

impl Design {
    pub fn name(self) -> &'static str {
        match self {
            Design::Carpet => "Carpet",
            Design::Net => "Net",
            Design::Htree => "Htree",
            Design::Vtree => "Vtree",
            Design::Void => "Void",
            Design::Xtree => "Xtree",
            Design::Ytree => "Ytree",
            Design::Ztree => "Ztree",
        }
    }
    pub fn parse(name: &str) -> Result<Design> {
        match name {
            "Carpet" => Ok(Design::Carpet),
            "Net" => Ok(Design::Net),
            "Htree" => Ok(Design::Htree),
            "Vtree" => Ok(Design::Vtree),
            "Void" => Ok(Design::Void),
            "Xtree" => Ok(Design::Xtree),
            "Ytree" => Ok(Design::Ytree),
            "Ztree" => Ok(Design::Ztree),
            other => value_error(format!("unknown design {other:?}.")),
        }
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

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Source {
    Classic(Design),
    Code(u128),
}

impl Source {
    pub fn to_json(self) -> Value {
        match self {
            Source::Classic(design) => json!({ "design": design.name() }),
            Source::Code(code) => json!({ "code": code }),
        }
    }
    pub fn from_json(value: &Value) -> Result<Source> {
        if let Some(name) = value.get("design").and_then(|v| v.as_str()) {
            return Ok(Source::Classic(Design::parse(name)?));
        }
        if let Some(code) = value.get("code").and_then(|v| v.as_u64()) {
            return Ok(Source::Code(code as u128));
        }
        value_error("source must hold a \"design\" name or a \"code\".")
    }
}

pub fn classics(dimension: usize) -> Vec<Design> {
    match dimension {
        3 => CLASSICS_3D.to_vec(),
        _ => CLASSICS_2D.to_vec(),
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Tile {
    pub group: Group,
    pub factor: usize,
    pub sources: Vec<Source>,
    pub numbers: Vec<usize>,
    pub levels: Vec<usize>,
    pub rotations: Vec<usize>,
    pub anti: Vec<bool>,
    pub invert: bool,
    pub flip: bool,
    pub base: Base,
    pub width: usize,
    pub height: usize,
}

impl Tile {
    pub fn new(group: Group) -> Tile {
        Tile {
            group,
            factor: 0,
            sources: Vec::new(),
            numbers: Vec::new(),
            levels: Vec::new(),
            rotations: Vec::new(),
            anti: Vec::new(),
            invert: false,
            flip: false,
            base: Base::Two,
            width: 0,
            height: 0,
        }
    }
    pub fn size(mut self, width: usize, height: usize) -> Tile {
        self.width = width;
        self.height = height;
        self
    }
    pub fn max_size(&self) -> usize {
        self.width.max(self.height)
    }
    pub fn to_json(&self) -> Value {
        let sources: Vec<Value> = self.sources.iter().map(|s| s.to_json()).collect();
        json!({
            "v": 1,
            "group": self.group.name(),
            "factor": self.factor,
            "sources": sources,
            "numbers": self.numbers,
            "levels": self.levels,
            "rotations": self.rotations,
            "anti": self.anti,
            "invert": self.invert,
            "flip": self.flip,
            "base": self.base.value(),
            "width": self.width,
            "height": self.height,
        })
    }
    pub fn from_json(value: &Value) -> Result<Tile> {
        let group = Group::parse(string(value, "group")?.as_str())?;
        let mut tile = Tile::new(group);
        tile.factor = usize_at(value, "factor")?;
        tile.sources = source_list(value, "sources")?;
        tile.numbers = usize_list(value, "numbers")?;
        tile.levels = usize_list(value, "levels")?;
        tile.rotations = usize_list(value, "rotations")?;
        tile.anti = bool_list(value, "anti")?;
        tile.invert = bool_at(value, "invert")?;
        tile.flip = bool_at(value, "flip")?;
        tile.base = Base::Two;
        tile.width = usize_at(value, "width")?;
        tile.height = usize_at(value, "height")?;
        Ok(tile)
    }
}

const MIN_FACTOR: usize = 2;

fn factors(min_factor: usize, max_factor: usize, parity: Parity) -> Vec<usize> {
    (min_factor.max(MIN_FACTOR)..=max_factor)
        .filter(|&n| parity.keep(n))
        .collect()
}

pub fn generals(min_size: usize, max_size: usize, parity: Parity) -> Vec<usize> {
    factors(min_size, max_size, parity)
}

pub fn powers(min_size: usize, max_size: usize, parity: Parity) -> Vec<(usize, usize)> {
    let mut out = Vec::new();
    for n in factors(MIN_FACTOR, max_size, parity) {
        let mut level = 2;
        loop {
            match n.checked_pow(level as u32) {
                Some(size) if size <= max_size => {
                    if size >= min_size {
                        out.push((n, level));
                    }
                    level += 1;
                }
                _ => break,
            }
        }
    }
    out
}

pub fn products(min_size: usize, max_size: usize, count: usize, parity: Parity) -> Vec<Vec<usize>> {
    if count < 1 {
        return Vec::new();
    }
    fn walk(
        min_size: usize,
        max_size: usize,
        remaining: usize,
        parity: Parity,
        out: &mut Vec<Vec<usize>>,
    ) {
        if remaining == 1 {
            for n in factors(min_size, max_size, parity) {
                out.push(vec![n]);
            }
            return;
        }
        for n in factors(MIN_FACTOR, max_size, parity) {
            let next_min = min_size.div_ceil(n);
            let next_max = max_size / n;
            if next_max < MIN_FACTOR {
                continue;
            }
            let mut tails = Vec::new();
            walk(next_min, next_max, remaining - 1, parity, &mut tails);
            for tail in tails {
                let mut item = vec![n];
                item.extend(tail);
                out.push(item);
            }
        }
    }
    let mut out = Vec::new();
    walk(min_size, max_size, count, parity, &mut out);
    out
}

pub fn nestings(min_size: usize, max_size: usize, parity: Parity) -> Vec<Vec<usize>> {
    let mut out = Vec::new();
    let mut depth = 2;
    loop {
        let found = products(min_size, max_size, depth, parity);
        if found.is_empty() {
            if depth > 2 {
                break;
            }
            depth += 1;
            if depth > max_size {
                break;
            }
            continue;
        }
        out.extend(found);
        depth += 1;
    }
    out
}

fn field<'a>(value: &'a Value, key: &str) -> Result<&'a Value> {
    value
        .get(key)
        .ok_or_else(|| MrlyError::Value(format!("missing field {key:?}.")))
}

fn string(value: &Value, key: &str) -> Result<String> {
    field(value, key)?
        .as_str()
        .map(|s| s.to_string())
        .ok_or_else(|| MrlyError::Value(format!("field {key:?} must be a string.")))
}

fn usize_at(value: &Value, key: &str) -> Result<usize> {
    field(value, key)?
        .as_u64()
        .map(|n| n as usize)
        .ok_or_else(|| MrlyError::Value(format!("field {key:?} must be an integer.")))
}

fn bool_at(value: &Value, key: &str) -> Result<bool> {
    field(value, key)?
        .as_bool()
        .ok_or_else(|| MrlyError::Value(format!("field {key:?} must be a boolean.")))
}

fn usize_list(value: &Value, key: &str) -> Result<Vec<usize>> {
    let array = field(value, key)?
        .as_array()
        .ok_or_else(|| MrlyError::Value(format!("field {key:?} must be a list.")))?;
    array
        .iter()
        .map(|v| {
            v.as_u64()
                .map(|n| n as usize)
                .ok_or_else(|| MrlyError::Value(format!("field {key:?} must hold integers.")))
        })
        .collect()
}

fn source_list(value: &Value, key: &str) -> Result<Vec<Source>> {
    let array = field(value, key)?
        .as_array()
        .ok_or_else(|| MrlyError::Value(format!("field {key:?} must be a list.")))?;
    array.iter().map(Source::from_json).collect()
}

fn bool_list(value: &Value, key: &str) -> Result<Vec<bool>> {
    let array = field(value, key)?
        .as_array()
        .ok_or_else(|| MrlyError::Value(format!("field {key:?} must be a list.")))?;
    array
        .iter()
        .map(|v| {
            v.as_bool()
                .ok_or_else(|| MrlyError::Value(format!("field {key:?} must hold booleans.")))
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn parity_filters() {
        assert!(Parity::Odds.keep(3));
        assert!(!Parity::Odds.keep(4));
        assert!(Parity::Evens.keep(4));
        assert!(!Parity::Evens.keep(3));
        assert!(Parity::Both.keep(3));
        assert!(Parity::Both.keep(4));
    }
    #[test]
    fn generals_respects_parity_and_range() {
        assert_eq!(generals(3, 9, Parity::Odds), vec![3, 5, 7, 9]);
        assert_eq!(generals(3, 9, Parity::Evens), vec![4, 6, 8]);
        assert_eq!(generals(3, 9, Parity::Both), vec![3, 4, 5, 6, 7, 8, 9]);
    }
    #[test]
    fn powers_are_in_range() {
        for (n, level) in powers(3, 100, Parity::Odds) {
            let size = n.pow(level as u32);
            assert!((3..=100).contains(&size));
            assert!(level >= 2);
        }
        assert!(powers(3, 100, Parity::Odds).contains(&(3, 2)));
        assert!(powers(3, 100, Parity::Odds).contains(&(3, 4)));
    }
    #[test]
    fn products_multiply_into_range() {
        for option in products(3, 64, 2, Parity::Odds) {
            let size: usize = option.iter().product();
            assert!((3..=64).contains(&size));
            assert_eq!(option.len(), 2);
        }
    }
    #[test]
    fn nestings_go_deeper_than_two() {
        let deep = nestings(3, 300, Parity::Odds);
        assert!(deep.iter().any(|opt| opt.len() >= 3));
        for option in &deep {
            let size: usize = option.iter().product();
            assert!(size <= 300);
        }
    }
    #[test]
    fn tile_json_round_trips() {
        let mut tile = Tile::new(Group::Magic).size(45, 45);
        tile.sources = vec![Source::Classic(Design::Carpet), Source::Code(14)];
        tile.numbers = vec![5, 9];
        tile.levels = vec![1, 1];
        tile.rotations = vec![0, 0];
        tile.anti = vec![false, true];
        tile.factor = 5;
        let json = tile.to_json();
        let back = Tile::from_json(&json).unwrap();
        assert_eq!(tile, back);
    }
    #[test]
    fn source_json_round_trips() {
        for source in [Source::Classic(Design::Vtree), Source::Code(232)] {
            let back = Source::from_json(&source.to_json()).unwrap();
            assert_eq!(source, back);
        }
    }
    #[test]
    fn powers_generalize_beyond_classic_bases() {
        let options = powers(3, 1000, Parity::Odds);
        assert!(options.contains(&(3, 2)));
        assert!(options.contains(&(5, 2)));
        assert!(options.contains(&(7, 2)));
        assert!(options.contains(&(9, 2)));
        assert!(options.contains(&(13, 2)));
    }
    #[test]
    fn evens_factors_work() {
        assert!(powers(4, 1000, Parity::Evens)
            .iter()
            .all(|(n, _)| n % 2 == 0));
        assert!(powers(4, 1000, Parity::Evens).contains(&(4, 2)));
        assert!(powers(4, 1000, Parity::Evens).contains(&(6, 2)));
    }
}
