use mrly_error::{Result, value_error};
use mrly_name::named_enum;
use serde::{Deserialize, Serialize};

pub use mrly_bang::catalog::{Catalog, Source};
pub use mrly_math::catalog::{CLASSICS_2D, CLASSICS_3D, Design, classics};

pub const MIN_SIDE: usize = 2;

pub const MAX_SIDE: usize = 64;

pub const MAX_LEVEL: usize = 6;

pub const MAX_SLOTS: usize = 6;

named_enum! {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub enum Group {
        General => "General",
        Fractal => "Fractal",
        Magic => "Magic",
        Special => "Special",
        Mosaic => "Mosaic",
    }
}

named_enum! {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub enum Parity {
        Evens => "Evens",
        Odds => "Odds",
        Both => "Both",
    }
}

impl Parity {
    pub fn keep(self, n: usize) -> bool {
        match self {
            Parity::Evens => n.is_multiple_of(2),
            Parity::Odds => !n.is_multiple_of(2),
            Parity::Both => true,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Tile {
    pub group: Group,
    pub factor: usize,
    pub sources: Vec<Source>,
    pub numbers: Vec<usize>,
    pub levels: Vec<usize>,
    pub rotations: Vec<usize>,
    pub invert: bool,
    pub flip: bool,
    pub width: usize,
    pub height: usize,
}

impl Tile {
    pub fn new(group: Group) -> Tile {
        Tile {
            group,
            factor: 0,
            sources: Vec::new(),
            numbers: Vec::new(),
            levels: Vec::new(),
            rotations: Vec::new(),
            invert: false,
            flip: false,
            width: 0,
            height: 0,
        }
    }
    pub fn size(mut self, width: usize, height: usize) -> Tile {
        self.width = width;
        self.height = height;
        self
    }
    pub fn max_size(&self) -> usize {
        self.width.max(self.height)
    }
    pub fn degenerate(&self) -> bool {
        self.group == Group::Magic
            && self.sources.len() > 1
            && uniform(&self.sources)
            && uniform(&self.numbers)
    }
    pub fn resize(&mut self) {
        let lead = self.numbers.first().copied().unwrap_or(0);
        if matches!(self.group, Group::General | Group::Fractal | Group::Magic) {
            self.factor = lead;
        }
        let size = match self.group {
            Group::General => lead,
            Group::Fractal => u32::try_from(self.levels.first().copied().unwrap_or(1))
                .ok()
                .and_then(|level| lead.checked_pow(level))
                .unwrap_or(0),
            Group::Magic => self
                .numbers
                .iter()
                .try_fold(1usize, |acc, &n| acc.checked_mul(n))
                .unwrap_or(0),
            Group::Special | Group::Mosaic => self.factor.checked_mul(lead).unwrap_or(0),
        };
        self.width = size;
        self.height = size;
    }
    pub fn check(&self) -> Result<()> {
        let slots = self.sources.len();
        let wanted = match self.group {
            Group::Mosaic => slots == 3,
            Group::Magic => (2..=MAX_SLOTS).contains(&slots),
            _ => slots == 1,
        };
        if !wanted {
            return value_error("wrong slot count");
        }
        if self.numbers.len() != slots
            || self.levels.len() != slots
            || self.rotations.len() != slots
        {
            return value_error("ragged slots");
        }
        if self
            .numbers
            .iter()
            .any(|&n| !(MIN_SIDE..=MAX_SIDE).contains(&n))
        {
            return value_error("numbers are 2 to 64");
        }
        if self.rotations.iter().any(|&r| r > 3) {
            return value_error("rotation is 0 to 3");
        }
        if self.flip && self.group != Group::Special {
            return value_error("flip is special only");
        }
        if self.group == Group::Fractal {
            if !(1..=MAX_LEVEL).contains(&self.levels[0]) {
                return value_error("level is 1 to 6");
            }
        } else if self.levels.iter().any(|&l| l != 1) {
            return value_error("level is fractal only");
        }
        if matches!(self.group, Group::Special | Group::Mosaic)
            && !(MIN_SIDE..=MAX_SIDE).contains(&self.factor)
        {
            return value_error("factor is 2 to 64");
        }
        if self.group == Group::Mosaic && self.numbers.iter().any(|&n| n != self.numbers[0]) {
            return value_error("mosaic shares one number");
        }
        let mut probe = self.clone();
        probe.resize();
        if probe.width != self.width || probe.height != self.height || probe.factor != self.factor {
            return value_error("sizes disagree");
        }
        if !(MIN_SIDE..=MAX_SIDE).contains(&self.max_size()) {
            return value_error("size is 2 to 64");
        }
        Ok(())
    }
}

const MIN_FACTOR: usize = 2;

pub fn uniform<T: PartialEq>(items: &[T]) -> bool {
    items.windows(2).all(|pair| pair[0] == pair[1])
}

fn factors(min_factor: usize, max_factor: usize, parity: Parity) -> Vec<usize> {
    (min_factor.max(MIN_FACTOR)..=max_factor)
        .filter(|&n| parity.keep(n))
        .collect()
}

pub fn generals(min_size: usize, max_size: usize, parity: Parity) -> Vec<usize> {
    factors(min_size, max_size, parity)
}

pub fn powers(min_size: usize, max_size: usize, parity: Parity) -> Vec<(usize, usize)> {
    let mut out = Vec::new();
    for n in factors(MIN_FACTOR, max_size, parity) {
        let mut level = 2;
        loop {
            match n.checked_pow(level as u32) {
                Some(size) if size <= max_size => {
                    if size >= min_size {
                        out.push((n, level));
                    }
                    level += 1;
                }
                _ => break,
            }
        }
    }
    out
}

pub fn size(number: i64, level: i64) -> Option<usize> {
    let number = usize::try_from(number).ok()?;
    let level = u32::try_from(level).ok()?;
    number.checked_pow(level)
}

pub fn products(min_size: usize, max_size: usize, count: usize, parity: Parity) -> Vec<Vec<usize>> {
    if count < 1 {
        return Vec::new();
    }
    fn walk(
        min_size: usize,
        max_size: usize,
        remaining: usize,
        parity: Parity,
        out: &mut Vec<Vec<usize>>,
    ) {
        if remaining == 1 {
            for n in factors(min_size, max_size, parity) {
                out.push(vec![n]);
            }
            return;
        }
        for n in factors(MIN_FACTOR, max_size, parity) {
            let next_min = min_size.div_ceil(n);
            let next_max = max_size / n;
            if next_max < MIN_FACTOR {
                continue;
            }
            let mut tails = Vec::new();
            walk(next_min, next_max, remaining - 1, parity, &mut tails);
            for tail in tails {
                let mut item = vec![n];
                item.extend(tail);
                out.push(item);
            }
        }
    }
    let mut out = Vec::new();
    walk(min_size, max_size, count, parity, &mut out);
    out
}

pub fn nestings(min_size: usize, max_size: usize, parity: Parity) -> Vec<Vec<usize>> {
    let mut out = Vec::new();
    let mut depth = 2;
    loop {
        let found = products(min_size, max_size, depth, parity);
        if found.is_empty() {
            if depth > 2 {
                break;
            }
            depth += 1;
            if depth > max_size {
                break;
            }
            continue;
        }
        out.extend(found);
        depth += 1;
    }
    out
}
