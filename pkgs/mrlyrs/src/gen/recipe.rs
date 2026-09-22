use crate::core::error::{value_error, Result};
use crate::core::named_enum;
use serde::{Deserialize, Serialize};

pub use crate::math::bang::catalog::{
    antis, classics, Catalog, Design, Source, ANTIS_2D, ANTIS_3D, CLASSICS_2D, CLASSICS_3D,
};

/// The smallest side, number or factor a tile may take.
pub const MIN_SIDE: usize = 2;

/// The largest side, number or factor a tile may take.
pub const MAX_SIDE: usize = 64;

/// The deepest fractal level a tile may take.
pub const MAX_LEVEL: usize = 6;

/// The most slots a magic tile may take.
pub const MAX_SLOTS: usize = 6;

named_enum! {
    /// The five construction families a tile can belong to.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub enum Group {
        /// One source at one flat size.
        General => "General",
        /// One source raised to a power.
        Fractal => "Fractal",
        /// A magic-recipe construction.
        Magic => "Magic",
        /// A one-off special construction.
        Special => "Special",
        /// Sources nested as a product of factors.
        Mosaic => "Mosaic",
    }
}

named_enum! {
    /// The parity filter over candidate sizes.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub enum Parity {
        /// Even sizes only.
        Evens => "Evens",
        /// Odd sizes only.
        Odds => "Odds",
        /// Every size.
        Both => "Both",
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
}

/// A complete recipe for one tile.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
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
    /// The tile's width in cells.
    pub width: usize,
    /// The tile's height in cells.
    pub height: usize,
}

impl Tile {
    /// Builds an empty tile in a group.
    ///
    /// ```
    /// use mrlyrs::gen::recipe::{Group, Tile};
    /// assert_eq!(Tile::new(Group::General).max_size(), 0);
    /// ```
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
            width: 0,
            height: 0,
        }
    }
    /// Sets the tile's width and height.
    ///
    /// ```
    /// use mrlyrs::gen::recipe::{Group, Tile};
    /// assert_eq!(Tile::new(Group::General).size(3, 5).max_size(), 5);
    /// ```
    pub fn size(mut self, width: usize, height: usize) -> Tile {
        self.width = width;
        self.height = height;
        self
    }
    /// Returns the larger of width and height.
    pub fn max_size(&self) -> usize {
        self.width.max(self.height)
    }
    /// Returns whether the recipe is a magic tile of one repeated source at one repeated number,
    /// the shape a fractal tile of the same factor and level already draws.
    pub fn degenerate(&self) -> bool {
        self.group == Group::Magic
            && self.sources.len() > 1
            && uniform(&self.sources)
            && uniform(&self.numbers)
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
    /// Checks that the slots, numbers and sizes agree.
    ///
    /// ```
    /// use mrlyrs::gen::recipe::{Group, Tile};
    /// assert!(Tile::new(Group::General).check().is_err());
    /// ```
    ///
    /// # Errors
    ///
    /// Errs with a terse note for the first broken law: the slot count, a ragged slot list, a
    /// number, rotation, level, flip or factor out of range, or sizes the group does not imply.
    pub fn check(&self) -> Result<()> {
        let slots = self.sources.len();
        let wanted = match self.group {
            Group::Mosaic => slots == 3,
            Group::Magic => (2..=MAX_SLOTS).contains(&slots),
            _ => slots == 1,
        };
        if !wanted {
            return value_error("wrong slot count");
        }
        if self.numbers.len() != slots
            || self.levels.len() != slots
            || self.rotations.len() != slots
            || self.anti.len() != slots
        {
            return value_error("ragged slots");
        }
        if self
            .numbers
            .iter()
            .any(|&n| !(MIN_SIDE..=MAX_SIDE).contains(&n))
        {
            return value_error("numbers are 2 to 64");
        }
        if self.rotations.iter().any(|&r| r > 3) {
            return value_error("rotation is 0 to 3");
        }
        if self.flip && self.group != Group::Special {
            return value_error("flip is special only");
        }
        if self.group == Group::Fractal {
            if !(1..=MAX_LEVEL).contains(&self.levels[0]) {
                return value_error("level is 1 to 6");
            }
        } else if self.levels.iter().any(|&l| l != 1) {
            return value_error("level is fractal only");
        }
        if matches!(self.group, Group::Special | Group::Mosaic)
            && !(MIN_SIDE..=MAX_SIDE).contains(&self.factor)
        {
            return value_error("factor is 2 to 64");
        }
        if self.group == Group::Mosaic && self.numbers.iter().any(|&n| n != self.numbers[0]) {
            return value_error("mosaic shares one number");
        }
        let mut probe = self.clone();
        probe.resize();
        if probe.width != self.width || probe.height != self.height || probe.factor != self.factor {
            return value_error("sizes disagree");
        }
        if !(MIN_SIDE..=MAX_SIDE).contains(&self.max_size()) {
            return value_error("size is 2 to 64");
        }
        Ok(())
    }
}

const MIN_FACTOR: usize = 2;

