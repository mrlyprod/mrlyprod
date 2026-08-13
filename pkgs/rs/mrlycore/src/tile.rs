use super::errors::{value_error, MrlyError, Result};
use crate::{json, Json};

/// The smallest side, number or factor a tile may take.
pub const MIN_SIDE: usize = 2;

/// The largest side, number or factor a tile may take.
pub const MAX_SIDE: usize = 64;

/// The deepest fractal level a tile may take.
pub const MAX_LEVEL: usize = 6;

/// The most slots a magic tile may take.
pub const MAX_SLOTS: usize = 6;

/// The five construction families a tile can belong to.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Group {
    /// One source at one flat size.
    General,
    /// One source raised to a power.
    Fractal,
    /// A magic-recipe construction.
    Magic,
    /// A one-off special construction.
    Special,
    /// Sources nested as a product of factors.
    Mosaic,
}

/// The parity filter over candidate sizes.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Parity {
    /// Even sizes only.
    Evens,
    /// Odd sizes only.
    Odds,
    /// Every size.
    Both,
}

/// The numeral base tile codes are read in.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Base {
    /// Base two.
    Two,
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

impl Group {
    /// Returns the group's display name.
    pub fn name(self) -> &'static str {
        match self {
            Group::General => "General",
            Group::Fractal => "Fractal",
            Group::Magic => "Magic",
            Group::Special => "Special",
            Group::Mosaic => "Mosaic",
        }
    }
    /// Parses a display name back into its group, or an error for an unknown name.
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
    /// Returns every group in canonical order.
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
    /// Returns true when the number passes the filter.
    pub fn keep(self, n: usize) -> bool {
        match self {
            Parity::Evens => n.is_multiple_of(2),
            Parity::Odds => !n.is_multiple_of(2),
            Parity::Both => true,
        }
    }
    /// Returns the parity's display name.
    pub fn name(self) -> &'static str {
        match self {
            Parity::Evens => "Evens",
            Parity::Odds => "Odds",
            Parity::Both => "Both",
        }
    }
    /// Parses a display name back into its parity, or an error for an unknown name.
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
    /// Returns the base as a number.
    pub fn value(self) -> usize {
        match self {
            Base::Two => 2,
        }
    }
}

/// The classic named designs a source can point at.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Design {
    /// The carpet with a lattice of holes.
    Carpet,
    /// The net of crossing lines.
    Net,
    /// The stripes along the even rows.
    Htree,
    /// The stripes along the even columns.
    Vtree,
    /// The checkerboard lattice.
    Void,
    /// The beams along the x axis.
    Xtree,
    /// The beams along the y axis.
    Ytree,
    /// The beams along the z axis.
    Ztree,
}

impl Design {
    /// Returns the design's display name.
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
    /// Parses a display name back into its design, or an error for an unknown name.
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

/// The origin of one tile layer.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Source {
    /// A classic named design.
    Classic(Design),
    /// A numbered rule code.
    Code(u128),
}

impl Source {
    /// Encodes the source as a one-field JSON object, codes spelled as decimal strings.
    pub fn to_json(self) -> Json {
        match self {
            Source::Classic(design) => json!({ "design": design.name() }),
            Source::Code(code) => json!({ "code": code.to_string() }),
        }
    }
    /// Decodes a source from its JSON object, or an error when neither field is readable.
    pub fn from_json(value: &Json) -> Result<Source> {
        if let Some(name) = value.get("design").and_then(|v| v.as_str()) {
            return Ok(Source::Classic(Design::parse(name)?));
        }
        if let Some(code) = value.get("code") {
            if let Some(text) = code.as_str() {
                return match text.parse::<u128>() {
                    Ok(code) => Ok(Source::Code(code)),
                    Err(_) => value_error(format!("code {text:?} does not read as a number.")),
                };
            }
            if let Some(code) = code.as_u64() {
                return Ok(Source::Code(code as u128));
            }
        }
        value_error("source must hold a \"design\" name or a \"code\".")
    }
}

/// Returns the classic designs for a dimension.
pub fn classics(dimension: usize) -> Vec<Design> {
    match dimension {
        3 => CLASSICS_3D.to_vec(),
        _ => CLASSICS_2D.to_vec(),
    }
}

/// A complete recipe for one tile.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Tile {
    /// The construction family.
    pub group: Group,
    /// The base factor of the construction.
    pub factor: usize,
    /// The origin of each layer.
    pub sources: Vec<Source>,
    /// The grid size of each source.
    pub numbers: Vec<usize>,
    /// The fractal level of each source.
    pub levels: Vec<usize>,
    /// The quarter-turn rotation of each source.
    pub rotations: Vec<usize>,
    /// Whether each source swaps fill and void.
    pub anti: Vec<bool>,
    /// Whether the finished tile inverts.
    pub invert: bool,
    /// Whether the finished tile flips.
    pub flip: bool,
    /// The numeral base of the codes.
    pub base: Base,
    /// The tile's width in cells.
    pub width: usize,
    /// The tile's height in cells.
    pub height: usize,
}

