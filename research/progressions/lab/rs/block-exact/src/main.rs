use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::time::Instant;

type Mask = u128;

const THREADS: usize = 8;
const KEEP: usize = 4096;

// THE GRID

struct Grid {
    m: usize,
    n: usize,
    mid: Vec<u8>,
    sub: Vec<u8>,
    line_pts: Vec<u8>,
    line_mask: Vec<Mask>,
    mats: Vec<Vec<u8>>,
    class_pts: Vec<u8>,
    maxpeel: Vec<u8>,
}

fn bits(mask: Mask, out: &mut [u8; 128]) -> usize {
    let mut k = 0;
    let mut rest = mask;
    while rest != 0 {
        out[k] = rest.trailing_zeros() as u8;
        rest &= rest - 1;
        k += 1;
    }
    k
}

fn peelable_1d(m: usize, set: usize) -> bool {
    let inv2 = (m + 1) / 2;
    let mut live = set;
    while live != 0 {
        let mut mids = 0usize;
        for x in 0..m {
            for z in x + 1..m {
                if live >> x & 1 == 1 && live >> z & 1 == 1 {
                    mids |= 1 << ((x + z) * inv2 % m);
                }
            }
        }
        if live & !mids == 0 {
            return false;
        }
        live &= mids;
    }
    true
}

impl Grid {
    fn new(m: usize) -> Grid {
        assert!(
            m % 2 == 1 && m >= 3 && (2..m).all(|d| m % d != 0),
            "m must be an odd prime"
        );
        let n = m * m;
        let inv2 = (m + 1) / 2;
        let at = |a: usize, b: usize| (a % m) * m + (b % m);
        let coord = |i: usize| (i / m, i % m);
        let mut mid = vec![0u8; n * n];
        let mut sub = vec![0u8; n * n];
        for x in 0..n {
            for z in 0..n {
                let (xa, xb) = coord(x);
                let (za, zb) = coord(z);
                mid[x * n + z] = at((xa + za) * inv2, (xb + zb) * inv2) as u8;
                sub[x * n + z] = at(xa + m - za, xb + m - zb) as u8;
            }
        }
        let mut line_pts = vec![0u8; n * n * m];
        let mut line_mask = vec![0 as Mask; n * n];
        for t in 0..n {
            for u in 0..n {
                if t == u {
                    continue;
                }
                let (ta, tb) = coord(t);
                let (da, db) = coord(sub[u * n + t] as usize);
                for c in 0..m {
                    let p = at(ta + c * da, tb + c * db);
                    line_pts[(t * n + u) * m + c] = p as u8;
                    line_mask[t * n + u] |= 1 << p;
                }
            }
        }
        let mut mats = vec![Vec::new(); n];
        for p in 0..m {
            for q in 0..m {
                for r in 0..m {
                    for s in 0..m {
                        if (p * s + m * m - q * r) % m == 0 {
                            continue;
                        }
                        for d in 1..n {
                            let (da, db) = coord(d);
                            if (p * da + q * db) % m == 0 && (r * da + s * db) % m == 1 {
                                for v in 0..n {
                                    let (a, b) = coord(v);
                                    mats[d].push(at(p * a + q * b, r * a + s * b) as u8);
                                }
                            }
                        }
                    }
                }
            }
        }
        for d in 1..n {
            assert_eq!(mats[d].len(), (n - m) * n);
        }
        let mut class_pts = vec![0u8; (m + 1) * m * m];
        let dirs: Vec<(usize, usize)> = std::iter::once((0, 1))
            .chain((0..m).map(|j| (1, j)))
            .collect();
        for (ci, &(da, db)) in dirs.iter().enumerate() {
            for l in 0..m {
                let (ba, bb) = if da == 0 { (l, 0) } else { (0, l) };
                for c in 0..m {
                    class_pts[(ci * m + l) * m + c] = at(ba + c * da, bb + c * db) as u8;
                }
            }
        }
        let full = 1usize << m;
        let mut maxpeel = vec![0u8; full * full];
        for p in 0..full {
            if !peelable_1d(m, p) {
                continue;
            }
            let k = p.count_ones() as u8;
            let comp = !p & (full - 1);
            let mut f = p;
            loop {
                let mut s = comp;
                loop {
                    let e = &mut maxpeel[f * full + (p | s)];
                    if *e < k {
                        *e = k;
                    }
                    if s == 0 {
                        break;
                    }
                    s = (s - 1) & comp;
                }
                if f == 0 {
                    break;
                }
                f = (f - 1) & p;
            }
        }
        Grid {
            m,
            n,
            mid,
            sub,
            line_pts,
            line_mask,
            mats,
            class_pts,
            maxpeel,
        }
    }

