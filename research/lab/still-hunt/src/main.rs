use mrlycore::state::{choice, randint, seed};
use mrlycore::tensor::Tensor;
use mrlygame::frames::secondaries;
use mrlygame::music::score;
use mrlygame::sequence::rulebook;
use mrlygame::variations::{board, mask, Board};
use mrlygame::{pivot_options, quest, Config, Path, Way};
use mrlymath::life::crop;
use mrlymath::life::metrics::{churn, entropy};
use mrlymath::life::{moore, next_grid, Boundary, Counts, Fate};
use mrlymath::two::Cell2d;
use mrlynum::fft::{convolve_with, embed_kernel, peak_ring, radial_profile, transform};
use std::collections::HashMap;
use std::time::Instant;

// ENGINE

#[derive(Clone)]
struct Grid {
    side: usize,
    data: Vec<u8>,
}

impl Grid {
    fn of(cell: &Cell2d) -> Grid {
        let side = cell.width();
        Grid {
            side,
            data: cell.types().bytes().to_vec(),
        }
    }
    fn cell(&self) -> Cell2d {
        Cell2d::new(Tensor::of(self.data.clone(), vec![self.side, self.side]))
    }
    fn tile(&self, reps: usize) -> Grid {
        let side = self.side * reps;
        let mut data = vec![0u8; side * side];
        for r in 0..side {
            for c in 0..side {
                data[r * side + c] = self.data[(r % self.side) * self.side + c % self.side];
            }
        }
        Grid { side, data }
    }
    fn pad(&self, count: usize) -> Grid {
        let side = self.side + 2 * count;
        let mut data = vec![0u8; side * side];
        for r in 0..self.side {
            let src = &self.data[r * self.side..(r + 1) * self.side];
            data[(r + count) * side + count..(r + count) * side + count + self.side]
                .copy_from_slice(src);
        }
        Grid { side, data }
    }
}

struct Plan {
    n: usize,
    rho: usize,
    shift: bool,
    kre: Vec<f64>,
    kim: Vec<f64>,
}

struct Stepper {
    side: usize,
    wrap: bool,
    offsets: Vec<(isize, isize)>,
    plan: Plan,
}

impl Stepper {
    fn new(side: usize, mask: &Grid, wrap: bool) -> Stepper {
        let m = mask.side;
        let rho = m / 2;
        let mut offsets = Vec::new();
        for r in 0..m {
            for c in 0..m {
                if mask.data[r * m + c] == 1 {
                    offsets.push((r as isize - rho as isize, c as isize - rho as isize));
                }
            }
        }
        let need = if wrap && side.is_power_of_two() && m <= side {
            side
        } else if wrap {
            side + 2 * rho
        } else {
            side + rho
        };
        let n = need.max(m).next_power_of_two();
        let shift_cost = side * side * offsets.len();
        let fft_cost = 16 * n * n * (n.trailing_zeros() as usize + 1);
        let shift = shift_cost <= fft_cost;
        let (kre, kim) = if shift {
            (Vec::new(), Vec::new())
        } else {
            transform(&embed_kernel(&mask.data, m, n), n)
        };
        Stepper {
            side,
            wrap,
            offsets,
            plan: Plan {
                n,
                rho,
                shift,
                kre,
                kim,
            },
        }
    }
    fn cost(&self) -> u64 {
        if self.plan.shift {
            (self.side * self.side * self.offsets.len()) as u64
        } else {
            (16 * self.plan.n * self.plan.n * (self.plan.n.trailing_zeros() as usize + 1)) as u64
        }
    }
    fn counts(&self, grid: &[u8]) -> Vec<u32> {
        if self.plan.shift {
            self.counts_shift(grid)
        } else {
            self.counts_fft(grid)
        }
    }
    fn counts_shift(&self, grid: &[u8]) -> Vec<u32> {
        let s = self.side;
        let si = s as isize;
        let mut out = vec![0u32; s * s];
        for &(dr, dc) in &self.offsets {
            for r in 0..s {
                let sr = r as isize + dr;
                let sr = if self.wrap {
                    sr.rem_euclid(si)
                } else if sr < 0 || sr >= si {
                    continue;
                } else {
                    sr
                } as usize;
                let row = &grid[sr * s..(sr + 1) * s];
                let dst = &mut out[r * s..(r + 1) * s];
                if self.wrap {
                    let shift = dc.rem_euclid(si) as usize;
                    for (c, slot) in dst.iter_mut().enumerate() {
                        let mut sc = c + shift;
                        if sc >= s {
                            sc -= s;
                        }
                        *slot += row[sc] as u32;
                    }
                } else {
                    let lo = (-dc).max(0) as usize;
                    let hi = (si - dc).min(si) as usize;
                    for c in lo..hi {
                        dst[c] += row[(c as isize + dc) as usize] as u32;
                    }
                }
            }
        }
        out
    }
    fn counts_fft(&self, grid: &[u8]) -> Vec<u32> {
        let s = self.side;
        let n = self.plan.n;
        let rho = self.plan.rho;
        let mut field = vec![0.0; n * n];
        let periodic = self.wrap && n != s;
        let (span, off) = if periodic { (s + 2 * rho, rho) } else { (s, 0) };
        for r in 0..span {
            let sr = (r + s - off % s) % s;
            for c in 0..span {
                let sc = (c + s - off % s) % s;
                field[r * n + c] = grid[sr * s + sc] as f64;
            }
        }
        let conv = convolve_with(&field, &self.plan.kre, &self.plan.kim, n);
        let mut out = vec![0u32; s * s];
        for r in 0..s {
            for c in 0..s {
                out[r * s + c] = conv[(r + off) * n + c + off].round() as u32;
            }
        }
        out
    }
    fn next(&self, grid: &[u8], birth: &[bool], survive: &[bool]) -> Vec<u8> {
        let counts = self.counts(grid);
        grid.iter()
            .zip(&counts)
            .map(|(&v, &k)| {
                let table = if v == 1 { survive } else { birth };
                u8::from(table.get(k as usize).copied().unwrap_or(false))
            })
            .collect()
    }
}

