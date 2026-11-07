use crate::recipe;
use mrly_error::{Result, shape_error, value_error};
use mrly_math::cell::base::Cell;
use mrly_name::named_enum;
use mrly_tensor::Tensor;
use mrly_two::Cell2d;
use serde::{Deserialize, Serialize};
use std::cell::RefCell;
use std::collections::HashMap;
use std::sync::Arc;

const SLIDE: usize = 4;
const PLANS: usize = 256;
const KEPT_CELLS: usize = 1 << 22;
const BOARD: u32 = 4096;

type Plans = (HashMap<Grid, Arc<Kernel>>, usize);

thread_local! {
    static KEPT: RefCell<Plans> = RefCell::new((HashMap::new(), 0));
}

named_enum! {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
    pub enum Mode {
        Island => "island",
        Wallpaper => "wallpaper",
        Board => "board",
    }
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

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub(crate) struct Grid {
    pub(crate) h: usize,
    pub(crate) w: usize,
    bits: Vec<u8>,
}

impl Grid {
    pub(crate) fn new(h: usize, w: usize) -> Grid {
        Grid {
            h,
            w,
            bits: vec![0; h * w],
        }
    }

    pub(crate) fn full(h: usize, w: usize) -> Grid {
        Grid {
            h,
            w,
            bits: vec![1; h * w],
        }
    }

    pub(crate) fn of(h: usize, w: usize, bits: Vec<u8>) -> Grid {
        debug_assert_eq!(bits.len(), h * w);
        Grid { h, w, bits }
    }

    fn read(tensor: &Tensor) -> Result<Grid> {
        let [h, w] = tensor.shape[..] else {
            return shape_error("a life grid is two dimensional.");
        };
        Ok(Grid::of(
            h,
            w,
            (0..tensor.size())
                .map(|i| u8::from(tensor.at(i) != 0))
                .collect(),
        ))
    }

    pub(crate) fn get(&self, r: usize, c: usize) -> u8 {
        self.bits[r * self.w + c]
    }

    pub(crate) fn set(&mut self, r: usize, c: usize, v: u8) {
        self.bits[r * self.w + c] = v;
    }

    pub(crate) fn population(&self) -> usize {
        self.bits.iter().filter(|&&b| b != 0).count()
    }

    pub(crate) fn is_empty(&self) -> bool {
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

    pub(crate) fn window(&self, r0: usize, c0: usize, h: usize, w: usize) -> Grid {
        let mut out = Grid::new(h, w);
        for r in 0..h {
            out.bits[r * w..(r + 1) * w]
                .copy_from_slice(&self.bits[(r0 + r) * self.w + c0..(r0 + r) * self.w + c0 + w]);
        }
        out
    }

    pub(crate) fn cropped(&self, origin: (i64, i64)) -> (Grid, (i64, i64)) {
        match self.bbox() {
            Some((r0, c0, r1, c1)) => (
                self.window(r0, c0, r1 - r0 + 1, c1 - c0 + 1),
                (origin.0 + r0 as i64, origin.1 + c0 as i64),
            ),
            None => (Grid::new(0, 0), origin),
        }
    }

    pub(crate) fn embed(&self, h: usize, w: usize, r0: usize, c0: usize) -> Grid {
        let mut out = Grid::new(h, w);
        for r in 0..self.h {
            out.bits[(r0 + r) * w + c0..(r0 + r) * w + c0 + self.w]
                .copy_from_slice(&self.bits[r * self.w..(r + 1) * self.w]);
        }
        out
    }

    pub(crate) fn kron(&self, other: &Grid) -> Grid {
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

    pub(crate) fn tile(&self, t: usize) -> Grid {
        Grid::full(t, t).kron(self)
    }

    pub(crate) fn invert(&self) -> Grid {
        Grid::of(self.h, self.w, self.bits.iter().map(|&b| 1 - b).collect())
    }

    fn centred(&self, bit: u8) -> Grid {
        let mut out = self.clone();
        if self.h > 0 && self.w > 0 {
            out.set((self.h - 1) / 2, (self.w - 1) / 2, bit);
        }
        out
    }

    pub(crate) fn popped(&self) -> Grid {
        self.centred(0)
    }

    pub(crate) fn tensor(&self) -> Tensor {
        Tensor::of(self.bits.clone(), vec![self.h, self.w]).expect("the bits fill the shape")
    }

    fn flipped(&self, background: u8) -> Grid {
        Grid::of(
            self.h,
            self.w,
            self.bits.iter().map(|&b| b ^ background).collect(),
        )
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
struct Line {
    taps: Vec<i64>,
    runs: Vec<(i64, i64, usize)>,
    low: i64,
    high: i64,
}

impl Line {
    fn of(taps: Vec<i64>) -> Line {
        let mut runs = Vec::new();
        let mut i = 0;
        while i < taps.len() {
            if i + 1 < taps.len() {
                let step = taps[i + 1] - taps[i];
                let mut j = i + 1;
                while j + 1 < taps.len() && taps[j + 1] - taps[j] == step {
                    j += 1;
                }
                if j - i + 1 >= SLIDE {
                    runs.push((taps[i], step, j - i + 1));
                    i = j + 1;
                    continue;
                }
            }
            runs.push((taps[i], 1, 1));
            i += 1;
        }
        let low = runs
            .iter()
            .map(|&(a, step, count)| if count > 1 { a - step } else { a })
            .min()
            .unwrap_or(0);
        let high = taps.last().copied().unwrap_or(0);
        Line {
            taps,
            runs,
            low,
            high,
        }
    }

    fn cost(&self) -> usize {
        self.runs
            .iter()
            .map(|&(_, _, count)| if count > 1 { 3 } else { 1 })
            .sum()
    }

    fn still(&self) -> bool {
        self.taps == [0]
    }
}

#[derive(Clone, Debug)]
struct Group {
    across: Line,
    down: Line,
}

impl Group {
    fn cost(&self) -> usize {
        if self.down.taps.len() == 1 {
            self.across.cost()
        } else if self.across.still() {
            self.down.cost()
        } else {
            self.across.cost() + self.down.cost() + 1
        }
    }
}

#[derive(Clone, Debug)]
struct Factor {
    side: usize,
    groups: Vec<Group>,
}

fn grouped(mask: &Grid, cross: bool) -> Vec<Group> {
    let (hr, hc) = (((mask.h - 1) / 2) as i64, ((mask.w - 1) / 2) as i64);
    let (outer, inner) = if cross {
        (mask.w, mask.h)
    } else {
        (mask.h, mask.w)
    };
    let mut keys: Vec<(Vec<i64>, Vec<i64>)> = Vec::new();
    for i in 0..outer {
        let line: Vec<i64> = (0..inner)
            .filter(|&j| {
                if cross {
                    mask.get(j, i) != 0
                } else {
                    mask.get(i, j) != 0
                }
            })
            .map(|j| j as i64 - if cross { hr } else { hc })
            .collect();
        if line.is_empty() {
            continue;
        }
        let at = i as i64 - if cross { hc } else { hr };
        match keys.iter_mut().find(|(key, _)| *key == line) {
            Some((_, list)) => list.push(at),
            None => keys.push((line, vec![at])),
        }
    }
    keys.into_iter()
        .map(|(line, list)| {
            let (across, down) = if cross { (list, line) } else { (line, list) };
            Group {
                across: Line::of(across),
                down: Line::of(down),
            }
        })
        .collect()
}

fn price(groups: &[Group]) -> usize {
    groups.iter().map(Group::cost).sum()
}

impl Factor {
    fn of(mask: &Grid) -> Factor {
        let rows = grouped(mask, false);
        let cols = grouped(mask, true);
        Factor {
            side: mask.h,
            groups: if price(&cols) < price(&rows) {
                cols
            } else {
                rows
            },
        }
    }

    fn split(mask: &Grid) -> Vec<Factor> {
        let n = mask.h;
        if n == mask.w {
            for p in (3..n).step_by(2).filter(|p| n.is_multiple_of(*p)) {
                let q = n / p;
                let mut outer = Grid::new(p, p);
                let mut inner: Option<Grid> = None;
                let mut kron = true;
                for at in 0..p * p {
                    let block = mask.window((at / p) * q, (at % p) * q, q, q);
                    if block.is_empty() {
                        continue;
                    }
                    outer.set(at / p, at % p, 1);
                    match &inner {
                        None => inner = Some(block),
                        Some(first) if *first != block => {
                            kron = false;
                            break;
                        }
                        Some(_) => {}
                    }
                }
                if let (true, Some(inner)) = (kron, inner) {
                    let mut out = Factor::split(&outer);
                    out.extend(Factor::split(&inner));
                    return out;
                }
            }
        }
        vec![Factor::of(mask)]
    }

    fn cost(&self) -> usize {
        price(&self.groups)
    }

    fn pass(&self, src: &[i32], h: usize, w: usize, by: i64, wrap: bool) -> Vec<i32> {
        let mut out = vec![0i32; h * w];
        for group in &self.groups {
            if group.down.taps.len() == 1 {
                across(
                    &mut out,
                    src,
                    (h, w),
                    &group.across,
                    group.down.taps[0] * by,
                    by,
                    wrap,
                );
            } else if group.across.still() {
                down(&mut out, src, (h, w), &group.down, by, wrap);
            } else {
                let mut tmp = vec![0i32; h * w];
                across(&mut tmp, src, (h, w), &group.across, 0, by, wrap);
                down(&mut out, &tmp, (h, w), &group.down, by, wrap);
            }
        }
        out
    }
}

fn row(r: i64, h: usize, wrap: bool) -> Option<usize> {
    if wrap {
        Some(r.rem_euclid(h as i64) as usize)
    } else {
        usize::try_from(r).ok().filter(|&r| r < h)
    }
}

fn add(out: &mut [i32], from: &[i32]) {
    for (o, f) in out.iter_mut().zip(from) {
        *o = o.wrapping_add(*f);
    }
}

fn across(
    out: &mut [i32],
    src: &[i32],
    (h, w): (usize, usize),
    line: &Line,
    dr: i64,
    by: i64,
    wrap: bool,
) {
    let pad = ((-line.low).max(line.high).max(0) * by) as usize;
    let span = w + 2 * pad;
    let mut strip = vec![0i32; span];
    let mut win = vec![0i32; w];
    for r in 0..h {
        let Some(sr) = row(r as i64 + dr, h, wrap) else {
            continue;
        };
        let line_in = &src[sr * w..(sr + 1) * w];
        if wrap {
            for (j, slot) in strip.iter_mut().enumerate() {
                *slot = line_in[(j as i64 - pad as i64).rem_euclid(w as i64) as usize];
            }
        } else {
            strip[pad..pad + w].copy_from_slice(line_in);
        }
        let o = &mut out[r * w..(r + 1) * w];
        for &(a, step, count) in &line.runs {
            let start = (pad as i64 + a * by) as usize;
            if count == 1 {
                add(o, &strip[start..start + w]);
                continue;
            }
            let e = (step * by) as usize;
            let first = e.min(w);
            for c in 0..first {
                let total = (0..count).fold(0i32, |t, k| t.wrapping_add(strip[start + c + k * e]));
                win[c] = total;
                o[c] = o[c].wrapping_add(total);
            }
            let (hi, lo) = (start + (count - 1) * e, start - e);
            for c in first..w {
                let total = win[c - e]
                    .wrapping_add(strip[hi + c])
                    .wrapping_sub(strip[lo + c]);
                win[c] = total;
                o[c] = o[c].wrapping_add(total);
            }
        }
    }
}

fn down(out: &mut [i32], src: &[i32], (h, w): (usize, usize), line: &Line, by: i64, wrap: bool) {
    for &(a, step, count) in &line.runs {
        if count == 1 {
            for r in 0..h {
                if let Some(sr) = row(r as i64 + a * by, h, wrap) {
                    add(&mut out[r * w..(r + 1) * w], &src[sr * w..(sr + 1) * w]);
                }
            }
            continue;
        }
        let e = (step * by) as usize;
        let mut win = vec![0i32; h * w];
        for r in 0..h {
            let (done, rest) = win.split_at_mut(r * w);
            let here = &mut rest[..w];
            if r < e {
                for k in 0..count as i64 {
                    if let Some(sr) = row(r as i64 + (a + k * step) * by, h, wrap) {
                        add(here, &src[sr * w..(sr + 1) * w]);
                    }
                }
            } else {
                here.copy_from_slice(&done[(r - e) * w..(r - e + 1) * w]);
                let last = a + (count as i64 - 1) * step;
                if let Some(sr) = row(r as i64 + last * by, h, wrap) {
                    add(here, &src[sr * w..(sr + 1) * w]);
                }
                if let Some(sr) = row(r as i64 + (a - step) * by, h, wrap) {
                    for (o, f) in here.iter_mut().zip(&src[sr * w..(sr + 1) * w]) {
                        *o = o.wrapping_sub(*f);
                    }
                }
            }
            add(&mut out[r * w..(r + 1) * w], here);
        }
    }
}

#[derive(Clone, Debug)]
struct Kernel {
    mask: Grid,
    reach: (usize, usize),
    pad: usize,
    budget: usize,
    terms: Vec<(i32, Vec<Factor>)>,
    centre: i32,
    plan: &'static str,
}

impl Kernel {
    fn plan(mask: Grid) -> Result<Kernel> {
        if mask.h.is_multiple_of(2) || mask.w.is_multiple_of(2) {
            return value_error("a mask has odd sides.");
        }
        let own = i32::from(mask.get((mask.h - 1) / 2, (mask.w - 1) / 2));
        let full = Factor::split(&Grid::full(mask.h, mask.w));
        let rest = mask.invert();
        let mut options = vec![("direct", vec![(1, vec![Factor::of(&mask)])], 0)];
        for bit in [0u8, 1] {
            options.push((
                "factored",
                vec![(1, Factor::split(&mask.centred(bit)))],
                i32::from(bit) - own,
            ));
            options.push((
                "complement",
                vec![(1, full.clone()), (-1, Factor::split(&rest.centred(bit)))],
                1 - i32::from(bit) - own,
            ));
        }
        let cost = |terms: &[(i32, Vec<Factor>)]| -> usize {
            terms
                .iter()
                .map(|(_, chain)| 1 + chain.iter().map(Factor::cost).sum::<usize>())
                .sum()
        };
        let (plan, terms, centre) = options
            .into_iter()
            .reduce(|best, next| {
                if cost(&next.1) < cost(&best.1) {
                    next
                } else {
                    best
                }
            })
            .expect("the direct plan is always there");
        let pad = terms
            .iter()
            .map(|(_, chain)| (chain.iter().skip(1).map(|f| f.side).product::<usize>() - 1) / 2)
            .max()
            .unwrap_or(0);
        Ok(Kernel {
            reach: ((mask.h - 1) / 2, (mask.w - 1) / 2),
            pad,
            budget: mask.population(),
            mask,
            terms,
            centre,
            plan,
        })
    }

    fn count(&self, grid: &Grid, wrap: bool) -> Vec<i32> {
        let (h, w) = (grid.h, grid.w);
        let mut out = vec![0i32; h * w];
        if h == 0 || w == 0 {
            return out;
        }
        let base: Vec<i32> = grid.bits.iter().map(|&b| i32::from(b)).collect();
        for (sign, chain) in &self.terms {
            let mut acc = base.clone();
            let mut by = 1i64;
            for factor in chain.iter().rev() {
                acc = factor.pass(&acc, h, w, by, wrap);
                by *= factor.side as i64;
            }
            for (o, a) in out.iter_mut().zip(&acc) {
                *o = o.wrapping_add(sign.wrapping_mul(*a));
            }
        }
        if self.centre != 0 {
            for (o, b) in out.iter_mut().zip(&base) {
                *o = o.wrapping_sub(self.centre * b);
            }
        }
        out
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
    fn new(birth: &[usize], survive: &[usize], budget: usize, flipping: bool) -> Table {
        let n = budget;
        let member = |counts: &[usize]| {
            let mut on = vec![false; n + 1];
            for &k in counts.iter().filter(|&&k| k <= n) {
                on[k] = true;
            }
            on
        };
        let (b, s) = (member(birth), member(survive));
        let next_background = if flipping {
            [u8::from(b[0]), u8::from(s[n])]
        } else {
            [0, 0]
        };
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
        let keep = |counts: &[usize]| counts.iter().copied().filter(|&k| k <= n).collect();
        Table {
            budget,
            birth: keep(birth),
            survive: keep(survive),
            fired,
            next_background,
        }
    }

    fn apply(&self, background: u8, cells: &[u8], counts: &[i32]) -> Vec<u8> {
        let (dead, live) = self.fired[usize::from(background)].split_at(self.budget + 1);
        cells
            .iter()
            .zip(counts)
            .map(|(&y, &n)| if y == 0 { dead } else { live }[n as usize])
            .collect()
    }
}

#[derive(Clone, Debug)]
struct Walls {
    lo: (i64, i64),
    hi: (i64, i64),
    reach: i64,
}

impl Walls {
    fn hit(&self, frame: &Grid, origin: (i64, i64)) -> bool {
        if frame.h == 0 {
            return false;
        }
        let (r0, c0) = origin;
        let (r1, c1) = (r0 + frame.h as i64 - 1, c0 + frame.w as i64 - 1);
        r0 - self.lo.0 < self.reach
            || c0 - self.lo.1 < self.reach
            || self.hi.0 - r1 < self.reach
            || self.hi.1 - c1 < self.reach
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
        room(self.hi.0 - self.reach - r1, dr);
        room(r0 - self.lo.0 - self.reach, -dr);
        room(self.hi.1 - self.reach - c1, dc);
        room(c0 - self.lo.1 - self.reach, -dc);
        (k != u64::MAX).then_some(k)
    }
}

#[derive(Clone, Debug)]
pub struct Run {
    mode: Mode,
    kernel: Arc<Kernel>,
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

fn table(kernel: &Kernel, birth: &[usize], survive: &[usize], mode: Mode) -> Table {
    Table::new(birth, survive, kernel.budget, mode != Mode::Board)
}

fn bounded(mode: Mode, cap: u32) -> Result<()> {
    if mode == Mode::Board && cap > BOARD {
        return value_error(format!("a board is {BOARD} cells wide at most."));
    }
    Ok(())
}

impl Run {
    pub fn new(recipe: &recipe::Recipe) -> Result<Run> {
        bounded(recipe.mode, recipe.cap)?;
        let (board, mask, birth, survive) = recipe::parts(recipe)?;
        let kernel = planned(mask)?;
        let birth = birth.values(kernel.budget)?;
        let survive = survive.values(kernel.budget)?;
        let table = table(&kernel, &birth, &survive, recipe.mode);
        Ok(Run::start(
            recipe.mode,
            kernel,
            table,
            board,
            recipe.cap,
            recipe.x,
        ))
    }

    pub fn of(
        cells: &Cell2d,
        mask: &Tensor,
        birth: &[usize],
        survive: &[usize],
        mode: Mode,
        cap: u32,
        x: u32,
    ) -> Result<Run> {
        bounded(mode, cap)?;
        let kernel = planned(Grid::read(mask)?)?;
        let table = table(&kernel, birth, survive, mode);
        Ok(Run::start(
            mode,
            kernel,
            table,
            Grid::read(cells.types())?,
            cap,
            x,
        ))
    }

    fn start(mode: Mode, kernel: Arc<Kernel>, table: Table, board: Grid, cap: u32, x: u32) -> Run {
        let board = if mode == Mode::Board && cap as usize > board.h.max(board.w) {
            let (h, w) = (board.h.max(cap as usize), board.w.max(cap as usize));
            board.embed(h, w, (h - board.h) / 2, (w - board.w) / 2)
        } else {
            board
        };
        let side = board.h.max(board.w) as u32;
        let cap = match mode {
            Mode::Island => cap.max(side),
            Mode::Wallpaper | Mode::Board => side,
        };
        let lo = (
            -(i64::from(cap) - board.h as i64) / 2,
            -(i64::from(cap) - board.w as i64) / 2,
        );
        let walls = Walls {
            lo,
            hi: (lo.0 + i64::from(cap) - 1, lo.1 + i64::from(cap) - 1),
            reach: kernel.reach.0.max(kernel.reach.1) as i64,
        };
        let (frame, origin) = match mode {
            Mode::Island => board.cropped((0, 0)),
            Mode::Wallpaper | Mode::Board => (board, (0, 0)),
        };
        Run {
            mode,
            kernel,
            table,
            x,
            cap,
            side,
            walls,
            frame,
            origin,
            background: 0,
            generation: 0,
            history: HashMap::new(),
            stop: None,
        }
        .begun()
    }

    pub fn carry(&self, mask: &str, birth: &str, survive: &str, x: u32) -> Result<Run> {
        let kernel = planned(recipe::design(mask)?)?;
        let birth = crate::Counts::parse(birth)?.values(kernel.budget)?;
        let survive = crate::Counts::parse(survive)?.values(kernel.budget)?;
        Ok(self.continued(kernel, &birth, &survive, x))
    }

    pub fn then(&self, mask: &Tensor, birth: &[usize], survive: &[usize], x: u32) -> Result<Run> {
        Ok(self.continued(planned(Grid::read(mask)?)?, birth, survive, x))
    }

    fn continued(&self, kernel: Arc<Kernel>, birth: &[usize], survive: &[usize], x: u32) -> Run {
        let table = table(&kernel, birth, survive, self.mode);
        let walls = Walls {
            reach: kernel.reach.0.max(kernel.reach.1) as i64,
            ..self.walls.clone()
        };
        Run {
            mode: self.mode,
            kernel,
            table,
            x,
            cap: self.cap,
            side: self.side,
            walls,
            frame: self.frame.clone(),
            origin: self.origin,
            background: self.background,
            generation: 0,
            history: HashMap::new(),
            stop: None,
        }
        .begun()
    }

    fn begun(mut self) -> Run {
        let key = self.key();
        self.remember(key);
        if self.mode == Mode::Island && self.walls.hit(&self.frame, self.origin) {
            self.stop = Some(Stop::at("wall", 0, 0));
        }
        self
    }

    fn key(&self) -> u128 {
        self.frame.hash() ^ u128::from(self.background)
    }

    fn remember(&mut self, key: u128) {
        self.history
            .insert(key, (self.generation, self.origin.0, self.origin.1));
    }

    pub fn step(&mut self) -> Option<Stop> {
        if self.stop.is_some() {
            return self.stop.clone();
        }
        if self.generation >= self.x {
            self.stop = Some(Stop::at("limit", self.generation, 0));
            return self.stop.clone();
        }
        self.advance();
        let key = self.key();
        match self.judge(key) {
            Some(stop) => self.stop = Some(stop),
            None => self.remember(key),
        }
        self.stop.clone()
    }

    fn advance(&mut self) {
        let (rr, rc) = self.kernel.reach;
        if self.mode == Mode::Wallpaper {
            let counts = self.kernel.count(&self.frame, true);
            self.frame = Grid::of(
                self.frame.h,
                self.frame.w,
                self.table.apply(self.background, &self.frame.bits, &counts),
            );
        } else if self.mode == Mode::Board {
            let (h, w, pad) = (self.frame.h, self.frame.w, self.kernel.pad);
            let grown = if pad == 0 {
                self.frame.clone()
            } else {
                self.frame.embed(h + 2 * pad, w + 2 * pad, pad, pad)
            };
            let counts = self.kernel.count(&grown, false);
            let next = Grid::of(
                grown.h,
                grown.w,
                self.table.apply(self.background, &grown.bits, &counts),
            );
            self.frame = if pad == 0 {
                next
            } else {
                next.window(pad, pad, h, w)
            };
        } else {
            let (h, w) = (self.frame.h, self.frame.w);
            let grown = self.frame.embed(h + 2 * rr, w + 2 * rc, rr, rc);
            let counts = self.kernel.count(&grown, false);
            let next = Grid::of(
                grown.h,
                grown.w,
                self.table.apply(self.background, &grown.bits, &counts),
            );
            let origin = (self.origin.0 - rr as i64, self.origin.1 - rc as i64);
            (self.frame, self.origin) = next.cropped(origin);
        }
        self.background = self.table.next_background[usize::from(self.background)];
        self.generation += 1;
    }

    fn judge(&self, key: u128) -> Option<Stop> {
        if self.frame.is_empty() {
            return Some(Stop::at("dead", self.generation, 1));
        }
        if self.mode == Mode::Island && self.walls.hit(&self.frame, self.origin) {
            return Some(Stop::at("wall", self.generation, 0));
        }
        let (seen, r0, c0) = self.history.get(&key).copied()?;
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

    pub fn finish(&mut self) -> Stop {
        loop {
            if let Some(stop) = self.step() {
                return stop;
            }
        }
    }

    pub fn frame(&self) -> Cell {
        Cell::new(self.frame.tensor())
    }

    pub fn cells(&self) -> Cell {
        Cell::new(self.frame.flipped(self.background).tensor())
    }

    pub fn mode(&self) -> Mode {
        self.mode
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
        self.table.next_background[0] == 1
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
        self.kernel.reach.0.max(self.kernel.reach.1) as u32
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
        self.kernel.mask.tensor()
    }

    pub fn plan(&self) -> String {
        self.kernel.plan.to_string()
    }

    pub fn stop(&self) -> Option<Stop> {
        self.stop.clone()
    }
}

fn planned(mask: Grid) -> Result<Arc<Kernel>> {
    if let Some(kernel) = KEPT.with(|kept| kept.borrow().0.get(&mask).cloned()) {
        return Ok(kernel);
    }
    let kernel = Arc::new(Kernel::plan(mask.clone())?);
    KEPT.with(|kept| {
        let (plans, cells) = &mut *kept.borrow_mut();
        let size = mask.h * mask.w;
        if plans.len() >= PLANS || *cells + size > KEPT_CELLS {
            plans.clear();
            *cells = 0;
        }
        *cells += size;
        plans.insert(mask, kernel.clone());
    });
    Ok(kernel)
}

pub fn evolve(
    frame: &Cell2d,
    mask: &Tensor,
    birth: &[usize],
    survive: &[usize],
    mode: Mode,
    background: u8,
) -> Result<(Cell, [i64; 2], u8)> {
    let kernel = planned(Grid::read(mask)?)?;
    let table = table(&kernel, birth, survive, mode);
    let grid = Grid::read(frame.types())?;
    let side = grid.h.max(grid.w) as u32;
    let mut run = Run {
        mode,
        walls: Walls {
            lo: (0, 0),
            hi: (0, 0),
            reach: 0,
        },
        kernel,
        table,
        x: 1,
        cap: side,
        side,
        frame: grid,
        origin: (0, 0),
        background: if mode == Mode::Board {
            0
        } else {
            background & 1
        },
        generation: 0,
        history: HashMap::new(),
        stop: None,
    };
    run.advance();
    Ok((
        Cell::new(run.frame.tensor()),
        [run.origin.0, run.origin.1],
        run.background,
    ))
}

pub fn tick(
    cells: &Cell2d,
    mask: &Tensor,
    birth: &[usize],
    survive: &[usize],
    mode: Mode,
) -> Result<Cell2d> {
    if mode == Mode::Island {
        return value_error("a tick keeps its board; an island runs with Run.");
    }
    let (next, _, background) = evolve(cells, mask, birth, survive, mode, 0)?;
    Cell2d::new(Grid::read(&next.types)?.flipped(background).tensor())
}

#[derive(Clone, Debug, Default)]
pub struct Reel {
    shots: Vec<(Grid, (i64, i64))>,
}

impl Reel {
    pub fn new() -> Reel {
        Reel::default()
    }

    pub fn shoot(&mut self, run: &Run) {
        self.shots.push((run.frame.clone(), run.origin));
    }

    pub fn roll(&mut self, run: &mut Run) -> Stop {
        loop {
            let before = run.generation;
            let stop = run.step();
            if run.generation > before {
                self.shoot(run);
            }
            if let Some(stop) = stop {
                return stop;
            }
        }
    }

    pub fn len(&self) -> usize {
        self.shots.len()
    }

    pub fn is_empty(&self) -> bool {
        self.shots.is_empty()
    }

    pub fn populations(&self) -> Vec<u32> {
        self.shots
            .iter()
            .map(|(frame, _)| frame.population() as u32)
            .collect()
    }

    fn bounds(&self) -> ((i64, i64), usize, usize) {
        let mut lit = self.shots.iter().filter(|(frame, _)| frame.h > 0);
        let Some((first, origin)) = lit.next() else {
            return ((0, 0), 1, 1);
        };
        let (mut r0, mut c0) = *origin;
        let (mut r1, mut c1) = (r0 + first.h as i64, c0 + first.w as i64);
        for (frame, (r, c)) in lit {
            r0 = r0.min(*r);
            c0 = c0.min(*c);
            r1 = r1.max(r + frame.h as i64);
            c1 = c1.max(c + frame.w as i64);
        }
        ((r0, c0), (r1 - r0) as usize, (c1 - c0) as usize)
    }

    pub fn shape(&self) -> [usize; 2] {
        let (_, h, w) = self.bounds();
        [h, w]
    }

    pub fn frames(&self) -> Result<Vec<Cell2d>> {
        let ((r0, c0), h, w) = self.bounds();
        self.shots
            .iter()
            .map(|(frame, (r, c))| {
                let placed = if frame.h == 0 {
                    Grid::new(h, w)
                } else {
                    frame.embed(h, w, (r - r0) as usize, (c - c0) as usize)
                };
                Cell2d::new(placed.tensor())
            })
            .collect()
    }
}
