use std::env;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Mutex;
use std::time::Instant;

// SETS

fn gcd(a: u128, b: u128) -> u128 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn sigma(bases: &[u64]) -> (u128, u128) {
    let den = bases.iter().fold(1u128, |l, &d| {
        let e = (d - 1) as u128;
        l / gcd(l, e) * e
    });
    let num = bases.iter().map(|&d| den / (d - 1) as u128).sum();
    (num, den)
}

fn coprime(bases: &[u64]) -> bool {
    bases.iter().fold(0u128, |g, &d| gcd(g, d as u128)) == 1
}

fn family(bases: &[u64]) -> bool {
    let (num, den) = sigma(bases);
    !bases.is_empty() && num > den && coprime(bases)
}

fn subsets(r: u64) -> Vec<Vec<u64>> {
    let pool: Vec<u64> = (3..=r).collect();
    (1u64..(1 << pool.len()))
        .map(|mask| {
            pool.iter()
                .enumerate()
                .filter(|(i, _)| mask >> i & 1 == 1)
                .map(|(_, &d)| d)
                .collect()
        })
        .collect()
}

fn order(sets: &mut [Vec<u64>]) {
    sets.sort_by(|a, b| {
        a.iter()
            .max()
            .cmp(&b.iter().max())
            .then(a.len().cmp(&b.len()))
            .then(a.cmp(b))
    });
}

fn minimal(r: u64) -> Vec<Vec<u64>> {
    let mut out: Vec<Vec<u64>> = subsets(r)
        .into_iter()
        .filter(|s| family(s))
        .filter(|s| {
            (0..s.len()).all(|i| {
                let mut t = s.clone();
                t.remove(i);
                !family(&t)
            })
        })
        .collect();
    order(&mut out);
    out
}

fn unit(r: u64) -> Vec<Vec<u64>> {
    let mut out: Vec<Vec<u64>> = subsets(r)
        .into_iter()
        .filter(|s| {
            let (num, den) = sigma(s);
            num == den && coprime(s)
        })
        .collect();
    order(&mut out);
    out
}

// ELEMENTS

fn elements(bases: &[u64], k: u32, cap: u128) -> Vec<u128> {
    let mut v = vec![];
    for &d in bases {
        let mut x = (d as u128).pow(k);
        while x <= cap {
            v.push(x);
            x *= d as u128;
        }
    }
    v.sort();
    v
}

// BITS

struct Bits {
    w: Vec<u64>,
}

impl Bits {
    fn new(bits: usize) -> Bits {
        let mut w = vec![0u64; bits / 64 + 2];
        w[0] = 1;
        Bits { w }
    }

    fn fold(&mut self, a: usize, top: usize) {
        let q = a / 64;
        let r = (a % 64) as u32;
        for i in (q..=top / 64).rev() {
            let v = if r == 0 {
                self.w[i - q]
            } else {
                let lo = if i > q {
                    self.w[i - q - 1] >> (64 - r)
                } else {
                    0
                };
                (self.w[i - q] << r) | lo
            };
            self.w[i] |= v;
        }
    }

    fn hole(&self, h: usize) -> Option<usize> {
        let mut i = h / 64;
        let keep = h % 64;
        let mut word = !self.w[i];
        if keep < 63 {
            word &= (1u64 << (keep + 1)) - 1;
        }
        loop {
            if word != 0 {
                return Some(i * 64 + 63 - word.leading_zeros() as usize);
            }
            if i == 0 {
                return None;
            }
            i -= 1;
            word = !self.w[i];
        }
    }

    fn gaps(&self, below: usize) -> usize {
        (0..below)
            .filter(|&x| self.w[x / 64] >> (x % 64) & 1 == 0)
            .count()
    }
}

// CERTIFY

#[derive(Debug, Clone, PartialEq)]
enum Verdict {
    Found {
        f: u128,
        gaps: usize,
        n: usize,
        last: u128,
        sum: u128,
        reach: u128,
    },
    Cap,
}

fn far() -> u128 {
    10u128.pow(30)
}

