use mrlycore::errors::{value_error, Result};

// THE WORD

/// Returns the Thue-Morse letter at the place, the parity of its binary digit sum.
///
/// ```
/// let word: Vec<u8> = (0..8).map(mrlynum::morse::letter).collect();
/// assert_eq!(word, vec![0, 1, 1, 0, 1, 0, 0, 1]);
/// ```
pub fn letter(place: u64) -> u8 {
    (place.count_ones() % 2) as u8
}

/// Builds the first letters of the Thue-Morse word by the digit rule.
pub fn digits(length: usize) -> Vec<u8> {
    (0..length as u64).map(letter).collect()
}

/// Builds the first letters of the Thue-Morse word by the substitution `0 -> 01`, `1 -> 10`.
///
/// The seed is the single letter 0, and the rounds double the length until it covers the ask.
///
/// ```
/// assert_eq!(mrlynum::morse::substitution(8), mrlynum::morse::digits(8));
/// ```
pub fn substitution(length: usize) -> Vec<u8> {
    let mut word = vec![0u8];
    while word.len() < length {
        word = word.iter().flat_map(|&bit| [bit, 1 - bit]).collect();
    }
    word.truncate(length);
    word
}

/// Returns the substitution stage after the rounds, a word of length two to the rounds.
pub fn stage(rounds: usize) -> Vec<u8> {
    let mut word = vec![0u8];
    for _ in 0..rounds {
        word = word.iter().flat_map(|&bit| [bit, 1 - bit]).collect();
    }
    word
}

// RUNS

/// Returns the lengths of the maximal blocks of one repeated letter, in order.
///
/// ```
/// assert_eq!(mrlynum::morse::runs(&[0, 1, 1, 0]), vec![1, 2, 1]);
/// ```
pub fn runs(word: &[u8]) -> Vec<usize> {
    let mut out: Vec<usize> = Vec::new();
    for (place, &bit) in word.iter().enumerate() {
        if place > 0 && bit == word[place - 1] {
            *out.last_mut().unwrap() += 1;
        } else {
            out.push(1);
        }
    }
    out
}

/// Returns the run-boundary word, one wherever a letter differs from the next.
pub fn boundary(word: &[u8]) -> Vec<u8> {
    word.windows(2).map(|pair| pair[0] ^ pair[1]).collect()
}

/// Builds the period-doubling word by the substitution `1 -> 10`, `0 -> 11`, from the seed 1.
///
/// ```
/// let word = mrlynum::morse::doubling(8);
/// assert_eq!(word, vec![1, 0, 1, 1, 1, 0, 1, 0]);
/// assert_eq!(word, mrlynum::morse::boundary(&mrlynum::morse::digits(9)));
/// ```
pub fn doubling(length: usize) -> Vec<u8> {
    let mut word = vec![1u8];
    while word.len() < length {
        word = word
            .iter()
            .flat_map(|&bit| if bit == 1 { [1, 0] } else { [1, 1] })
            .collect();
    }
    word.truncate(length);
    word
}

// LIFTS

/// The four ways the word lifts from a line to the plane, one sign at every site.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lift {
    /// `t(i) xor t(j)`, the sign grid of the two-by-two tile `[[+1, -1], [-1, +1]]`.
    Parity,
    /// `t(i and j)`, the Walsh-Hadamard pattern.
    And,
    /// `t(i xor j)`, which the parity of a xor forces equal to the first lift.
    Xor,
    /// `t(i + j)`, the one that carries and so does not fold.
    Sum,
}

/// Lists the lifts in the order the gallery draws them.
pub const LIFTS: [Lift; 4] = [Lift::Parity, Lift::And, Lift::Xor, Lift::Sum];

