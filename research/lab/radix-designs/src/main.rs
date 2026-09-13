use mrlymath::bang::factory::create;
use mrlynum::gauss::Ring;
use mrlynum::radix::{flowsnake, gasket, koch, terdragon, tile, twindragon, Base, Radix};
use std::collections::{HashMap, HashSet};
use std::time::Instant;

type Point = (f64, f64);
type Map = (Point, Point);

const BUDGET: usize = 200_000;
const COMPARE: usize = 20_000;

fn ring_word(ring: Ring) -> &'static str {
    match ring {
        Ring::Gaussian => "Z[i]",
        Ring::Eisenstein => "Z[omega]",
    }
}

fn spell(ring: Ring, (a, b): (i64, i64)) -> String {
    let unit = match ring {
        Ring::Gaussian => "i",
        Ring::Eisenstein => "w",
    };
    match (a, b) {
        (a, 0) => format!("{a}"),
        (0, 1) => unit.to_string(),
        (0, -1) => format!("-{unit}"),
        (0, b) => format!("{b}{unit}"),
        (a, 1) => format!("{a}+{unit}"),
        (a, -1) => format!("{a}-{unit}"),
        (a, b) if b > 0 => format!("{a}+{b}{unit}"),
        (a, b) => format!("{a}{b}{unit}"),
    }
}

fn complex_mul(z: (f64, f64), w: (f64, f64)) -> (f64, f64) {
    (z.0 * w.0 - z.1 * w.1, z.0 * w.1 + z.1 * w.0)
}

fn complex_div(z: (f64, f64), w: (f64, f64)) -> (f64, f64) {
    let den = w.0 * w.0 + w.1 * w.1;
    ((z.0 * w.0 + z.1 * w.1) / den, (z.1 * w.0 - z.0 * w.1) / den)
}

// IFS

fn iterate(maps: &[Map], level: usize) -> Vec<(f64, f64)> {
    let mut out = vec![(0.0f64, 0.0f64)];
    for _ in 0..level {
        let mut next = Vec::with_capacity(out.len() * maps.len());
        for &(scale, shift) in maps {
            for &p in &out {
                let t = complex_mul(scale, p);
                next.push((t.0 + shift.0, t.1 + shift.1));
            }
        }
        out = next;
    }
    out
}

fn koch_maps() -> Vec<Map> {
    let root = 3f64.sqrt();
    vec![
        ((1.0 / 3.0, 0.0), (0.0, 0.0)),
        ((1.0 / 6.0, root / 6.0), (1.0 / 3.0, 0.0)),
        ((1.0 / 6.0, -root / 6.0), (0.5, root / 6.0)),
        ((1.0 / 3.0, 0.0), (2.0 / 3.0, 0.0)),
    ]
}

fn gasket_maps() -> Vec<Map> {
    let root = 3f64.sqrt();
    let corners = [(1.0, 1.0), (3.0, 1.0), (2.0, 1.0 + root)];
    corners
        .iter()
        .map(|&(x, y)| ((0.5, 0.0), (x / 2.0, y / 2.0)))
        .collect()
}

fn twindragon_maps() -> Vec<Map> {
    let scale = complex_div((1.0, 0.0), (1.0, 1.0));
    vec![(scale, (0.0, 0.0)), (scale, scale)]
}

fn tile_maps(base: Base) -> Vec<Map> {
    let ring = base.ring();
    let b = ring.place(base.value().0, base.value().1);
    let scale = complex_div((1.0, 0.0), b);
    base.residues()
        .into_iter()
        .map(|d| {
            let place = ring.place(d.0, d.1);
            (scale, complex_div(place, b))
        })
        .collect()
}