fn grows(a: &[u128], n: usize, s: u128, t: u128, surplus: (u128, u128, u128)) -> Option<u128> {
    let (gain, den, c) = surplus;
    let mut sm = s;
    for &x in &a[n + 1..] {
        if gain * x >= c + den * (2 * t).saturating_sub(1) {
            return Some(x);
        }
        if x + 2 * t > sm + 1 {
            return None;
        }
        sm += x;
    }
    None
}

fn certify(bases: &[u64], k: u32, cap: usize) -> Verdict {
    let (num, den) = sigma(bases);
    if num <= den {
        return Verdict::Cap;
    }
    let c: u128 = bases
        .iter()
        .map(|&d| (d as u128).pow(k) * (den / (d - 1) as u128))
        .sum();
    let a = elements(bases, k, far());
    let mut bits = Bits::new(cap + 64);
    let mut s: u128 = 0;
    for n in 0..a.len() - 1 {
        if s + a[n] > cap as u128 {
            return Verdict::Cap;
        }
        bits.fold(a[n] as usize, (s + a[n]) as usize);
        s += a[n];
        let t = bits.hole((s / 2) as usize).map_or(0, |x| x as u128 + 1);
        if t > a[n + 1] || t == 0 {
            continue;
        }
        if let Some(reach) = grows(&a, n, s, t, (num - den, den, c)) {
            return Verdict::Found {
                f: t - 1,
                gaps: bits.gaps(t as usize),
                n: n + 1,
                last: a[n],
                sum: s,
                reach,
            };
        }
    }
    Verdict::Cap
}

// CONTROL

fn knapsack(bases: &[u64], k: u32, b: usize) -> Vec<bool> {
    let mut r = vec![false; b + 1];
    r[0] = true;
    for a in elements(bases, k, b as u128) {
        let a = a as usize;
        for x in (a..=b).rev() {
            if r[x - a] {
                r[x] = true;
            }
        }
    }
    r
}

fn control() {
    let cells: [(&[u64], u32); 9] = [
        (&[3, 4, 5], 1),
        (&[3, 4, 5], 2),
        (&[3, 4, 5], 3),
        (&[3, 4, 6], 2),
        (&[3, 5, 6, 7], 3),
        (&[4, 5, 6, 7, 8], 3),
        (&[3, 4, 7, 8], 3),
        (&[3, 5, 6, 9], 2),
        (&[3, 4, 9, 10], 2),
    ];
    for (bases, k) in cells {
        let clock = Instant::now();
        let v = certify(bases, k, 1 << 30);
        let Verdict::Found { f, gaps, .. } = v else {
            println!("{:?} k {} cap", bases, k);
            continue;
        };
        let b = (4 * f as usize).max(1000);
        let r = knapsack(bases, k, b);
        let last = (0..=b).rev().find(|&x| !r[x]).unwrap();
        let count = (0..=b).filter(|&x| !r[x]).count();
        println!(
            "{:?} k {} certificate F {} gaps {} | knapsack to {} F {} gaps {} | {} | {:.2} s",
            bases,
            k,
            f,
            gaps,
            b,
            last,
            count,
            if last as u128 == f && count == gaps {
                "agree"
            } else {
                "DISAGREE"
            },
            clock.elapsed().as_secs_f64()
        );
    }
    for bases in [&[3u64, 4][..], &[3, 6, 9, 12, 15, 21][..]] {
        println!("{:?} k 1 {:?} to 2^24", bases, certify(bases, 1, 1 << 24));
    }
}

// CENSUS

fn show(bases: &[u64]) -> String {
    format!(
        "{{{}}}",
        bases
            .iter()
            .map(|d| d.to_string())
            .collect::<Vec<_>>()
            .join(",")
    )
}