impl Lift {
    /// Parses a lift's display name, or errs on an unknown name.
    pub fn parse(name: &str) -> Result<Lift> {
        match name {
            "parity" => Ok(Lift::Parity),
            "and" => Ok(Lift::And),
            "xor" => Ok(Lift::Xor),
            "sum" => Ok(Lift::Sum),
            other => value_error(format!("unknown lift {other:?}.")),
        }
    }
    /// Returns the lift's display name.
    pub fn name(self) -> &'static str {
        match self {
            Lift::Parity => "parity",
            Lift::And => "and",
            Lift::Xor => "xor",
            Lift::Sum => "sum",
        }
    }
    /// Returns the lift's formula, written the way the page prints it.
    pub fn formula(self) -> &'static str {
        match self {
            Lift::Parity => "t(i) xor t(j)",
            Lift::And => "t(i and j)",
            Lift::Xor => "t(i xor j)",
            Lift::Sum => "t(i + j)",
        }
    }
    /// Returns the sign at a site, zero for plus one and one for minus one.
    pub fn at(self, i: u64, j: u64) -> u8 {
        match self {
            Lift::Parity => letter(i) ^ letter(j),
            Lift::And => letter(i & j),
            Lift::Xor => letter(i ^ j),
            Lift::Sum => letter(i + j),
        }
    }
}

/// Builds a lift as a row-major sign grid of the side, zero for plus one and one for minus one.
pub fn lift(kind: Lift, side: usize) -> Vec<u8> {
    let mut out = vec![0u8; side * side];
    for i in 0..side {
        for j in 0..side {
            out[i * side + j] = kind.at(i as u64, j as u64);
        }
    }
    out
}

// KRONECKER

/// Folds a tile of the side into its Kronecker power at the level, one bit per site.
///
/// The bits ride the exclusive or, so the power is the digit rule of the tile: a site takes the
/// exclusive or of the tile's bits at its base-side digit pairs.
pub fn power(tile: &[u8], number: usize, level: usize) -> Result<Vec<u8>> {
    if tile.len() != number * number {
        return value_error(format!(
            "a side-{number} tile wants {} bits.",
            number * number
        ));
    }
    let side = match number.checked_pow(level as u32) {
        Some(side) => side,
        None => return value_error("that level passes what a machine holds."),
    };
    let mut out = vec![0u8; side * side];
    for r in 0..side {
        for c in 0..side {
            let (mut bit, mut row, mut col) = (0u8, r, c);
            for _ in 0..level {
                bit ^= tile[(row % number) * number + col % number];
                row /= number;
                col /= number;
            }
            out[r * side + c] = bit;
        }
    }
    Ok(out)
}

/// The verdict on whether a grid is the Kronecker power of its own corner tile.
#[derive(Clone, Debug)]
pub struct Fold {
    /// The corner tile the test folds, row major.
    pub tile: Vec<u8>,
    /// The side of the tile.
    pub number: usize,
    /// The count of tile factors the side asks for.
    pub level: usize,
    /// Whether the grid is that tile's Kronecker power.
    pub folds: bool,
    /// The count of sites where the grid and the power differ.
    pub faults: usize,
    /// The first differing site in row-major order, when there is one.
    pub first: Option<(usize, usize)>,
}

/// Tests a grid against the Kronecker power of its own corner tile.
///
/// The corner tile is the only candidate worth testing. If the grid is `T` folded `L` times then
/// its corner block is `T` with every bit flipped by `(L - 1) t00`, and folding that block `L`
/// times flips the grid by `L (L - 1) t00`, which is even. So a grid folds if and only if it
/// folds from its corner block, and no search over tiles is needed.
pub fn fold(grid: &[u8], side: usize, number: usize) -> Result<Fold> {
    if grid.len() != side * side {
        return value_error(format!("a side-{side} grid wants {} bits.", side * side));
    }
    if number < 2 {
        return value_error("a tile side is two or more.");
    }
    let mut level = 0usize;
    let mut reach = 1usize;
    while reach < side {
        reach *= number;
        level += 1;
    }
    if reach != side {
        return value_error(format!("side {side} is no power of {number}."));
    }
    let mut tile = vec![0u8; number * number];
    for r in 0..number {
        for c in 0..number {
            tile[r * number + c] = grid[r * side + c];
        }
    }
    let folded = power(&tile, number, level)?;
    let mut faults = 0usize;
    let mut first = None;
    for (at, (&here, &there)) in grid.iter().zip(&folded).enumerate() {
        if here != there {
            faults += 1;
            first.get_or_insert((at / side, at % side));
        }
    }
    Ok(Fold {
        tile,
        number,
        level,
        folds: faults == 0,
        faults,
        first,
    })
}

// THE FILTER