fn pin(got: &[(f64, f64)], want: &[(f64, f64)]) -> (f64, (f64, f64), f64) {
    let last = got.len() - 1;
    let dp = (got[last].0 - got[0].0, got[last].1 - got[0].1);
    let dt = (want[last].0 - want[0].0, want[last].1 - want[0].1);
    let den = dp.0 * dp.0 + dp.1 * dp.1;
    let scale = (dt.0 * dp.0 + dt.1 * dp.1) / den;
    let turn = (dt.1 * dp.0 - dt.0 * dp.1) / den;
    let shift = (want[0].0 - scale * got[0].0, want[0].1 - scale * got[0].1);
    (scale, shift, turn)
}

fn deviation(got: &[(f64, f64)], want: &[(f64, f64)], scale: f64, shift: (f64, f64)) -> f64 {
    got.iter()
        .zip(want.iter())
        .map(|(a, b)| {
            let x = scale * a.0 + shift.0 - b.0;
            let y = scale * a.1 + shift.1 - b.1;
            (x * x + y * y).sqrt()
        })
        .fold(0.0f64, f64::max)
}

fn top_level(size: usize, budget: usize) -> usize {
    let mut top = 1;
    while size.pow(top as u32 + 1) <= budget {
        top += 1;
    }
    top
}

fn face_off(name: &str, design: &Radix, maps: &[Map], free: bool, note: &str) {
    let top = top_level(design.size(), COMPARE);
    let got = design.plane(top);
    let want = iterate(maps, top);
    assert_eq!(got.len(), want.len(), "{name} point counts differ");
    let (scale, shift, turn) = if free {
        pin(&got, &want)
    } else {
        (1.0, (0.0, 0.0), 0.0)
    };
    let worst = deviation(&got, &want, scale, shift);
    let motion = if free {
        format!(
            "scale {scale:.6} shift ({:.6}, {:.6}) turn {turn:.1e}",
            shift.0, shift.1
        )
    } else {
        "none".to_string()
    };
    println!(
        "{name}  level {top}  points {}  worst {worst:.3e}",
        got.len()
    );
    println!("  motion allowed: {motion}");
    println!("  {note}");
}

// TERDRAGON

fn lsystem(level: usize) -> Vec<u8> {
    let mut out = b"F".to_vec();
    for _ in 0..level {
        let mut next = Vec::with_capacity(out.len() * 5);
        for &c in &out {
            if c == b'F' {
                next.extend_from_slice(b"F+F-F");
            } else {
                next.push(c);
            }
        }
        out = next;
    }
    out
}

fn turtle(level: usize) -> Vec<(f64, f64)> {
    let root = 3f64.sqrt();
    let step = [(1.0, 0.0), (-0.5, root / 2.0), (-0.5, -root / 2.0)];
    let mut heading = 0usize;
    let mut at = (0.0f64, 0.0f64);
    let mut starts = Vec::new();
    for c in lsystem(level) {
        match c {
            b'F' => {
                starts.push(at);
                at = (at.0 + step[heading].0, at.1 + step[heading].1);
            }
            b'+' => heading = (heading + 1) % 3,
            _ => heading = (heading + 2) % 3,
        }
    }
    starts.into_iter().map(|p| complex_div(p, at)).collect()
}

fn terdragon_reading(name: &str, design: &Radix, level: usize) -> f64 {
    let got = design.plane(level);
    let want = turtle(level);
    assert_eq!(got.len(), want.len(), "{name} point counts differ");
    let worst = deviation(&got, &want, 1.0, (0.0, 0.0));
    println!(
        "{name}  twists {}  level {level}  points {}  worst {worst:.3e}",
        design
            .twists()
            .iter()
            .map(|&u| spell(design.ring(), u))
            .collect::<Vec<_>>()
            .join(" "),
        got.len()
    );
    worst
}

