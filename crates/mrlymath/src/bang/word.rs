use super::factory::{self, MagicLayer};
use crate::name::Bang;
use mrlycore::errors::{value_error, Result};
use mrlycore::Tensor;

/// The plane geometry of one letter, the numbers a word's counts fold through.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Letter {
    /// The count of filled cells.
    pub fill: u128,
    /// The count of maximal horizontal runs of filled cells.
    pub runs_h: u128,
    /// The count of maximal vertical runs of filled cells.
    pub runs_v: u128,
    /// The count of rows whose first and last cells are both filled.
    pub touch_h: u128,
    /// The count of columns whose first and last cells are both filled.
    pub touch_v: u128,
    /// The count of 4-connected components.
    pub components: u128,
}

/// The counts a word carries at one prefix length.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Counts {
    /// The side, the product of the prefix's letter sides.
    pub side: u128,
    /// The filled cells, the product of the prefix's letter fills.
    pub fill: u128,
    /// The 4-connected components.
    pub components: u128,
    /// The maximal horizontal runs of filled cells.
    pub runs_h: u128,
    /// The maximal vertical runs of filled cells.
    pub runs_v: u128,
}

// GEOMETRY

fn tile_of(layer: &MagicLayer) -> Result<Tensor> {
    factory::create(
        layer.design.code,
        layer.number,
        layer.design.dimension,
        layer.design.base,
        1,
    )
}

fn pieces(tile: &Tensor) -> u128 {
    let (rows, cols) = (tile.shape[0], tile.shape[1]);
    let bytes = tile.bytes();
    let mut seen = vec![false; rows * cols];
    let mut count = 0u128;
    let mut stack: Vec<usize> = Vec::new();
    for start in 0..rows * cols {
        if bytes[start] == 0 || seen[start] {
            continue;
        }
        count += 1;
        seen[start] = true;
        stack.push(start);
        while let Some(at) = stack.pop() {
            let (r, c) = (at / cols, at % cols);
            let walk = |next: usize, seen: &mut Vec<bool>, stack: &mut Vec<usize>| {
                if bytes[next] != 0 && !seen[next] {
                    seen[next] = true;
                    stack.push(next);
                }
            };
            if r > 0 {
                walk(at - cols, &mut seen, &mut stack);
            }
            if r + 1 < rows {
                walk(at + cols, &mut seen, &mut stack);
            }
            if c > 0 {
                walk(at - 1, &mut seen, &mut stack);
            }
            if c + 1 < cols {
                walk(at + 1, &mut seen, &mut stack);
            }
        }
    }
    count
}

/// Reads one plane letter: its fill, its runs, the rows and columns that wrap into a
/// neighbouring copy, and its own components.
///
/// ```
/// use mrlymath::bang::{word, MagicLayer};
/// use mrlymath::name::Bang;
/// let gasket = word::letter(&MagicLayer::new(Bang::new(7, 2, 2), 2)).unwrap();
/// assert_eq!((gasket.fill, gasket.components), (3, 1));
/// assert_eq!((gasket.touch_h, gasket.touch_v), (1, 1));
/// ```
pub fn letter(layer: &MagicLayer) -> Result<Letter> {
    if layer.design.dimension != 2 {
        return value_error("a letter's run and contact counts are a plane reading.");
    }
    let tile = tile_of(layer)?;
    let (rows, cols) = (tile.shape[0], tile.shape[1]);
    let on = |r: usize, c: usize| tile.bytes()[r * cols + c] != 0;
    let (mut fill, mut runs_h, mut runs_v) = (0u128, 0u128, 0u128);
    let (mut touch_h, mut touch_v) = (0u128, 0u128);
    for r in 0..rows {
        for c in 0..cols {
            if !on(r, c) {
                continue;
            }
            fill += 1;
            if c == 0 || !on(r, c - 1) {
                runs_h += 1;
            }
            if r == 0 || !on(r - 1, c) {
                runs_v += 1;
            }
        }
        if on(r, 0) && on(r, cols - 1) {
            touch_h += 1;
        }
    }
    for c in 0..cols {
        if on(0, c) && on(rows - 1, c) {
            touch_v += 1;
        }
    }
    Ok(Letter {
        fill,
        runs_h,
        runs_v,
        touch_h,
        touch_v,
        components: pieces(&tile),
    })
}

// FOLD

fn mul(a: u128, b: u128) -> Option<u128> {
    a.checked_mul(b)
}

fn closed(read: &Letter) -> bool {
    read.components == 1 || (read.touch_h == 0 && read.touch_v == 0)
}

