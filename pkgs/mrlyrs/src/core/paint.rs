use super::cell::{moore, Cell, Mode};
use super::colors::{gradient, Color};
use super::colors::{
    BLACK, BLUE, BROWN, CYAN, GRAY, GREEN, INDIGO, MINT, ORANGE, PINK, PURPLE, RED, TEAL, WHITE,
    YELLOW,
};
use super::error::{value_error, Result};
use super::named_enum;
use super::rng::Rng;
use super::tensor::{Dtype, Tensor};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

named_enum! {
    /// The seven ways a paint distributes its colors over a cell.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub enum Edition {
        /// One color per cell type.
        Simple => "Simple",
        /// Color by cell index.
        Index => "Index",
        /// Color by concentric layer.
        Layers => "Layers",
        /// Color by neighbor count.
        Neighbors => "Neighbors",
        /// Color by row.
        Rows => "Rows",
        /// Color by column.
        Columns => "Columns",
        /// A random color per cell.
        Random => "Random",
    }
}

named_enum! {
    /// The fifteen named inks a paint draws from.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub enum Ink {
        /// Black (0, 0, 0).
        Black => "Black",
        /// White (255, 255, 255).
        White => "White",
        /// Red (255, 61, 64).
        Red => "Red",
        /// Orange (255, 143, 44).
        Orange => "Orange",
        /// Yellow (255, 209, 0).
        Yellow => "Yellow",
        /// Green (50, 204, 88).
        Green => "Green",
        /// Mint (0, 209, 187).
        Mint => "Mint",
        /// Teal (0, 202, 216).
        Teal => "Teal",
        /// Cyan (30, 201, 243).
        Cyan => "Cyan",
        /// Blue (0, 140, 255).
        Blue => "Blue",
        /// Indigo (103, 104, 250).
        Indigo => "Indigo",
        /// Purple (211, 50, 233).
        Purple => "Purple",
        /// The palette pink.
        Pink => "Pink",
        /// Brown (177, 132, 98).
        Brown => "Brown",
        /// Gray (142, 142, 147).
        Gray => "Gray",
    }
}

named_enum! {
    /// The two ways secondary colors are drawn.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub enum Scheme {
        /// Distinct secondary inks.
        Multicolor => "Multicolor",
        /// One secondary ink stepped through shades.
        Multitone => "Multitone",
    }
}

named_enum! {
    /// The side of the figure the primary ink lands on.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub enum Target {
        /// The primary on the filled cells, secondaries on the empty.
        Fill => "Fill",
        /// The primary on the empty cells, secondaries on the filled.
        Void => "Void",
    }
}

impl Edition {
    /// Returns the cell-painting mode this edition renders with, or None for Random, which scatters.
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
}

const LEVELS: [u8; 2] = [33, 66];

/// The constraints a caller may put on a random paint.
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Config {
    /// The editions allowed, or None for all seven.
    pub editions: Option<Vec<Edition>>,
    /// The primary inks allowed, or None for black and white.
    pub primaries: Option<Vec<Ink>>,
    /// The forced target, or None for a coin flip.
    pub target: Option<Target>,
}

/// A complete coloring recipe for one cell.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
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
}

/// Draws a random edition from the allowed list, or from all seven.
pub fn random_edition(editions: Option<&[Edition]>, rng: &mut Rng) -> Edition {
    match editions {
        Some(list) if !list.is_empty() => *rng.choice(list).expect("a list with items"),
        _ => *rng.choice(&Edition::all()).expect("seven editions"),
    }
}

