use crate::core::error::{value_error, Error, Result};
use crate::gen::recipe::{Design, Group, Source, Tile as Recipe};
use crate::math::name::{kind, Named};
use serde::{Deserialize, Serialize};

kind!("tile");

const CODES_2D: [(Design, u128); 10] = [
    (Design::Carpet, 7),
    (Design::Net, 14),
    (Design::Htree, 3),
    (Design::Vtree, 5),
    (Design::Void, 9),
    (Design::Point, 8),
    (Design::Dust, 1),
    (Design::Hline, 12),
    (Design::Vline, 10),
    (Design::Star, 6),
];

const TOTAL_2D: u128 = 16;

/// Returns the plane's bang code of a classic design, or None for one outside the plane.
pub fn classic_code(design: Design) -> Option<u128> {
    CODES_2D
        .iter()
        .find(|&&(d, _)| d == design)
        .map(|&(_, code)| code)
}

fn code_of(source: Source) -> Result<u128> {
    match source {
        Source::Classic(design) => match classic_code(design) {
            Some(code) => Ok(code),
            None => value_error(format!(
                "design {} has no code in the plane.",
                design.name()
            )),
        },
        Source::Code(code) => Ok(code),
    }
}

fn slot<T: Copy>(values: &[T], what: &str) -> Result<T> {
    match values.first() {
        Some(&value) => Ok(value),
        None => value_error(format!("a recipe with no {what} has no name.")),
    }
}

fn is_false(flag: &bool) -> bool {
    !flag
}

/// One value for every slot of a tile, or one value per slot.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Slots {
    /// The one value the slots share.
    One(usize),
    /// One value per slot, in slot order.
    Each(Vec<usize>),
}

impl Slots {
    fn is_still(&self) -> bool {
        match self {
            Slots::One(turn) => *turn == 0,
            Slots::Each(turns) => turns.iter().all(|&turn| turn == 0),
        }
    }
    fn one(&self, what: &str) -> Result<usize> {
        match self {
            Slots::One(value) => Ok(*value),
            Slots::Each(_) => value_error(format!("a one-slot tile wants one {what}, not a list.")),
        }
    }
    fn each(&self, count: usize, what: &str) -> Result<Vec<usize>> {
        match self {
            Slots::One(0) if what == "turn" => Ok(vec![0; count]),
            Slots::One(_) => value_error(format!("a {count}-slot tile wants a {what} per slot.")),
            Slots::Each(values) if values.len() == count => Ok(values.clone()),
            Slots::Each(values) => value_error(format!(
                "a {count}-slot tile wants {count} {what}s, not {}.",
                values.len()
            )),
        }
    }
}

impl Default for Slots {
    fn default() -> Slots {
        Slots::One(0)
    }
}

/// A tile recipe folded to its one canonical object.
///
/// The key that carries the codes says the group: `code` is one design flat or, with `level`, raised
/// to a power; `magic` is a list of letters; `special` is one mask code over a factor; `mosaic` is
/// three codes behind a tree mask. Classics fold to their codes, a lone anti folds into `invert`,
/// and a level of one folds away, so aliases that draw one picture share one name.
///
/// ```
/// use mrlyrs::gen::name::Tile;
/// use mrlyrs::math::name::Named;
/// let carpet = Tile::from_json(r#"{"kind":"tile","code":7,"side":3,"level":2}"#).unwrap();
/// assert_eq!(carpet.recipe().unwrap().width, 9);
/// assert_eq!(Tile::of(&carpet.recipe().unwrap()).unwrap(), carpet);
/// ```
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Tile {
    /// The kind word.
    pub kind: Kind,
    /// The one design of a flat or fractal tile.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<u128>,
    /// The mask code of a special tile.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub special: Option<u128>,
    /// The letters of a magic tile, first letter outermost.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub magic: Vec<u128>,
    /// The three codes of a mosaic tile.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub mosaic: Vec<u128>,
    /// The side of the mask of a special or mosaic tile.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub factor: Option<usize>,
    /// The side each slot renders at, one per letter for a magic tile.
    pub side: Slots,
    /// The power a fractal tile is raised to, absent at one.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub level: Option<usize>,
    /// The quarter turns of each slot, absent when nothing turns.
    #[serde(default, skip_serializing_if = "Slots::is_still")]
    pub turn: Slots,
    /// Whether each slot swaps fill and void, absent when none does.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub anti: Vec<bool>,
    /// Whether a special tile flips its mask.
    #[serde(default, skip_serializing_if = "is_false")]
    pub flip: bool,
    /// Whether the finished tile inverts.
    #[serde(default, skip_serializing_if = "is_false")]
    pub invert: bool,
}