fn census(r: u64, kmax: u32, bits: u32, threads: usize) {
    let clock = Instant::now();
    let sets = minimal(r);
    let next = AtomicUsize::new(0);
    let rows: Mutex<Vec<(usize, Vec<(u32, Verdict, f64)>)>> = Mutex::new(vec![]);
    std::thread::scope(|scope| {
        for _ in 0..threads {
            scope.spawn(|| loop {
                let i = next.fetch_add(1, Ordering::SeqCst);
                if i >= sets.len() {
                    break;
                }
                let mut row = vec![];
                for k in 1..=kmax {
                    let t = Instant::now();
                    let v = certify(&sets[i], k, 1usize << bits);
                    let stop = v == Verdict::Cap;
                    row.push((k, v, t.elapsed().as_secs_f64()));
                    if stop {
                        break;
                    }
                }
                rows.lock().unwrap().push((i, row));
            });
        }
    });
    let mut rows = rows.into_inner().unwrap();
    rows.sort_by_key(|x| x.0);
    let mut depth = vec![0usize; kmax as usize + 1];
    for (i, row) in &rows {
        let (num, den) = sigma(&sets[*i]);
        let mut line = format!(
            "{} sigma {}/{}",
            show(&sets[*i]),
            num / gcd(num, den),
            den / gcd(num, den)
        );
        let mut d = 0;
        for (k, v, t) in row {
            match v {
                Verdict::Found {
                    f,
                    gaps,
                    n,
                    last,
                    sum,
                    reach,
                } => {
                    d = *k;
                    line += &format!(
                        " | k {} F {} gaps {} n {} last {} sum {} reach {} {:.2}s",
                        k, f, gaps, n, last, sum, reach, t
                    );
                }
                Verdict::Cap => line += &format!(" | k {} cap 2^{}", k, bits),
            }
        }
        depth[d as usize] += 1;
        println!("{}", line);
    }
    println!("minimal sets {} below {}", sets.len(), r + 1);
    for (d, count) in depth.iter().enumerate() {
        if *count > 0 {
            println!("certified to k = {}: {} sets", d, count);
        }
    }
    let floor = rows
        .iter()
        .map(|(_, row)| row.iter().filter(|x| x.1 != Verdict::Cap).count())
        .min()
        .unwrap_or(0);
    println!(
        "every D in [3, {}] with sigma > 1 and gcd 1 is complete at k = 1..{}",
        r, floor
    );
    for s in unit(r) {
        println!("sigma = 1, gcd 1, outside the certificate: {}", show(&s));
    }
    println!("{:.1} s", clock.elapsed().as_secs_f64());
}

fn cell(bases: &[u64], k: u32, bits: u32) {
    let clock = Instant::now();
    let v = certify(bases, k, 1usize << bits);
    println!(
        "{} k {} {:?} {:.1} s",
        show(bases),
        k,
        v,
        clock.elapsed().as_secs_f64()
    );
}

fn sets(bases: &[u64], k: u32, b: usize) {
    let mut terms = elements(bases, k, b as u128);
    terms.dedup();
    let mut r = vec![false; b + 1];
    r[0] = true;
    for a in terms {
        let a = a as usize;
        for x in (a..=b).rev() {
            if r[x - a] {
                r[x] = true;
            }
        }
    }
    let last = (0..=b).rev().find(|&x| !r[x]).unwrap();
    println!(
        "{} k {} as a set: largest non-sum below {} is {}",
        show(bases),
        k,
        b,
        last
    );
}

// WINDOWS

fn windows(bases: &[u64], k: u32, lo: u128, hi: u128) {
    let a = elements(bases, k, far());
    let mut sums = vec![];
    let mut s: u128 = 0;
    for &x in &a {
        s += x;
        sums.push(s);
    }
    let open: Vec<usize> = (0..a.len() - 2)
        .filter(|&n| a[n + 1] >= lo && a[n + 1] <= hi && a[n + 2] > sums[n])
        .collect();
    let seen = (0..a.len() - 2)
        .filter(|&n| a[n + 1] >= lo && a[n + 1] <= hi)
        .count();
    let mut chains = 0;
    for &m in &open {
        for &n in &open {
            if n > m && sums[n].saturating_sub(a[n + 1]) < a[m + 2] && sums[m] < a[n + 2] - a[n + 1]
            {
                chains += 1;
            }
        }
    }
    println!(
        "{} k {}: {} of {} terms a_(n+1) in [{}, {}] open a window a_(n+2) > S_n; {} pairs m < n of open windows where (S_m, a_(m+2)) meets (S_n - a_(n+1), a_(n+2) - a_(n+1))",
        show(bases), k, open.len(), seen, lo, hi, chains
    );
}