    fn group(&self) -> u128 {
        (self.n * (self.n - 1) * (self.n - self.m)) as u128
    }

    fn peelable(&self, set: Mask) -> bool {
        let mut pts = [0u8; 128];
        let mut live = set;
        while live != 0 {
            let k = bits(live, &mut pts);
            let mut mids: Mask = 0;
            for i in 0..k {
                let row = pts[i] as usize * self.n;
                for j in i + 1..k {
                    mids |= 1 << self.mid[row + pts[j] as usize];
                }
            }
            if live & !mids == 0 {
                return false;
            }
            live &= mids;
        }
        true
    }

    fn rounds(&self, set: Mask) -> Option<Vec<u8>> {
        let mut pts = [0u8; 128];
        let mut level = vec![255u8; self.n];
        let mut live = set;
        let mut r = 0u8;
        while live != 0 {
            let k = bits(live, &mut pts);
            let mut mids: Mask = 0;
            for i in 0..k {
                let row = pts[i] as usize * self.n;
                for j in i + 1..k {
                    mids |= 1 << self.mid[row + pts[j] as usize];
                }
            }
            let rem = live & !mids;
            if rem == 0 {
                return None;
            }
            let kr = bits(rem, &mut pts);
            for i in 0..kr {
                level[pts[i] as usize] = r;
            }
            live &= mids;
            r += 1;
        }
        Some(level)
    }

    fn canonical(&self, t: Mask, aut: &mut u64) -> bool {
        let (m, n) = (self.m, self.n);
        let lo: Mask = (1 << m) - 1;
        let t_line = (t & lo) as usize;
        let t_off = t & !lo;
        let mut pts = [0u8; 128];
        let mut offpts = [0u8; 128];
        let k = bits(t, &mut pts);
        if k < 2 {
            *aut = (self.group() / if k == 0 { 1 } else { n as u128 }) as u64;
            return k == 0 || t == 1;
        }
        let per = n - m;
        for i in 0..k {
            let a = pts[i] as usize;
            for j in 0..k {
                if i == j {
                    continue;
                }
                let b = pts[j] as usize;
                let pair = a * n + b;
                let lp = &self.line_pts[pair * m..(pair + 1) * m];
                let mut f = 0usize;
                for c in 0..m {
                    f |= (((t >> lp[c]) & 1) as usize) << c;
                }
                if f != t_line {
                    let diff = f ^ t_line;
                    if f >> diff.trailing_zeros() & 1 == 1 {
                        return false;
                    }
                    continue;
                }
                let d = self.sub[b * n + a] as usize;
                let ko = bits(t & !self.line_mask[pair], &mut offpts);
                let tab = &self.mats[d];
                for x in 0..per {
                    let perm = &tab[x * n..(x + 1) * n];
                    let mut h: Mask = 0;
                    for v in 0..ko {
                        h |= 1 << perm[self.sub[offpts[v] as usize * n + a] as usize];
                    }
                    if h == t_off {
                        *aut += 1;
                        continue;
                    }
                    let diff = h ^ t_off;
                    if h >> diff.trailing_zeros() & 1 == 1 {
                        return false;
                    }
                }
            }
        }
        true
    }

    fn line_bound(&self, s: Mask, u: Mask) -> usize {
        let m = self.m;
        let mut best = usize::MAX;
        for ci in 0..m + 1 {
            let mut total = 0usize;
            for l in 0..m {
                let base = (ci * m + l) * m;
                let (mut f, mut a) = (0usize, 0usize);
                for c in 0..m {
                    let p = self.class_pts[base + c];
                    f |= (((s >> p) & 1) as usize) << c;
                    a |= (((u >> p) & 1) as usize) << c;
                }
                total += self.maxpeel[(f << m) | a] as usize;
            }
            best = best.min(total);
        }
        best
    }