impl Tile {
    /// Builds an empty tile in a group.
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
    /// Sets the tile's width and height.
    pub fn size(mut self, width: usize, height: usize) -> Tile {
        self.width = width;
        self.height = height;
        self
    }
    /// Returns the larger of width and height.
    pub fn max_size(&self) -> usize {
        self.width.max(self.height)
    }
    /// Recomputes the factor and side length the group and numbers imply, zero when they overflow.
    pub fn resize(&mut self) {
        let lead = self.numbers.first().copied().unwrap_or(0);
        if matches!(self.group, Group::General | Group::Fractal | Group::Magic) {
            self.factor = lead;
        }
        let size = match self.group {
            Group::General => lead,
            Group::Fractal => u32::try_from(self.levels.first().copied().unwrap_or(1))
                .ok()
                .and_then(|level| lead.checked_pow(level))
                .unwrap_or(0),
            Group::Magic => self
                .numbers
                .iter()
                .try_fold(1usize, |acc, &n| acc.checked_mul(n))
                .unwrap_or(0),
            Group::Special | Group::Mosaic => self.factor.checked_mul(lead).unwrap_or(0),
        };
        self.width = size;
        self.height = size;
    }
    /// Checks that the slots, numbers and sizes agree, or a terse note for the first broken law.
    pub fn check(&self) -> std::result::Result<(), &'static str> {
        let slots = self.sources.len();
        let wanted = match self.group {
            Group::Mosaic => slots == 3,
            Group::Magic => (2..=MAX_SLOTS).contains(&slots),
            _ => slots == 1,
        };
        if !wanted {
            return Err("wrong slot count");
        }
        if self.numbers.len() != slots
            || self.levels.len() != slots
            || self.rotations.len() != slots
            || self.anti.len() != slots
        {
            return Err("ragged slots");
        }
        if self
            .numbers
            .iter()
            .any(|&n| !(MIN_SIDE..=MAX_SIDE).contains(&n))
        {
            return Err("numbers are 2 to 64");
        }
        if self.rotations.iter().any(|&r| r > 3) {
            return Err("rotation is 0 to 3");
        }
        if self.flip && self.group != Group::Special {
            return Err("flip is special only");
        }
        if self.group == Group::Fractal {
            if !(1..=MAX_LEVEL).contains(&self.levels[0]) {
                return Err("level is 1 to 6");
            }
        } else if self.levels.iter().any(|&l| l != 1) {
            return Err("level is fractal only");
        }
        if matches!(self.group, Group::Special | Group::Mosaic)
            && !(MIN_SIDE..=MAX_SIDE).contains(&self.factor)
        {
            return Err("factor is 2 to 64");
        }
        if self.group == Group::Mosaic && self.numbers.iter().any(|&n| n != self.numbers[0]) {
            return Err("mosaic shares one number");
        }
        let mut probe = self.clone();
        probe.resize();
        if probe.width != self.width || probe.height != self.height || probe.factor != self.factor {
            return Err("sizes disagree");
        }
        if !(MIN_SIDE..=MAX_SIDE).contains(&self.max_size()) {
            return Err("size is 2 to 64");
        }
        Ok(())
    }
    /// Encodes the tile as a versioned JSON object.
    pub fn to_json(&self) -> Json {
        let sources: Vec<Json> = self.sources.iter().map(|s| s.to_json()).collect();
        json!({
            "v": 1,
            "group": self.group.name(),
            "factor": self.factor,
            "sources": sources,
            "numbers": self.numbers.clone(),
            "levels": self.levels.clone(),
            "rotations": self.rotations.clone(),
            "anti": self.anti.clone(),
            "invert": self.invert,
            "flip": self.flip,
            "base": self.base.value(),
            "width": self.width,
            "height": self.height,
        })
    }
    /// Decodes a tile from its JSON object, or an error naming the broken field.
    pub fn from_json(value: &Json) -> Result<Tile> {
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

/// Returns every flat size in the range that passes the parity filter.
pub fn generals(min_size: usize, max_size: usize, parity: Parity) -> Vec<usize> {
    factors(min_size, max_size, parity)
}

/// Returns every factor and level whose power lands in the size range.
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

/// Returns the side a factor raised to a level makes, or None when no usize holds it.
///
/// ```
/// assert_eq!(mrlycore::tile::size(3, 3), Some(27));
/// assert_eq!(mrlycore::tile::size(3, 4294967298), None);
/// ```
pub fn size(number: i64, level: i64) -> Option<usize> {
    let number = usize::try_from(number).ok()?;
    let level = u32::try_from(level).ok()?;
    number.checked_pow(level)
}

/// Returns every count-long factor list whose product lands in the size range.
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

/// Returns every factor list of depth two and beyond whose product lands in the size range.
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

fn field<'a>(value: &'a Json, key: &str) -> Result<&'a Json> {
    value
        .get(key)
        .ok_or_else(|| MrlyError::Value(format!("missing field {key:?}.")))
}