fn run_compare() {
    println!("COMPARE  each design against an independent f64 iterated function system, seed 0");
    face_off(
        "gasket    ",
        &gasket(),
        &gasket_maps(),
        true,
        "independent: the three ratio 1/2 similarities fixing an equilateral triangle placed at (1,1), (3,1), (2,1+sqrt 3); a translation and a positive scaling are allowed because that statement fixes the gasket only up to similarity, and the turn residual printed above is the check that no rotation was needed",
    );
    face_off(
        "koch      ",
        &koch(),
        &koch_maps(),
        false,
        "independent in f64 only: z/3, e^(i pi/3) z/3 + 1/3, e^(-i pi/3) z/3 + 1/2 + i sqrt(3)/6, z/3 + 2/3 are the crate maps coefficient for coefficient, so this is a float self-check of exact ring arithmetic and no motion is allowed",
    );
    face_off(
        "twindragon",
        &twindragon(),
        &twindragon_maps(),
        false,
        "self-check by definition: z/(1+i) and (z+1)/(1+i) are the radix maps of base 1+i on its two residues, so no motion is allowed and the number is f64 round-off",
    );
    let seven = flowsnake();
    face_off(
        "tile7     ",
        &seven,
        &tile_maps(seven.base()),
        false,
        "self-check by definition: the seven maps (z + d)/(3+w) over the canonical residues are the radix maps of that base, so no motion is allowed; the flowsnake name is not tested here",
    );
    println!("  terdragon against the L-system F -> F + F - F at 120 degrees, three segments, unsourced reading");
    let level = 8;
    let twisted = terdragon_reading("  twisted  ", &terdragon(), level);
    let plain = terdragon_reading("  untwisted", &terdragon().with_twists(&[0, 0, 0]), level);
    println!(
        "  verdict: the twisted reading {} and the untwisted code 7 {}",
        if twisted < 1e-9 { "matches" } else { "misses" },
        if plain < 1e-9 { "matches" } else { "misses" }
    );
    assert!(twisted < 1e-9, "the twisted reading misses by {twisted}");
    assert!(plain > 1e-3, "the untwisted code 7 matches after all");
}

// KOCH

fn run_koch() {
    let design = koch();
    println!(
        "KOCH  base {} on {}",
        spell(design.ring(), design.base().value()),
        ring_word(design.ring())
    );
    println!(
        "  digits {}  twists {}",
        design
            .digits()
            .iter()
            .map(|&d| spell(design.ring(), d))
            .collect::<Vec<_>>()
            .join(" "),
        design
            .twists()
            .iter()
            .map(|&u| spell(design.ring(), u))
            .collect::<Vec<_>>()
            .join(" ")
    );
    println!(
        "  canonical digits {}  class code {}",
        design.canonical(),
        design.code()
    );
    println!("  the four maps are phi_d coefficient for coefficient, so the column below is a float self-check");
    println!("  level  words  fill  distinct  worst");
    let maps = koch_maps();
    for level in 1..=5 {
        let got = design.plane(level);
        let want = iterate(&maps, level);
        let worst = deviation(&got, &want, 1.0, (0.0, 0.0));
        println!(
            "  {level:>5}  {:>5}  {:>4}  {:>8}  {worst:.3e}",
            got.len(),
            design.fill(level),
            design.distinct(level)
        );
        assert_eq!(got.len() as u128, design.fill(level));
        assert_eq!(design.fill(level), 4u128.pow(level as u32));
        assert!(worst < 1e-9, "level {level} misses by {worst}");
    }
}

// NAMED

fn report(name: &str, design: &Radix) {
    let ring = design.ring();
    let q = design.base().norm();
    let size = design.size();
    let top = top_level(size, BUDGET);
    let counts: Vec<String> = (1..=top)
        .map(|level| design.distinct(level).to_string())
        .collect();
    let pairwise = (1..=top).all(|level| design.distinct(level) as u128 == design.fill(level));
    println!(
        "{name}  {}  base {}  q {q}  code {}  |F| {size}  fill |F|^L  dim {:.6}",
        ring_word(ring),
        spell(ring, design.base().value()),
        design.code(),
        design.dimension()
    );
    println!(
        "  digits {}",
        design
            .digits()
            .iter()
            .map(|&d| spell(ring, d))
            .collect::<Vec<_>>()
            .join(" ")
    );
    println!("  distinct levels 1..{top}: {}", counts.join(" "));
    println!("  pairwise distinct {pairwise}");
}

