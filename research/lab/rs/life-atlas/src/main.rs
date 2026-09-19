use mrlycore::tensor::Tensor;
use mrlylab::atlas::run::heat_frame;
use mrlylab::atlas::{
    block_split, census, factors, fate_table, first_negative_lobe, mask_tensor, mini, pack, Census,
    Reading, Run,
};
use mrlymath::life::{animate, mask_offsets, moore, Boundary, Config, Fate};
use mrlymath::two::{carpet, create, Cell2d};
use std::collections::{BTreeMap, BTreeSet};

const CAP: usize = 5000;
const SAMPLE_SEED: u64 = 1729;
const ROWS: usize = 40;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
enum Kind {
    None,
    Confined,
    Tiled,
    Board,
    Proper,
}

type Witness = (usize, usize, usize, usize, u128, usize);

struct Scan {
    best: Kind,
    in_place: Kind,
    witness: Option<Witness>,
}

struct Entry {
    label: String,
    index: usize,
    fill: usize,
    line: String,
    scan: Scan,
}

struct Blocks {
    outer: Vec<u8>,
    inner: Vec<u8>,
    block: Vec<u8>,
}

impl Blocks {
    fn new(n: usize) -> Blocks {
        Blocks {
            outer: vec![0u8; n * n],
            inner: vec![0u8; n * n],
            block: vec![0u8; n * n],
        }
    }
}

fn split_into(bytes: &[u8], n: usize, shift: (usize, usize), d: usize, buf: &mut Blocks) -> bool {
    let m = n / d;
    let (dr, dc) = shift;
    let outer = &mut buf.outer[..d * d];
    let inner = &mut buf.inner[..m * m];
    let block = &mut buf.block[..m * m];
    outer.fill(0);
    let mut have = false;
    for i in 0..d {
        for j in 0..d {
            let mut live = false;
            for p in 0..m {
                let row = ((i * m + p + dr) % n) * n;
                for q in 0..m {
                    let v = bytes[row + (j * m + q + dc) % n];
                    block[p * m + q] = v;
                    live |= v != 0;
                }
            }
            if !live {
                continue;
            }
            outer[i * d + j] = 1;
            if !have {
                inner.copy_from_slice(block);
                have = true;
            } else if *inner != *block {
                return false;
            }
        }
    }
    have
}

fn one_run(live: &[bool]) -> bool {
    let d = live.len();
    if !live.iter().any(|&v| v) {
        return false;
    }
    (0..d).filter(|&i| live[i] && !live[(i + 1) % d]).count() <= 1
}

fn rect(outer: &[u8], d: usize) -> Option<(usize, usize)> {
    let rows: Vec<bool> = (0..d)
        .map(|i| (0..d).any(|j| outer[i * d + j] != 0))
        .collect();
    let cols: Vec<bool> = (0..d)
        .map(|j| (0..d).any(|i| outer[i * d + j] != 0))
        .collect();
    if !one_run(&rows) || !one_run(&cols) {
        return None;
    }
    let high = rows.iter().filter(|&&v| v).count();
    let wide = cols.iter().filter(|&&v| v).count();
    if outer.iter().filter(|&&v| v != 0).count() == high * wide {
        Some((high, wide))
    } else {
        None
    }
}

fn classify(outer: &[u8], d: usize, foot: usize) -> Kind {
    let fill = outer.iter().filter(|&&v| v != 0).count();
    if fill == 1 {
        Kind::Confined
    } else if fill == d * d {
        Kind::Tiled
    } else if rect(outer, d) == Some((foot, foot)) {
        Kind::Board
    } else {
        Kind::Proper
    }
}

fn footprint(canvas: usize, side: usize, d: usize) -> usize {
    let m = canvas / d;
    let pad = (canvas - side) / 2;
    (pad + side - 1) / m - pad / m + 1
}