fn graham(t: u128, p: u128, q: u128, bits: u32) {
    let cap = 1usize << bits;
    let mut terms = vec![];
    let (mut num, mut den) = (t, 1u128);
    loop {
        num *= p;
        den *= q;
        let x = num / den;
        if x as usize > cap {
            break;
        }
        if x > 0 {
            terms.push(x as usize);
        }
    }
    terms.dedup();
    let mut bits_ = Bits::new(terms.iter().sum::<usize>() + 64);
    let mut s = 0usize;
    let mut sums = vec![];
    for &x in &terms {
        bits_.fold(x, s + x);
        s += x;
        sums.push(s);
    }
    let mut chained = 0;
    let mut windows = 0;
    for n in 0..terms.len().saturating_sub(2) {
        if terms[n + 2] > sums[n] && terms[n + 2] <= cap {
            windows += 1;
            if (sums[n] + 1..terms[n + 2]).any(|x| bits_.w[x / 64] >> (x % 64) & 1 == 0) {
                chained += 1;
            }
        }
    }
    let last = bits_
        .hole(cap / 2)
        .map_or("none".to_string(), |x| x.to_string());
    println!(
        "floor({} ({}/{})^n), n >= 1, {} terms to 2^{}: {} windows a_(n+2) > S_n, {} of them holding a non-sum; largest non-sum up to 2^{} is {}",
        t, p, q, terms.len(), bits, windows, chained, bits - 1, last
    );
}

fn fibonacci(cap: usize) -> Vec<usize> {
    let mut v = vec![1usize, 3];
    while v[v.len() - 1] + v[v.len() - 2] < cap {
        let n = v.len();
        v.push(v[n - 1] + v[n - 2] + 1);
    }
    v
}

fn missing(terms: &[usize]) -> Bits {
    let mut bits = Bits::new(terms.iter().sum::<usize>() + 64);
    let mut s = 0;
    for &x in terms {
        bits.fold(x, s + x);
        s += x;
    }
    bits
}

fn refute(bits_: u32) {
    let terms = fibonacci(1 << bits_);
    let bits = missing(&terms);
    let mut s = 0i128;
    let mut least = i128::MAX;
    let (mut windows, mut held) = (0, 0);
    let mut sums = vec![];
    for &x in &terms {
        s += x as i128;
        sums.push(s as usize);
    }
    for n in 0..terms.len() - 1 {
        least = least.min(2 * (sums[n] as i128 - terms[n + 1] as i128) - terms[n + 1] as i128);
    }
    for n in 0..terms.len() - 2 {
        if terms[n + 2] > sums[n] {
            windows += 1;
            if (sums[n] + 1..terms[n + 2]).any(|x| bits.w[x / 64] >> (x % 64) & 1 == 0) {
                held += 1;
            }
        }
    }
    let low: Vec<usize> = (1..1_000_000)
        .filter(|&x| bits.w[x / 64] >> (x % 64) & 1 == 0)
        .collect();
    println!(
        "2 F_m - 1, m >= 2, {} terms to 2^{}: least 2 (S_n - a_(n+1)) - a_(n+1) = {}; {} windows, {} holding a non-sum; {} non-sums below 10^6: {:?}",
        terms.len(), bits_, least, windows, held, low.len(), low
    );
}

// TERNARY

const WORDS: usize = 128;

#[derive(Clone, Copy)]
struct Sums {
    w: [u64; WORDS],
    top: usize,
}

impl Sums {
    fn zero() -> Sums {
        let mut w = [0u64; WORDS];
        w[0] = 1;
        Sums { w, top: 0 }
    }

    fn meets(&self, a: usize) -> bool {
        let q = a / 64;
        let r = (a % 64) as u32;
        for i in q..=(self.top + a) / 64 {
            let lo = if r > 0 && i > q {
                self.w[i - q - 1] >> (64 - r)
            } else {
                0
            };
            let v = if r == 0 {
                self.w[i - q]
            } else {
                (self.w[i - q] << r) | lo
            };
            if v & self.w[i] != 0 {
                return true;
            }
        }
        false
    }