fn run_named() {
    println!("NAMED  codes only; the names are tested by the verb compare and nowhere here");
    report("code 7 at 2        ", &gasket());
    report("code 3 at 1+i      ", &twindragon());
    report("code 7 at 2+w      ", &terdragon().with_twists(&[0, 0, 0]));
    report("code 127 at 3+w    ", &flowsnake());
    report("code 147 at 3      ", &koch());
    let glue = Radix::from_code(Base::new(Ring::Gaussian, (2, 0)), 3).with_twists(&[0, 2]);
    println!(
        "twist glue  Z[i]  base 2  q 4  code 3  |F| 2  twists 1 -1  dim {:.6}",
        glue.dimension()
    );
    println!("  words scaled by b^2: {:?}", glue.words(2));
    let top = 16;
    println!(
        "  fill levels 1..{top}:     {}",
        (1..=top)
            .map(|l| glue.fill(l).to_string())
            .collect::<Vec<_>>()
            .join(" ")
    );
    let counts: Vec<String> = (1..=top)
        .map(|level| glue.distinct(level).to_string())
        .collect();
    println!("  distinct levels 1..{top}: {}", counts.join(" "));
    for level in 1..=top {
        assert_eq!(
            glue.distinct(level),
            (1usize << (level - 1)) + 1,
            "the distinct count leaves 2^(L-1) + 1 at level {level}"
        );
    }
    println!("  distinct equals 2^(L-1) + 1 at every level to {top}");
}

// TODAY

fn run_today() {
    let level = 2;
    let mut checked = 0usize;
    let mut missed = 0usize;
    let mut first: Option<String> = None;
    for m in [2u64, 3u64] {
        let cells = (m * m) as usize;
        for code in 0..(1u128 << cells) {
            let design = tile(m, code);
            let got: HashSet<(i64, i64)> = design.words(level).into_iter().collect();
            let tensor = create(code, m as usize, 2, m as usize, level).unwrap();
            let side = m.pow(level as u32) as usize;
            let mut want = HashSet::new();
            for row in 0..side {
                for col in 0..side {
                    if tensor.bytes()[row * side + col] == 1 {
                        want.insert((col as i64, row as i64));
                    }
                }
            }
            checked += 1;
            assert_eq!(design.words(level).len() as u128, design.fill(level));
            if got != want {
                missed += 1;
                if first.is_none() {
                    first = Some(format!("m {m} code {code}"));
                }
            }
        }
    }
    println!("TODAY  level {level}, bases 2 and 3, every plane code");
    println!("  codes checked {checked}  mismatches {missed}");
    println!(
        "  the code is read in box row-major order, bit r m + c, not in canonical residue order"
    );
    for m in [2u64, 3u64] {
        let full = tile(m, (1u128 << (m * m)) - 1);
        let base = Base::new(Ring::Gaussian, (m as i64, 0));
        println!(
            "  m {m}: box residues are the canonical system {}",
            full.canonical()
        );
        println!(
            "    box {}",
            full.digits()
                .iter()
                .map(|&d| spell(Ring::Gaussian, d))
                .collect::<Vec<_>>()
                .join(" ")
        );
        println!(
            "    canonical {}",
            base.residues()
                .iter()
                .map(|&z| spell(Ring::Gaussian, z))
                .collect::<Vec<_>>()
                .join(" ")
        );
    }
    match first {
        None => println!("  first mismatch none"),
        Some(where_) => println!("  first mismatch {where_}"),
    }
}

// CENSUS

fn cycles(map: &[usize]) -> u32 {
    let mut seen = vec![false; map.len()];
    let mut count = 0;
    for start in 0..map.len() {
        if seen[start] {
            continue;
        }
        count += 1;
        let mut at = start;
        while !seen[at] {
            seen[at] = true;
            at = map[at];
        }
    }
    count
}