fn table(values: &[usize], budget: usize) -> Vec<bool> {
    let mut out = vec![false; budget + 1];
    for &v in values {
        if v <= budget {
            out[v] = true;
        }
    }
    out
}

struct Chapter {
    way: Way,
    path: Path,
    mask: Grid,
    rule: String,
    frames: Vec<Vec<u8>>,
    fate: Fate,
}

struct Replay {
    outer: u64,
    tile: Grid,
    inner: u64,
    boundary: Boundary,
    canvas: usize,
    unit: usize,
    t: usize,
    climb: usize,
    chapters: Vec<Chapter>,
    attempts: usize,
    cost: u64,
}

fn hex_key() {
    for _ in 0..8 {
        randint(0, 15);
    }
}

fn run_chapter(
    start: Grid,
    mask: &Grid,
    rule: (&[usize], &[usize]),
    boundary: Boundary,
    cap: usize,
    cost: &mut u64,
    deadline: Option<Instant>,
) -> Option<(Vec<Vec<u8>>, Fate)> {
    let (birth, survive) = rule;
    let budget = mask.data.iter().filter(|&&v| v == 1).count();
    let birth = table(birth, budget);
    let survive = table(survive, budget);
    let stepper = Stepper::new(start.side, mask, boundary.wrap());
    let mut current = start.data;
    let mut frames = vec![current.clone()];
    let mut history: HashMap<Vec<u8>, usize> = HashMap::new();
    history.insert(current.clone(), 0);
    let mut fate = Fate::Timeout;
    for i in 1..cap {
        if let Some(d) = deadline {
            if Instant::now() > d {
                return None;
            }
        }
        let next = stepper.next(&current, &birth, &survive);
        *cost += stepper.cost();
        if next == current {
            fate = if next.iter().all(|&v| v == 0) {
                Fate::Dead
            } else {
                Fate::Alive
            };
            break;
        }
        if history.contains_key(&next) {
            fate = Fate::Loop;
            break;
        }
        history.insert(next.clone(), i);
        current = next;
        frames.push(current.clone());
    }
    Some((frames, fate))
}

fn replay(outer: u64, config: &Config, deadline: Option<Instant>) -> Option<Replay> {
    seed(outer);
    let mut cost = 0u64;
    for attempt in 0..config.attempts.max(1) {
        let inner = randint(0, i64::MAX) as u64;
        seed(inner);
        hex_key();
        let boundary = choice(&[Boundary::Constant, Boundary::Wrap]);
        let mut chapters: Vec<Chapter> = Vec::new();
        let mut prev: Option<Grid> = None;
        let mut climb = (0, 0, 0, 0);
        let mut tile = Grid {
            side: 0,
            data: Vec::new(),
        };
        for index in 0..config.max_segments {
            hex_key();
            choice(&secondaries());
            let (way, stage) = match (&prev, chapters.last()) {
                (Some(grid), Some(last)) => (last.way.flip(), Board::of(grid.cell())),
                _ => {
                    let way = choice(&[Way::Conway, Way::Mrly]);
                    (way, board(config).unwrap())
                }
            };
            let (path, mask_cell, rule) = match way {
                Way::Conway => (Path::Simple, moore(), None),
                Way::Mrly => {
                    let (path, cell) = mask(&stage, config).unwrap();
                    let rule = rulebook(path == Path::Simple);
                    (path, cell, Some(rule))
                }
            };
            let _ = score(way);
            if index == 0 {
                climb = (stage.canvas_unit(), stage.unit, stage.grid, stage.canvas);
                tile = Grid::of(&stage.cell);
            }
            let (birth, survive, name) = match &rule {
                Some(r) => (
                    r.birth(),
                    r.survive(),
                    format!(
                        "B{}/S{}{}{}",
                        r.birth_sequence.name(),
                        r.survive_sequence.name(),
                        if r.zeros { "+0" } else { "" },
                        if r.ones { "+1" } else { "" }
                    ),
                ),
                None => (
                    Counts::from(vec![3]),
                    Counts::from(vec![2, 3]),
                    "B3/S23".to_string(),
                ),
            };
            let mask_grid = Grid::of(&mask_cell);
            let budget = mask_cell.types().sum() as usize;
            let birth = birth.values(budget).unwrap();
            let survive = survive.values(budget).unwrap();
            let mut start = Grid::of(&stage.cell);
            if stage.grid > 1 {
                start = start.tile(stage.grid);
            }
            if stage.padding() > 0 {
                start = start.pad(stage.padding());
            }
            let (frames, fate) = run_chapter(
                start,
                &mask_grid,
                (&birth, &survive),
                boundary,
                config.max_generations,
                &mut cost,
                deadline,
            )?;
            chapters.push(Chapter {
                way,
                path,
                mask: mask_grid,
                rule: name,
                frames,
                fate,
            });
            if fate == Fate::Alive {
                return Some(Replay {
                    outer,
                    tile: tile.clone(),
                    inner,
                    boundary,
                    canvas: climb.0,
                    unit: climb.1,
                    t: climb.2,
                    climb: climb.3,
                    chapters,
                    attempts: attempt + 1,
                    cost,
                });
            }
            let last = chapters.last_mut().unwrap();
            let count = last.frames.len();
            let options = pivot_options(count);
            let length = if options.is_empty() {
                count
            } else {
                choice(&options)
            };
            if length > 0 && length < last.frames.len() {
                last.frames.truncate(length);
            }
            prev = Some(Grid {
                side: climb.0,
                data: last.frames.last().unwrap().clone(),
            });
        }
    }
    None
}