    fn with(&self, a: usize) -> Sums {
        let mut out = *self;
        for shift in [a, 2 * a] {
            let q = shift / 64;
            let r = (shift % 64) as u32;
            for i in q..=(self.top + shift) / 64 {
                let lo = if r > 0 && i > q {
                    self.w[i - q - 1] >> (64 - r)
                } else {
                    0
                };
                let v = if r == 0 {
                    self.w[i - q]
                } else {
                    (self.w[i - q] << r) | lo
                };
                out.w[i] |= v;
            }
        }
        out.top = self.top + 2 * a;
        out
    }
}

struct Hunt<'a> {
    floor: &'a [usize],
    near: usize,
    need: u128,
    nodes: u64,
    found: Option<Vec<usize>>,
}

impl Hunt<'_> {
    fn dfs(&mut self, sums: &Sums, chosen: &mut Vec<usize>, sq: u128, left: usize) {
        self.nodes += 1;
        if self.found.is_some() {
            return;
        }
        if left == 0 {
            self.found = Some(chosen.clone());
            return;
        }
        let top = *chosen.last().unwrap();
        let mut lo = self.floor[left - 1] + 1;
        if left >= 2 {
            lo = lo.max(self.near);
        }
        for a in (lo..top).rev() {
            let rest = (left - 1) as u128 * ((a - 1) as u128).pow(2);
            if sq + (a as u128).pow(2) + rest < self.need {
                break;
            }
            if sums.meets(a) || sums.meets(2 * a) {
                continue;
            }
            chosen.push(a);
            let next = sums.with(a);
            self.dfs(&next, chosen, sq + (a as u128).pow(2), left - 1);
            chosen.pop();
            if self.found.is_some() {
                return;
            }
        }
    }
}

fn hunt(n: usize, top: usize, floor: &[usize], threads: usize) -> (Option<Vec<usize>>, u64) {
    near(n, top, floor, threads, 0)
}

fn near(
    n: usize,
    top: usize,
    floor: &[usize],
    threads: usize,
    band: usize,
) -> (Option<Vec<usize>>, u64) {
    let need = (9u128.pow(n as u32) - 1).div_ceil(8);
    let root = Sums::zero().with(top);
    let lo = floor[n - 2] + 1;
    let seconds: Vec<usize> = (lo..top).rev().collect();
    let next = AtomicUsize::new(0);
    let out: Mutex<(Option<Vec<usize>>, u64)> = Mutex::new((None, 0));
    std::thread::scope(|scope| {
        for _ in 0..threads {
            scope.spawn(|| loop {
                let i = next.fetch_add(1, Ordering::SeqCst);
                if i >= seconds.len() || out.lock().unwrap().0.is_some() {
                    break;
                }
                let a = seconds[i];
                let sq = (top as u128).pow(2) + (a as u128).pow(2);
                if sq + (n as u128 - 2) * ((a - 1) as u128).pow(2) < need
                    || root.meets(a)
                    || root.meets(2 * a)
                {
                    continue;
                }
                if band > 0 && a + band < top {
                    continue;
                }
                let mut h = Hunt {
                    floor,
                    near: if band > 0 { top - band } else { 0 },
                    need,
                    nodes: 0,
                    found: None,
                };
                let mut chosen = vec![top, a];
                h.dfs(&root.with(a), &mut chosen, sq, n - 2);
                let mut o = out.lock().unwrap();
                o.1 += h.nodes;
                if h.found.is_some() && o.0.is_none() {
                    o.0 = h.found;
                }
            });
        }
    });
    out.into_inner().unwrap()
}

fn admissible(a: &[usize]) -> bool {
    let n = a.len();
    let mut seen = std::collections::HashSet::new();
    for c in 0..3usize.pow(n as u32) {
        let (mut x, mut s) = (c, 0);
        for &v in a {
            s += (x % 3) * v;
            x /= 3;
        }
        if !seen.insert(s) {
            return false;
        }
    }
    true
}

fn probe(n: usize, top: usize, threads: usize) {
    let floor = [0usize, 1, 3, 8, 22, 60, 168];
    let clock = Instant::now();
    let (found, nodes) = hunt(n, top, &floor[..n], threads);
    println!(
        "n {} top {}: {:?}, {} nodes, {:.1} s",
        n,
        top,
        found,
        nodes,
        clock.elapsed().as_secs_f64()
    );
}