fn burnside(group: &[Vec<usize>]) -> u128 {
    let total: u128 = group.iter().map(|map| 1u128 << cycles(map)).sum();
    assert_eq!(total % group.len() as u128, 0, "Burnside is not an integer");
    total / group.len() as u128
}

fn image(group: &[Vec<usize>]) -> usize {
    group.iter().collect::<HashSet<_>>().len()
}

fn act(map: &[usize], code: usize) -> usize {
    let mut out = 0usize;
    for (i, &j) in map.iter().enumerate() {
        if (code >> i) & 1 == 1 {
            out |= 1 << j;
        }
    }
    out
}

fn orbit(group: &[Vec<usize>], code: usize) -> Vec<usize> {
    let mut seen = HashSet::new();
    let mut stack = vec![code];
    seen.insert(code);
    while let Some(at) = stack.pop() {
        for map in group {
            let next = act(map, at);
            if seen.insert(next) {
                stack.push(next);
            }
        }
    }
    let mut out: Vec<usize> = seen.into_iter().collect();
    out.sort_unstable();
    out
}

fn walk(group: &[Vec<usize>], q: usize) -> u128 {
    let mut seen = vec![false; 1usize << q];
    let mut orbits = 0u128;
    for code in 0..(1usize << q) {
        if seen[code] {
            continue;
        }
        orbits += 1;
        for member in orbit(group, code) {
            seen[member] = true;
        }
    }
    orbits
}

fn run_census() {
    let bases = [
        (Ring::Gaussian, (2i64, 0i64)),
        (Ring::Gaussian, (1, 1)),
        (Ring::Gaussian, (2, 1)),
        (Ring::Eisenstein, (2, 0)),
        (Ring::Eisenstein, (2, 1)),
        (Ring::Eisenstein, (3, 0)),
        (Ring::Eisenstein, (3, 1)),
    ];
    println!(
        "CENSUS  classes of digit CODES under the residue action, never designs up to similarity"
    );
    println!("  ring  base  q  abstract  image  mirror  codes  classes  walk");
    for (ring, value) in bases {
        let base = Base::new(ring, value);
        let q = base.norm() as usize;
        let group = base.group();
        let count = burnside(&group);
        let seen = walk(&group, q);
        assert_eq!(count, seen, "Burnside and the orbit walk disagree");
        println!(
            "  {:>9}  {:>4}  {q}  {:>8}  {:>5}  {:>6}  {:>5}  {count:>7}  {seen:>4}",
            ring_word(ring),
            spell(ring, value),
            group.len(),
            image(&group),
            base.mirrored(),
            1u128 << q
        );
        println!(
            "    residues {}",
            base.residues()
                .iter()
                .map(|&z| spell(ring, z))
                .collect::<Vec<_>>()
                .join(" ")
        );
    }
    println!("  abstract is the order of R^* semidirect <conj>, image the order it acts through on the residues");
    let base = Base::new(Ring::Eisenstein, (3, 0));
    let q = base.norm() as usize;
    let twisted: u128 = (0..1u128 << q)
        .map(|code| 6u128.pow(code.count_ones()))
        .sum();
    println!(
        "  base 3 on Z[omega]: {} codes in {} classes of codes; over the codes, not the classes, the twist vectors number sum_k binom(9,k) 6^k = {twisted} = 7^{q}",
        1u128 << q,
        burnside(&base.group())
    );
    assert_eq!(twisted, 7u128.pow(q as u32));
    let mut sizes: HashMap<usize, usize> = HashMap::new();
    for code in 0..1u128 << q {
        *sizes.entry(code.count_ones() as usize).or_default() += 1;
    }
    let mut keys: Vec<usize> = sizes.keys().copied().collect();
    keys.sort_unstable();
    println!(
        "  codes by |F|: {}",
        keys.iter()
            .map(|k| format!("{k}:{}", sizes[k]))
            .collect::<Vec<_>>()
            .join(" ")
    );
}

// AFFINE

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Qw {
    p: i128,
    q: i128,
    r: i128,
}

