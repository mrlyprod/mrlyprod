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
use crate::{json, Json};
use std::collections::HashMap;

/// The seven ways a paint distributes its colors over a cell.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Edition {
    /// One color per cell type.
    Simple,
    /// Color by cell index.
    Index,
    /// Color by concentric layer.
    Layers,
    /// Color by neighbor count.
    Neighbors,
    /// Color by row.
    Rows,
    /// Color by column.
    Columns,
    /// A random color per cell.
    Random,
}

/// The fifteen named inks a paint draws from.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Ink {
    /// Black (0, 0, 0).
    Black,
    /// White (255, 255, 255).
    White,
    /// Red (255, 61, 64).
    Red,
    /// Orange (255, 143, 44).
    Orange,
    /// Yellow (255, 209, 0).
    Yellow,
    /// Green (50, 204, 88).
    Green,
    /// Mint (0, 209, 187).
    Mint,
    /// Teal (0, 202, 216).
    Teal,
    /// Cyan (30, 201, 243).
    Cyan,
    /// Blue (0, 140, 255).
    Blue,
    /// Indigo (103, 104, 250).
    Indigo,
    /// Purple (211, 50, 233).
    Purple,
    /// Pink (255, 50, 90).
    Pink,
    /// Brown (177, 132, 98).
    Brown,
    /// Gray (142, 142, 147).
    Gray,
}

/// The two ways secondary colors are drawn.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Scheme {
    /// Distinct secondary inks.
    Multicolor,
    /// One secondary ink stepped through shades.
    Multitone,
}

/// The side of the figure the primary ink lands on.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Target {
    /// The primary on the filled cells, secondaries on the empty.
    Fill,
    /// The primary on the empty cells, secondaries on the filled.
    Void,
}

impl Edition {
    /// Returns every edition in canonical order.
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
    /// Returns the cell-painting mode this edition renders with.
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
    /// Returns the edition's display name.
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
    /// Parses a display name back into its edition, or an error for an unknown name.
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
    /// Returns the ink's color.
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
    /// Returns every ink in canonical order.
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
    /// Returns the ink's display name.
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
    /// Parses a display name back into its ink, or an error for an unknown name.
    pub fn parse(name: &str) -> Result<Ink> {
        Ink::all()
            .into_iter()
            .find(|ink| ink.name() == name)
            .ok_or_else(|| MrlyError::Value(format!("unknown ink {name:?}.")))
    }
}

impl Scheme {
    /// Returns both schemes.
    pub fn all() -> [Scheme; 2] {
        [Scheme::Multicolor, Scheme::Multitone]
    }
    /// Returns the scheme's display name.
    pub fn name(self) -> &'static str {
        match self {
            Scheme::Multicolor => "Multicolor",
            Scheme::Multitone => "Multitone",
        }
    }
    /// Parses a display name back into its scheme, or an error for an unknown name.
    pub fn parse(name: &str) -> Result<Scheme> {
        match name {
            "Multicolor" => Ok(Scheme::Multicolor),
            "Multitone" => Ok(Scheme::Multitone),
            other => value_error(format!("unknown scheme {other:?}.")),
        }
    }
}

impl Target {
    /// Returns both targets.
    pub fn all() -> [Target; 2] {
        [Target::Fill, Target::Void]
    }
    /// Returns the target's display name.
    pub fn name(self) -> &'static str {
        match self {
            Target::Fill => "Fill",
            Target::Void => "Void",
        }
    }
    /// Parses a display name back into its target, or an error for an unknown name.
    pub fn parse(name: &str) -> Result<Target> {
        match name {
            "Fill" => Ok(Target::Fill),
            "Void" => Ok(Target::Void),
            other => value_error(format!("unknown target {other:?}.")),
        }
    }
}

const LEVELS: [u8; 2] = [33, 66];

/// The constraints a caller may put on a random paint.
#[derive(Clone, Debug, Default)]
pub struct Config {
    /// The editions allowed, or None for all seven.
    pub editions: Option<Vec<Edition>>,
    /// The primary inks allowed, or None for black and white.
    pub primaries: Option<Vec<Ink>>,
    /// The forced target, or None for a coin flip.
    pub target: Option<Target>,
}

/// A complete coloring recipe for one cell.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Paint {
    /// The coloring edition.
    pub edition: Edition,
    /// The secondary color scheme.
    pub scheme: Scheme,
    /// The side the primary ink lands on.
    pub target: Target,
    /// The primary ink.
    pub primary: Ink,
    /// The secondary inks.
    pub secondary: Vec<Ink>,
    /// The shade indices of a multitone ramp.
    pub shades: Vec<usize>,
}

impl Paint {
    /// Builds a black-primary, fill-target, multicolor paint for an edition.
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
    /// Returns true for the Simple edition.
    pub fn is_simple(&self) -> bool {
        self.edition == Edition::Simple
    }
    fn wipe(&mut self) {
        self.secondary.clear();
        self.shades.clear();
    }
    /// Encodes the paint as a versioned JSON object.
    pub fn to_json(&self) -> Json {
        json!({
            "v": 1,
            "edition": self.edition.name(),
            "scheme": self.scheme.name(),
            "target": self.target.name(),
            "primary": self.primary.name(),
            "secondary": self.secondary.iter().map(|ink| ink.name()).collect::<Vec<_>>(),
            "shades": self.shades.clone(),
        })
    }
    /// Decodes a paint from its JSON object, or an error naming the broken field.
    pub fn from_json(value: &Json) -> Result<Paint> {
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

fn string_list(value: &Json, key: &str) -> Result<Vec<String>> {
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

/// Draws a random edition from the allowed list, or from all seven.
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

/// Redraws the paint's secondary inks and shades under its scheme.
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

/// Draws the paint's scheme, target and primary under the config, then rerolls the rest.
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

/// Tags the cell for the Layers and Neighbors editions and returns the distinct tag count on the secondary side.
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

/// Tags the cell for Layers and Neighbors paints and sizes the palette to the tag count.
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

/// Colors the cell from the paint's inks under its edition mode.
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

/// Replays a stored paint onto a cell, tagging first and rendering deterministically.
pub fn coat(cell: &mut Cell, paint: &Paint, mask: Option<&Tensor>) -> Result<()> {
    tag(cell, paint.edition, paint.target, mask)?;
    if paint.edition.mode() == Mode::Random {
        scatter(paint, cell)
    } else {
        apply(paint, cell)
    }
}

/// Draws a random paint under the config, applies it to the cell, and returns the recipe.
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
    use crate::atoms;
    use crate::state::{guard, seed};
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
