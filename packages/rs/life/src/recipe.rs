use crate::source::{Source, counts};
use mrly_core::cell::Cell;
use mrly_core::error::{Result, value_error};
use mrly_core::named_enum;
use mrly_core::tensor::Tensor;
use mrly_math::bang::Code;
use mrly_math::bang::catalog::Design;
use mrly_math::two;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

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

const ROLLS: [&str; 4] = ["", "_zeros", "_ones", "_zeros_ones"];

named_enum! {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
    pub enum Mode {
        Island => "island",
        Wallpaper => "wallpaper",
    }
}

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

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Stop {
    pub kind: String,
    pub generation: u32,
    pub period: u32,
    pub dr: i64,
    pub dc: i64,
    pub wall_by: u32,
}

impl Stop {
    fn at(kind: &str, generation: u32, period: u32) -> Stop {
        Stop {
            kind: kind.to_string(),
            generation,
            period,
            dr: 0,
            dc: 0,
            wall_by: 0,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Shape {
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

    fn token(self) -> String {
        match self {
            Shape::Named(design) => NAMED.iter().find(|(d, _)| *d == design).map_or_else(
                || design.name().to_lowercase(),
                |(_, name)| (*name).to_string(),
            ),
            Shape::Code(code) => format!("code{code}"),
        }
    }

    fn build(self, side: usize, level: usize) -> Result<Grid> {
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

fn shapes() -> Vec<Shape> {
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
    Ok((shape, side, level))
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Mask {
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

    fn token(self) -> String {
        match self {
            Mask::Design { shape, side, level } => format!("{}.{side}.{level}", shape.token()),
            Mask::Copy { inverted: false } => "copy".to_string(),
            Mask::Copy { inverted: true } => "copyinv".to_string(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum Side {
    Drawn {
        seq: Source,
        zeros: bool,
        ones: bool,
    },
    List(Vec<usize>),
}

impl Side {
    fn parse(token: &str) -> Result<Side> {
        if token.starts_with(|c: char| c.is_ascii_digit()) {
            let mut values = Vec::new();
            for part in token.split(',') {
                match part.parse::<usize>() {
                    Ok(v) => values.push(v),
                    Err(_) => return value_error(format!("count {part:?} is not a number.")),
                }
            }
            values.sort_unstable();
            values.dedup();
            return Ok(Side::List(values));
        }
        let Some((seq, tail)) = Source::read(token) else {
            return value_error(format!("no source starts {token:?}."));
        };
        let (zeros, tail) = match tail.strip_prefix("_zeros") {
            Some(rest) => (true, rest),
            None => (false, tail),
        };
        let (ones, tail) = match tail.strip_prefix("_ones") {
            Some(rest) => (true, rest),
            None => (false, tail),
        };
        if !tail.is_empty() {
            return value_error(format!("side {token:?} has a tail {tail:?}."));
        }
        Ok(Side::Drawn { seq, zeros, ones })
    }

    fn token(&self) -> String {
        match self {
            Side::Drawn { seq, zeros, ones } => {
                let mut out = seq.name();
                if *zeros {
                    out.push_str("_zeros");
                }
                if *ones {
                    out.push_str("_ones");
                }
                out
            }
            Side::List(values) => values
                .iter()
                .map(ToString::to_string)
                .collect::<Vec<_>>()
                .join(","),
        }
    }

    fn values(&self, budget: usize) -> Result<Vec<usize>> {
        match self {
            Side::Drawn { seq, zeros, ones } => counts(*seq, budget, *zeros, *ones),
            Side::List(values) => Ok(values.iter().copied().filter(|&v| v <= budget).collect()),
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
            "birth" => birth = Some(Side::parse(value)?.token()),
            "survive" => survive = Some(Side::parse(value)?.token()),
            "x" => x = value.parse::<u32>().ok(),
            "cap" => cap = value.parse::<u32>().ok(),
            other => return value_error(format!("unknown key {other:?}.")),
        }
    }
    let (Some(mask), Some(birth), Some(survive), Some(x)) = (mask, birth, survive, x) else {
        return value_error(format!("recipe {name:?} misses a part."));
    };
    let cap = match (mode, cap) {
        (Mode::Island, Some(cap)) => cap,
        (Mode::Island, None) => return value_error("an island names its cap."),
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
    if recipe.mode == Mode::Island {
        out.push_str(&format!(" cap:{}", recipe.cap));
    }
    out
}

pub fn grammar() -> Grammar {
    Grammar {
        shapes: shapes().iter().map(|s| s.token()).collect(),
        sides: vec![3, 5, 7],
        levels: vec![1, 2],
        tilings: vec![1, 3, 5],
        modes: Mode::all().iter().map(|m| m.name().to_string()).collect(),
        mask_sides: vec![3, 5, 7, 9],
        mask_levels: vec![1, 2],
        copies: vec!["copy".to_string(), "copyinv".to_string()],
        sources: Source::all().iter().map(|s| s.name()).collect(),
        rolls: ROLLS.iter().map(ToString::to_string).collect(),
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Grid {
    h: usize,
    w: usize,
    bits: Vec<u8>,
}

impl Grid {
    fn new(h: usize, w: usize) -> Grid {
        Grid {
            h,
            w,
            bits: vec![0; h * w],
        }
    }

    fn full(h: usize, w: usize) -> Grid {
        Grid {
            h,
            w,
            bits: vec![1; h * w],
        }
    }

    fn of(h: usize, w: usize, bits: Vec<u8>) -> Grid {
        debug_assert_eq!(bits.len(), h * w);
        Grid { h, w, bits }
    }

    fn get(&self, r: usize, c: usize) -> u8 {
        self.bits[r * self.w + c]
    }

    fn set(&mut self, r: usize, c: usize, v: u8) {
        self.bits[r * self.w + c] = v;
    }

    fn population(&self) -> usize {
        self.bits.iter().filter(|&&b| b != 0).count()
    }

    fn is_empty(&self) -> bool {
        self.bits.iter().all(|&b| b == 0)
    }

    fn bbox(&self) -> Option<(usize, usize, usize, usize)> {
        let (mut r0, mut c0, mut r1, mut c1) = (usize::MAX, usize::MAX, 0, 0);
        for r in 0..self.h {
            let row = &self.bits[r * self.w..(r + 1) * self.w];
            let Some(first) = row.iter().position(|&b| b != 0) else {
                continue;
            };
            let last = row.iter().rposition(|&b| b != 0).unwrap_or(first);
            r0 = r0.min(r);
            r1 = r;
            c0 = c0.min(first);
            c1 = c1.max(last);
        }
        (r0 != usize::MAX).then_some((r0, c0, r1, c1))
    }

    fn window(&self, r0: usize, c0: usize, h: usize, w: usize) -> Grid {
        let mut out = Grid::new(h, w);
        for r in 0..h {
            out.bits[r * w..(r + 1) * w]
                .copy_from_slice(&self.bits[(r0 + r) * self.w + c0..(r0 + r) * self.w + c0 + w]);
        }
        out
    }

    fn cropped(&self, origin: (i64, i64)) -> (Grid, (i64, i64)) {
        match self.bbox() {
            Some((r0, c0, r1, c1)) => (
                self.window(r0, c0, r1 - r0 + 1, c1 - c0 + 1),
                (origin.0 + r0 as i64, origin.1 + c0 as i64),
            ),
            None => (Grid::new(0, 0), origin),
        }
    }

    fn embed(&self, h: usize, w: usize, r0: usize, c0: usize) -> Grid {
        let mut out = Grid::new(h, w);
        for r in 0..self.h {
            out.bits[(r0 + r) * w + c0..(r0 + r) * w + c0 + self.w]
                .copy_from_slice(&self.bits[r * self.w..(r + 1) * self.w]);
        }
        out
    }

    fn kron(&self, other: &Grid) -> Grid {
        let (h, w) = (self.h * other.h, self.w * other.w);
        let mut out = Grid::new(h, w);
        for r in 0..h {
            for c in 0..w {
                out.bits[r * w + c] =
                    self.get(r / other.h, c / other.w) & other.get(r % other.h, c % other.w);
            }
        }
        out
    }

    fn tile(&self, t: usize) -> Grid {
        Grid::full(t, t).kron(self)
    }

    fn tensor(&self) -> Tensor {
        Tensor::of(self.bits.clone(), vec![self.h, self.w]).expect("the bits fill the shape")
    }

    fn hash(&self) -> u128 {
        let mut a: u64 = 0xcbf29ce484222325;
        let mut b: u64 = 0x9e3779b97f4a7c15;
        for chunk in self.bits.chunks(64) {
            let mut word = 0u64;
            for (i, &bit) in chunk.iter().enumerate() {
                word |= u64::from(bit) << i;
            }
            a = (a ^ word).wrapping_mul(0x100000001b3);
            b = (b.rotate_left(29) ^ word).wrapping_mul(0xbf58476d1ce4e5b9);
        }
        a = (a ^ (self.w as u64)).wrapping_mul(0x100000001b3);
        b = (b ^ (self.h as u64)).wrapping_mul(0x94d049bb133111eb);
        (u128::from(a) << 64) | u128::from(b)
    }
}

#[derive(Clone, Debug)]
struct Factor {
    n: usize,
    taps: Vec<(i64, i64)>,
}

impl Factor {
    fn of(grid: &Grid) -> Factor {
        let n = grid.h;
        let centre = (n as i64 - 1) / 2;
        let mut taps = Vec::new();
        for r in 0..n {
            for c in 0..n {
                if grid.get(r, c) != 0 {
                    taps.push((r as i64 - centre, c as i64 - centre));
                }
            }
        }
        Factor { n, taps }
    }

    fn ones(n: usize) -> Factor {
        Factor::of(&Grid::full(n, n))
    }
}

#[derive(Clone, Debug)]
struct Kernel {
    side: usize,
    reach: usize,
    budget: usize,
    terms: Vec<(i32, Vec<Factor>)>,
    centre: i32,
}

impl Kernel {
    fn of(terms: Vec<(i32, Vec<Factor>)>, centre_value: u8) -> Result<Kernel> {
        let span = |chain: &[Factor]| chain.iter().map(|f| f.n).product::<usize>();
        let side = terms.first().map_or(1, |(_, chain)| span(chain));
        if terms.iter().any(|(_, chain)| span(chain) != side) {
            return value_error("every term of a kernel spans the same side.");
        }
        if side.is_multiple_of(2) {
            return value_error("a kernel side is odd.");
        }
        let mut kernel = Kernel {
            side,
            reach: (side - 1) / 2,
            budget: 0,
            terms,
            centre: i32::from(centre_value),
        };
        let dense = kernel.dense()?;
        if dense.get((side - 1) / 2, (side - 1) / 2) != 0 {
            return value_error("the kernel centre must be popped.");
        }
        kernel.budget = dense.population();
        Ok(kernel)
    }

    fn design(shape: Shape, side: usize, level: usize) -> Result<Kernel> {
        let base = shape.build(side, 1)?;
        let full = shape.build(side, level)?;
        let centre = full.get((full.h - 1) / 2, (full.w - 1) / 2);
        let chain: Vec<Factor> = (0..level).map(|_| Factor::of(&base)).collect();
        Kernel::of(vec![(1, chain)], centre)
    }

    fn copy(
        shape: Shape,
        side: usize,
        level: usize,
        tiling: usize,
        inverted: bool,
    ) -> Result<Kernel> {
        let base = shape.build(side, 1)?;
        let board = shape.build(side, level)?.tile(tiling);
        let centre = board.get((board.h - 1) / 2, (board.w - 1) / 2);
        let mut chain = Vec::new();
        if tiling > 1 {
            chain.push(Factor::ones(tiling));
        }
        chain.extend((0..level).map(|_| Factor::of(&base)));
        if !inverted {
            return Kernel::of(vec![(1, chain)], centre);
        }
        let mut full = Vec::new();
        if tiling > 1 {
            full.push(Factor::ones(tiling));
        }
        full.extend((0..level).map(|_| Factor::ones(side)));
        Kernel::of(vec![(1, full), (-1, chain)], 1 - centre)
    }

    fn dense(&self) -> Result<Grid> {
        let n = self.side;
        let mut acc = vec![0i32; n * n];
        for (sign, chain) in &self.terms {
            let mut grid = Grid::full(1, 1);
            for factor in chain {
                let mut g = Grid::new(factor.n, factor.n);
                let centre = (factor.n as i64 - 1) / 2;
                for &(dr, dc) in &factor.taps {
                    g.set((dr + centre) as usize, (dc + centre) as usize, 1);
                }
                grid = grid.kron(&g);
            }
            for (a, &b) in acc.iter_mut().zip(&grid.bits) {
                *a += sign * i32::from(b);
            }
        }
        acc[(n / 2) * n + n / 2] -= self.centre;
        if acc.iter().any(|v| !(0..=1).contains(v)) {
            return value_error("a kernel is a 0/1 mask.");
        }
        Ok(Grid::of(n, n, acc.iter().map(|&v| v as u8).collect()))
    }

    fn count_wrapped(&self, grid: &Grid) -> Vec<i32> {
        self.chain(grid, true)
    }

    fn count_grown(&self, grid: &Grid) -> (Grid, Vec<i32>) {
        let r = self.reach;
        let grown = grid.embed(grid.h + 2 * r, grid.w + 2 * r, r, r);
        let counts = self.chain(&grown, false);
        (grown, counts)
    }

    fn chain(&self, grid: &Grid, wrap: bool) -> Vec<i32> {
        let (h, w) = (grid.h, grid.w);
        let base: Vec<i32> = grid.bits.iter().map(|&b| i32::from(b)).collect();
        let mut out = vec![0i32; h * w];
        for (sign, chain) in &self.terms {
            let mut acc = base.clone();
            let mut dilation = 1i64;
            for factor in chain.iter().rev() {
                acc = convolve(&acc, h, w, factor, dilation, wrap);
                dilation *= factor.n as i64;
            }
            for (o, a) in out.iter_mut().zip(&acc) {
                *o += sign * a;
            }
        }
        if self.centre != 0 {
            for (o, b) in out.iter_mut().zip(&base) {
                *o -= self.centre * b;
            }
        }
        out
    }
}

fn convolve(src: &[i32], h: usize, w: usize, factor: &Factor, d: i64, wrap: bool) -> Vec<i32> {
    let mut out = vec![0i32; h * w];
    for &(tr, tc) in &factor.taps {
        let (dr, dc) = (tr * d, tc * d);
        if wrap {
            let (hh, ww) = (h as i64, w as i64);
            for r in 0..h {
                let sr = (r as i64 + dr).rem_euclid(hh) as usize;
                for c in 0..w {
                    let sc = (c as i64 + dc).rem_euclid(ww) as usize;
                    out[r * w + c] += src[sr * w + sc];
                }
            }
        } else {
            shift_add(&mut out, src, h, w, dr, dc);
        }
    }
    out
}

fn shift_add(out: &mut [i32], src: &[i32], h: usize, w: usize, dr: i64, dc: i64) {
    let (hh, ww) = (h as i64, w as i64);
    let (r_lo, r_hi) = ((-dr).max(0), (hh - dr).min(hh));
    let (c_lo, c_hi) = ((-dc).max(0), (ww - dc).min(ww));
    if r_lo >= r_hi || c_lo >= c_hi {
        return;
    }
    let (c_lo, c_hi) = (c_lo as usize, c_hi as usize);
    for r in r_lo as usize..r_hi as usize {
        let sr = (r as i64 + dr) as usize;
        let sc = (c_lo as i64 + dc) as usize;
        let o = &mut out[r * w + c_lo..r * w + c_hi];
        let s = &src[sr * w + sc..sr * w + sc + (c_hi - c_lo)];
        for (a, b) in o.iter_mut().zip(s) {
            *a += *b;
        }
    }
}

#[derive(Clone, Debug)]
struct Table {
    budget: usize,
    birth: Vec<usize>,
    survive: Vec<usize>,
    fired: [Vec<u8>; 2],
    next_background: [u8; 2],
}

impl Table {
    fn new(birth: &[usize], survive: &[usize], budget: usize) -> Table {
        let n = budget;
        let member = |counts: &[usize]| {
            let mut on = vec![false; n + 1];
            for &k in counts.iter().filter(|&&k| k <= n) {
                on[k] = true;
            }
            on
        };
        let (b, s) = (member(birth), member(survive));
        let next_background = [u8::from(b[0]), u8::from(s[n])];
        let mut fired = [vec![0u8; 2 * (n + 1)], vec![0u8; 2 * (n + 1)]];
        for bg in 0..2usize {
            for y in 0..2usize {
                for ny in 0..=n {
                    let x = y ^ bg;
                    let nx = if bg == 0 { ny } else { n - ny };
                    let on = if x == 1 { s[nx] } else { b[nx] };
                    fired[bg][y * (n + 1) + ny] = u8::from(on) ^ next_background[bg];
                }
            }
        }
        Table {
            budget,
            birth: birth.to_vec(),
            survive: survive.to_vec(),
            fired,
            next_background,
        }
    }

    fn flips(&self) -> bool {
        self.next_background[0] == 1
    }

    fn next_background(&self, background: u8) -> u8 {
        self.next_background[usize::from(background)]
    }

    fn apply(&self, background: u8, cells: &[u8], counts: &[i32]) -> Vec<u8> {
        let table = &self.fired[usize::from(background)];
        let width = self.budget + 1;
        cells
            .iter()
            .zip(counts)
            .map(|(&y, &n)| table[usize::from(y) * width + n as usize])
            .collect()
    }
}

#[derive(Clone, Debug)]
struct Walls {
    lo: i64,
    hi: i64,
    reach: i64,
}

impl Walls {
    fn hit(&self, frame: &Grid, origin: (i64, i64)) -> bool {
        if frame.h == 0 {
            return false;
        }
        let (r0, c0) = origin;
        let (r1, c1) = (r0 + frame.h as i64 - 1, c0 + frame.w as i64 - 1);
        r0 - self.lo < self.reach
            || c0 - self.lo < self.reach
            || self.hi - r1 < self.reach
            || self.hi - c1 < self.reach
    }

    fn wall_by(&self, frame: &Grid, origin: (i64, i64), shift: (i64, i64)) -> Option<u64> {
        let (r0, c0) = origin;
        let (r1, c1) = (r0 + frame.h as i64 - 1, c0 + frame.w as i64 - 1);
        let (dr, dc) = shift;
        let mut k = u64::MAX;
        let mut room = |space: i64, speed: i64| {
            if speed > 0 {
                k = k.min((space / speed) as u64 + 1);
            }
        };
        room(self.hi - self.reach - r1, dr);
        room(r0 - self.lo - self.reach, -dr);
        room(self.hi - self.reach - c1, dc);
        room(c0 - self.lo - self.reach, -dc);
        (k != u64::MAX).then_some(k)
    }
}

#[derive(Clone, Debug)]
pub struct Run {
    mode: Mode,
    kernel: Kernel,
    table: Table,
    x: u32,
    cap: u32,
    side: u32,
    walls: Walls,
    frame: Grid,
    origin: (i64, i64),
    background: u8,
    generation: u32,
    history: HashMap<u128, (u32, i64, i64)>,
    stop: Option<Stop>,
}

impl Run {
    pub fn new(recipe: &Recipe) -> Result<Run> {
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
        let kernel = match Mask::parse(&recipe.mask)? {
            Mask::Design { shape, side, level } => Kernel::design(shape, side, level)?,
            Mask::Copy { inverted } => Kernel::copy(shape, side, level, tiling, inverted)?,
        };
        let birth = Side::parse(&recipe.birth)?.values(kernel.budget)?;
        let survive = Side::parse(&recipe.survive)?.values(kernel.budget)?;
        let table = Table::new(&birth, &survive, kernel.budget);
        let side = board.h as u32;
        let cap = match recipe.mode {
            Mode::Island => recipe.cap.max(side),
            Mode::Wallpaper => side,
        };
        let lo = -(i64::from(cap - side) / 2);
        let walls = Walls {
            lo,
            hi: lo + i64::from(cap) - 1,
            reach: kernel.reach as i64,
        };
        let (frame, origin) = match recipe.mode {
            Mode::Island => board.cropped((0, 0)),
            Mode::Wallpaper => (board, (0, 0)),
        };
        let mut run = Run {
            mode: recipe.mode,
            kernel,
            table,
            x: recipe.x,
            cap,
            side,
            walls,
            frame,
            origin,
            background: 0,
            generation: 0,
            history: HashMap::new(),
            stop: None,
        };
        run.remember();
        if run.mode == Mode::Island && run.walls.hit(&run.frame, run.origin) {
            run.stop = Some(Stop::at("wall", 0, 0));
        }
        Ok(run)
    }

    fn remember(&mut self) {
        let key = self.frame.hash() ^ u128::from(self.background);
        self.history
            .insert(key, (self.generation, self.origin.0, self.origin.1));
    }

    fn seen(&self) -> Option<(u32, i64, i64)> {
        let key = self.frame.hash() ^ u128::from(self.background);
        self.history.get(&key).copied()
    }

    pub fn step(&mut self) -> Option<Stop> {
        if self.stop.is_some() {
            return self.stop.clone();
        }
        if self.generation >= self.x {
            self.stop = Some(Stop::at("limit", self.generation, 0));
            return self.stop.clone();
        }
        let stop = match self.mode {
            Mode::Island => self.step_island(),
            Mode::Wallpaper => self.step_wallpaper(),
        };
        match stop {
            Some(stop) => self.stop = Some(stop),
            None => self.remember(),
        }
        self.stop.clone()
    }

    fn step_island(&mut self) -> Option<Stop> {
        let reach = self.kernel.reach as i64;
        let (grown, counts) = self.kernel.count_grown(&self.frame);
        let next = Grid::of(
            grown.h,
            grown.w,
            self.table.apply(self.background, &grown.bits, &counts),
        );
        let (frame, origin) = next.cropped((self.origin.0 - reach, self.origin.1 - reach));
        self.frame = frame;
        self.origin = origin;
        self.background = self.table.next_background(self.background);
        self.generation += 1;
        if self.frame.h == 0 {
            return Some(Stop::at("dead", self.generation, 1));
        }
        if self.walls.hit(&self.frame, self.origin) {
            return Some(Stop::at("wall", self.generation, 0));
        }
        let (seen, r0, c0) = self.seen()?;
        let period = self.generation - seen;
        let (dr, dc) = (self.origin.0 - r0, self.origin.1 - c0);
        if (dr, dc) == (0, 0) {
            return Some(if period == 1 {
                Stop::at("still", self.generation, 1)
            } else {
                Stop::at("loop", self.generation, period)
            });
        }
        let wall_by = self
            .walls
            .wall_by(&self.frame, self.origin, (dr, dc))
            .map(|k| u64::from(self.generation) + k * u64::from(period))
            .filter(|&at| at <= u64::from(self.x))
            .map_or(0, |at| at as u32);
        Some(Stop {
            kind: "mover".to_string(),
            generation: self.generation,
            period,
            dr,
            dc,
            wall_by,
        })
    }

    fn step_wallpaper(&mut self) -> Option<Stop> {
        let counts = self.kernel.count_wrapped(&self.frame);
        self.frame = Grid::of(
            self.frame.h,
            self.frame.w,
            self.table.apply(self.background, &self.frame.bits, &counts),
        );
        self.background = self.table.next_background(self.background);
        self.generation += 1;
        if self.frame.is_empty() {
            return Some(Stop::at("dead", self.generation, 1));
        }
        let (seen, _, _) = self.seen()?;
        let period = self.generation - seen;
        Some(if period == 1 {
            Stop::at("still", self.generation, 1)
        } else {
            Stop::at("loop", self.generation, period)
        })
    }

    pub fn frame(&self) -> Cell {
        Cell::new(self.frame.tensor())
    }

    pub fn origin(&self) -> [i64; 2] {
        [self.origin.0, self.origin.1]
    }

    pub fn generation(&self) -> u32 {
        self.generation
    }

    pub fn background(&self) -> u8 {
        self.background
    }

    pub fn flips(&self) -> bool {
        self.table.flips()
    }

    pub fn population(&self) -> u32 {
        self.frame.population() as u32
    }

    pub fn side(&self) -> u32 {
        self.side
    }

    pub fn cap(&self) -> u32 {
        self.cap
    }

    pub fn reach(&self) -> u32 {
        self.kernel.reach as u32
    }

    pub fn budget(&self) -> u32 {
        self.kernel.budget as u32
    }

    pub fn birth(&self) -> Vec<u32> {
        self.table.birth.iter().map(|&v| v as u32).collect()
    }

    pub fn survive(&self) -> Vec<u32> {
        self.table.survive.iter().map(|&v| v as u32).collect()
    }

    pub fn mask(&self) -> Tensor {
        self.kernel
            .dense()
            .unwrap_or_else(|_| Grid::new(0, 0))
            .tensor()
    }

    pub fn stop(&self) -> Option<Stop> {
        self.stop.clone()
    }
}