fn random_primary(primaries: Option<&[Ink]>, rng: &mut Rng) -> Ink {
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

/// Redraws the paint's secondary inks and shades under its scheme.
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

/// Draws the paint's scheme, target and primary under the config, then rerolls the rest.
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
        .filter(|(&t, _)| t == target_value)
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

/// Tags the cell for the Layers and Neighbors editions and returns the distinct tag count on the secondary side.
///
/// # Errors
///
/// Errs when the Neighbors mask does not fit the cell.
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

/// Tags the cell for Layers and Neighbors paints and sizes the palette to the tag count.
///
/// # Errors
///
/// Errs when the tagging does.
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

/// Colors the cell from the paint's inks under its edition mode, scattering the Random edition from the stream.
///
/// # Errors
///
/// Errs when a multitone paint carries no base color to shade.
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

/// Replays a stored paint onto a cell, tagging first and applying it from the stream.
///
/// # Errors
///
/// Errs when the tagging or the coloring does.
pub fn coat(cell: &mut Cell, paint: &Paint, mask: Option<&Tensor>, rng: &mut Rng) -> Result<()> {
    tag(cell, paint.edition, paint.target, mask)?;
    apply(paint, cell, rng)
}

/// Draws a random paint under the config, applies it to the cell, and returns the recipe.
///
/// # Errors
///
/// Errs when the tagging or the coloring does.
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::colors::{BLUE, GREEN, RED, WHITE};
    use crate::core::json;
    use crate::math::atoms;
    fn round_trip(paint: &Paint) -> Paint {
        serde_json::from_value(serde_json::to_value(paint).unwrap()).unwrap()
    }
    #[test]
    fn simple_paint_colors_every_cell() {
        let mut rng = Rng::new(1);
        let mut cell = Cell::new(atoms::carpet_2d(9));
        let config = Config::default();
        let _ = paint(&mut cell, &config, None, &mut rng).unwrap();
        assert!(cell.colors.is_some());
        let colors = cell.colors.as_ref().unwrap();
        assert_eq!(colors.len(), cell.size());
        assert!(colors.iter().all(|rgba| rgba[3] == 255));
        assert_eq!(cell.size(), 81);
    }
    #[test]
    fn every_edition_paints_2d() {
        for (i, edition) in Edition::all().into_iter().enumerate() {
            let mut rng = Rng::new(i as u64);
            let mut cell = Cell::new(atoms::carpet_2d(9));
            let mut p = Paint::new(edition);
            p = setup(p, &Config::default(), &mut rng);
            p = prime(p, &mut cell, None, &mut rng).unwrap();
            apply(&p, &mut cell, &mut rng).unwrap();
            let colors = cell.colors.as_ref().unwrap();
            assert_eq!(colors.len(), cell.size(), "edition {:?}", edition);
        }
    }
    #[test]
    fn every_edition_paints_3d() {
        for (i, edition) in Edition::all().into_iter().enumerate() {
            let mut rng = Rng::new(100 + i as u64);
            let mut cell = Cell::new(atoms::carpet_3d(3));
            let mut p = Paint::new(edition);
            p = setup(p, &Config::default(), &mut rng);
            p = prime(p, &mut cell, None, &mut rng).unwrap();
            apply(&p, &mut cell, &mut rng).unwrap();
            let colors = cell.colors.as_ref().unwrap();
            assert_eq!(colors.len(), cell.size(), "edition {:?} 3d", edition);
        }
    }
    #[test]
    fn paint_replays_its_seed() {
        let mut a = Cell::new(atoms::carpet_2d(5));
        let pa = paint(&mut a, &Config::default(), None, &mut Rng::new(7)).unwrap();
        let mut b = Cell::new(atoms::carpet_2d(5));
        let pb = paint(&mut b, &Config::default(), None, &mut Rng::new(7)).unwrap();
        assert_eq!(pa, pb);
        assert_eq!(a, b);
    }
    #[test]
    fn random_edition_scatters_from_the_stream() {
        let types = Tensor::of(vec![0, 1, 1, 0], vec![2, 2]).unwrap();
        let scattered = |seed: u64| {
            let mut cell = Cell::new(types.clone());
            scatter(
                &mut cell,
                &[RED, GREEN],
                &[BLUE, WHITE],
                &mut Rng::new(seed),
            );
            cell.colors.unwrap()
        };
        let colors = scattered(5);
        assert_eq!(colors, scattered(5));
        assert!(colors.iter().all(|c| c[3] == 255));
        assert!(colors[0] == [255, 61, 64, 255] || colors[0] == [50, 204, 88, 255]);
        assert!(colors[1] == [0, 140, 255, 255] || colors[1] == [255, 255, 255, 255]);
        assert!((0..40).any(|seed| scattered(seed) != colors));
    }
    #[test]
    fn multitone_builds_shade_ramp() {
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
        assert_eq!(p, round_trip(&p));
        let json = serde_json::to_value(&p).unwrap();
        assert_eq!(json["edition"], "Layers");
        assert_eq!(json["secondary"], json!(["Blue"]));
        let mut q = Paint::new(Edition::Simple);
        q.secondary = vec![Ink::Teal];
        assert_eq!(q, round_trip(&q));
    }
    #[test]
    fn paint_json_rejects_garbage() {
        let read = |value| serde_json::from_value::<Paint>(value);
        assert!(read(json!({})).is_err());
        assert!(read(json!({
            "edition": "Sparkle", "scheme": "Multicolor", "target": "Fill",
            "primary": "Black", "secondary": [], "shades": [],
        }))
        .is_err());
        assert!(read(json!({
            "edition": "Simple", "scheme": "Multicolor", "target": "Fill",
            "primary": "Black", "secondary": ["Beige"], "shades": [],
        }))
        .is_err());
        assert!(read(json!({
            "edition": "Simple", "scheme": "Multicolor", "target": "Fill",
            "primary": "Black", "secondary": [], "shades": ["soup"],
        }))
        .is_err());
    }
    #[test]
    fn names_parse_back() {
        for edition in Edition::all() {
            assert_eq!(edition, edition.name().parse().unwrap());
        }
        for ink in Ink::all() {
            assert_eq!(ink, ink.name().parse().unwrap());
        }
        for scheme in Scheme::all() {
            assert_eq!(scheme, scheme.name().parse().unwrap());
        }
        for target in Target::all() {
            assert_eq!(target, target.name().parse().unwrap());
        }
    }
    #[test]
    fn coat_replays_a_stored_paint_under_one_seed() {
        for edition in Edition::all() {
            let mut rng = Rng::new(11);
            let mut primed = Cell::new(atoms::carpet_2d(9));
            let mut p = Paint::new(edition);
            p = setup(p, &Config::default(), &mut rng);
            p = prime(p, &mut primed, None, &mut rng).unwrap();
            let stored = round_trip(&p);
            let mut a = Cell::new(atoms::carpet_2d(9));
            coat(&mut a, &stored, None, &mut Rng::new(1)).unwrap();
            let mut b = Cell::new(atoms::carpet_2d(9));
            coat(&mut b, &stored, None, &mut Rng::new(1)).unwrap();
            assert_eq!(a, b, "edition {:?}", edition);
            assert_eq!(a.colors.as_ref().unwrap().len(), a.size());
        }
    }
    #[test]
    fn coat_matches_the_generative_render() {
        for edition in [
            Edition::Simple,
            Edition::Index,
            Edition::Layers,
            Edition::Rows,
        ] {
            let mut rng = Rng::new(21);
            let mut lived = Cell::new(atoms::carpet_2d(9));
            let p = paint(
                &mut lived,
                &Config {
                    editions: Some(vec![edition]),
                    ..Config::default()
                },
                None,
                &mut rng,
            )
            .unwrap();
            let mut coated = Cell::new(atoms::carpet_2d(9));
            coat(&mut coated, &p, None, &mut rng).unwrap();
            assert_eq!(lived.colors, coated.colors, "edition {:?}", edition);
        }
    }
    #[test]
    fn default_neighbors_mask_is_the_moore_ring() {
        let offsets: [(usize, usize); 8] = [
            (0, 0),
            (0, 1),
            (0, 2),
            (1, 0),
            (1, 2),
            (2, 0),
            (2, 1),
            (2, 2),
        ];
        let mut types = Tensor::new(vec![45, 5]);
        for count in 0..9 {
            for &(dy, dx) in offsets.iter().take(count) {
                types.set(&[5 * count + 1 + dy, 1 + dx], 1).unwrap();
            }
        }
        let mut cell = Cell::new(types);
        let classes = tag(&mut cell, Edition::Neighbors, Target::Fill, None).unwrap();
        assert_eq!(classes, 9);
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