fn scan(frame: &Tensor, cuts: &[usize], side: usize) -> Scan {
    let n = frame.shape[0];
    let bytes = frame.bytes();
    let mut buf = Blocks::new(n);
    let mut best = Kind::None;
    let mut in_place = Kind::None;
    let mut witness = None;
    for dr in 0..n {
        for dc in 0..n {
            for &d in cuts {
                let m = n / d;
                if !split_into(bytes, n, (dr, dc), d, &mut buf) {
                    continue;
                }
                let kind = classify(&buf.outer[..d * d], d, footprint(n, side, d));
                if dr == 0 && dc == 0 && kind > in_place {
                    in_place = kind;
                }
                if kind > best {
                    best = kind;
                }
                if kind == Kind::Proper && witness.is_none() {
                    let fill = buf.outer[..d * d].iter().filter(|&&v| v != 0).count();
                    let tile = Tensor::of(buf.inner[..m * m].to_vec(), vec![m, m]);
                    let code = pack(&tile).unwrap_or(0);
                    witness = Some((dr, dc, d, fill, code, tile.sum() as usize));
                }
            }
        }
    }
    Scan {
        best,
        in_place,
        witness,
    }
}

fn witness_line(entry: &Entry) -> String {
    match entry.scan.witness {
        Some((dr, dc, d, outer, code, inner)) => format!(
            "{} fill {} shift ({dr},{dc}) d {d} outer fill {outer} inner tile {code} of {inner} cells",
            entry.label, entry.fill
        ),
        None => format!("{} fill {} none", entry.label, entry.fill),
    }
}

fn report(title: &str, head: &str, entries: &[Entry]) {
    println!("\n{title}");
    println!("  {head}");
    println!("  a cut is confined when the nonzero blocks sit in one block of the cut, a tiling when every block is equal, the board when the outer is the board's own footprint block up to a torus shift, proper otherwise");
    let mut place = [0usize; 5];
    let mut after = [0usize; 5];
    let mut per: BTreeMap<usize, [usize; 3]> = BTreeMap::new();
    let mut first: BTreeMap<usize, String> = BTreeMap::new();
    for entry in entries {
        place[entry.scan.in_place as usize] += 1;
        after[entry.scan.best as usize] += 1;
        let row = per.entry(entry.index).or_default();
        row[0] += 1;
        row[1] += usize::from(entry.scan.in_place == Kind::Proper);
        row[2] += usize::from(entry.scan.best == Kind::Proper);
        if entry.scan.best == Kind::Proper {
            first
                .entry(entry.index)
                .or_insert_with(|| witness_line(entry));
        }
    }
    let show = |what: &str, counts: &[usize; 5]| {
        println!(
            "  {what:<12} no cut {} confined {} tiling {} board {} proper {}",
            counts[0], counts[1], counts[2], counts[3], counts[4]
        );
    };
    show("in place", &place);
    show("after shifts", &after);
    for (index, counts) in &per {
        println!(
            "  index {index}: frames {}, proper in place {}, proper after some shift {}",
            counts[0], counts[1], counts[2]
        );
        if let Some(line) = first.get(index) {
            println!("    first witness {line}");
        }
    }
    let mut rows = 0;
    for entry in entries.iter().filter(|e| e.scan.best == Kind::Proper) {
        rows += 1;
        if rows <= ROWS {
            println!("  proper {} | {}", witness_line(entry), entry.line);
        }
    }
    if rows > ROWS {
        println!("  and {} more rows", rows - ROWS);
    }
}

fn reading_line(r: &Reading) -> String {
    format!(
        "fill {} comp {} holes {} euler {} box {:.2} sym {} ring {} share {:.2} cut {} ent {} churn {:.3}",
        r.fill,
        r.components,
        r.holes,
        r.euler,
        r.box_slope,
        r.symmetry.name,
        r.ring,
        r.share,
        r.ring_cut,
        r.entropy,
        r.churn
    )
}

fn board_of(run: &Run) -> Cell2d {
    create(run.seed.code, run.seed.side, run.seed.level, 0, 2)
        .unwrap()
        .tile(run.tessellation, run.tessellation)
}