/// Blows a grid up by the scale, every site becoming a scale-by-scale block.
pub fn upsample(grid: &[u8], side: usize, scale: usize) -> Vec<u8> {
    let wide = side * scale;
    let mut out = vec![0u8; wide * wide];
    for r in 0..wide {
        for c in 0..wide {
            out[r * wide + c] = grid[(r / scale) * side + c / scale];
        }
    }
    out
}

/// Repeats a tile until it fills a grid of the side.
pub fn repeat(tile: &[u8], number: usize, side: usize) -> Vec<u8> {
    let mut out = vec![0u8; side * side];
    for r in 0..side {
        for c in 0..side {
            out[r * side + c] = tile[(r % number) * number + c % number];
        }
    }
    out
}

/// Exclusive-ors two grids of the same length, site by site.
pub fn difference(a: &[u8], b: &[u8]) -> Vec<u8> {
    a.iter().zip(b).map(|(&x, &y)| x ^ y).collect()
}

/// Counts the sites where two grids of the same length differ.
pub fn faults(a: &[u8], b: &[u8]) -> usize {
    a.iter().zip(b).filter(|(x, y)| x != y).count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_two_constructions_of_the_word_agree() {
        assert_eq!(digits(4096), substitution(4096));
        assert_eq!(stage(6), digits(64));
        assert_eq!(
            digits(16),
            vec![0, 1, 1, 0, 1, 0, 0, 1, 1, 0, 0, 1, 0, 1, 1, 0]
        );
    }

    #[test]
    fn the_word_is_cube_free_at_the_letter() {
        let word = digits(1 << 16);
        assert_eq!(runs(&word).into_iter().max(), Some(2));
        let counts = runs(&word);
        assert_eq!(counts.iter().filter(|&&run| run == 1).count(), 21846);
        assert_eq!(counts.iter().filter(|&&run| run == 2).count(), 21845);
        assert_eq!(word.iter().map(|&bit| bit as usize).sum::<usize>(), 1 << 15);
    }

    #[test]
    fn the_boundary_word_is_the_period_doubling_word() {
        let word = digits(65537);
        assert_eq!(boundary(&word), doubling(65536));
    }

    #[test]
    fn three_of_the_four_lifts_fold_and_the_sum_does_not() {
        let side = 64;
        for kind in [Lift::Parity, Lift::And, Lift::Xor] {
            let read = fold(&lift(kind, side), side, 2).unwrap();
            assert!(read.folds, "{}", kind.formula());
            assert_eq!(read.level, 6);
        }
        assert_eq!(
            fold(&lift(Lift::Parity, side), side, 2).unwrap().tile,
            vec![0, 1, 1, 0]
        );
        assert_eq!(
            fold(&lift(Lift::And, side), side, 2).unwrap().tile,
            vec![0, 0, 0, 1]
        );
        let sum = fold(&lift(Lift::Sum, side), side, 2).unwrap();
        assert!(!sum.folds);
        assert_eq!(sum.first, Some((1, 3)));
        assert_eq!(sum.faults, 1376);
    }

    #[test]
    fn the_xor_lift_is_the_parity_lift() {
        for side in [2, 4, 8, 16, 32] {
            assert_eq!(lift(Lift::Xor, side), lift(Lift::Parity, side));
        }
        assert_eq!(faults(&lift(Lift::Sum, 32), &lift(Lift::Parity, 32)), 448);
    }

    #[test]
    fn the_sum_lift_is_flat_on_every_antidiagonal() {
        let side = 32;
        let grid = lift(Lift::Sum, side);
        for r in 0..side {
            for c in 0..side {
                assert_eq!(grid[r * side + c], grid[c * side + r]);
                if r > 0 && c + 1 < side {
                    assert_eq!(grid[r * side + c], grid[(r - 1) * side + c + 1]);
                }
            }
        }
    }

    #[test]
    fn the_difference_of_two_sign_levels_is_the_tile_repeated() {
        let tile = vec![0, 1, 1, 0];
        for level in 1..6 {
            let coarse = power(&tile, 2, level).unwrap();
            let fine = power(&tile, 2, level + 1).unwrap();
            let side = 1 << level;
            let grown = upsample(&coarse, side, 2);
            assert_eq!(difference(&grown, &fine), repeat(&tile, 2, side * 2));
        }
    }
}