fn string(value: &Json, key: &str) -> Result<String> {
    field(value, key)?
        .as_str()
        .map(|s| s.to_string())
        .ok_or_else(|| MrlyError::Value(format!("field {key:?} must be a string.")))
}

fn usize_at(value: &Json, key: &str) -> Result<usize> {
    field(value, key)?
        .as_u64()
        .map(|n| n as usize)
        .ok_or_else(|| MrlyError::Value(format!("field {key:?} must be an integer.")))
}

fn bool_at(value: &Json, key: &str) -> Result<bool> {
    field(value, key)?
        .as_bool()
        .ok_or_else(|| MrlyError::Value(format!("field {key:?} must be a boolean.")))
}

fn usize_list(value: &Json, key: &str) -> Result<Vec<usize>> {
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

fn source_list(value: &Json, key: &str) -> Result<Vec<Source>> {
    let array = field(value, key)?
        .as_array()
        .ok_or_else(|| MrlyError::Value(format!("field {key:?} must be a list.")))?;
    array.iter().map(Source::from_json).collect()
}

fn bool_list(value: &Json, key: &str) -> Result<Vec<bool>> {
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
    fn source_json_spells_codes_as_strings() {
        let wide = u128::MAX - 1;
        assert_eq!(
            Source::Code(wide).to_json(),
            json!({ "code": wide.to_string() })
        );
        let back = Source::from_json(&Source::Code(wide).to_json()).unwrap();
        assert_eq!(back, Source::Code(wide));
    }
    #[test]
    fn source_json_reads_legacy_int_codes() {
        assert_eq!(
            Source::from_json(&json!({ "code": 7 })).unwrap(),
            Source::Code(7)
        );
        assert!(Source::from_json(&json!({ "code": "soup" })).is_err());
        assert!(Source::from_json(&json!({ "code": true })).is_err());
    }
    #[test]
    fn resize_follows_the_size_law() {
        let mut tile = Tile::new(Group::Fractal);
        tile.sources = vec![Source::Code(7)];
        tile.numbers = vec![3];
        tile.levels = vec![2];
        tile.rotations = vec![0];
        tile.anti = vec![false];
        tile.resize();
        assert_eq!((tile.factor, tile.width, tile.height), (3, 9, 9));
        tile.group = Group::Special;
        tile.factor = 5;
        tile.resize();
        assert_eq!((tile.width, tile.height), (15, 15));
        tile.group = Group::Magic;
        tile.numbers = vec![3, 5];
        tile.resize();
        assert_eq!((tile.factor, tile.width), (3, 15));
    }
    #[test]
    fn resize_survives_empty_and_huge_tiles() {
        let mut bare = Tile::new(Group::Magic);
        bare.resize();
        assert_eq!(bare.width, 1);
        let mut huge = Tile::new(Group::Fractal);
        huge.numbers = vec![3];
        huge.levels = vec![4_294_967_298];
        huge.resize();
        assert_eq!(huge.width, 0);
    }
    #[test]
    fn check_names_the_first_broken_law() {
        let mut tile = Tile::new(Group::General);
        assert_eq!(tile.check(), Err("wrong slot count"));
        tile.sources = vec![Source::Code(7)];
        assert_eq!(tile.check(), Err("ragged slots"));
        tile.numbers = vec![3];
        tile.levels = vec![1];
        tile.rotations = vec![0];
        tile.anti = vec![false];
        tile.resize();
        assert_eq!(tile.check(), Ok(()));
        tile.rotations = vec![4];
        assert_eq!(tile.check(), Err("rotation is 0 to 3"));
        tile.rotations = vec![0];
        tile.flip = true;
        assert_eq!(tile.check(), Err("flip is special only"));
        tile.flip = false;
        tile.width = 5;
        assert_eq!(tile.check(), Err("sizes disagree"));
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
    fn size_refuses_what_it_cannot_hold() {
        assert_eq!(size(3, 3), Some(27));
        assert_eq!(size(3, 0), Some(1));
        assert_eq!(size(-1, 2), None);
        assert_eq!(size(3, -1), None);
        assert_eq!(size(3, 64), None);
        assert_eq!(size(3, 4294967296), None);
        assert_eq!(size(3, 4294967298), None);
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