fn rows_of(tile: &Tensor) -> String {
    let n = tile.shape[0];
    (0..n)
        .map(|r| {
            (0..n)
                .map(|c| if tile.get(&[r, c]) != 0 { '#' } else { '.' })
                .collect::<String>()
        })
        .collect::<Vec<String>>()
        .join("/")
}

fn cuts_of(canvas: usize) -> Vec<usize> {
    (2..canvas).filter(|d| canvas.is_multiple_of(*d)).collect()
}

fn rules(census: &Census) {
    println!("\nRULES birth = survive counts at budget 8 and 80");
    for rule in &census.preset.rules {
        let at = |budget: usize| rule.birth().values(budget).unwrap();
        println!("  {:<14} {:?} {:?}", rule.name(), at(8), at(80));
    }
}

fn seeds(census: &Census) {
    println!("\nSEEDS the level-1 tile of every seed class, rows top to bottom");
    for seed in &census.preset.seeds {
        let tile = create(seed.code, seed.side, seed.level, 0, 2).unwrap();
        println!(
            "  {}s{}l{} {}",
            seed.code,
            seed.side,
            seed.level,
            rows_of(tile.types())
        );
    }
}

fn budgets(census: &Census) {
    println!("\nBUDGETS the largest neighbour count of every mask cell (seed, tessellation, mask)");
    let mut seen = BTreeSet::new();
    let mut cells = Vec::new();
    for run in &census.runs {
        let key = format!(
            "{}s{}l{} t{} {}",
            run.seed.code,
            run.seed.side,
            run.seed.level,
            run.tessellation,
            run.mask.name()
        );
        if !seen.insert(key.clone()) {
            continue;
        }
        let budget = mask_tensor(&run.mask, &board_of(run)).unwrap().sum() as usize;
        let copy = run.mask.name().starts_with("copy") && run.tessellation == 3;
        cells.push((key, budget, copy));
    }
    for (key, budget, _) in &cells {
        println!("  {key:<22} budget {budget}");
    }
    let small = cells.iter().filter(|(_, b, _)| *b <= 8).count();
    let big: Vec<&(String, usize, bool)> = cells.iter().filter(|(_, b, _)| *b > 8).collect();
    let low = big.iter().map(|(_, b, _)| *b).min().unwrap_or(0);
    let high = big.iter().map(|(_, b, _)| *b).max().unwrap_or(0);
    let copies = big.iter().filter(|(_, _, c)| *c).count();
    println!(
        "mask cells {}: {small} at budget 8 or less, {} above (budgets {low} to {high}), {copies} of those tessellation-3 copy masks",
        cells.len(),
        big.len()
    );
}

fn fates(census: &Census) {
    println!("\nFATES per rule (dead alive loop timeout)");
    let mut family = "";
    let mut subtotal = [0usize; 4];
    let mut total = [0usize; 4];
    let flush = |family: &str, subtotal: &[usize; 4]| {
        if !family.is_empty() {
            println!("  {family:<8} total {subtotal:?}");
        }
    };
    for (fam, rule, counts) in fate_table(census) {
        if fam != family {
            flush(family, &subtotal);
            family = fam;
            subtotal = [0; 4];
        }
        println!("  {fam:<8} {rule:<14} {counts:?}");
        for i in 0..4 {
            subtotal[i] += counts[i];
            total[i] += counts[i];
        }
    }
    flush(family, &subtotal);
    println!("  all      total {total:?}");
    let timeouts = census
        .runs
        .iter()
        .filter(|r| r.fate == Fate::Timeout)
        .count();
    let movers = census.runs.iter().filter(|r| r.mover.is_some()).count();
    println!(
        "timeouts {timeouts} of {} runs, movers among them {movers}",
        census.runs.len()
    );
    for run in census.runs.iter().filter(|r| r.mover.is_some()).take(ROWS) {
        let (dr, dc, lag) = run.mover.unwrap();
        println!("  mover {} shift ({dr},{dc}) lag {lag}", run.label());
    }
    budgets(census);
}

