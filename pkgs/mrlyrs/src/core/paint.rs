use super::cell::Cell;
use super::colors::{gradient, Color};
use super::colors::{
    BLACK, BLUE, BROWN, CYAN, GRAY, GREEN, INDIGO, MINT, ORANGE, PINK, PURPLE, RED, TEAL, WHITE,
    YELLOW,
};
use super::enums::Mode;
use super::errors::{value_error, MrlyError, Result};
use super::rng::Rng;
use super::state::{choice, randint, sample, shuffle};
use super::tensor::{Dtype, Tensor};
use serde_json::{json, Value};
use std::collections::HashMap;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Edition {
    Simple,
    Index,
    Layers,
    Neighbors,
    Rows,
    Columns,
    Random,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Ink {
    Black,
    White,
    Red,
    Orange,
    Yellow,
    Green,
    Mint,
    Teal,
    Cyan,
    Blue,
    Indigo,
    Purple,
    Pink,
    Brown,
    Gray,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Scheme {
    Multicolor,
    Multitone,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Target {
    Fill,
    Void,
}

impl Edition {
    pub fn all() -> [Edition; 7] {
        [
            Edition::Simple,
            Edition::Index,
            Edition::Layers,
            Edition::Neighbors,
            Edition::Rows,
            Edition::Columns,
            Edition::Random,
        ]
    }
    pub fn mode(self) -> Mode {
        match self {
            Edition::Simple => Mode::Type,
            Edition::Index => Mode::Index,
            Edition::Layers => Mode::Tag,
            Edition::Neighbors => Mode::Tag,
            Edition::Rows => Mode::Row,
            Edition::Columns => Mode::Column,
            Edition::Random => Mode::Random,
        }
    }
    pub fn name(self) -> &'static str {
        match self {
            Edition::Simple => "Simple",
            Edition::Index => "Index",
            Edition::Layers => "Layers",
            Edition::Neighbors => "Neighbors",
            Edition::Rows => "Rows",
            Edition::Columns => "Columns",
            Edition::Random => "Random",
        }
    }
    pub fn parse(name: &str) -> Result<Edition> {
        match name {
            "Simple" => Ok(Edition::Simple),
            "Index" => Ok(Edition::Index),
            "Layers" => Ok(Edition::Layers),
            "Neighbors" => Ok(Edition::Neighbors),
            "Rows" => Ok(Edition::Rows),
            "Columns" => Ok(Edition::Columns),
            "Random" => Ok(Edition::Random),
            other => value_error(format!("unknown edition {other:?}.")),
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
    pub fn all() -> [Ink; 15] {
        [
            Ink::Black,
            Ink::White,
            Ink::Red,
            Ink::Orange,
            Ink::Yellow,
            Ink::Green,
            Ink::Mint,
            Ink::Teal,
            Ink::Cyan,
            Ink::Blue,
            Ink::Indigo,
            Ink::Purple,
            Ink::Pink,
            Ink::Brown,
            Ink::Gray,
        ]
    }
    pub fn name(self) -> &'static str {
        match self {
            Ink::Black => "Black",
            Ink::White => "White",
            Ink::Red => "Red",
            Ink::Orange => "Orange",
            Ink::Yellow => "Yellow",
            Ink::Green => "Green",
            Ink::Mint => "Mint",
            Ink::Teal => "Teal",
            Ink::Cyan => "Cyan",
            Ink::Blue => "Blue",
            Ink::Indigo => "Indigo",
            Ink::Purple => "Purple",
            Ink::Pink => "Pink",
            Ink::Brown => "Brown",
            Ink::Gray => "Gray",
        }
    }
    pub fn parse(name: &str) -> Result<Ink> {
        Ink::all()
            .into_iter()
            .find(|ink| ink.name() == name)
            .ok_or_else(|| MrlyError::Value(format!("unknown ink {name:?}.")))
    }
}

impl Scheme {
    pub fn all() -> [Scheme; 2] {
        [Scheme::Multicolor, Scheme::Multitone]
    }
    pub fn name(self) -> &'static str {
        match self {
            Scheme::Multicolor => "Multicolor",
            Scheme::Multitone => "Multitone",
        }
    }
    pub fn parse(name: &str) -> Result<Scheme> {
        match name {
            "Multicolor" => Ok(Scheme::Multicolor),
            "Multitone" => Ok(Scheme::Multitone),
            other => value_error(format!("unknown scheme {other:?}.")),
        }
    }
}

impl Target {
    pub fn all() -> [Target; 2] {
        [Target::Fill, Target::Void]
    }
    pub fn name(self) -> &'static str {
        match self {
            Target::Fill => "Fill",
            Target::Void => "Void",
        }
    }
    pub fn parse(name: &str) -> Result<Target> {
        match name {
            "Fill" => Ok(Target::Fill),
            "Void" => Ok(Target::Void),
            other => value_error(format!("unknown target {other:?}.")),
        }
    }
}

const LEVELS: [u8; 2] = [33, 66];

#[derive(Clone, Debug, Default)]
pub struct Config {
    pub editions: Option<Vec<Edition>>,
    pub primaries: Option<Vec<Ink>>,
    pub target: Option<Target>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
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
    pub fn to_json(&self) -> Value {
        json!({
            "v": 1,
            "edition": self.edition.name(),
            "scheme": self.scheme.name(),
            "target": self.target.name(),
            "primary": self.primary.name(),
            "secondary": self.secondary.iter().map(|ink| ink.name()).collect::<Vec<_>>(),
            "shades": self.shades,
        })
    }
    pub fn from_json(value: &Value) -> Result<Paint> {
        let mut paint = Paint::new(Edition::parse(string(value, "edition")?.as_str())?);
        paint.scheme = Scheme::parse(string(value, "scheme")?.as_str())?;
        paint.target = Target::parse(string(value, "target")?.as_str())?;
        paint.primary = Ink::parse(string(value, "primary")?.as_str())?;
        paint.secondary = string_list(value, "secondary")?
            .iter()
            .map(|name| Ink::parse(name))
            .collect::<Result<Vec<Ink>>>()?;
        paint.shades = usize_list(value, "shades")?;
        Ok(paint)
    }
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

fn string_list(value: &Value, key: &str) -> Result<Vec<String>> {
    let array = field(value, key)?
        .as_array()
        .ok_or_else(|| MrlyError::Value(format!("field {key:?} must be a list.")))?;
    array
        .iter()
        .map(|v| {
            v.as_str()
                .map(|s| s.to_string())
                .ok_or_else(|| MrlyError::Value(format!("field {key:?} must hold strings.")))
        })
        .collect()
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

pub fn random_edition(editions: Option<&[Edition]>) -> Edition {
    match editions {
        Some(list) if !list.is_empty() => choice(list),
        _ => choice(&Edition::all()),
    }
}

fn random_primary(primaries: Option<&[Ink]>) -> Ink {
    if let Some(list) = primaries {
        if list.len() == 1 {
            return list[0];
        }
    }
    let mut choices = vec![Ink::Black, Ink::White];
    if let Some(list) = primaries {
        choices.retain(|ink| list.contains(ink));
    }
    if choices.is_empty() {
        choices = vec![Ink::Black, Ink::White];
    }
    choice(&choices)
}

fn random_secondary(count: Option<usize>, primary: Option<Ink>) -> Vec<Ink> {
    let mut inks = Ink::all().to_vec();
    match primary {
        Some(p) => inks.retain(|&ink| ink != p),
        None => inks.retain(|&ink| ink != Ink::Black && ink != Ink::White),
    }
    let count = count.unwrap_or_else(|| randint(2, 9) as usize);
    let count = count.min(inks.len());
    sample(&inks, count)
}

fn random_shades(count: Option<usize>, primary: Option<Ink>) -> Vec<usize> {
    if count == Some(1) {
        return if primary == Some(Ink::Black) {
            vec![0]
        } else {
            vec![1]
        };
    }
    let count = count.unwrap_or_else(|| randint(2, 9) as usize);
    let mut shades: Vec<usize> = (0..count).collect();
    shuffle(&mut shades);
    shades
}

pub fn reroll(mut paint: Paint) -> Paint {
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
            paint.secondary = random_secondary(colors, Some(paint.primary));
        }
        Scheme::Multitone => {
            paint.wipe();
            paint.secondary = random_secondary(Some(1), None);
            paint.shades = random_shades(shades, Some(paint.primary));
        }
    }
    paint
}

pub fn setup(mut paint: Paint, config: &Config) -> Paint {
    paint.scheme = choice(&[Scheme::Multicolor, Scheme::Multitone]);
    paint.target = config
        .target
        .unwrap_or_else(|| choice(&[Target::Fill, Target::Void]));
    paint.primary = random_primary(config.primaries.as_deref());
    reroll(paint)
}

fn remap_tags(target: Target, cell: &mut Cell) -> usize {
    let target_value = match target {
        Target::Fill => 0,
        Target::Void => 1,
    };
    let tags = match &cell.tags {
        Some(tags) => tags.clone(),
        None => return 0,
    };
    let relevant: Vec<u8> = cell
        .types
        .bytes()
        .iter()
        .zip(tags.bytes().iter())
        .filter(|(&t, _)| t == target_value)
        .map(|(_, &tag)| tag)
        .collect();
    if relevant.is_empty() {
        return 0;
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
        .bytes()
        .iter()
        .map(|tag| *lookup.get(tag).unwrap_or(&0))
        .collect();
    cell.tags = Some(Tensor::of(data, tags.shape.clone()));
    unique.len()
}

fn apply_colors(mut paint: Paint, max_val: usize) -> Paint {
    match paint.scheme {
        Scheme::Multicolor => {
            paint.wipe();
            paint.secondary = random_secondary(Some(max_val), Some(paint.primary));
        }
        Scheme::Multitone => {
            paint.wipe();
            paint.secondary = random_secondary(Some(1), None);
            paint.shades = random_shades(Some(max_val), Some(paint.primary));
        }
    }
    paint
}

fn von_neumann(dimension: usize) -> Tensor {
    let shape = vec![3usize; dimension];
    let mut mask = Tensor::new(shape);
    for flat in 0..mask.bytes().len() {
        let mut rem = flat;
        let mut distance = 0usize;
        for _ in 0..dimension {
            let coord = rem % 3;
            rem /= 3;
            distance += (coord as isize - 1).unsigned_abs();
        }
        mask.bytes_mut()[flat] = u8::from(distance == 1);
    }
    mask
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
            Ok(remap_tags(target, cell))
        }
        Edition::Neighbors => {
            let owned;
            let neighbor_mask = match mask {
                Some(m) => m,
                None => {
                    owned = von_neumann(cell.types.shape.len());
                    &owned
                }
            };
            *cell = cell.clone().neighbors(neighbor_mask, 1, false, Dtype::U8)?;
            Ok(remap_tags(target, cell))
        }
        _ => Ok(0),
    }
}