fn stepper_check() -> usize {
    let mut failures = 0;
    let mut state = 88172645463325252u64;
    let mut next = || {
        state ^= state << 13;
        state ^= state >> 7;
        state ^= state << 17;
        state
    };
    for trial in 0..24 {
        let side = [16, 17, 32, 45][trial % 4];
        let m = [3, 5, 9, 15][(trial / 4) % 4];
        let wrap = trial % 2 == 0;
        let mut mask = Grid {
            side: m,
            data: (0..m * m).map(|_| (next() % 2) as u8).collect(),
        };
        mask.data[(m / 2) * m + m / 2] = 0;
        let grid: Vec<u8> = (0..side * side).map(|_| (next() % 3 == 0) as u8).collect();
        let budget = mask.data.iter().filter(|&&v| v == 1).count();
        let birth: Vec<usize> = (0..=budget).filter(|_| next() % 3 == 0).collect();
        let survive: Vec<usize> = (0..=budget).filter(|_| next() % 2 == 0).collect();
        let boundary = if wrap {
            Boundary::Wrap
        } else {
            Boundary::Constant
        };
        let cell = Cell2d::new(Tensor::of(grid.clone(), vec![side, side]));
        let want = next_grid(&cell, &birth, &survive, mask.cell().types(), boundary).unwrap();
        for force in [true, false] {
            let mut stepper = Stepper::new(side, &mask, wrap);
            if stepper.plan.shift != force {
                stepper.plan.shift = force;
                if !force {
                    let (kre, kim) =
                        transform(&embed_kernel(&mask.data, m, stepper.plan.n), stepper.plan.n);
                    stepper.plan.kre = kre;
                    stepper.plan.kim = kim;
                }
            }
            let got = stepper.next(&grid, &table(&birth, budget), &table(&survive, budget));
            if got != want.types().bytes() {
                failures += 1;
            }
        }
    }
    failures
}

// READINGS

struct Rings {
    n: usize,
    index: Vec<usize>,
    count: Vec<usize>,
}

impl Rings {
    fn new(n: usize) -> Rings {
        let mut index = vec![0; n * n];
        let mut count = Vec::new();
        for r in 0..n {
            for c in 0..n {
                let k = (r.min(n - r) as f64).hypot(c.min(n - c) as f64).round() as usize;
                index[r * n + c] = k;
                if k >= count.len() {
                    count.resize(k + 1, 0);
                }
                count[k] += 1;
            }
        }
        Rings { n, index, count }
    }
    fn sums(&self, values: &[f64]) -> Vec<f64> {
        let mut sums = vec![0.0; self.count.len()];
        for (i, &v) in values.iter().enumerate() {
            sums[self.index[i]] += v;
        }
        sums
    }
    fn means(&self, values: &[f64]) -> Vec<f64> {
        self.sums(values)
            .iter()
            .zip(&self.count)
            .map(|(&s, &c)| if c == 0 { 0.0 } else { s / c as f64 })
            .collect()
    }
}

fn argmax(values: &[f64], count: &[usize], cap: usize) -> usize {
    let mut best = 0;
    let mut top = f64::NEG_INFINITY;
    for (k, &v) in values.iter().enumerate().skip(1).take(cap) {
        if count[k] >= MIN_BINS && v > top {
            top = v;
            best = k;
        }
    }
    best
}

fn argmin(values: &[f64]) -> usize {
    let mut best = 0;
    let mut low = f64::INFINITY;
    for (k, &v) in values.iter().enumerate().skip(1) {
        if v < low {
            low = v;
            best = k;
        }
    }
    best
}

struct Spectrum {
    k: usize,
    full_k: usize,
    shares: Vec<f64>,
    cut_k: usize,
}