fn band(lo: usize, hi: usize, width: usize, threads: usize) {
    let floor = [0usize, 1, 3, 8, 22, 60, 168];
    let clock = Instant::now();
    for top in lo..=hi {
        let (found, nodes) = near(7, top, &floor, threads, width);
        if let Some(mut set) = found {
            set.reverse();
            assert!(admissible(&set));
            println!(
                "band {}: first top {} with six terms within {} of it: {:?}, {} nodes, {:.1} s",
                width,
                top,
                width,
                set,
                nodes,
                clock.elapsed().as_secs_f64()
            );
            return;
        }
    }
    println!(
        "band {}: none with top in [{}, {}], {:.1} s",
        width,
        lo,
        hi,
        clock.elapsed().as_secs_f64()
    );
}

fn offsets(lo: usize, hi: usize) {
    let families: [[usize; 6]; 3] = [
        [0, 1, 3, 8, 22, 60],
        [0, 2, 6, 9, 23, 61],
        [0, 2, 5, 7, 21, 60],
    ];
    for fam in families {
        let mut best: Option<Vec<usize>> = None;
        'outer: for top in lo..=hi {
            for x in fam[5] + 1..top {
                let mut set: Vec<usize> = fam.iter().map(|o| top - o).collect();
                set.push(top - x);
                set.sort();
                if admissible(&set) {
                    best = Some(set);
                    break 'outer;
                }
            }
        }
        println!("offsets {:?} plus one: {:?}", fam, best);
    }
}

fn ternary(nmax: usize, hi: usize, threads: usize) {
    let mut floor = vec![0usize, 1];
    println!("g_3(1) = 1");
    for n in 2..=nmax {
        let clock = Instant::now();
        let mut total = 0;
        let mut hit = None;
        for top in floor[n - 1] + 1..=hi {
            let (found, nodes) = hunt(n, top, &floor, threads);
            total += nodes;
            if let Some(set) = found {
                hit = Some((top, set));
                break;
            }
        }
        match hit {
            Some((top, mut set)) => {
                set.reverse();
                assert!(admissible(&set));
                println!(
                    "g_3({}) = {}, witness {:?}, {} nodes, {:.1} s",
                    n,
                    top,
                    set,
                    total,
                    clock.elapsed().as_secs_f64()
                );
                floor.push(top);
            }
            None => {
                println!(
                    "g_3({}) > {}, {} nodes, {:.1} s",
                    n,
                    hi,
                    total,
                    clock.elapsed().as_secs_f64()
                );
                return;
            }
        }
    }
}

// ROUTE