fn grow(state: &Counts, side: u128, read: &Letter) -> Option<Counts> {
    let fill = mul(state.fill, read.fill)?;
    let runs_h = mul(state.fill, read.runs_h - read.touch_h)?
        .checked_add(mul(state.runs_h, read.touch_h)?)?;
    let runs_v = mul(state.fill, read.runs_v - read.touch_v)?
        .checked_add(mul(state.runs_v, read.touch_v)?)?;
    let components = match (read.touch_h, read.touch_v) {
        (0, 0) => mul(state.fill, read.components)?,
        (0, _) => state.runs_v,
        (_, 0) => state.runs_h,
        _ => state.components,
    };
    Some(Counts {
        side: mul(state.side, side)?,
        fill,
        components,
        runs_h,
        runs_v,
    })
}

/// Folds a plane word letter by letter and returns the counts at every prefix.
///
/// Each letter folds the component count by its own geometry: a letter with no wrap-around
/// contact isolates every block, so the count becomes the outer fill times the letter's own
/// pieces; a connected letter with contacts on one axis merges blocks along that axis alone,
/// so the count becomes the outer run count; a connected letter with both contacts leaves the
/// block graph isomorphic to the cell graph and the count unmoved. A letter that is neither
/// connected nor contact-free both splits and merges and is refused.
///
/// The list stops at the last prefix whose counts fit a u128 rather than wrapping.
pub fn prefixes(layers: &[MagicLayer]) -> Result<Vec<Counts>> {
    let mut state = Counts {
        side: 1,
        fill: 1,
        components: 1,
        runs_h: 1,
        runs_v: 1,
    };
    let mut out = Vec::with_capacity(layers.len());
    for layer in layers {
        let read = letter(layer)?;
        if !closed(&read) {
            return value_error(format!(
                "letter c{} at side {} is neither connected nor contact-free, so its word has no closed component count.",
                layer.design.code, layer.number
            ));
        }
        match grow(&state, layer.number as u128, &read) {
            Some(next) => state = next,
            None => break,
        }
        out.push(state);
    }
    Ok(out)
}

/// Counts the 4-connected components of a plane word without drawing it.
///
/// ```
/// use mrlymath::bang::{word, MagicLayer};
/// use mrlymath::name::Bang;
/// let domino = MagicLayer::new(Bang::new(3, 2, 2), 2);
/// let diagonal = MagicLayer::new(Bang::new(6, 2, 2), 2);
/// assert_eq!(word::components(&[domino, diagonal]).unwrap(), 4);
/// assert_eq!(word::components(&[diagonal, domino]).unwrap(), 2);
/// ```
pub fn components(layers: &[MagicLayer]) -> Result<u128> {
    let counts = prefixes(layers)?;
    if counts.len() < layers.len() {
        return value_error("the word's component count passes what a u128 holds.");
    }
    match counts.last() {
        Some(last) => Ok(last.components),
        None => value_error("a word needs at least one letter."),
    }
}

// PRODUCTS

/// Lists the filled cells of every letter, the product of which is the word's fill.
pub fn fills(layers: &[MagicLayer]) -> Result<Vec<u128>> {
    layers
        .iter()
        .map(|layer| Ok(u128::from(tile_of(layer)?.sum())))
        .collect()
}

/// Returns the side of a word, the product of its letter sides.
pub fn side(layers: &[MagicLayer]) -> Result<u128> {
    let mut out = 1u128;
    for layer in layers {
        match out.checked_mul(layer.number as u128) {
            Some(next) => out = next,
            None => return value_error("the word's side passes what a u128 holds."),
        }
    }
    Ok(out)
}

/// Returns the filled cells of a word, the product of its letter fills.
pub fn fill(layers: &[MagicLayer]) -> Result<u128> {
    let mut out = 1u128;
    for count in fills(layers)? {
        match out.checked_mul(count) {
            Some(next) => out = next,
            None => return value_error("the word's fill passes what a u128 holds."),
        }
    }
    Ok(out)
}

/// Returns the scale dimension of a word, the sum of the log fills over the sum of the log sides.
///
/// ```
/// use mrlymath::bang::{word, MagicLayer};
/// use mrlymath::name::Bang;
/// let carpet = MagicLayer::new(Bang::new(7, 2, 2), 3);
/// let two = word::dimension(&[carpet, carpet]).unwrap();
/// assert!((two - 8f64.ln() / 3f64.ln()).abs() < 1e-12);
/// ```
pub fn dimension(layers: &[MagicLayer]) -> Result<f64> {
    if layers.is_empty() {
        return value_error("a word needs at least one letter.");
    }
    let counts = fills(layers)?;
    let mut top = 0.0;
    let mut bottom = 0.0;
    for (layer, count) in layers.iter().zip(&counts) {
        top += (*count as f64).ln();
        bottom += (layer.number as f64).ln();
    }
    if bottom <= 0.0 {
        return value_error("a word needs a letter of side two or more.");
    }
    Ok(top / bottom)
}