fn spectrum(frame: &[u8], s: usize, rings: &Rings) -> Spectrum {
    let n = rings.n;
    let ones = frame.iter().filter(|&&v| v == 1).count() as f64;
    let mean = ones / (s * s) as f64;
    let mut field = vec![0.0; n * n];
    for r in 0..s {
        for c in 0..s {
            field[r * n + c] = frame[r * s + c] as f64 - mean;
        }
    }
    let (re, im) = transform(&field, n);
    let power: Vec<f64> = re.iter().zip(&im).map(|(a, b)| a * a + b * b).collect();
    let sums = rings.sums(&power);
    let total: f64 = sums.iter().skip(1).sum();
    let shares: Vec<f64> = sums
        .iter()
        .map(|&v| if total > 0.0 { v / total } else { 0.0 })
        .collect();
    let means: Vec<f64> = sums
        .iter()
        .zip(&rings.count)
        .map(|(&s, &c)| if c == 0 { 0.0 } else { s / c as f64 })
        .collect();
    let half = n / 2;
    let k = argmax(&means, &rings.count, half);
    let full_k = argmax(&means, &rings.count, means.len());
    let mut centred = vec![0.0; n * n];
    for r in 0..n {
        for c in 0..n {
            centred[((r + half) % n) * n + (c + half) % n] = power[r * n + c];
        }
    }
    let cut_k = peak_ring(&radial_profile(&centred, n));
    Spectrum {
        k,
        full_k,
        shares,
        cut_k,
    }
}

struct Xor(u64);

impl Xor {
    fn unit(&mut self) -> f64 {
        self.0 ^= self.0 << 13;
        self.0 ^= self.0 >> 7;
        self.0 ^= self.0 << 17;
        (self.0 >> 11) as f64 / (1u64 << 53) as f64
    }
}

const BASELINE_SEED: u64 = 1729;
const BASELINE_SAMPLES: usize = 4;
const BUCKETS: usize = 20;

struct Baseline {
    cache: HashMap<(usize, usize, usize), Vec<f64>>,
}

impl Baseline {
    fn shares(&mut self, s: usize, rings: &Rings, fill: f64) -> &Vec<f64> {
        let bucket = (fill * BUCKETS as f64).round() as usize;
        let key = (s, rings.n, bucket);
        self.cache.entry(key).or_insert_with(|| {
            let p = (bucket as f64 / BUCKETS as f64)
                .clamp(0.5 / BUCKETS as f64, 1.0 - 0.5 / BUCKETS as f64);
            let mut rng = Xor(BASELINE_SEED ^ ((s as u64) << 20) ^ ((bucket as u64) << 40));
            let mut total = vec![0.0; rings.count.len()];
            for _ in 0..BASELINE_SAMPLES {
                let frame: Vec<u8> = (0..s * s).map(|_| u8::from(rng.unit() < p)).collect();
                for (slot, v) in total.iter_mut().zip(spectrum(&frame, s, rings).shares) {
                    *slot += v / BASELINE_SAMPLES as f64;
                }
            }
            total
        })
    }
}

struct Lobe {
    first: Option<(usize, usize)>,
    neg: usize,
    band: Option<(usize, usize)>,
}

fn lobes(mask: &Grid, rings: &Rings) -> Lobe {
    let (re, _) = transform(&embed_kernel(&mask.data, mask.side, rings.n), rings.n);
    let g = rings.means(&re);
    let mut first = None;
    let mut k = 1;
    while k < g.len() {
        if g[k] < 0.0 {
            let lo = k;
            while k < g.len() && g[k] < 0.0 {
                k += 1;
            }
            first = Some((lo, k - 1));
            break;
        }
        k += 1;
    }
    let neg = argmin(&g);
    let band = if g[neg] < 0.0 {
        let mut lo = neg;
        while lo > 1 && g[lo - 1] < 0.0 {
            lo -= 1;
        }
        let mut hi = neg;
        while hi + 1 < g.len() && g[hi + 1] < 0.0 {
            hi += 1;
        }
        Some((lo, hi))
    } else {
        None
    };
    Lobe { first, neg, band }
}

fn period(tile: &Grid) -> usize {
    let u = tile.side;
    for d in 1..=u {
        if !u.is_multiple_of(d) {
            continue;
        }
        let ok = (0..u).all(|r| {
            (0..u).all(|c| {
                let v = tile.data[r * u + c];
                v == tile.data[((r + d) % u) * u + c] && v == tile.data[r * u + (c + d) % u]
            })
        });
        if ok {
            return d;
        }
    }
    u
}

// HUNT

#[derive(Clone)]
struct Row {
    seed: u64,
    chapter: usize,
    way: Way,
    path: Path,
    mask: usize,
    crop: usize,
    t: usize,
    generation: usize,
    still: bool,
    fill: f64,
    k: usize,
    wavelength: f64,
    share: f64,
    ratio: f64,
    churn: f64,
    entropy: i64,
    lobe: Option<(usize, usize)>,
    band: Option<(usize, usize)>,
    comb: f64,
    field: usize,
}