pub fn prime(mut paint: Paint, cell: &mut Cell, mask: Option<&Tensor>) -> Result<Paint> {
    if matches!(paint.edition, Edition::Layers | Edition::Neighbors) {
        let max_val = tag(cell, paint.edition, paint.target, mask)?;
        paint = apply_colors(paint, max_val.max(1));
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

pub fn apply(paint: &Paint, cell: &mut Cell) -> Result<()> {
    let primary = primary_colors(paint);
    let secondary = secondary_colors(paint)?;
    let mapping: HashMap<u8, Vec<Color>> = match paint.target {
        Target::Fill => HashMap::from([(0, secondary), (1, primary)]),
        Target::Void => HashMap::from([(0, primary), (1, secondary)]),
    };
    *cell = cell.clone().paint(&mapping, paint.edition.mode());
    Ok(())
}

fn scatter(paint: &Paint, cell: &mut Cell) -> Result<()> {
    let rgba = |colors: Vec<Color>| -> Vec<[u8; 4]> {
        colors.iter().map(|c| [c.r, c.g, c.b, c.a]).collect()
    };
    let primary = rgba(primary_colors(paint));
    let secondary = rgba(secondary_colors(paint)?);
    let (void_inks, fill_inks) = match paint.target {
        Target::Fill => (secondary, primary),
        Target::Void => (primary, secondary),
    };
    let mut rng = Rng::new(0);
    let size = cell.size();
    let mut colors = cell
        .colors
        .take()
        .unwrap_or_else(|| vec![[0, 0, 0, 0]; size]);
    for (flat, &t) in cell.types.bytes().iter().enumerate() {
        let palette = match t {
            0 => &void_inks,
            1 => &fill_inks,
            _ => continue,
        };
        if !palette.is_empty() {
            colors[flat] = palette[rng.below(palette.len())];
        }
    }
    cell.colors = Some(colors);
    Ok(())
}

pub fn coat(cell: &mut Cell, paint: &Paint, mask: Option<&Tensor>) -> Result<()> {
    tag(cell, paint.edition, paint.target, mask)?;
    if paint.edition.mode() == Mode::Random {
        scatter(paint, cell)
    } else {
        apply(paint, cell)
    }
}

pub fn paint(cell: &mut Cell, config: &Config, mask: Option<&Tensor>) -> Result<Paint> {
    let edition = random_edition(config.editions.as_deref());
    let mut p = Paint::new(edition);
    p = setup(p, config);
    p = prime(p, cell, mask)?;
    apply(&p, cell)?;
    Ok(p)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::atoms;
    use crate::core::state::{guard, seed};
    #[test]
    fn simple_paint_colors_every_cell() {
        let _g = guard();
        seed(1);
        let mut cell = Cell::new(atoms::carpet_2d(9));
        let config = Config::default();
        let _ = paint(&mut cell, &config, None).unwrap();
        assert!(cell.colors.is_some());
        let colors = cell.colors.as_ref().unwrap();
        assert_eq!(colors.len(), cell.size());
        assert!(colors.iter().all(|rgba| rgba[3] == 255));
        assert_eq!(cell.size(), 81);
    }
    #[test]
    fn every_edition_paints_2d() {
        let _g = guard();
        for (i, edition) in Edition::all().into_iter().enumerate() {
            seed(i as u64);
            let mut cell = Cell::new(atoms::carpet_2d(9));
            let mut p = Paint::new(edition);
            p = setup(p, &Config::default());
            p = prime(p, &mut cell, None).unwrap();
            apply(&p, &mut cell).unwrap();
            let colors = cell.colors.as_ref().unwrap();
            assert_eq!(colors.len(), cell.size(), "edition {:?}", edition);
        }
    }
    #[test]
    fn every_edition_paints_3d() {
        let _g = guard();
        for (i, edition) in Edition::all().into_iter().enumerate() {
            seed(100 + i as u64);
            let mut cell = Cell::new(atoms::carpet_3d(3));
            let mut p = Paint::new(edition);
            p = setup(p, &Config::default());
            p = prime(p, &mut cell, None).unwrap();
            apply(&p, &mut cell).unwrap();
            let colors = cell.colors.as_ref().unwrap();
            assert_eq!(colors.len(), cell.size(), "edition {:?} 3d", edition);
        }
    }
    #[test]
    fn paint_is_seeded() {
        let _g = guard();
        seed(7);
        let mut a = Cell::new(atoms::carpet_2d(5));
        let pa = paint(&mut a, &Config::default(), None).unwrap();
        seed(7);
        let mut b = Cell::new(atoms::carpet_2d(5));
        let pb = paint(&mut b, &Config::default(), None).unwrap();
        assert_eq!(pa, pb);
        assert_eq!(a, b);
    }
    #[test]
    fn multitone_builds_shade_ramp() {
        let _g = guard();
        seed(3);
        let mut p = Paint::new(Edition::Layers);
        p.scheme = Scheme::Multitone;
        p.primary = Ink::Black;
        p.secondary = vec![Ink::Blue];
        p.shades = vec![0, 1, 2, 1, 0];
        let colors = secondary_colors(&p).unwrap();
        assert_eq!(colors.len(), p.shades.len());
    }
    #[test]
    fn paint_json_round_trips() {
        let mut p = Paint::new(Edition::Layers);
        p.scheme = Scheme::Multitone;
        p.target = Target::Void;
        p.primary = Ink::White;
        p.secondary = vec![Ink::Blue];
        p.shades = vec![2, 0, 1];
        let back = Paint::from_json(&p.to_json()).unwrap();
        assert_eq!(p, back);
        let mut q = Paint::new(Edition::Simple);
        q.secondary = vec![Ink::Teal];
        assert_eq!(q, Paint::from_json(&q.to_json()).unwrap());
    }
    #[test]
    fn paint_json_rejects_garbage() {
        assert!(Paint::from_json(&json!({})).is_err());
        assert!(Paint::from_json(&json!({
            "edition": "Sparkle", "scheme": "Multicolor", "target": "Fill",
            "primary": "Black", "secondary": [], "shades": [],
        }))
        .is_err());
        assert!(Paint::from_json(&json!({
            "edition": "Simple", "scheme": "Multicolor", "target": "Fill",
            "primary": "Black", "secondary": ["Beige"], "shades": [],
        }))
        .is_err());
        assert!(Paint::from_json(&json!({
            "edition": "Simple", "scheme": "Multicolor", "target": "Fill",
            "primary": "Black", "secondary": [], "shades": ["soup"],
        }))
        .is_err());
    }
    #[test]
    fn names_parse_back() {
        for edition in Edition::all() {
            assert_eq!(edition, Edition::parse(edition.name()).unwrap());
        }
        for ink in Ink::all() {
            assert_eq!(ink, Ink::parse(ink.name()).unwrap());
        }
        for scheme in Scheme::all() {
            assert_eq!(scheme, Scheme::parse(scheme.name()).unwrap());
        }
        for target in Target::all() {
            assert_eq!(target, Target::parse(target.name()).unwrap());
        }
    }
    #[test]
    fn coat_renders_a_stored_paint_exactly() {
        let _g = guard();
        for edition in Edition::all() {
            seed(11);
            let mut primed = Cell::new(atoms::carpet_2d(9));
            let mut p = Paint::new(edition);
            p = setup(p, &Config::default());
            p = prime(p, &mut primed, None).unwrap();
            let stored = Paint::from_json(&p.to_json()).unwrap();
            seed(1);
            let mut a = Cell::new(atoms::carpet_2d(9));
            coat(&mut a, &stored, None).unwrap();
            seed(2);
            let mut b = Cell::new(atoms::carpet_2d(9));
            coat(&mut b, &stored, None).unwrap();
            assert_eq!(a, b, "edition {:?}", edition);
            assert_eq!(a.colors.as_ref().unwrap().len(), a.size());
        }
    }
    #[test]
    fn coat_matches_the_generative_render() {
        let _g = guard();
        for edition in [
            Edition::Simple,
            Edition::Index,
            Edition::Layers,
            Edition::Rows,
        ] {
            seed(21);
            let mut lived = Cell::new(atoms::carpet_2d(9));
            let p = paint(
                &mut lived,
                &Config {
                    editions: Some(vec![edition]),
                    ..Config::default()
                },
                None,
            )
            .unwrap();
            let mut coated = Cell::new(atoms::carpet_2d(9));
            coat(&mut coated, &p, None).unwrap();
            assert_eq!(lived.colors, coated.colors, "edition {:?}", edition);
        }
    }
    #[test]
    fn tag_is_deterministic() {
        let mut a = Cell::new(atoms::carpet_2d(9));
        let mut b = Cell::new(atoms::carpet_2d(9));
        let ka = tag(&mut a, Edition::Layers, Target::Fill, None).unwrap();
        let kb = tag(&mut b, Edition::Layers, Target::Fill, None).unwrap();
        assert_eq!(a, b);
        assert_eq!(ka, kb);
        assert!(ka >= 1);
        let mut c = Cell::new(atoms::carpet_2d(9));
        assert_eq!(tag(&mut c, Edition::Simple, Target::Fill, None).unwrap(), 0);
        assert!(c.tags.is_none());
    }
}