/// Returns the shortest whole period of the letter list, its own length when no shorter block repeats.
///
/// ```
/// use mrlymath::bang::{word, MagicLayer};
/// use mrlymath::name::Bang;
/// let a = MagicLayer::new(Bang::new(7, 2, 2), 3);
/// let b = MagicLayer::new(Bang::new(9, 2, 2), 5);
/// assert_eq!(word::period(&[a, b, a, b]), 2);
/// assert_eq!(word::period(&[a, b, a]), 3);
/// ```
pub fn period(layers: &[MagicLayer]) -> usize {
    let length = layers.len();
    for step in 1..length {
        if !length.is_multiple_of(step) {
            continue;
        }
        if (0..length).all(|i| layers[i] == layers[i % step]) {
            return step;
        }
    }
    length
}

/// Returns whether every letter renders at its own residue base, the native case where a
/// periodic word folds to one residue rule at the product base.
pub fn native(layers: &[MagicLayer]) -> bool {
    !layers.is_empty() && layers.iter().all(|l| l.number == l.design.base)
}

/// Returns the constant-word component functional of a plane word's letter frequencies,
/// in log two units.
///
/// It reads `sum_c f_c log2 comp(A_c)`, the one linear functional exact on constant words,
/// which on the plane alphabet at side two is `(f_6 + f_9) log 2`. It is a prediction and not
/// a theorem: at interior frequency it is refuted on 78 of the 105 letter pairs and exact on 27.
pub fn constant_functional(layers: &[MagicLayer]) -> Result<f64> {
    if layers.is_empty() {
        return value_error("a word needs at least one letter.");
    }
    let mut total = 0.0;
    for layer in layers {
        total += (letter(layer)?.components as f64).log2();
    }
    Ok(total / layers.len() as f64)
}

// SCHEDULES

/// The named infinite schedules over an ordered pair of letters.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Schedule {
    /// The Thue-Morse word, the parity of the binary digit sum of the place.
    ThueMorse,
    /// The two letters alternating, the periodic control at the same frequencies.
    Periodic,
    /// The first letter repeated, the constant control.
    Constant,
}

impl Schedule {
    /// Parses a schedule's display name, or an error for an unknown name.
    pub fn parse(name: &str) -> Result<Schedule> {
        match name {
            "thue-morse" => Ok(Schedule::ThueMorse),
            "periodic" => Ok(Schedule::Periodic),
            "constant" => Ok(Schedule::Constant),
            other => value_error(format!("unknown schedule {other:?}.")),
        }
    }
    /// Returns the letter frequencies the schedule tends to.
    pub fn frequencies(self) -> (f64, f64) {
        match self {
            Schedule::Constant => (1.0, 0.0),
            _ => (0.5, 0.5),
        }
    }
    /// Returns the letter the schedule takes at the place, zero or one.
    pub fn place(self, index: usize) -> usize {
        match self {
            Schedule::ThueMorse => thue_morse(index),
            Schedule::Periodic => index % 2,
            Schedule::Constant => 0,
        }
    }
}

/// Returns the Thue-Morse letter at the place, the parity of its binary digit sum.
///
/// ```
/// let word: Vec<usize> = (0..8).map(mrlymath::bang::word::thue_morse).collect();
/// assert_eq!(word, vec![0, 1, 1, 0, 1, 0, 0, 1]);
/// ```
pub fn thue_morse(index: usize) -> usize {
    index.count_ones() as usize % 2
}

/// Spells the first letters of a schedule over an ordered pair of letters.
pub fn spell(schedule: Schedule, pair: (MagicLayer, MagicLayer), length: usize) -> Vec<MagicLayer> {
    (0..length)
        .map(|index| {
            if schedule.place(index) == 0 {
                pair.0
            } else {
                pair.1
            }
        })
        .collect()
}

/// Returns the prefix rates of a plane word in log two units, the component rate
/// `(1/L) log2 comp` and the fill rate `(1/L) log2 fill` at every prefix length.
///
/// At interior letter frequency the two meet: the component exponent is order-blind and equals
/// the fill exponent on every one of the 105 letter pairs but the domino against the full tile.
/// The list stops at the last prefix whose counts fit a u128.
pub fn rates(layers: &[MagicLayer]) -> Result<Vec<(f64, f64)>> {
    Ok(prefixes(layers)?
        .iter()
        .enumerate()
        .map(|(index, counts)| {
            let length = (index + 1) as f64;
            (
                (counts.components as f64).log2() / length,
                (counts.fill as f64).log2() / length,
            )
        })
        .collect())
}

// STAIRCASE