fn blank() -> Tile {
    Tile {
        kind: Kind,
        code: None,
        special: None,
        magic: Vec::new(),
        mosaic: Vec::new(),
        factor: None,
        side: Slots::One(0),
        level: None,
        turn: Slots::One(0),
        anti: Vec::new(),
        flip: false,
        invert: false,
    }
}

fn turns(recipe: &Recipe) -> Slots {
    Slots::Each(recipe.rotations.iter().map(|r| r % 4).collect())
}

fn plane(code: u128) -> Result<u128> {
    if code >= TOTAL_2D {
        return value_error(format!("code {code} is not in the plane (0..15)."));
    }
    Ok(code)
}

impl Tile {
    /// Folds a recipe to its name, or an error when the recipe has no name to fold to.
    pub fn of(recipe: &Recipe) -> Result<Tile> {
        let codes = recipe
            .sources
            .iter()
            .map(|&s| code_of(s))
            .collect::<Result<Vec<u128>>>()?;
        let mut name = blank();
        name.invert = recipe.invert;
        match recipe.group {
            Group::General | Group::Fractal => {
                name.code = Some(slot(&codes, "source")?);
                name.side = Slots::One(slot(&recipe.numbers, "number")?);
                if recipe.group == Group::Fractal {
                    let level = slot(&recipe.levels, "level")?;
                    name.level = (level != 1).then_some(level);
                }
                name.turn = Slots::One(slot(&recipe.rotations, "rotation")? % 4);
                name.invert = slot(&recipe.anti, "anti flag")? ^ recipe.invert;
            }
            Group::Magic => {
                name.magic = codes;
                name.side = Slots::Each(recipe.numbers.clone());
                name.turn = turns(recipe);
                name.anti = recipe.anti.clone();
            }
            Group::Special => {
                name.special = Some(slot(&codes, "source")?);
                name.factor = Some(recipe.factor);
                name.side = Slots::One(slot(&recipe.numbers, "number")?);
                name.turn = Slots::One(slot(&recipe.rotations, "rotation")? % 4);
                name.flip = recipe.flip;
            }
            Group::Mosaic => {
                name.mosaic = codes;
                name.factor = Some(recipe.factor);
                name.side = Slots::One(slot(&recipe.numbers, "number")?);
                name.turn = turns(recipe);
                name.anti = recipe.anti.clone();
            }
        }
        Ok(name.fold())
    }
    fn fold(mut self) -> Tile {
        if self.turn.is_still() {
            self.turn = Slots::One(0);
        }
        if self.anti.iter().all(|&a| !a) {
            self.anti.clear();
        }
        self
    }
    fn group(&self) -> Result<Group> {
        let carried = [
            (self.code.is_some(), Group::General),
            (self.special.is_some(), Group::Special),
            (!self.magic.is_empty(), Group::Magic),
            (!self.mosaic.is_empty(), Group::Mosaic),
        ];
        let mut groups = carried
            .iter()
            .filter(|(held, _)| *held)
            .map(|&(_, group)| group);
        let (Some(group), None) = (groups.next(), groups.next()) else {
            return value_error("a tile carries exactly one of code, special, magic or mosaic.");
        };
        if group == Group::General && self.level.is_some_and(|level| level != 1) {
            return Ok(Group::Fractal);
        }
        if group != Group::General && self.level.is_some() {
            return value_error("level is fractal only.");
        }
        Ok(group)
    }
    fn anti(&self, count: usize) -> Result<Vec<bool>> {
        match self.anti.len() {
            0 => Ok(vec![false; count]),
            n if n == count => Ok(self.anti.clone()),
            n => value_error(format!(
                "a {count}-slot tile wants {count} anti flags, not {n}."
            )),
        }
    }
    fn lone(&self, count: usize) -> Result<()> {
        if !self.anti.is_empty() {
            return value_error("anti folds into invert on a one-slot tile.");
        }
        if count == 1 && self.factor.is_some() {
            return value_error("factor is special or mosaic only.");
        }
        Ok(())
    }
    /// Builds the recipe the name folds, resized and checked, or an error naming what fails.
    pub fn recipe(&self) -> Result<Recipe> {
        let group = self.group()?;
        let mut recipe = Recipe::new(group);
        recipe.invert = self.invert;
        match group {
            Group::General | Group::Fractal => {
                self.lone(1)?;
                if self.flip {
                    return value_error("flip is special only.");
                }
                recipe.sources = vec![Source::Code(plane(self.code.expect("a code"))?)];
                recipe.numbers = vec![self.side.one("side")?];
                recipe.levels = vec![self.level.unwrap_or(1)];
                recipe.rotations = vec![self.turn.one("turn")?];
                recipe.anti = vec![false];
            }
            Group::Magic => {
                let count = self.magic.len();
                if self.flip || self.factor.is_some() {
                    return value_error("a magic tile carries no flip or factor.");
                }
                recipe.sources = self
                    .magic
                    .iter()
                    .map(|&code| Ok(Source::Code(plane(code)?)))
                    .collect::<Result<Vec<Source>>>()?;
                recipe.numbers = self.side.each(count, "side")?;
                recipe.levels = vec![1; count];
                recipe.rotations = self.turn.each(count, "turn")?;
                recipe.anti = self.anti(count)?;
            }
            Group::Special => {
                if !self.anti.is_empty() {
                    return value_error("anti is dead on a special tile.");
                }
                let Some(factor) = self.factor else {
                    return value_error("a special tile wants its factor.");
                };
                recipe.sources = vec![Source::Code(plane(self.special.expect("a mask code"))?)];
                recipe.factor = factor;
                recipe.numbers = vec![self.side.one("side")?];
                recipe.levels = vec![1];
                recipe.rotations = vec![self.turn.one("turn")?];
                recipe.anti = vec![false];
                recipe.flip = self.flip;
            }
            Group::Mosaic => {
                if self.flip {
                    return value_error("flip is special only.");
                }
                let Some(factor) = self.factor else {
                    return value_error("a mosaic tile wants its factor.");
                };
                recipe.sources = self
                    .mosaic
                    .iter()
                    .map(|&code| Ok(Source::Code(plane(code)?)))
                    .collect::<Result<Vec<Source>>>()?;
                recipe.factor = factor;
                recipe.numbers = vec![self.side.one("side")?; 3];
                recipe.levels = vec![1; 3];
                recipe.rotations = self.turn.each(3, "turn")?;
                recipe.anti = self.anti(3)?;
            }
        }
        recipe.resize();
        recipe
            .check()
            .map_err(|note| Error::Value(format!("tile fails its check: {note}.")))?;
        Ok(recipe)
    }
}