fn closure(census: &Census) {
    let cuts = cuts_of(census.preset.canvas);
    let mut entries = Vec::new();
    for run in &census.runs {
        let Some(frame) = &run.settled else { continue };
        let reading = run.reading.as_ref().unwrap();
        entries.push(Entry {
            label: run.label(),
            index: run.mask_index,
            fill: reading.fill,
            line: reading_line(reading),
            scan: scan(frame.types(), &cuts, board_of(run).types().shape[0]),
        });
    }
    let head = format!(
        "{} settled frames, {} distinct up to the dihedral group and torus translation, every one of the {} torus shifts and both cuts {:?}",
        entries.len(),
        census.distinct,
        census.preset.canvas * census.preset.canvas,
        cuts
    );
    report(
        "CLOSURE the block test on every settled frame",
        &head,
        &entries,
    );
}

fn config(run: &Run, mask: Tensor, canvas: usize, board_side: usize) -> Config {
    Config {
        mask: Cell2d::new(mask),
        birth: run.rule.birth(),
        survive: run.rule.survive(),
        boundary: run.boundary,
        max_generations: 64,
        grid_size: 1,
        padding: (canvas - board_side) / 2,
    }
}

fn replay(run: &Run) -> Vec<Cell2d> {
    let board = board_of(run);
    let side = board.types().shape[0];
    let mask = mask_tensor(&run.mask, &board).unwrap();
    animate(&board, &config(run, mask, 27, side)).unwrap().grids
}

fn heat(census: &Census) {
    let cuts = cuts_of(census.preset.canvas);
    let mut entries = Vec::new();
    for run in &census.runs {
        let frame = heat_frame(&replay(run));
        entries.push(Entry {
            label: run.label(),
            index: run.mask_index,
            fill: run.heat.fill,
            line: reading_line(&run.heat),
            scan: scan(frame.types(), &cuts, board_of(run).types().shape[0]),
        });
    }
    let head = format!(
        "{} heat frames, every one of the {} torus shifts and both cuts {:?}",
        entries.len(),
        census.preset.canvas * census.preset.canvas,
        cuts
    );
    report(
        "HEAT the block test on the median level set of every run's visit counts",
        &head,
        &entries,
    );
}

fn lemma(census: &Census) {
    println!("\nLEMMA a mask on the sublattice 3Z^2 with 0 outside birth keeps every frame a Kronecker product A(t) (x) seed, A(t) the same rule on the 9-torus under mask/3");
    let mut checked = 0;
    let mut rules = BTreeSet::new();
    let mut masks = BTreeSet::new();
    for run in &census.runs {
        let board = board_of(run);
        let mask = mask_tensor(&run.mask, &board).unwrap();
        let offsets = mask_offsets(&mask);
        if offsets.is_empty() || offsets.iter().any(|o| o[0] % 3 != 0 || o[1] % 3 != 0) {
            continue;
        }
        let budget = mask.sum() as usize;
        if run.rule.birth().values(budget).unwrap().contains(&0) || run.tessellation != 3 {
            continue;
        }
        let mut small = Tensor::new(vec![3, 3]);
        for o in &offsets {
            small.set(&[(1 + o[0] / 3) as usize, (1 + o[1] / 3) as usize], 1);
        }
        let seed = create(run.seed.code, run.seed.side, run.seed.level, 0, 2).unwrap();
        let outer = Cell2d::new(Tensor::full(vec![3, 3], 1));
        let quotient = animate(&outer, &config(run, small, 9, 3)).unwrap();
        let full = animate(&board, &config(run, mask, 27, 9)).unwrap();
        assert_eq!(full.fate, quotient.fate, "{} fates differ", run.label());
        assert_eq!(
            full.grids.len(),
            quotient.grids.len(),
            "{} lengths differ",
            run.label()
        );
        for (a, x) in quotient.grids.iter().zip(&full.grids) {
            assert_eq!(
                a.types().kron(seed.types()).bytes(),
                x.types().bytes(),
                "{} frame differs",
                run.label()
            );
        }
        checked += 1;
        rules.insert(run.rule.name());
        masks.insert(format!(
            "{}s{} {} budget {budget}",
            run.seed.code,
            run.seed.side,
            run.mask.name()
        ));
    }
    println!(
        "verified frame by frame on {checked} runs, {} rules, masks {:?}",
        rules.len(),
        masks
    );
}