impl Row {
    fn in_lobe(&self) -> bool {
        self.lobe
            .is_some_and(|(lo, hi)| self.k >= lo && self.k <= hi)
    }
    fn in_band(&self) -> bool {
        self.band
            .is_some_and(|(lo, hi)| self.k >= lo && self.k <= hi)
    }
    fn comb_distance(&self) -> f64 {
        let j = (self.k as f64 / self.comb).round().max(1.0);
        (self.k as f64 - j * self.comb).abs()
    }
    fn comb_chance(&self) -> f64 {
        (1.0 / self.comb).min(1.0)
    }
    fn drawn(&self) -> bool {
        self.path != Path::Simple
    }
    fn resolved(&self) -> bool {
        self.crop >= 4 * self.mask
    }
    fn text(&self) -> String {
        format!(
            "seed {} chapter {} gen {} {} {} m{} crop {} t {} k {} wavelength {:.2} ratio {:.1} share {:.4} fill {:.3} churn {:.4} entropy {} lobe {:?} comb {:.2} field {}{}",
            self.seed, self.chapter, self.generation, self.way.name(), self.path.name(), self.mask, self.crop, self.t, self.k, self.wavelength, self.ratio, self.share, self.fill, self.churn, self.entropy, self.lobe, self.comb, self.field, if self.still { " still" } else { "" }
        )
    }
}

struct Tally {
    quests: usize,
    frames: usize,
    skipped: usize,
    cut_disagree: usize,
    corner: usize,
    rows: Vec<Row>,
    per_path: HashMap<&'static str, (usize, usize)>,
    verified: Vec<(u64, usize, usize, bool)>,
    mismatches: usize,
}

impl Tally {
    fn new() -> Tally {
        Tally {
            quests: 0,
            frames: 0,
            skipped: 0,
            cut_disagree: 0,
            corner: 0,
            rows: Vec::new(),
            per_path: HashMap::new(),
            verified: Vec::new(),
            mismatches: 0,
        }
    }
}

const CHLADNI_GAIN: f64 = 3.0;
const LOW_RING: usize = 2;
const MIN_BINS: usize = 8;
const MIN_LIVE: usize = 8;
const PRINT_CAP: usize = 3;
const VERIFY_COST: u64 = 100_000_000;
const VERIFY_CAP: usize = 4;

fn verify(r: &Replay, config: &Config) -> usize {
    seed(r.outer);
    let q = quest(config).unwrap();
    let mut bad = 0;
    bad += usize::from(q.seed != r.inner || q.canvas != r.canvas);
    let lengths: Vec<usize> = r.chapters.iter().map(|c| c.frames.len()).collect();
    bad += usize::from(q.story.chapter_lengths() != lengths);
    for (a, b) in q.segments.iter().zip(&r.chapters) {
        bad += usize::from(
            a.way != b.way || a.path != b.path || a.mask.types().bytes() != b.mask.data.as_slice(),
        );
    }
    let mine: Vec<&Vec<u8>> = r.chapters.iter().flat_map(|c| c.frames.iter()).collect();
    for (a, b) in q.story.grids().iter().zip(&mine) {
        bad += usize::from(a.types().bytes() != b.as_slice());
    }
    bad
}

fn hunt(r: &Replay, baseline: &mut Baseline, tally: &mut Tally, label: &str) {
    let cells: Vec<Cell2d> = r
        .chapters
        .iter()
        .flat_map(|c| {
            c.frames
                .iter()
                .map(|f| Cell2d::new(Tensor::of(f.clone(), vec![r.canvas, r.canvas])))
        })
        .collect();
    let cropped = crop(&cells);
    let s = cropped[0].width();
    let widest = r.chapters.iter().map(|c| c.mask.side).max().unwrap_or(3);
    let n = s.max(widest).next_power_of_two();
    let rings = Rings::new(n);
    let p = period(&r.tile);
    let comb = n as f64 / p as f64;
    let summary: Vec<String> = r
        .chapters
        .iter()
        .map(|c| {
            format!(
                "{}:{}:{}:m{}:{}:{}",
                c.way.name(),
                c.path.name(),
                c.rule,
                c.mask.side,
                c.frames.len(),
                c.fate.name()
            )
        })
        .collect();
    println!(
        "{label} seed {} inner {} {:?} canvas {} crop {} field {} unit {} period {} t {} climb {} attempts {} comb {:.2} chapters {}",
        r.outer, r.inner, r.boundary, r.canvas, s, n, r.unit, p, r.t, r.climb, r.attempts, comb, summary.join(" ")
    );
    tally.quests += 1;
    let mut peaks: Vec<usize> = Vec::new();
    let mut index = 0;
    for (ci, chapter) in r.chapters.iter().enumerate() {
        let lobe = lobes(&chapter.mask, &rings);
        let mut found: Vec<Row> = Vec::new();
        let mut shown = 0;
        let last = chapter.frames.len() - 1;
        for gen in 0..chapter.frames.len() {
            let frame = cropped[index].types().bytes();
            let live = frame.iter().filter(|&&v| v == 1).count();
            let fill = live as f64 / (s * s) as f64;
            let churned = if index == 0 {
                0.0
            } else {
                churn(&cropped[index - 1..=index])
            };
            let ent = entropy(&cropped[index]);
            index += 1;
            tally.frames += 1;
            let entry = tally.per_path.entry(chapter.path.name()).or_insert((0, 0));
            entry.0 += 1;
            if live < MIN_LIVE || live + MIN_LIVE > s * s {
                tally.skipped += 1;
                continue;
            }
            let spec = spectrum(frame, s, &rings);
            if spec.cut_k != spec.k {
                tally.cut_disagree += 1;
            }
            if spec.full_k != spec.k {
                tally.corner += 1;
            }
            let base = baseline.shares(s, &rings, fill)[spec.k];
            let share = spec.shares[spec.k];
            let ratio = if base > 0.0 { share / base } else { 0.0 };
            let wavelength = n as f64 / spec.k.max(1) as f64;
            if ratio >= CHLADNI_GAIN && wavelength <= 2.0 * chapter.mask.side as f64 {
                entry.1 += 1;
                let row = Row {
                    seed: r.outer,
                    chapter: ci,
                    way: chapter.way,
                    path: chapter.path,
                    mask: chapter.mask.side,
                    crop: s,
                    t: r.t,
                    generation: gen,
                    still: chapter.fate == Fate::Alive && gen == last,
                    fill,
                    k: spec.k,
                    wavelength,
                    share,
                    ratio,
                    churn: churned,
                    entropy: ent,
                    lobe: lobe.first,
                    band: lobe.band,
                    comb,
                    field: n,
                };
                if gen > 0 && shown < PRINT_CAP {
                    println!(
                        "  gen {gen} fill {:.3} k {} wavelength {:.2} ratio {:.1} share {:.4} churn {:.4} entropy {}{}",
                        fill, spec.k, wavelength, ratio, share, churned, ent, if row.still { " still" } else { "" }
                    );
                    shown += 1;
                }
                found.push(row);
            }
        }
        if !found.is_empty() {
            println!(
                "  chapter {ci} {} {} m{} lobe {:?} neg {} band {:?} chladni {} of {} frames, {} at gen 0",
                chapter.way.name(),
                chapter.path.name(),
                chapter.mask.side,
                lobe.first,
                lobe.neg,
                lobe.band,
                found.len(),
                chapter.frames.len(),
                found.iter().filter(|r| r.generation == 0).count()
            );
        }
        peaks.extend(found.iter().filter(|r| r.generation > 0).map(|r| r.k));
        tally.rows.extend(found);
    }
    if !peaks.is_empty() {
        let mut hist: HashMap<usize, usize> = HashMap::new();
        for k in &peaks {
            *hist.entry(*k).or_default() += 1;
        }
        let mut bins: Vec<(usize, usize)> = hist.into_iter().collect();
        bins.sort();
        let top = bins
            .iter()
            .max_by_key(|(k, c)| (*c, usize::MAX - *k))
            .unwrap();
        let cells: Vec<String> = bins.iter().map(|(k, c)| format!("{k}:{c}")).collect();
        println!(
            "  peak rings past gen 0 {} plurality ring {} in {} of {}",
            cells.join(" "),
            top.0,
            top.1,
            peaks.len()
        );
    }
}

