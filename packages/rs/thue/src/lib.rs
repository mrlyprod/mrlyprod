use mrly_error::{Result, value_error};
use mrly_name::named_enum;
use serde::{Deserialize, Serialize};

pub fn letter(place: u64) -> u8 {
    (place.count_ones() % 2) as u8
}

pub fn digits(length: usize) -> Vec<u8> {
    (0..length as u64).map(letter).collect()
}

pub fn substitution(length: usize) -> Vec<u8> {
    let mut word = vec![0u8];
    while word.len() < length {
        word = word.iter().flat_map(|&bit| [bit, 1 - bit]).collect();
    }
    word.truncate(length);
    word
}

pub fn stage(rounds: usize) -> Vec<u8> {
    let mut word = vec![0u8];
    for _ in 0..rounds {
        word = word.iter().flat_map(|&bit| [bit, 1 - bit]).collect();
    }
    word
}

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

pub fn boundary(word: &[u8]) -> Vec<u8> {
    word.windows(2).map(|pair| pair[0] ^ pair[1]).collect()
}

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

named_enum! {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
    pub enum Lift {
        Parity => "parity",
        And => "and",
        Xor => "xor",
        Sum => "sum",
    }
}

pub const LIFTS: [Lift; 4] = Lift::all();

impl Lift {
    pub fn formula(self) -> String {
        match self {
            Lift::Parity => "t(i) xor t(j)",
            Lift::And => "t(i and j)",
            Lift::Xor => "t(i xor j)",
            Lift::Sum => "t(i + j)",
        }
        .to_string()
    }
    pub fn at(self, i: u64, j: u64) -> u8 {
        match self {
            Lift::Parity => letter(i) ^ letter(j),
            Lift::And => letter(i & j),
            Lift::Xor => letter(i ^ j),
            Lift::Sum => letter(i + j),
        }
    }
}

pub fn lift(kind: Lift, side: usize) -> Vec<u8> {
    let mut out = vec![0u8; side * side];
    for i in 0..side {
        for j in 0..side {
            out[i * side + j] = kind.at(i as u64, j as u64);
        }
    }
    out
}

pub fn signs(types: &[u8]) -> Vec<u8> {
    types.iter().map(|&t| u8::from(t == 0)).collect()
}

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

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Fold {
    pub tile: Vec<u8>,
    pub number: usize,
    pub level: usize,
    pub folds: bool,
    pub faults: usize,
    pub first: Option<(usize, usize)>,
}

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

pub fn repeat(tile: &[u8], number: usize, side: usize) -> Vec<u8> {
    let mut out = vec![0u8; side * side];
    for r in 0..side {
        for c in 0..side {
            out[r * side + c] = tile[(r % number) * number + c % number];
        }
    }
    out
}

pub fn mask(a: &[u8], b: &[u8]) -> Vec<u8> {
    a.iter()
        .zip(b)
        .map(|(&x, &y)| u8::from(x != 0 && y != 0))
        .collect()
}

pub fn difference(a: &[u8], b: &[u8]) -> Vec<u8> {
    a.iter().zip(b).map(|(&x, &y)| x ^ y).collect()
}

pub fn faults(a: &[u8], b: &[u8]) -> usize {
    a.iter().zip(b).filter(|(x, y)| x != y).count()
}