    fn triples_hold(&self, set: Mask, g: &[u64]) -> bool {
        let mut pts = [0u8; 128];
        let k = bits(set, &mut pts);
        for i in 0..k {
            for j in i + 1..k {
                let (x, z) = (pts[i] as usize, pts[j] as usize);
                let y = self.mid[x * self.n + z] as usize;
                if set >> y & 1 == 1 && g[x] + g[z] <= 2 * g[y] {
                    return false;
                }
            }
        }
        true
    }

    fn bitmap(&self, set: Mask, level: Option<&[u8]>) -> Vec<String> {
        let m = self.m;
        (0..m)
            .rev()
            .map(|b| {
                (0..m)
                    .map(|a| {
                        let v = a * m + b;
                        match (set >> v & 1 == 1, level) {
                            (false, _) => '.',
                            (true, Some(lv)) => char::from_digit(lv[v] as u32, 36).unwrap(),
                            (true, None) => '#',
                        }
                    })
                    .collect()
            })
            .collect()
    }
}

// THE SEARCH

#[derive(Default, Clone)]
struct Stats {
    nodes: u64,
    tests: u64,
    hist: Vec<u64>,
    found: Vec<(Mask, u64)>,
    orbits: u128,
    sets: u128,
    origin: u128,
}

struct Run<'a> {
    grid: &'a Grid,
    target: usize,
    census: bool,
    stop: &'a AtomicBool,
    deadline: Option<Instant>,
}

impl<'a> Run<'a> {
    fn visit(&self, s: Mask, size: usize, aut: u64, st: &mut Stats) {
        st.nodes += 1;
        if st.nodes & 4095 == 0 {
            if let Some(d) = self.deadline {
                if Instant::now() >= d {
                    self.stop.store(true, Ordering::Relaxed);
                }
            }
        }
        if size >= self.target {
            st.hist[size] += 1;
            if st.found.len() < KEEP {
                st.found.push((s, aut));
            }
        }
        if self.census {
            let orbit = self.grid.group() / aut as u128;
            st.orbits += 1;
            st.sets += orbit;
            st.origin += orbit * size as u128 / self.grid.n as u128;
        }
    }

    fn children(
        &self,
        s: Mask,
        size: usize,
        cands: &[u8],
        mut f: impl FnMut(Mask, usize, u64, Vec<u8>),
    ) {
        for (i, &v) in cands.iter().enumerate() {
            let rest = cands.len() - 1 - i;
            if size + 1 + rest < self.target {
                break;
            }
            if self.stop.load(Ordering::Relaxed) {
                return;
            }
            let t = s | (1 << v);
            let mut aut = 0u64;
            if !self.grid.canonical(t, &mut aut) {
                continue;
            }
            let mut newc = Vec::with_capacity(rest);
            let mut u = t;
            for &c in &cands[i + 1..] {
                if self.grid.peelable(t | (1 << c)) {
                    newc.push(c);
                    u |= 1 << c;
                }
            }
            if size + 1 + newc.len() < self.target || self.grid.line_bound(t, u) < self.target {
                continue;
            }
            f(t, size + 1, aut, newc);
        }
    }

    fn dfs(&self, s: Mask, size: usize, aut: u64, cands: &[u8], st: &mut Stats) {
        self.visit(s, size, aut, st);
        self.children(s, size, cands, |t, sz, au, newc| {
            st.tests += 1;
            self.dfs(t, sz, au, &newc, st);
        });
    }

    fn frontier(
        &self,
        s: Mask,
        size: usize,
        aut: u64,
        cands: &[u8],
        depth: usize,
        st: &mut Stats,
        out: &mut Vec<(Mask, usize, u64, Vec<u8>)>,
    ) {
        if size == depth {
            out.push((s, size, aut, cands.to_vec()));
            return;
        }
        self.visit(s, size, aut, st);
        self.children(s, size, cands, |t, sz, au, newc| {
            self.frontier(t, sz, au, &newc, depth, st, out);
        });
    }
}