fn route(r: u64, lo: u128) {
    for bases in minimal(r) {
        let a = elements(&bases, 1, far());
        let mut below: u128 = 0;
        let mut best = (u128::MAX, 0u128);
        for &x in &a {
            let key = below * 1_000_000 / x;
            if x >= lo && key < best.0 {
                best = (key, x);
            }
            below += x;
        }
        println!(
            "{} least sum(M below N)/N over N in M, {} <= N <= 10^30: {}.{:06} at N = {}",
            show(&bases),
            lo,
            best.0 / 1_000_000,
            best.0 % 1_000_000,
            best.1
        );
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let num = |i: usize, d: u64| args.get(i).map(|s| s.parse().unwrap()).unwrap_or(d);
    match args.get(1).map(|s| s.as_str()).unwrap_or("") {
        "census" => census(num(2, 10), num(3, 4) as u32, num(4, 31) as u32, num(5, 4) as usize),
        "control" => control(),
        "cell" => {
            let bases: Vec<u64> = args[2].split(',').map(|x| x.parse().unwrap()).collect();
            cell(&bases, num(3, 1) as u32, num(4, 31) as u32)
        }
        "set" => {
            let bases: Vec<u64> = args[2].split(',').map(|x| x.parse().unwrap()).collect();
            sets(&bases, num(3, 1) as u32, num(4, 20000) as usize)
        }
        "windows" => {
            let bases: Vec<u64> = args[2].split(',').map(|x| x.parse().unwrap()).collect();
            windows(&bases, num(3, 1) as u32, 10u128.pow(num(4, 6) as u32), 10u128.pow(num(5, 30) as u32))
        }
        "graham" => graham(num(2, 2) as u128, num(3, 5) as u128, num(4, 3) as u128, num(5, 22) as u32),
        "refute" => refute(num(2, 27) as u32),
        "ternary" => ternary(num(2, 6) as usize, num(3, 200) as usize, num(4, 8) as usize),
        "probe" => probe(num(2, 7) as usize, num(3, 420) as usize, num(4, 8) as usize),
        "offsets" => offsets(num(2, 420) as usize, num(3, 504) as usize),
        "band" => band(num(2, 419) as usize, num(3, 504) as usize, num(4, 70) as usize, num(5, 8) as usize),
        "route" => route(num(2, 10), 10u128.pow(num(3, 12) as u32)),
        _ => println!("verbs census R K BITS THREADS, cell D K BITS, set D K B, windows D K E F, graham T P Q BITS, refute BITS, ternary N HI THREADS, probe N TOP THREADS, band LO HI W THREADS, offsets LO HI, control, route R E"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_unit_sets_below_eight_are_one() {
        assert_eq!(unit(8), vec![vec![3, 4, 7]]);
        assert_eq!(sigma(&[3, 4, 7]), (6, 6));
    }

    #[test]
    fn the_minimal_sets_below_nine() {
        let want: Vec<Vec<u64>> = vec![
            vec![3, 4, 5],
            vec![3, 4, 6],
            vec![3, 4, 7, 8],
            vec![3, 5, 6, 7],
            vec![3, 5, 6, 8],
            vec![3, 5, 7, 8],
            vec![3, 6, 7, 8],
            vec![4, 5, 6, 7, 8],
        ];
        let mut got = minimal(8);
        got.sort();
        assert_eq!(got, want);
    }

    #[test]
    fn the_bit_array_matches_a_plain_array() {
        let mut bits = Bits::new(4096);
        let mut plain = vec![false; 4096];
        plain[0] = true;
        let mut s = 0;
        for a in [3usize, 64, 5, 130, 77, 1000, 128] {
            bits.fold(a, s + a);
            for x in (a..=s + a).rev() {
                if plain[x - a] {
                    plain[x] = true;
                }
            }
            s += a;
            for h in [1usize, 63, 64, 65, s / 2, s] {
                let want = (0..=h).rev().find(|&x| !plain[x]);
                assert_eq!(bits.hole(h), want);
            }
        }
    }

    #[test]
    fn three_four_five_at_k_one_misses_seventy_nine_last() {
        let Verdict::Found { f, .. } = certify(&[3, 4, 5], 1, 1 << 20) else {
            panic!()
        };
        assert_eq!(f, 79);
        let r = knapsack(&[3, 4, 5], 1, 5000);
        assert!(!r[79]);
        assert!((80..=5000).all(|x| r[x]));
    }

    #[test]
    fn surplus_and_a_full_spectrum_leave_a_hole_chain() {
        let terms = fibonacci(1 << 20);
        assert_eq!(&terms[..8], &[1, 3, 5, 9, 15, 25, 41, 67]);
        let bits = missing(&terms);
        for x in [
            2usize, 7, 22, 63, 172, 459, 1212, 3185, 11, 36, 103, 280, 745, 1964, 5157,
        ] {
            assert_eq!(bits.w[x / 64] >> (x % 64) & 1, 0);
        }
        let mut s = 0i64;
        for n in 0..terms.len() - 1 {
            s += terms[n] as i64;
            assert!(2 * (s - terms[n + 1] as i64) - terms[n + 1] as i64 >= -9);
        }
    }

    #[test]
    fn the_seven_set_under_four_hundred_seventy_five_is_admissible() {
        assert!(admissible(&[302, 409, 447, 459, 465, 466, 474]));
        assert!(!admissible(&[302, 409, 447, 459, 465, 467, 474]));
        let floor = [0usize, 1, 3, 8, 22];
        assert_eq!(hunt(5, 59, &floor, 2).0, None);
        assert!(hunt(5, 60, &floor, 2).0.is_some());
    }

    #[test]
    fn a_set_below_the_threshold_never_certifies() {
        assert_eq!(certify(&[3, 4], 1, 1 << 22), Verdict::Cap);
        assert_eq!(certify(&[3, 6, 9, 12, 15, 21], 1, 1 << 22), Verdict::Cap);
    }
}