fn tag(pass: usize, total: usize, witness: Option<&Row>) -> String {
    if total == 0 {
        "Empty".to_string()
    } else if pass == total {
        "Verified on the printed domain".to_string()
    } else {
        format!(
            "Refuted, witness {}",
            witness.map(|r| r.text()).unwrap_or_default()
        )
    }
}

type Domain = (&'static str, Box<dyn Fn(&Row) -> bool>);

fn law_table(name: &str, rows: &[&Row]) {
    let n = rows.len();
    let mut quests: Vec<u64> = rows.iter().map(|r| r.seed).collect();
    quests.sort_unstable();
    quests.dedup();
    let mut chapters: Vec<(u64, usize)> = rows.iter().map(|r| (r.seed, r.chapter)).collect();
    chapters.sort_unstable();
    chapters.dedup();
    let mut combs: Vec<f64> = rows.iter().map(|r| r.comb).collect();
    combs.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let flat = rows.iter().filter(|r| r.t == 1).count();
    let lobe = rows.iter().filter(|r| r.in_lobe()).count();
    let band = rows.iter().filter(|r| r.in_band()).count();
    let half = rows.iter().filter(|r| r.comb_distance() <= 0.5).count();
    let one = rows.iter().filter(|r| r.comb_distance() <= 1.0).count();
    let chance: f64 = rows.iter().map(|r| r.comb_chance()).sum::<f64>();
    let product = rows
        .iter()
        .filter(|r| r.in_lobe() && r.comb_distance() <= 0.5)
        .count();
    println!(
        "  {name}: rows {n} chapters {} quests {} lobe {lobe} band {band} comb-half {half} (chance {chance:.1}) comb-one {one} product {product}",
        chapters.len(),
        quests.len()
    );
    if n > 0 {
        println!(
            "    rows on one-tile boards (t = 1) {flat}, comb spacing min {:.2} median {:.2} max {:.2}",
            combs[0],
            combs[n / 2],
            combs[n - 1]
        );
    }
    println!(
        "    lobe {}",
        tag(lobe, n, rows.iter().find(|r| !r.in_lobe()).copied())
    );
    println!(
        "    comb-half {}",
        tag(
            half,
            n,
            rows.iter().find(|r| r.comb_distance() > 0.5).copied()
        )
    );
    println!(
        "    product {}",
        tag(
            product,
            n,
            rows.iter()
                .find(|r| !(r.in_lobe() && r.comb_distance() <= 0.5))
                .copied()
        )
    );
}

fn laws(tally: &Tally) {
    let rows = &tally.rows;
    println!();
    println!(
        "quests {} frames {} skipped {} ring-cut-disagreements {} corner-peaks {}",
        tally.quests, tally.frames, tally.skipped, tally.cut_disagree, tally.corner
    );
    println!(
        "chladni-like frames {} (ring share at least {CHLADNI_GAIN} times the random share at the same ring, wavelength at most twice the mask side), {} at gen 0, {} stills",
        rows.len(),
        rows.iter().filter(|r| r.generation == 0).count(),
        rows.iter().filter(|r| r.still).count()
    );
    let moved: Vec<&Row> = rows.iter().filter(|r| r.generation > 0).collect();
    let low = moved.iter().filter(|r| r.k <= LOW_RING).count();
    let long = moved
        .iter()
        .filter(|r| r.wavelength > r.crop as f64)
        .count();
    let resolved = moved.iter().filter(|r| r.resolved()).count();
    println!(
        "envelope of the {} hits past gen 0: {low} peak at ring 1 or 2 of their field, {long} read a wavelength longer than the crop, {resolved} are resolved (crop at least four mask sides); the cut admits all of them",
        moved.len()
    );
    let mut env: HashMap<&'static str, (usize, usize)> = HashMap::new();
    for row in &moved {
        let e = env.entry(row.path.name()).or_insert((0, 0));
        e.0 += 1;
        if row.k <= LOW_RING {
            e.1 += 1;
        }
    }
    let mut env: Vec<_> = env.into_iter().collect();
    env.sort();
    let cells: Vec<String> = env
        .iter()
        .map(|(path, (n, low))| format!("{path} {low} of {n}"))
        .collect();
    println!(
        "envelope by path, ring 1 or 2 past gen 0: {}",
        cells.join(" ")
    );
    let mut by: HashMap<String, usize> = HashMap::new();
    for row in rows.iter().filter(|r| r.generation > 0) {
        *by.entry(format!("way {}", row.way.name())).or_default() += 1;
        *by.entry(format!("path {}", row.path.name())).or_default() += 1;
        *by.entry(format!("mask {}", row.mask)).or_default() += 1;
        *by.entry(format!("t {}", row.t)).or_default() += 1;
        *by.entry(format!("chapter {}", row.chapter)).or_default() += 1;
    }
    let mut keys: Vec<_> = by.iter().collect();
    keys.sort();
    let cells: Vec<String> = keys.iter().map(|(k, v)| format!("{k}:{v}")).collect();
    println!("where (gen > 0) {}", cells.join(" "));
    let mut paths: Vec<_> = tally.per_path.iter().collect();
    paths.sort();
    for (path, (frames, hits)) in paths {
        println!(
            "path {path} frames {frames} chladni {hits} rate {:.4}",
            *hits as f64 / (*frames).max(1) as f64
        );
    }
    println!("laws: lobe = k in the mask's first negative lobe, band = k in the negative band around the most negative ring, comb = k within half (one) ring of a multiple of field/period, product = lobe and comb-half");
    let domains: [Domain; 7] = [
        (
            "gen > 0, drawn masks",
            Box::new(|r| r.drawn() && r.generation > 0),
        ),
        (
            "gen > 0, drawn masks, crop >= 4 mask",
            Box::new(|r| r.drawn() && r.generation > 0 && r.resolved()),
        ),
        (
            "gen > 0, basic path",
            Box::new(|r| r.path == Path::Basic && r.generation > 0),
        ),
        (
            "gen > 0, copy path",
            Box::new(|r| r.path == Path::Copy && r.generation > 0),
        ),
        (
            "gen > 0, simple masks",
            Box::new(|r| !r.drawn() && r.generation > 0),
        ),
        ("stills, drawn masks", Box::new(|r| r.drawn() && r.still)),
        ("stills, simple masks", Box::new(|r| !r.drawn() && r.still)),
    ];
    for (name, keep) in &domains {
        let group: Vec<&Row> = rows.iter().filter(|r| keep(r)).collect();
        law_table(name, &group);
    }
    let sharp: Vec<&Row> = rows
        .iter()
        .filter(|r| r.generation > 0 && r.t >= 3 && r.comb >= 4.0)
        .collect();
    law_table("gen > 0, t >= 3, comb spacing >= 4", &sharp);
    for (name, keep) in [
        ("gen > 0, drawn masks, crop >= 4 mask", true),
        ("stills, drawn masks", false),
    ] {
        let group: Vec<&Row> = rows
            .iter()
            .filter(|r| {
                r.drawn()
                    && if keep {
                        r.generation > 0 && r.resolved()
                    } else {
                        r.still
                    }
            })
            .collect();
        let mut bins = [0usize; 13];
        let mut near = 0;
        let mut lobe_centre = Vec::new();
        for r in &group {
            let x = r.wavelength / r.mask as f64;
            bins[((x / 0.25) as usize).min(12)] += 1;
            if (0.8..=1.25).contains(&x) {
                near += 1;
            }
            if let Some((lo, hi)) = r.lobe {
                lobe_centre.push(2.0 * r.field as f64 / (lo + hi) as f64 / r.mask as f64);
            }
        }
        lobe_centre.sort_by(|a, b| a.partial_cmp(b).unwrap());
        let cells: Vec<String> = bins
            .iter()
            .enumerate()
            .map(|(i, c)| format!("{:.2}:{c}", i as f64 * 0.25))
            .collect();
        println!(
            "wavelength over mask side ({name}): rows {} within 0.8..1.25 {near} histogram {}",
            group.len(),
            cells.join(" ")
        );
        if !lobe_centre.is_empty() {
            println!(
                "lobe centre wavelength over mask side ({name}): min {:.2} median {:.2} max {:.2}",
                lobe_centre[0],
                lobe_centre[lobe_centre.len() / 2],
                lobe_centre[lobe_centre.len() - 1]
            );
        }
    }
    let mut stills: Vec<&Row> = rows.iter().filter(|r| r.still).collect();
    stills.sort_by(|a, b| b.ratio.partial_cmp(&a.ratio).unwrap());
    for row in stills.iter().take(3) {
        println!("best still {}", row.text());
    }
    let mut moving: Vec<&Row> = rows
        .iter()
        .filter(|r| r.generation > 0 && r.drawn() && r.resolved())
        .collect();
    moving.sort_by(|a, b| b.ratio.partial_cmp(&a.ratio).unwrap());
    for row in moving.iter().take(3) {
        println!("best resolved drawn-mask frame {}", row.text());
    }
    let cells: Vec<String> = tally
        .verified
        .iter()
        .map(|(seed, attempts, climb, copy)| {
            format!("seed {seed} attempts {attempts} canvas climb {climb} copy path {copy}")
        })
        .collect();
    if cells.is_empty() {
        println!("no quest here steps under the replay cost {VERIFY_COST}, so none is checked cell for cell");
    } else {
        println!(
            "replay verified against mrlygame::quest cell for cell on the {} cheapest quests, those stepping under the replay cost {VERIFY_COST}, mismatches {}: {}",
            tally.verified.len(),
            tally.mismatches,
            cells.join(", ")
        );
    }
    println!(
        "the remaining {} quests rest on the same draw order and the stepper's 24-case agreement with mrlymath::life::next_grid; the engine's naive count is the cost, no wall clock is claimed",
        tally.quests.saturating_sub(tally.verified.len())
    );
}

const SEEDS: u64 = 64;
const LARGE_SEEDS: std::ops::RangeInclusive<u64> = 65..=68;
const BUDGET_MS: u64 = 110_000;

fn main() {
    let start = Instant::now();
    let failures = stepper_check();
    println!("stepper check against mrlymath::life::next_grid: failures {failures}");
    assert_eq!(failures, 0);
    let config = Config::default();
    println!(
        "config default max_generations {} max_segments {} max_canvas {} tile {}..{} mask {}..{} attempts {}; outer seeds 1..={SEEDS}; baseline seed {BASELINE_SEED} samples {BASELINE_SAMPLES} buckets {BUCKETS}",
        config.max_generations, config.max_segments, config.max_canvas, config.min_tile, config.max_tile, config.min_mask, config.max_mask, config.attempts
    );
    println!("frames are cropped by mrlymath::life::crop, mean removed, zero padded to the next power of two at or above the crop and the widest mask; rings read over the full square, a peak ring holds at least {MIN_BINS} bins and sits at or below half the field (wavelength at least 2 cells), peaks past it are counted as corner peaks");
    let mut baseline = Baseline {
        cache: HashMap::new(),
    };
    let mut tally = Tally::new();
    for s in 1..=SEEDS {
        match replay(s, &config, None) {
            Some(r) => {
                if r.cost <= VERIFY_COST && tally.verified.len() < VERIFY_CAP {
                    tally.mismatches += verify(&r, &config);
                    tally.verified.push((
                        s,
                        r.attempts,
                        r.climb,
                        r.chapters.iter().any(|c| c.path == Path::Copy),
                    ));
                }
                hunt(&r, &mut baseline, &mut tally, "quest");
            }
            None => println!("quest seed {s} no settle"),
        }
    }
    laws(&tally);
    let mut per_field: HashMap<usize, Vec<f64>> = HashMap::new();
    for ((_, n, _), v) in &baseline.cache {
        per_field
            .entry(*n)
            .or_default()
            .push(v.iter().skip(1).cloned().fold(0.0, f64::max));
    }
    let mut fields: Vec<_> = per_field.into_iter().collect();
    fields.sort_by_key(|(n, _)| *n);
    for (n, mut v) in fields {
        v.sort_by(|a, b| a.partial_cmp(b).unwrap());
        println!(
            "baseline field {n} keys {} largest ring share min {:.4} median {:.4} max {:.4}",
            v.len(),
            v[0],
            v[v.len() / 2],
            v[v.len() - 1]
        );
    }
    let large = Config {
        max_canvas: 512,
        max_mask: 128,
        ..Config::default()
    };
    println!(
        "large config max_canvas 512 max_mask 128 outer seeds {:?}, abandoned past {BUDGET_MS} ms",
        LARGE_SEEDS
    );
    let deadline = start + std::time::Duration::from_millis(BUDGET_MS);
    let mut large_tally = Tally::new();
    for s in LARGE_SEEDS {
        match replay(s, &large, Some(deadline)) {
            Some(r) => hunt(&r, &mut baseline, &mut large_tally, "large"),
            None if Instant::now() > deadline => {
                println!("large seed {s} abandoned at the budget");
                break;
            }
            None => println!("large seed {s} no settle"),
        }
    }
    if large_tally.quests > 0 {
        laws(&large_tally);
    }
}
