use super::cell::{Cell, Mode, moore};
use super::colors::{
    BLACK, BLUE, BROWN, CYAN, GRAY, GREEN, INDIGO, MINT, ORANGE, PINK, PURPLE, RED, TEAL, WHITE,
    YELLOW,
};
use super::colors::{Color, gradient};
use super::error::{Result, value_error};
use super::rng::Rng;
use super::tensor::{Dtype, Tensor};
use crate::named_enum;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

named_enum! {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub enum Edition {
        Simple => "Simple",
        Index => "Index",
        Layers => "Layers",
        Neighbors => "Neighbors",
        Rows => "Rows",
        Columns => "Columns",
        Random => "Random",
    }
}

named_enum! {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub enum Ink {
        Black => "Black",
        White => "White",
        Red => "Red",
        Orange => "Orange",
        Yellow => "Yellow",
        Green => "Green",
        Mint => "Mint",
        Teal => "Teal",
        Cyan => "Cyan",
        Blue => "Blue",
        Indigo => "Indigo",
        Purple => "Purple",
        Pink => "Pink",
        Brown => "Brown",
        Gray => "Gray",
    }
}

named_enum! {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub enum Scheme {
        Multicolor => "Multicolor",
        Multitone => "Multitone",
    }
}

named_enum! {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub enum Target {
        Fill => "Fill",
        Void => "Void",
    }
}

impl Edition {
    pub fn mode(self) -> Option<Mode> {
        match self {
            Edition::Simple => Some(Mode::Type),
            Edition::Index => Some(Mode::Index),
            Edition::Layers => Some(Mode::Tag),
            Edition::Neighbors => Some(Mode::Tag),
            Edition::Rows => Some(Mode::Row),
            Edition::Columns => Some(Mode::Column),
            Edition::Random => None,
        }
    }
}

impl Ink {
    pub fn color(self) -> Color {
        match self {
            Ink::Black => BLACK,
            Ink::White => WHITE,
            Ink::Red => RED,
            Ink::Orange => ORANGE,
            Ink::Yellow => YELLOW,
            Ink::Green => GREEN,
            Ink::Mint => MINT,
            Ink::Teal => TEAL,
            Ink::Cyan => CYAN,
            Ink::Blue => BLUE,
            Ink::Indigo => INDIGO,
            Ink::Purple => PURPLE,
            Ink::Pink => PINK,
            Ink::Brown => BROWN,
            Ink::Gray => GRAY,
        }
    }
}