fn merge(into: &mut Stats, from: Stats) {
    into.nodes += from.nodes;
    into.tests += from.tests;
    for (a, b) in into.hist.iter_mut().zip(from.hist) {
        *a += b;
    }
    for item in from.found {
        if into.found.len() < KEEP {
            into.found.push(item);
        }
    }
    into.orbits += from.orbits;
    into.sets += from.sets;
    into.origin += from.origin;
}

fn search(grid: &Grid, target: usize, census: bool, seconds: Option<f64>) -> (Stats, bool) {
    let stop = AtomicBool::new(false);
    let deadline = seconds.map(|s| Instant::now() + std::time::Duration::from_secs_f64(s));
    let run = Run {
        grid,
        target,
        census,
        stop: &stop,
        deadline,
    };
    let n = grid.n;
    let mut root_aut = 0u64;
    assert!(grid.canonical(3, &mut root_aut));
    let mut st = Stats {
        hist: vec![0; n + 1],
        ..Default::default()
    };
    if census {
        run.visit(0, 0, grid.group() as u64, &mut st);
        run.visit(1, 1, (grid.group() / n as u128) as u64, &mut st);
    }
    let cands: Vec<u8> = (2..n as u8)
        .filter(|&c| grid.peelable(3 | (1 << c)))
        .collect();
    let mut depth = 3;
    let mut items = Vec::new();
    loop {
        let mut probe = Stats {
            hist: vec![0; n + 1],
            ..Default::default()
        };
        items.clear();
        run.frontier(3, 2, root_aut, &cands, depth, &mut probe, &mut items);
        if items.len() >= 64 * THREADS || items.is_empty() || depth >= 8 {
            st = Stats {
                hist: vec![0; n + 1],
                ..Default::default()
            };
            if census {
                run.visit(0, 0, grid.group() as u64, &mut st);
                run.visit(1, 1, (grid.group() / n as u128) as u64, &mut st);
            }
            merge(&mut st, probe);
            break;
        }
        depth += 1;
    }
    let next = AtomicUsize::new(0);
    let parts: Vec<Stats> = std::thread::scope(|sc| {
        let handles: Vec<_> = (0..THREADS)
            .map(|_| {
                sc.spawn(|| {
                    let mut local = Stats {
                        hist: vec![0; n + 1],
                        ..Default::default()
                    };
                    loop {
                        let i = next.fetch_add(1, Ordering::Relaxed);
                        if i >= items.len() || stop.load(Ordering::Relaxed) {
                            break;
                        }
                        let (s, size, aut, cands) = &items[i];
                        run.dfs(*s, *size, *aut, cands, &mut local);
                    }
                    local
                })
            })
            .collect();
        handles.into_iter().map(|h| h.join().unwrap()).collect()
    });
    for p in parts {
        merge(&mut st, p);
    }
    (st, stop.load(Ordering::Relaxed))
}

fn greedy(grid: &Grid, tries: u64) -> usize {
    let n = grid.n;
    let mut best = 0;
    let mut seed = 0x9e3779b97f4a7c15u64;
    for _ in 0..tries {
        let mut order: Vec<u8> = (0..n as u8).collect();
        for i in (1..n).rev() {
            seed ^= seed << 13;
            seed ^= seed >> 7;
            seed ^= seed << 17;
            order.swap(i, (seed % (i as u64 + 1)) as usize);
        }
        let mut set: Mask = 0;
        let mut size = 0;
        for &v in &order {
            if grid.peelable(set | (1 << v)) {
                set |= 1 << v;
                size += 1;
            }
        }
        best = best.max(size);
    }
    best
}

fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

// THE VERBS