/// Builds the carpet staircase word to the depth, the stacked prefixes `magic(3)`,
/// then `magic(3,5)`, then `magic(3,5,7)`, and so on.
///
/// The letter at odd side `2j + 1` occurs `depth - j + 1` times in the first `depth` blocks,
/// so the word holds `depth (depth + 1) / 2` letters and its dimension is the occurrence-weighted
/// average of the per-letter dimensions.
///
/// ```
/// use mrlymath::bang::word;
/// assert_eq!(word::staircase(3).unwrap().len(), 6);
/// let one = word::dimension(&word::staircase(1).unwrap()).unwrap();
/// assert!((one - 8f64.ln() / 3f64.ln()).abs() < 1e-9);
/// ```
pub fn staircase(depth: usize) -> Result<Vec<MagicLayer>> {
    if depth < 1 {
        return value_error("a staircase needs at least one block.");
    }
    let carpet = Bang::new(7, 2, 2);
    let mut out = Vec::new();
    for step in 1..=depth {
        for place in 1..=step {
            out.push(MagicLayer::new(carpet, 2 * place + 1));
        }
    }
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bang::magic;

    const CODES: [u128; 15] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];

    fn plain(code: u128, number: usize) -> MagicLayer {
        MagicLayer::new(Bang::new(code, 2, 2), number)
    }

    #[test]
    fn the_fold_matches_the_drawn_word_on_every_short_plane_word() {
        for a in CODES {
            for b in CODES {
                let two = [plain(a, 2), plain(b, 2)];
                assert_eq!(
                    components(&two).unwrap(),
                    pieces(&magic(&two).unwrap()),
                    "({a},{b})"
                );
                for c in CODES {
                    let three = [plain(a, 2), plain(b, 2), plain(c, 2)];
                    assert_eq!(
                        components(&three).unwrap(),
                        pieces(&magic(&three).unwrap()),
                        "({a},{b},{c})"
                    );
                }
            }
        }
    }

    #[test]
    fn order_moves_the_component_count_on_the_minimal_pair() {
        let a = [plain(3, 2), plain(6, 2)];
        let b = [plain(6, 2), plain(3, 2)];
        assert_eq!((components(&a).unwrap(), components(&b).unwrap()), (4, 2));
        assert_eq!(fill(&a).unwrap(), fill(&b).unwrap());
        assert_eq!(side(&a).unwrap(), side(&b).unwrap());
    }

    #[test]
    fn the_checkerboard_family_reaches_the_component_ceiling() {
        for length in 2..=8usize {
            let mut word = vec![plain(15, 2); length - 1];
            word.push(plain(6, 2));
            assert_eq!(components(&word).unwrap(), 2 * 4u128.pow(length as u32 - 1));
        }
    }

    #[test]
    fn a_heavy_letter_never_moves_the_count() {
        for code in [7, 11, 13, 14, 15] {
            let word = [plain(6, 2), plain(9, 2), plain(code, 2)];
            assert_eq!(components(&word).unwrap(), components(&word[..2]).unwrap());
        }
    }

    #[test]
    fn a_letter_that_splits_and_merges_is_refused() {
        let void = plain(9, 5);
        assert!(components(&[plain(7, 3), void]).is_err());
    }

    #[test]
    fn the_staircase_prints_its_five_dimensions() {
        let pinned = [
            1.892789261,
            1.892315261,
            1.893034267,
            1.894190425,
            1.895495742,
        ];
        for (step, want) in pinned.iter().enumerate() {
            let got = dimension(&staircase(step + 1).unwrap()).unwrap();
            assert!((got - want).abs() < 5e-10, "n={} {got}", step + 1);
        }
        assert!(pinned[1] < pinned[0]);
    }

    #[test]
    fn the_thue_morse_rate_climbs_to_the_fill_exponent() {
        let pair = (plain(3, 2), plain(7, 2));
        let word = spell(Schedule::ThueMorse, pair, 120);
        let rows = rates(&word).unwrap();
        assert_eq!(rows.len(), 98);
        for (at, row) in rows.iter().enumerate() {
            if (at + 1).is_multiple_of(2) {
                assert!((row.1 - 0.5 * 6f64.log2()).abs() < 1e-12, "L={}", at + 1);
            }
        }
        let (component, fill) = rows[97];
        assert!(component < fill && fill - component < 0.05);
        assert!(component > rows[31].0);
        assert_eq!(components(&word[..16]).unwrap(), 390573);
        assert_eq!(constant_functional(&word).unwrap(), 0.0);
    }

    #[test]
    fn the_period_reads_the_block_and_the_native_letters() {
        let carpet = plain(7, 3);
        let native_pair = [MagicLayer::new(Bang::new(7, 2, 2), 2), carpet];
        assert_eq!(period(&[carpet, carpet, carpet]), 1);
        assert!(native(&[MagicLayer::new(Bang::new(7, 2, 2), 2)]));
        assert!(!native(&native_pair));
    }
}