impl Named for Tile {
    const KIND: &'static str = "tile";
    const LISTS: &'static [&'static str] = &["magic", "mosaic", "anti"];
    fn checked(self) -> Result<Tile> {
        Tile::of(&self.recipe()?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::rng::Rng;
    use crate::gen::build::{build_2d, create_2d, Config2d};
    use crate::gen::recipe::{Catalog, Parity};
    use crate::math::bang::Code;
    use crate::math::two::designs;

    const CARPET: &str = r#"{"kind":"tile","code":7,"side":3,"level":2}"#;
    const GENERAL: &str = r#"{"kind":"tile","code":3,"side":5,"turn":1,"invert":true}"#;
    const MAGIC: &str = r#"{"kind":"tile","magic":[7,14],"side":[3,5],"turn":[0,2],"anti":[false,true],"invert":true}"#;
    const SPECIAL: &str = r#"{"kind":"tile","special":5,"factor":3,"side":5,"flip":true}"#;
    const MOSAIC: &str = r#"{"kind":"tile","mosaic":[7,14,5],"factor":3,"side":3,"turn":[0,1,0],"anti":[false,false,true],"invert":true}"#;

    fn built(recipe: &Recipe) -> crate::math::two::Cell2d {
        build_2d(recipe).unwrap()
    }
    fn fractal(design: Design) -> Recipe {
        let mut recipe = Recipe::new(Group::Fractal);
        recipe.sources = vec![Source::Classic(design)];
        recipe.numbers = vec![3];
        recipe.levels = vec![2];
        recipe.rotations = vec![0];
        recipe.anti = vec![false];
        recipe.resize();
        recipe
    }

    #[test]
    fn classic_codes_match_their_renders() {
        for (design, code) in CODES_2D {
            let by_name = designs::create(Code::from(code), 3, 1, 0, 2).unwrap();
            let by_classic = match design {
                Design::Carpet => designs::carpet(3, 1).unwrap(),
                Design::Net => designs::net(3, 1).unwrap(),
                Design::Htree => designs::htree(3, 1).unwrap(),
                Design::Vtree => designs::vtree(3, 1).unwrap(),
                Design::Void => designs::void(3, 1).unwrap(),
                Design::Point => designs::point(3, 1).unwrap(),
                Design::Dust => designs::dust(3, 1).unwrap(),
                Design::Hline => designs::hline(3, 1).unwrap(),
                Design::Vline => designs::vline(3, 1).unwrap(),
                Design::Star => designs::star(3, 1).unwrap(),
                _ => unreachable!(),
            };
            assert_eq!(by_name, by_classic, "{}", design.name());
        }
    }
    #[test]
    fn example_names_hold_verbatim() {
        assert_eq!(
            Tile::of(&fractal(Design::Carpet)).unwrap().to_json(),
            CARPET
        );
        let mut general = Recipe::new(Group::General);
        general.sources = vec![Source::Code(3)];
        general.numbers = vec![5];
        general.levels = vec![1];
        general.rotations = vec![1];
        general.anti = vec![false];
        general.invert = true;
        general.resize();
        assert_eq!(Tile::of(&general).unwrap().to_json(), GENERAL);
        let mut magic = Recipe::new(Group::Magic);
        magic.sources = vec![Source::Classic(Design::Carpet), Source::Code(14)];
        magic.numbers = vec![3, 5];
        magic.levels = vec![1, 1];
        magic.rotations = vec![0, 2];
        magic.anti = vec![false, true];
        magic.invert = true;
        magic.resize();
        assert_eq!(Tile::of(&magic).unwrap().to_json(), MAGIC);
        let mut special = Recipe::new(Group::Special);
        special.sources = vec![Source::Classic(Design::Vtree)];
        special.factor = 3;
        special.numbers = vec![5];
        special.levels = vec![1];
        special.rotations = vec![0];
        special.anti = vec![true];
        special.flip = true;
        special.resize();
        assert_eq!(Tile::of(&special).unwrap().to_json(), SPECIAL);
        let mut mosaic = Recipe::new(Group::Mosaic);
        mosaic.sources = vec![Source::Code(7), Source::Code(14), Source::Code(5)];
        mosaic.factor = 3;
        mosaic.numbers = vec![3, 3, 3];
        mosaic.levels = vec![1, 1, 1];
        mosaic.rotations = vec![0, 1, 0];
        mosaic.anti = vec![false, false, true];
        mosaic.invert = true;
        mosaic.resize();
        assert_eq!(Tile::of(&mosaic).unwrap().to_json(), MOSAIC);
    }
    #[test]
    fn the_views_hold_verbatim() {
        let magic = Tile::from_json(MAGIC).unwrap();
        assert_eq!(
            magic.to_url().unwrap(),
            "/tile?magic=7,14&side=3,5&turn=0,2&anti=false,true&invert=true"
        );
        assert_eq!(
            magic.to_file().unwrap(),
            "tile_magic=[7,14]_side=[3,5]_turn=[0,2]_anti=[false,true]_invert=true"
        );
        assert_eq!(
            magic.to_mrly().unwrap(),
            "tile magic [7 14], side [3 5], turn [0 2], anti [false true], invert"
        );
        let carpet = Tile::from_json(CARPET).unwrap();
        assert_eq!(carpet.to_url().unwrap(), "/tile?code=7&side=3&level=2");
        assert_eq!(carpet.to_file().unwrap(), "tile_code=7_side=3_level=2");
        assert_eq!(carpet.to_mrly().unwrap(), "tile code 7, side 3, level 2");
        assert_eq!(
            Tile::from_json(SPECIAL).unwrap().to_mrly().unwrap(),
            "tile special 5, factor 3, side 5, flip"
        );
    }
    #[test]
    fn parsed_tiles_pass_check_and_build() {
        for name in [CARPET, GENERAL, MAGIC, SPECIAL, MOSAIC] {
            let tile = Tile::from_json(name).unwrap();
            let recipe = tile.recipe().unwrap();
            assert!(recipe.check().is_ok());
            assert_eq!(tile.to_json(), name);
            assert_eq!(
                Tile::from_url(&tile.to_url().unwrap()).unwrap(),
                tile,
                "{name}"
            );
            assert_eq!(
                Tile::from_file(&tile.to_file().unwrap()).unwrap(),
                tile,
                "{name}"
            );
            let cell = built(&recipe);
            assert_eq!(cell.width(), recipe.width);
        }
    }
    #[test]
    fn a_level_of_one_folds_to_the_flat_tile() {
        let mut flat = fractal(Design::Carpet);
        flat.levels = vec![1];
        flat.resize();
        let name = Tile::of(&flat).unwrap();
        assert_eq!(name.to_json(), r#"{"kind":"tile","code":7,"side":3}"#);
        let spelt = Tile::from_json(r#"{"kind":"tile","code":7,"side":3,"level":1}"#).unwrap();
        assert_eq!(spelt, name);
        let recipe = spelt.recipe().unwrap();
        assert_eq!(recipe.group, Group::General);
        assert_eq!(built(&recipe), built(&flat));
    }
    #[test]
    fn anti_invert_pairs_share_one_name_and_one_picture() {
        let mut plain = fractal(Design::Carpet);
        let mut folded = plain.clone();
        folded.anti = vec![true];
        folded.invert = true;
        assert_eq!(Tile::of(&plain).unwrap(), Tile::of(&folded).unwrap());
        assert_eq!(built(&plain), built(&folded));
        plain.invert = true;
        let mut alias = plain.clone();
        alias.anti = vec![true];
        alias.invert = false;
        assert_eq!(Tile::of(&plain).unwrap(), Tile::of(&alias).unwrap());
        assert_eq!(built(&plain), built(&alias));
        assert_eq!(
            Tile::of(&plain).unwrap().to_json(),
            r#"{"kind":"tile","code":7,"side":3,"level":2,"invert":true}"#
        );
    }
    #[test]
    fn classics_and_codes_share_one_name() {
        let by_classic = fractal(Design::Net);
        let mut by_code = by_classic.clone();
        by_code.sources = vec![Source::Code(14)];
        assert_eq!(Tile::of(&by_classic).unwrap(), Tile::of(&by_code).unwrap());
        assert_eq!(built(&by_classic), built(&by_code));
    }
    #[test]
    fn dead_special_anti_folds_away() {
        let mut special = Recipe::new(Group::Special);
        special.sources = vec![Source::Code(5)];
        special.factor = 3;
        special.numbers = vec![3];
        special.levels = vec![1];
        special.rotations = vec![0];
        special.anti = vec![true];
        special.resize();
        let parsed = Tile::from_json(&Tile::of(&special).unwrap().to_json())
            .unwrap()
            .recipe()
            .unwrap();
        assert_eq!(parsed.anti, vec![false]);
        assert_eq!(built(&special), built(&parsed));
    }
    #[test]
    fn a_spelt_default_folds_to_the_canonical_string() {
        let spelt = r#"{"kind":"tile","side":[3,5],"magic":[7,14],"turn":[0,0],"anti":[false,false],"invert":false}"#;
        let tile = Tile::from_json(spelt).unwrap();
        assert_eq!(
            tile.to_json(),
            r#"{"kind":"tile","magic":[7,14],"side":[3,5]}"#
        );
        assert_eq!(tile.turn, Slots::One(0));
        assert!(tile.anti.is_empty());
    }
    #[test]
    fn refuses_a_name_or_a_recipe_that_cannot_draw() {
        for bad in [
            r#"{"kind":"tile","code":7,"side":3,"turn":4}"#,
            r#"{"kind":"tile","code":16,"side":3}"#,
            r#"{"kind":"tile","code":7,"side":99}"#,
            r#"{"kind":"tile","code":7,"side":3,"flip":true}"#,
            r#"{"kind":"tile","code":7,"side":3,"anti":[true]}"#,
            r#"{"kind":"tile","code":7,"side":3,"factor":3}"#,
            r#"{"kind":"tile","code":7,"side":[3]}"#,
            r#"{"kind":"tile","code":7,"side":3,"level":7}"#,
            r#"{"kind":"tile","code":7,"magic":[7,14],"side":3}"#,
            r#"{"kind":"tile","side":3}"#,
            r#"{"kind":"tile","sparkle":7,"side":3}"#,
            r#"{"kind":"tile","magic":[7],"side":[3]}"#,
            r#"{"kind":"tile","magic":[7,14],"side":3}"#,
            r#"{"kind":"tile","magic":[7,14],"side":[3,5],"anti":[true]}"#,
            r#"{"kind":"tile","magic":[7,14],"side":[3,5],"level":2}"#,
            r#"{"kind":"tile","mosaic":[7,14,5],"factor":3,"side":[3,3,3]}"#,
            r#"{"kind":"tile","mosaic":[7,14],"factor":3,"side":3}"#,
            r#"{"kind":"tile","mosaic":[7,14,5],"side":3}"#,
            r#"{"kind":"tile","special":5,"side":5}"#,
            r#"{"kind":"tile","special":5,"factor":3,"side":5,"anti":[true]}"#,
            r#"{"kind":"bang","dim":2,"code":7}"#,
            "tile code 7, side 3, level 2",
            "tile_code=7_side=3_level=2",
        ] {
            assert!(Tile::from_json(bad).is_err(), "{bad}");
        }
        assert!(Tile::of(&Recipe::new(Group::General)).is_err());
        let mut cubic = fractal(Design::Xtree);
        cubic.resize();
        assert!(classic_code(Design::Xtree).is_none());
        assert!(Tile::of(&cubic).is_err());
    }
    #[test]
    fn seeded_tiles_round_trip() {
        let config = Config2d {
            catalog: Catalog::Universe,
            min_size: 2,
            max_size: 64,
            parity: Parity::Both,
            ..Config2d::default()
        };
        for s in 0..300 {
            let mut rng = Rng::new(s);
            let recipe = create_2d(&config, &mut rng).unwrap();
            let name = Tile::of(&recipe).unwrap();
            let text = name.to_json();
            let parsed = Tile::from_json(&text).unwrap();
            assert_eq!(parsed.to_json(), text, "seed {s}");
            assert_eq!(
                Tile::from_url(&name.to_url().unwrap()).unwrap(),
                name,
                "seed {s}"
            );
            assert_eq!(
                Tile::from_file(&name.to_file().unwrap()).unwrap(),
                name,
                "seed {s}"
            );
            let back = parsed.recipe().unwrap();
            assert!(back.check().is_ok(), "seed {s}");
            assert_eq!(built(&back), built(&recipe), "seed {s}");
        }
    }
    #[test]
    fn seeded_classic_tiles_round_trip() {
        let config = Config2d {
            min_size: 2,
            max_size: 64,
            parity: Parity::Both,
            ..Config2d::default()
        };
        for s in 0..300 {
            let mut rng = Rng::new(s);
            let recipe = create_2d(&config, &mut rng).unwrap();
            let text = Tile::of(&recipe).unwrap().to_json();
            let parsed = Tile::from_json(&text).unwrap();
            assert_eq!(parsed.to_json(), text, "seed {s}");
            assert_eq!(built(&parsed.recipe().unwrap()), built(&recipe), "seed {s}");
        }
    }
}