fn stills(census: &Census) {
    println!("\nSTILLS tessellated seeds (t >= 3) reaching a still: peak ring against the seed comb and the mask lobe");
    let canvas = census.preset.canvas;
    let mut rows = 0;
    let mut comb_hits = 0;
    let mut lobe_hits = 0;
    let mut ring_stills = 0;
    for run in census
        .runs
        .iter()
        .filter(|r| r.tessellation >= 3 && r.fate == Fate::Alive)
    {
        let reading = run.reading.as_ref().unwrap();
        let comb = canvas / run.seed.side;
        let mask = mask_tensor(&run.mask, &board_of(run)).unwrap();
        let lobe = first_negative_lobe(&mask, canvas);
        let on_comb = reading.ring > 0 && reading.ring.is_multiple_of(comb);
        let in_lobe = lobe > 0 && reading.ring >= lobe;
        let ring_still = reading.share >= 0.5;
        comb_hits += usize::from(on_comb);
        lobe_hits += usize::from(in_lobe);
        ring_stills += usize::from(ring_still);
        rows += 1;
        if rows <= ROWS {
            println!(
                "  {} at {} ring {} share {:.2} comb {comb} {} lobe {lobe} {} | fill {} sym {} factors {:?}",
                run.label(),
                run.settled_at,
                reading.ring,
                reading.share,
                if on_comb { "hit" } else { "miss" },
                if in_lobe { "in" } else { "out" },
                reading.fill,
                reading.symmetry.name,
                reading.factors
            );
        }
    }
    if rows > ROWS {
        println!("  and {} more rows", rows - ROWS);
    }
    println!("stills at t >= 3: {rows}, peak on a comb harmonic {comb_hits}, peak at or past the mask lobe {lobe_hits}, peak share at least one half {ring_stills}");
}

fn self_check() {
    let mut t = Tensor::new(vec![5, 5]);
    t.set(&[1, 2], 1);
    t.set(&[2, 2], 1);
    t.set(&[3, 2], 1);
    let config = Config {
        boundary: Boundary::Constant,
        max_generations: 16,
        ..Config::new(moore(), vec![3], vec![2, 3])
    };
    let life = animate(&Cell2d::new(t), &config).unwrap();
    assert_eq!(
        (life.fate, life.loop_length),
        (Fate::Loop, 2),
        "the blinker must loop with period 2"
    );
    let frame = carpet(3, 2).unwrap();
    assert_eq!(
        factors(frame.types()),
        Some((3, 495, 495)),
        "the level-2 carpet must factor at 3"
    );
    let (outer, inner) = block_split(frame.types(), 3).unwrap();
    let mut buf = Blocks::new(9);
    assert!(
        split_into(frame.types().bytes(), 9, (0, 0), 3, &mut buf),
        "the shift split must find the carpet cut"
    );
    assert_eq!(
        (&buf.outer[..9], &buf.inner[..9]),
        (outer.bytes(), inner.bytes()),
        "the shift split must match block_split"
    );
    println!("\nSELF-CHECK blinker period 2 under b3s23, carpet level 2 factors as (3, 495, 495), the shift split agrees with block_split at shift (0,0)");
}

fn main() {
    let preset = mini();
    for line in preset.describe() {
        println!("{line}");
    }
    let census = census(&preset, CAP, SAMPLE_SEED).unwrap();
    println!(
        "cut: canvas {} generations {} sweep {} after {} duplicate-mask runs dropped, ran {}{}",
        preset.canvas,
        preset.generations,
        census.universe,
        census.duplicates,
        census.runs.len(),
        match census.sample {
            Some(seed) => format!(", a sample drawn with seed {seed}"),
            None => String::new(),
        }
    );
    seeds(&census);
    rules(&census);
    fates(&census);
    closure(&census);
    heat(&census);
    lemma(&census);
    stills(&census);
    self_check();
}