fn gcd(a: i128, b: i128) -> i128 {
    if b == 0 {
        a.abs().max(1)
    } else {
        gcd(b, a % b)
    }
}

impl Qw {
    fn new(p: i128, q: i128, r: i128) -> Qw {
        assert!(r != 0, "a rational needs a nonzero denominator");
        let (p, q, r) = if r < 0 { (-p, -q, -r) } else { (p, q, r) };
        if p == 0 && q == 0 {
            return Qw { p: 0, q: 0, r: 1 };
        }
        let g = gcd(gcd(p, q), r);
        Qw {
            p: p / g,
            q: q / g,
            r: r / g,
        }
    }
    fn whole(p: i64, q: i64) -> Qw {
        Qw::new(p as i128, q as i128, 1)
    }
    fn sub(self, other: Qw) -> Qw {
        Qw::new(
            self.p * other.r - other.p * self.r,
            self.q * other.r - other.q * self.r,
            self.r * other.r,
        )
    }
    fn add(self, other: Qw) -> Qw {
        Qw::new(
            self.p * other.r + other.p * self.r,
            self.q * other.r + other.q * self.r,
            self.r * other.r,
        )
    }
    fn mul(self, other: Qw) -> Qw {
        Qw::new(
            self.p * other.p - self.q * other.q,
            self.p * other.q + self.q * other.p - self.q * other.q,
            self.r * other.r,
        )
    }
    fn conjugate(self) -> Qw {
        Qw::new(self.p - self.q, -self.q, self.r)
    }
    fn div(self, other: Qw) -> Qw {
        let n = other.p * other.p - other.p * other.q + other.q * other.q;
        assert!(n != 0, "no division by zero");
        self.mul(Qw::new(
            other.r * (other.p - other.q),
            -other.r * other.q,
            n,
        ))
    }
}

fn permutations(n: usize) -> Vec<Vec<usize>> {
    let mut out = vec![vec![]];
    for _ in 0..n {
        let mut next = Vec::new();
        for partial in &out {
            for value in 0..n {
                if !partial.contains(&value) {
                    let mut grown = partial.clone();
                    grown.push(value);
                    next.push(grown);
                }
            }
        }
        out = next;
    }
    out
}

fn direct(left: &[Qw], right: &[Qw], perms: &[Vec<usize>]) -> bool {
    let span = left[1].sub(left[0]);
    perms.iter().any(|perm| {
        let v = right[perm[1]].sub(right[perm[0]]).div(span);
        let t = right[perm[0]].sub(v.mul(left[0]));
        left.iter()
            .zip(perm.iter())
            .all(|(&d, &j)| v.mul(d).add(t) == right[j])
    })
}

fn conjugate_sets(left: &[Qw], right: &[Qw], perms: &[Vec<usize>]) -> bool {
    let mirror: Vec<Qw> = left.iter().map(|d| d.conjugate()).collect();
    direct(left, right, perms) || direct(&mirror, right, perms)
}

fn digits_of(residues: &[(i64, i64)], code: usize) -> Vec<Qw> {
    residues
        .iter()
        .enumerate()
        .filter(|(i, _)| (code >> i) & 1 == 1)
        .map(|(_, &z)| Qw::whole(z.0, z.1))
        .collect()
}

fn affine_classes(residues: &[(i64, i64)], codes: &[usize]) -> Vec<usize> {
    let sets: Vec<Vec<Qw>> = codes.iter().map(|&c| digits_of(residues, c)).collect();
    let perms = permutations(sets[0].len());
    let mut label = vec![usize::MAX; codes.len()];
    let mut classes = 0;
    for i in 0..codes.len() {
        if label[i] != usize::MAX {
            continue;
        }
        label[i] = classes;
        for j in i + 1..codes.len() {
            if label[j] == usize::MAX && conjugate_sets(&sets[i], &sets[j], &perms) {
                label[j] = classes;
            }
        }
        classes += 1;
    }
    label
}