fn optimum(m: usize, target: Option<usize>, seconds: Option<f64>) {
    let t0 = Instant::now();
    let grid = Grid::new(m);
    let n = grid.n;
    let lower = greedy(&grid, 200);
    let mut target = target.unwrap_or(lower).max(lower);
    println!(
        "m = {m}: n = {n}, group {}, greedy lower bound {lower}, target {target}",
        grid.group()
    );
    loop {
        let (st, stopped) = search(&grid, target, false, seconds);
        let top = (0..=n).rev().find(|&k| st.hist[k] > 0);
        if stopped {
            println!(
                "  aborted after {:.1} s at {} nodes and {} canonicity tests: no value at m = {m}",
                t0.elapsed().as_secs_f64(),
                st.nodes,
                st.tests
            );
            return;
        }
        let Some(top) = top else {
            println!(
                "  no peelable set of {target} or more points, {} nodes, {} tests; target lowered",
                st.nodes, st.tests
            );
            target -= 1;
            continue;
        };
        let g = gcd(top, n);
        println!("  every peelable set of {target} or more points enumerated up to affine symmetry: {} nodes, {} canonicity tests, {} threads, {:.1} s", st.nodes, st.tests, THREADS, t0.elapsed().as_secs_f64());
        for k in target..=top {
            if st.hist[k] > 0 {
                let kept: Vec<&(Mask, u64)> = st
                    .found
                    .iter()
                    .filter(|(s, _)| s.count_ones() as usize == k)
                    .collect();
                let orbits: u128 = kept.iter().map(|(_, a)| grid.group() / *a as u128).sum();
                if kept.len() as u64 == st.hist[k] {
                    println!(
                        "  {k} points: {} sets up to symmetry, {orbits} sets",
                        st.hist[k]
                    );
                } else {
                    println!(
                        "  {k} points: {} sets up to symmetry, orbit total not computed",
                        st.hist[k]
                    );
                }
            }
        }
        let alpha = top as f64 / n as f64;
        let c = 2.0 * (1.0 / alpha).log2().sqrt();
        println!(
            "  alpha_{m} = {top}/{n} = {}/{} = {alpha:.5}, C(alpha) = {c:.4}, sqrt(card T) = {:.4}",
            top / g,
            n / g,
            (top as f64).sqrt()
        );
        let mut maxes: Vec<(Mask, u64)> = st
            .found
            .iter()
            .copied()
            .filter(|(s, _)| s.count_ones() as usize == top)
            .collect();
        maxes.sort();
        for (s, aut) in &maxes {
            let level = grid.rounds(*s).expect("maximiser peels");
            let rounds = *level.iter().filter(|&&l| l != 255).max().unwrap() as u32 + 1;
            let g: Vec<u64> = level
                .iter()
                .map(|&l| {
                    if l == 255 {
                        0
                    } else {
                        3u64.pow(rounds - 1 - l as u32)
                    }
                })
                .collect();
            assert!(grid.triples_hold(*s, &g));
            let mut pts = [0u8; 128];
            let k = bits(*s, &mut pts);
            let coords: Vec<String> = pts[..k]
                .iter()
                .map(|&v| format!("({},{})", v as usize / m, v as usize % m))
                .collect();
            println!(
                "  maximiser {} rows b = {}..0, automorphisms {aut}, orbit {}",
                grid.bitmap(*s, None).join("/"),
                m - 1,
                grid.group() / *aut as u128
            );
            println!("    points {}", coords.join(" "));
            println!("    peel rounds {} (digit = round removed), certificate g = 3^({} - round) checked on every triple", grid.bitmap(*s, Some(&level)).join("/"), rounds - 1);
        }
        return;
    }
}

