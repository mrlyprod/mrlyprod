use crate::catalog::{LEVELS, MASK_LEVELS, MASK_SIDES, ROLLS, SIDES, TILINGS};
use crate::engine::{Grid, Mode, Reel, Run, Stop};
use crate::source::{Counts, Source, roll};
use mrly_error::{Result, value_error};
use mrly_math::catalog::Design;
use mrly_math::code::Code;
use mrly_two as two;
use serde::{Deserialize, Serialize};

const CODES: [u128; 5] = [1, 3, 6, 7, 15];

const NAMED: [(Design, &str); 10] = [
    (Design::Carpet, "carpet"),
    (Design::Net, "net"),
    (Design::Htree, "htree"),
    (Design::Vtree, "vtree"),
    (Design::Void, "void"),
    (Design::Point, "point"),
    (Design::Dust, "dust"),
    (Design::Hline, "hline"),
    (Design::Vline, "vline"),
    (Design::Star, "star"),
];

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Recipe {
    pub seed: String,
    pub side: u32,
    pub level: u32,
    pub tiling: u32,
    pub mode: Mode,
    pub mask: String,
    pub birth: String,
    pub survive: String,
    pub x: u32,
    pub cap: u32,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Grammar {
    pub shapes: Vec<String>,
    pub sides: Vec<u32>,
    pub levels: Vec<u32>,
    pub tilings: Vec<u32>,
    pub modes: Vec<String>,
    pub mask_sides: Vec<u32>,
    pub mask_levels: Vec<u32>,
    pub copies: Vec<String>,
    pub sources: Vec<String>,
    pub rolls: Vec<String>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Shape {
    Named(Design),
    Code(u128),
}

impl Shape {
    fn parse(token: &str) -> Result<Shape> {
        if let Some(rest) = token.strip_prefix("code") {
            return match rest.parse::<u128>() {
                Ok(code) => Ok(Shape::Code(code)),
                Err(_) => value_error(format!("design code {rest:?} is not a number.")),
            };
        }
        match NAMED.iter().find(|(_, name)| *name == token) {
            Some((design, _)) => Ok(Shape::Named(*design)),
            None => value_error(format!("unknown design {token:?}.")),
        }
    }

    pub(crate) fn token(self) -> String {
        match self {
            Shape::Named(design) => NAMED.iter().find(|(d, _)| *d == design).map_or_else(
                || design.name().to_lowercase(),
                |(_, name)| (*name).to_string(),
            ),
            Shape::Code(code) => format!("code{code}"),
        }
    }

    pub(crate) fn build(self, side: usize, level: usize) -> Result<Grid> {
        let cell = match self {
            Shape::Named(design) => two::named(design, side, level, 0)?,
            Shape::Code(code) => two::create(Code::from(code), side, level, 0, 2)?,
        };
        let types = cell.types();
        Ok(Grid::of(
            types.shape[0],
            types.shape[1],
            types.bytes()?.to_vec(),
        ))
    }
}

pub(crate) fn shapes() -> Vec<Shape> {
    let mut out: Vec<Shape> = CODES.iter().map(|&c| Shape::Code(c)).collect();
    out.extend(NAMED.iter().map(|(d, _)| Shape::Named(*d)));
    out
}

fn parse_design(token: &str) -> Result<(Shape, usize, usize)> {
    let parts: Vec<&str> = token.split('.').collect();
    if parts.len() != 3 {
        return value_error(format!("design token {token:?} wants shape.side.level."));
    }
    let shape = Shape::parse(parts[0])?;
    let (Ok(side), Ok(level)) = (parts[1].parse::<usize>(), parts[2].parse::<usize>()) else {
        return value_error(format!("design token {token:?} wants numbers."));
    };
    if side == 0 || level == 0 {
        return value_error(format!(
            "design token {token:?} wants a side and level of 1 or more."
        ));
    }
    Ok((shape, side, level))
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Mask {
    Design {
        shape: Shape,
        side: usize,
        level: usize,
    },
    Copy {
        inverted: bool,
    },
}

impl Mask {
    fn parse(token: &str) -> Result<Mask> {
        match token {
            "copy" => Ok(Mask::Copy { inverted: false }),
            "copyinv" => Ok(Mask::Copy { inverted: true }),
            _ => {
                let (shape, side, level) = parse_design(token)?;
                Ok(Mask::Design { shape, side, level })
            }
        }
    }

    pub(crate) fn token(self) -> String {
        match self {
            Mask::Design { shape, side, level } => format!("{}.{side}.{level}", shape.token()),
            Mask::Copy { inverted: false } => "copy".to_string(),
            Mask::Copy { inverted: true } => "copyinv".to_string(),
        }
    }
}

pub fn parse(name: &str) -> Result<Recipe> {
    let words: Vec<&str> = name.split_whitespace().collect();
    if words.len() < 7 {
        return value_error(format!("recipe {name:?} is too short."));
    }
    let (shape, side, level) = parse_design(words[0])?;
    let Some(tiling) = words[1]
        .strip_prefix('x')
        .and_then(|t| t.parse::<u32>().ok())
    else {
        return value_error(format!("tiling {:?} wants x<n>.", words[1]));
    };
    let mode: Mode = words[2].parse()?;
    let (mut mask, mut birth, mut survive, mut x, mut cap) = (None, None, None, None, None);
    for word in &words[3..] {
        let Some((key, value)) = word.split_once(':') else {
            return value_error(format!("word {word:?} wants key:value."));
        };
        match key {
            "mask" => mask = Some(Mask::parse(value)?.token()),
            "birth" => birth = Some(Counts::parse(value)?.token()),
            "survive" => survive = Some(Counts::parse(value)?.token()),
            "x" => x = value.parse::<u32>().ok(),
            "cap" => cap = value.parse::<u32>().ok(),
            other => return value_error(format!("unknown key {other:?}.")),
        }
    }
    let (Some(mask), Some(birth), Some(survive), Some(x)) = (mask, birth, survive, x) else {
        return value_error(format!("recipe {name:?} misses a part."));
    };
    let cap = match (mode, cap) {
        (Mode::Island | Mode::Board, Some(cap)) => cap,
        (Mode::Island | Mode::Board, None) => return value_error(format!("{mode} names its cap.")),
        (Mode::Wallpaper, None) => 0,
        (Mode::Wallpaper, Some(_)) => return value_error("a wallpaper has no cap."),
    };
    if mode == Mode::Wallpaper && tiling != 1 {
        return value_error("a wallpaper tiles once.");
    }
    Ok(Recipe {
        seed: shape.token(),
        side: side as u32,
        level: level as u32,
        tiling,
        mode,
        mask,
        birth,
        survive,
        x,
        cap,
    })
}

pub fn name(recipe: &Recipe) -> String {
    let mut out = format!(
        "{}.{}.{} x{} {} mask:{} birth:{} survive:{} x:{}",
        recipe.seed,
        recipe.side,
        recipe.level,
        recipe.tiling,
        recipe.mode,
        recipe.mask,
        recipe.birth,
        recipe.survive,
        recipe.x
    );
    if recipe.mode != Mode::Wallpaper {
        out.push_str(&format!(" cap:{}", recipe.cap));
    }
    out
}

pub fn grammar() -> Grammar {
    Grammar {
        shapes: shapes().iter().map(|s| s.token()).collect(),
        sides: SIDES.iter().map(|&n| n as u32).collect(),
        levels: LEVELS.iter().map(|&n| n as u32).collect(),
        tilings: TILINGS.iter().map(|&n| n as u32).collect(),
        modes: [Mode::Island, Mode::Wallpaper]
            .iter()
            .map(|m| m.name().to_string())
            .collect(),
        mask_sides: MASK_SIDES.iter().map(|&n| n as u32).collect(),
        mask_levels: MASK_LEVELS.iter().map(|&n| n as u32).collect(),
        copies: vec!["copy".to_string(), "copyinv".to_string()],
        sources: Source::all().iter().map(|s| s.name()).collect(),
        rolls: ROLLS
            .iter()
            .map(|&(zeros, ones)| roll(zeros, ones).to_string())
            .collect(),
    }
}

fn kronecker(shape: Shape, side: usize, level: usize) -> Result<Grid> {
    let base = shape.build(side, 1)?;
    Ok((1..level).fold(base.clone(), |grid, _| grid.kron(&base)))
}

pub(crate) fn design(token: &str) -> Result<Grid> {
    match Mask::parse(token)? {
        Mask::Design { shape, side, level } => Ok(kronecker(shape, side, level)?.popped()),
        Mask::Copy { .. } => value_error("a copy mask needs its seed; it runs only from a recipe."),
    }
}

pub(crate) fn parts(recipe: &Recipe) -> Result<(Grid, Grid, Counts, Counts)> {
    let recipe = parse(&name(recipe))?;
    if recipe.tiling == 0 {
        return value_error("a tiling is at least 1.");
    }
    let shape = Shape::parse(&recipe.seed)?;
    let (side, level, tiling) = (
        recipe.side as usize,
        recipe.level as usize,
        recipe.tiling as usize,
    );
    let board = shape.build(side, level)?.tile(tiling);
    let mask = match Mask::parse(&recipe.mask)? {
        Mask::Copy { inverted } => {
            let copy = kronecker(shape, side, level)?.tile(tiling);
            if inverted { copy.invert() } else { copy }.popped()
        }
        Mask::Design { .. } => design(&recipe.mask)?,
    };
    Ok((
        board,
        mask,
        Counts::parse(&recipe.birth)?,
        Counts::parse(&recipe.survive)?,
    ))
}

pub fn film(recipe: &Recipe) -> Result<(Reel, Stop)> {
    let mut run = Run::new(recipe)?;
    let mut reel = Reel::new();
    reel.shoot(&run);
    let stop = reel.roll(&mut run);
    Ok((reel, stop))
}