const LEVELS: [u8; 2] = [33, 66];

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Config {
    pub editions: Option<Vec<Edition>>,
    pub primaries: Option<Vec<Ink>>,
    pub target: Option<Target>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Paint {
    pub edition: Edition,
    pub scheme: Scheme,
    pub target: Target,
    pub primary: Ink,
    pub secondary: Vec<Ink>,
    pub shades: Vec<usize>,
}

impl Paint {
    pub fn new(edition: Edition) -> Paint {
        Paint {
            edition,
            scheme: Scheme::Multicolor,
            target: Target::Fill,
            primary: Ink::Black,
            secondary: Vec::new(),
            shades: Vec::new(),
        }
    }
    pub fn is_simple(&self) -> bool {
        self.edition == Edition::Simple
    }
    fn wipe(&mut self) {
        self.secondary.clear();
        self.shades.clear();
    }
}

pub fn random_edition(editions: Option<&[Edition]>, rng: &mut Rng) -> Edition {
    match editions {
        Some(list) if !list.is_empty() => *rng.choice(list).expect("a list with items"),
        _ => *rng.choice(&Edition::all()).expect("seven editions"),
    }
}

fn random_primary(primaries: Option<&[Ink]>, rng: &mut Rng) -> Ink {
    if let Some(list) = primaries
        && list.len() == 1
    {
        return list[0];
    }
    let mut choices = vec![Ink::Black, Ink::White];
    if let Some(list) = primaries {
        choices.retain(|ink| list.contains(ink));
    }
    if choices.is_empty() {
        choices = vec![Ink::Black, Ink::White];
    }
    *rng.choice(&choices).expect("black and white at least")
}

fn random_secondary(count: Option<usize>, primary: Option<Ink>, rng: &mut Rng) -> Vec<Ink> {
    let mut inks = Ink::all().to_vec();
    match primary {
        Some(p) => inks.retain(|&ink| ink != p),
        None => inks.retain(|&ink| ink != Ink::Black && ink != Ink::White),
    }
    let count = count.unwrap_or_else(|| rng.range(2, 9) as usize);
    rng.sample_indices(inks.len(), count)
        .into_iter()
        .map(|i| inks[i])
        .collect()
}

fn random_shades(count: Option<usize>, primary: Option<Ink>, rng: &mut Rng) -> Vec<usize> {
    if count == Some(1) {
        return if primary == Some(Ink::Black) {
            vec![0]
        } else {
            vec![1]
        };
    }
    let count = count.unwrap_or_else(|| rng.range(2, 9) as usize);
    let mut shades: Vec<usize> = (0..count).collect();
    rng.shuffle(&mut shades);
    shades
}

pub fn reroll(mut paint: Paint, rng: &mut Rng) -> Paint {
    let (colors, shades) = if paint.is_simple() {
        (Some(1), Some(1))
    } else if paint.edition == Edition::Index {
        (Some(2), Some(2))
    } else {
        (None, None)
    };
    match paint.scheme {
        Scheme::Multicolor => {
            paint.wipe();
            paint.secondary = random_secondary(colors, Some(paint.primary), rng);
        }
        Scheme::Multitone => {
            paint.wipe();
            paint.secondary = random_secondary(Some(1), None, rng);
            paint.shades = random_shades(shades, Some(paint.primary), rng);
        }
    }
    paint
}

pub fn setup(mut paint: Paint, config: &Config, rng: &mut Rng) -> Paint {
    paint.scheme = *rng
        .choice(&[Scheme::Multicolor, Scheme::Multitone])
        .expect("two schemes");
    paint.target = config.target.unwrap_or_else(|| {
        *rng.choice(&[Target::Fill, Target::Void])
            .expect("two targets")
    });
    paint.primary = random_primary(config.primaries.as_deref(), rng);
    reroll(paint, rng)
}

fn remap_tags(target: Target, cell: &mut Cell) -> Result<usize> {
    let target_value = match target {
        Target::Fill => 0,
        Target::Void => 1,
    };
    let tags = match &cell.tags {
        Some(tags) => tags.clone(),
        None => return Ok(0),
    };
    let relevant: Vec<u8> = cell
        .types
        .bytes()?
        .iter()
        .zip(tags.bytes()?.iter())
        .filter(|&(&t, _)| t == target_value)
        .map(|(_, &tag)| tag)
        .collect();
    if relevant.is_empty() {
        return Ok(0);
    }
    let mut unique: Vec<u8> = relevant.clone();
    unique.sort_unstable();
    unique.dedup();
    let lookup: HashMap<u8, u8> = unique
        .iter()
        .enumerate()
        .map(|(i, &tag)| (tag, i as u8))
        .collect();
    let data: Vec<u8> = tags
        .bytes()?
        .iter()
        .map(|tag| *lookup.get(tag).unwrap_or(&0))
        .collect();
    cell.tags = Some(Tensor::of(data, tags.shape.clone())?);
    Ok(unique.len())
}

fn apply_colors(mut paint: Paint, max_val: usize, rng: &mut Rng) -> Paint {
    match paint.scheme {
        Scheme::Multicolor => {
            paint.wipe();
            paint.secondary = random_secondary(Some(max_val), Some(paint.primary), rng);
        }
        Scheme::Multitone => {
            paint.wipe();
            paint.secondary = random_secondary(Some(1), None, rng);
            paint.shades = random_shades(Some(max_val), Some(paint.primary), rng);
        }
    }
    paint
}

pub fn tag(
    cell: &mut Cell,
    edition: Edition,
    target: Target,
    mask: Option<&Tensor>,
) -> Result<usize> {
    match edition {
        Edition::Layers => {
            *cell = cell.clone().layers(Dtype::U8);
            remap_tags(target, cell)
        }
        Edition::Neighbors => {
            let owned;
            let neighbor_mask = match mask {
                Some(m) => m,
                None => {
                    owned = moore(cell.types.shape.len());
                    &owned
                }
            };
            *cell = cell.clone().neighbors(neighbor_mask, 1, false, Dtype::U8)?;
            remap_tags(target, cell)
        }
        _ => Ok(0),
    }
}

pub fn prime(
    mut paint: Paint,
    cell: &mut Cell,
    mask: Option<&Tensor>,
    rng: &mut Rng,
) -> Result<Paint> {
    if matches!(paint.edition, Edition::Layers | Edition::Neighbors) {
        let max_val = tag(cell, paint.edition, paint.target, mask)?;
        paint = apply_colors(paint, max_val.max(1), rng);
    }
    Ok(paint)
}

fn primary_colors(paint: &Paint) -> Vec<Color> {
    vec![paint.primary.color()]
}

fn secondary_colors(paint: &Paint) -> Result<Vec<Color>> {
    let mut colors: Vec<Color> = paint.secondary.iter().map(|ink| ink.color()).collect();
    if paint.scheme == Scheme::Multitone {
        if colors.is_empty() {
            return value_error("multitone paint needs a base color.");
        }
        let c1 = colors[0].lightness(LEVELS[0])?;
        let c2 = colors[0].lightness(LEVELS[1])?;
        let mut ramp = vec![c1, c2];
        let steps = paint.shades.len();
        if steps > 2 {
            ramp = gradient(&ramp, steps)?;
        }
        colors = paint
            .shades
            .iter()
            .map(|&i| ramp[i.min(ramp.len() - 1)])
            .collect();
    }
    Ok(colors)
}

pub fn apply(paint: &Paint, cell: &mut Cell, rng: &mut Rng) -> Result<()> {
    let primary = primary_colors(paint);
    let secondary = secondary_colors(paint)?;
    let (void_inks, fill_inks) = match paint.target {
        Target::Fill => (secondary, primary),
        Target::Void => (primary, secondary),
    };
    match paint.edition.mode() {
        Some(mode) => {
            let mapping = HashMap::from([(0, void_inks), (1, fill_inks)]);
            *cell = cell.clone().paint(&mapping, mode, Some(rng))?;
        }
        None => scatter(cell, &void_inks, &fill_inks, rng),
    }
    Ok(())
}

fn scatter(cell: &mut Cell, void_inks: &[Color], fill_inks: &[Color], rng: &mut Rng) {
    let size = cell.size();
    let mut colors = cell
        .colors
        .take()
        .filter(|colors| colors.len() == size)
        .unwrap_or_else(|| vec![[0, 0, 0, 0]; size]);
    for (flat, slot) in colors.iter_mut().enumerate() {
        let palette = match cell.types.at(flat) {
            0 => void_inks,
            1 => fill_inks,
            _ => continue,
        };
        if let Ok(c) = rng.choice(palette) {
            *slot = [c.r, c.g, c.b, c.a];
        }
    }
    cell.colors = Some(colors);
}

pub fn coat(cell: &mut Cell, paint: &Paint, mask: Option<&Tensor>, rng: &mut Rng) -> Result<()> {
    tag(cell, paint.edition, paint.target, mask)?;
    apply(paint, cell, rng)
}

pub fn paint(
    cell: &mut Cell,
    config: &Config,
    mask: Option<&Tensor>,
    rng: &mut Rng,
) -> Result<Paint> {
    let edition = random_edition(config.editions.as_deref(), rng);
    let mut p = Paint::new(edition);
    p = setup(p, config, rng);
    p = prime(p, cell, mask, rng)?;
    apply(&p, cell, rng)?;
    Ok(p)
}
