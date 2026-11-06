use crate::recipe::{Design, Group, Source, Tile as Recipe};
use mrly_error::{Error, Result, value_error};
use mrly_name::Named;
use mrly_name::kind;
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

const CODES_3D: [(Design, u128); 12] = [
    (Design::Carpet, 23),
    (Design::Net, 232),
    (Design::Xtree, 17),
    (Design::Ytree, 5),
    (Design::Ztree, 3),
    (Design::Void, 129),
    (Design::Point, 128),
    (Design::Dust, 1),
    (Design::Xline, 136),
    (Design::Yline, 160),
    (Design::Zline, 192),
    (Design::Star, 22),
];

const TOTAL_2D: u128 = 16;

pub fn classic_code_nd(design: Design, dimension: usize) -> Option<u128> {
    let table: &[(Design, u128)] = match dimension {
        2 => &CODES_2D,
        3 => &CODES_3D,
        _ => return None,
    };
    table
        .iter()
        .find(|&&(d, _)| d == design)
        .map(|&(_, code)| code)
}

pub fn classic_code(design: Design) -> Option<u128> {
    classic_code_nd(design, 2)
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

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Slots {
    One(usize),
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

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Tile {
    pub kind: Kind,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<u128>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub special: Option<u128>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub magic: Vec<u128>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub mosaic: Vec<u128>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub factor: Option<usize>,
    pub side: Slots,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub level: Option<usize>,
    #[serde(default, skip_serializing_if = "Slots::is_still")]
    pub turn: Slots,
    #[serde(default, skip_serializing_if = "is_false")]
    pub flip: bool,
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
            }
            Group::Magic => {
                name.magic = codes;
                name.side = Slots::Each(recipe.numbers.clone());
                name.turn = turns(recipe);
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
            }
        }
        Ok(name.fold())
    }
    fn fold(mut self) -> Tile {
        if self.turn.is_still() {
            self.turn = Slots::One(0);
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
    fn lone(&self) -> Result<()> {
        if self.factor.is_some() {
            return value_error("factor is special or mosaic only.");
        }
        Ok(())
    }
    pub fn recipe(&self) -> Result<Recipe> {
        let group = self.group()?;
        let mut recipe = Recipe::new(group);
        recipe.invert = self.invert;
        match group {
            Group::General | Group::Fractal => {
                self.lone()?;
                if self.flip {
                    return value_error("flip is special only.");
                }
                recipe.sources = vec![Source::Code(plane(self.code.expect("a code"))?)];
                recipe.numbers = vec![self.side.one("side")?];
                recipe.levels = vec![self.level.unwrap_or(1)];
                recipe.rotations = vec![self.turn.one("turn")?];
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
            }
            Group::Special => {
                let Some(factor) = self.factor else {
                    return value_error("a special tile wants its factor.");
                };
                recipe.sources = vec![Source::Code(plane(self.special.expect("a mask code"))?)];
                recipe.factor = factor;
                recipe.numbers = vec![self.side.one("side")?];
                recipe.levels = vec![1];
                recipe.rotations = vec![self.turn.one("turn")?];
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
    const LISTS: &'static [&'static str] = &["magic", "mosaic"];
    fn checked(self) -> Result<Tile> {
        Tile::of(&self.recipe()?)
    }
}