fn census(m: usize, seconds: Option<f64>) {
    let t0 = Instant::now();
    let grid = Grid::new(m);
    let (st, stopped) = search(&grid, 0, true, seconds);
    if stopped {
        println!(
            "m = {m}: census aborted after {:.1} s at {} nodes, nothing counted",
            t0.elapsed().as_secs_f64(),
            st.nodes
        );
        return;
    }
    let top = (0..=grid.n).rev().find(|&k| st.hist[k] > 0).unwrap();
    let by_size: Vec<String> = (0..=top).map(|k| st.hist[k].to_string()).collect();
    println!("m = {m}: every peelable set enumerated up to affine symmetry: {} orbits, {} sets, {} containing the origin, largest {top}, {} nodes, {:.1} s", st.orbits, st.sets, st.origin, st.nodes, t0.elapsed().as_secs_f64());
    println!("  orbits by size 0..{top}: {}", by_size.join(" "));
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let verb = args.first().map(String::as_str).unwrap_or("optimum");
    let mut ms = Vec::new();
    let mut target = None;
    let mut seconds = None;
    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--target" => {
                target = Some(args[i + 1].parse().unwrap());
                i += 1;
            }
            "--seconds" => {
                seconds = Some(args[i + 1].parse().unwrap());
                i += 1;
            }
            v => ms.push(v.parse::<usize>().unwrap()),
        }
        i += 1;
    }
    if ms.is_empty() {
        ms = vec![3, 5, 7];
    }
    for &m in &ms {
        match verb {
            "optimum" => optimum(m, target, seconds),
            "census" => census(m, seconds),
            other => panic!("unknown verb {other}"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn reflect_free(grid: &Grid, set: Mask) -> bool {
        let (m, n) = (grid.m, grid.n);
        let at = |a: usize, b: usize| (a % m) * m + (b % m);
        let mut live = set;
        loop {
            if live == 0 {
                return true;
            }
            let mut rem: Mask = 0;
            for y in 0..n {
                if live >> y & 1 == 0 {
                    continue;
                }
                let (ya, yb) = (y / m, y % m);
                let blocked = (0..n).any(|x| {
                    let z = at(2 * ya + m - x / m, 2 * yb + m - x % m);
                    x != y && live >> x & 1 == 1 && live >> z & 1 == 1
                });
                if !blocked {
                    rem |= 1 << y;
                }
            }
            if rem == 0 {
                return false;
            }
            live &= !rem;
        }
    }

    fn random_sets(n: usize, k: usize, count: usize) -> Vec<Mask> {
        let mut seed = 12345u64;
        (0..count)
            .map(|_| {
                let mut s: Mask = 0;
                while (s.count_ones() as usize) < k {
                    seed ^= seed << 13;
                    seed ^= seed >> 7;
                    seed ^= seed << 17;
                    s |= 1 << (seed % n as u64);
                }
                s
            })
            .collect()
    }

    #[test]
    fn peel_agrees_with_reflection() {
        for m in [3, 5, 7] {
            let grid = Grid::new(m);
            for k in [4, 8, 12] {
                for s in random_sets(grid.n, k.min(grid.n - 1), 300) {
                    assert_eq!(
                        grid.peelable(s),
                        reflect_free(&grid, s),
                        "m = {m}, set {s:b}"
                    );
                }
            }
        }
    }

    #[test]
    fn one_dimension_is_half_plus() {
        for m in [3, 5, 7, 11] {
            let grid = Grid::new(m);
            let full = (1usize << m) - 1;
            assert_eq!(grid.maxpeel[full] as usize, (m + 1) / 2);
        }
    }

    #[test]
    fn canonical_sets_match_brute_force_at_three() {
        let grid = Grid::new(3);
        let mut orbits = 0u128;
        let mut sets = 0u128;
        let mut origin = 0u128;
        for s in 0..(1 as Mask) << 9 {
            let mut aut = 0;
            if grid.peelable(s) && grid.canonical(s, &mut aut) {
                orbits += 1;
                sets += grid.group() / aut as u128;
                origin += grid.group() / aut as u128 * s.count_ones() as u128 / 9;
            }
        }
        let (st, stopped) = search(&grid, 0, true, None);
        assert!(!stopped);
        assert_eq!((st.orbits, st.sets, st.origin), (orbits, sets, origin));
        assert_eq!(origin, 57);
        let all = (0..(1 as Mask) << 9).filter(|&s| grid.peelable(s)).count() as u128;
        assert_eq!(sets, all);
    }

    #[test]
    fn line_bound_is_sound_on_random_sets() {
        let grid = Grid::new(5);
        for s in random_sets(grid.n, 10, 200) {
            if !grid.peelable(s) {
                continue;
            }
            let mut pts = [0u8; 128];
            let k = bits(s, &mut pts);
            let half: Mask = pts[..k / 2].iter().fold(0, |a, &v| a | 1 << v);
            assert!(k <= half.count_ones() as usize + grid.line_bound(half, s));
        }
    }
}