/// Returns whether every item equals the first, vacuously true for an empty or single list.
///
/// ```
/// assert!(mrlyrs::gen::recipe::uniform(&[3, 3, 3]));
/// assert!(!mrlyrs::gen::recipe::uniform(&[3, 5, 3]));
/// ```
pub fn uniform<T: PartialEq>(items: &[T]) -> bool {
    items.windows(2).all(|pair| pair[0] == pair[1])
}

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
/// assert_eq!(mrlyrs::gen::recipe::size(3, 3), Some(27));
/// assert_eq!(mrlyrs::gen::recipe::size(3, 4294967298), None);
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::json;
    #[test]
    fn names_parse_back() {
        for group in Group::all() {
            assert_eq!(group, group.name().parse().unwrap());
        }
        for parity in Parity::all() {
            assert_eq!(parity, parity.name().parse().unwrap());
        }
    }
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
        let json = serde_json::to_value(&tile).unwrap();
        assert_eq!(json["group"], "Magic");
        assert_eq!(json["sources"][1], json!({ "code": "14" }));
        let back: Tile = serde_json::from_value(json).unwrap();
        assert_eq!(tile, back);
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
    fn refuses_a_recipe_that_breaks_a_law() {
        let note = |tile: &Tile| tile.check().unwrap_err().to_string();
        let mut tile = Tile::new(Group::General);
        assert_eq!(note(&tile), "wrong slot count");
        tile.sources = vec![Source::Code(7)];
        assert_eq!(note(&tile), "ragged slots");
        tile.numbers = vec![3];
        tile.levels = vec![1];
        tile.rotations = vec![0];
        tile.anti = vec![false];
        tile.resize();
        assert!(tile.check().is_ok());
        let mut zero = tile.clone();
        zero.numbers = vec![0];
        zero.resize();
        assert_eq!(note(&zero), "numbers are 2 to 64");
        let mut wide = tile.clone();
        wide.numbers = vec![99];
        wide.resize();
        assert_eq!(note(&wide), "numbers are 2 to 64");
        let mut turned = tile.clone();
        turned.rotations = vec![4];
        assert_eq!(note(&turned), "rotation is 0 to 3");
        let mut flipped = tile.clone();
        flipped.flip = true;
        assert_eq!(note(&flipped), "flip is special only");
        let mut levelled = tile.clone();
        levelled.levels = vec![2];
        assert_eq!(note(&levelled), "level is fractal only");
        let mut deep = tile.clone();
        deep.group = Group::Fractal;
        deep.levels = vec![7];
        deep.resize();
        assert_eq!(note(&deep), "level is 1 to 6");
        assert_eq!(note(&tile.clone().size(5, 5)), "sizes disagree");
        let mut special = Tile::new(Group::Special);
        special.sources = vec![Source::Code(7)];
        special.numbers = vec![3];
        special.levels = vec![1];
        special.rotations = vec![0];
        special.anti = vec![false];
        special.factor = 1;
        special.resize();
        assert_eq!(note(&special), "factor is 2 to 64");
        let mut mosaic = Tile::new(Group::Mosaic);
        mosaic.sources = vec![Source::Code(7); 3];
        mosaic.numbers = vec![3, 3, 5];
        mosaic.levels = vec![1; 3];
        mosaic.rotations = vec![0; 3];
        mosaic.anti = vec![false; 3];
        mosaic.factor = 3;
        mosaic.resize();
        assert_eq!(note(&mosaic), "mosaic shares one number");
        let mut huge = tile.clone();
        huge.group = Group::Fractal;
        huge.numbers = vec![9];
        huge.levels = vec![3];
        huge.resize();
        assert_eq!(note(&huge), "size is 2 to 64");
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
    fn degenerate_marks_the_magic_tiles_a_fractal_already_draws() {
        let mut tile = Tile::new(Group::Magic);
        tile.sources = vec![Source::Classic(Design::Carpet); 2];
        tile.numbers = vec![3, 3];
        tile.levels = vec![1, 1];
        tile.rotations = vec![0, 0];
        tile.anti = vec![false, false];
        tile.resize();
        assert!(tile.degenerate());
        tile.numbers = vec![3, 5];
        tile.resize();
        assert!(!tile.degenerate());
        tile.numbers = vec![3, 3];
        tile.sources = vec![Source::Classic(Design::Carpet), Source::Code(7)];
        tile.resize();
        assert!(!tile.degenerate());
    }
    #[test]
    fn degenerate_is_a_magic_law_only() {
        let mut tile = Tile::new(Group::Mosaic);
        tile.sources = vec![Source::Classic(Design::Carpet); 3];
        tile.numbers = vec![3, 3, 3];
        assert!(!tile.degenerate());
        tile.group = Group::General;
        tile.sources = vec![Source::Classic(Design::Carpet)];
        tile.numbers = vec![3];
        assert!(!tile.degenerate());
    }
    #[test]
    fn uniform_holds_for_short_lists() {
        assert!(uniform::<usize>(&[]));
        assert!(uniform(&[3]));
        assert!(uniform(&[3, 3, 3]));
        assert!(!uniform(&[3, 3, 5]));
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