fn run_affine() {
    let base = Base::new(Ring::Eisenstein, (3, 0));
    let residues = base.residues();
    let group = base.group();
    println!("AFFINE  base 3 on Z[omega], untwisted canonical designs, exact arithmetic over Q(w)");
    println!("  the base is real, so a conjugacy h(x) = v x + s or h(x) = v conj(x) + s carries a design to a design and sends the digit set F to v F + s (b - 1) or to v conj(F) + s (b - 1); the classes below are the classes of F under x -> v x + t and x -> v conj(x) + t with v in Q(w)^* and t in Q(w)");
    for size in [2usize, 3] {
        let codes: Vec<usize> = (0..1usize << residues.len())
            .filter(|c| c.count_ones() as usize == size)
            .collect();
        let mut census: HashMap<usize, usize> = HashMap::new();
        let mut orbits = 0usize;
        for &code in &codes {
            if census.contains_key(&code) {
                continue;
            }
            for member in orbit(&group, code) {
                census.insert(member, orbits);
            }
            orbits += 1;
        }
        let classes = affine_classes(&residues, &codes);
        let total = classes.iter().copied().max().unwrap() + 1;
        println!(
            "  |F| {size}: codes {}, census orbits {orbits}, affine classes {total}",
            codes.len()
        );
        let mut split = None;
        let mut merge = None;
        for (i, &a) in codes.iter().enumerate() {
            for (j, &b) in codes.iter().enumerate().skip(i + 1) {
                let same_orbit = census[&a] == census[&b];
                let same_class = classes[i] == classes[j];
                if same_orbit && !same_class && split.is_none() {
                    split = Some((a, b));
                }
                if !same_orbit && same_class && merge.is_none() {
                    merge = Some((a, b));
                }
            }
        }
        let spell_code = |code: usize| {
            format!(
                "{code} ({})",
                residues
                    .iter()
                    .enumerate()
                    .filter(|(i, _)| (code >> i) & 1 == 1)
                    .map(|(_, &z)| spell(Ring::Eisenstein, z))
                    .collect::<Vec<_>>()
                    .join(", ")
            )
        };
        match split {
            Some((a, b)) => println!(
                "    split: codes {} and {} share a census orbit and are not affinely conjugate",
                spell_code(a),
                spell_code(b)
            ),
            None => println!("    split: none"),
        }
        match merge {
            Some((a, b)) => println!(
                "    merge: codes {} and {} are affinely conjugate and sit in different census orbits",
                spell_code(a),
                spell_code(b)
            ),
            None => println!("    merge: none"),
        }
        if size == 3 {
            assert_eq!(codes.len(), 84);
            assert_eq!(orbits, 13);
            assert_eq!(total, 9);
            let at = |code: usize| codes.iter().position(|&c| c == code).unwrap();
            println!(
                "    code 131 and code 137: same orbit {}, same affine class {}",
                census[&131] == census[&137],
                classes[at(131)] == classes[at(137)]
            );
            println!(
                "    code 7 and code 42: same orbit {}, same affine class {}",
                census[&7] == census[&42],
                classes[at(7)] == classes[at(42)]
            );
        } else {
            assert_eq!(codes.len(), 36);
            assert_eq!(orbits, 7);
            assert_eq!(total, 1);
        }
    }
}

fn main() {
    let verbs: Vec<String> = std::env::args().skip(1).collect();
    let verbs = if verbs.is_empty() {
        ["koch", "compare", "named", "today", "census", "affine"]
            .iter()
            .map(|s| s.to_string())
            .collect()
    } else {
        verbs
    };
    for verb in verbs {
        let clock = Instant::now();
        match verb.as_str() {
            "koch" => run_koch(),
            "compare" => run_compare(),
            "named" => run_named(),
            "today" => run_today(),
            "census" => run_census(),
            "affine" => run_affine(),
            other => panic!("unknown verb {other}"),
        }
        println!("  {verb} {:.2} s", clock.elapsed().as_secs_f64());
    }
}
