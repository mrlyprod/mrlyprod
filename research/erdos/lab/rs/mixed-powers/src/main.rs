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

    fn get(&self, x: usize) -> bool {
        self.w[x / 64] >> (x % 64) & 1 == 1
    }

    fn each_zero(&self, lo: usize, hi: usize, mut f: impl FnMut(usize) -> bool) -> bool {
        if lo > hi {
            return false;
        }
        for i in lo / 64..=hi / 64 {
            let mut word = !self.w[i];
            while word != 0 {
                let x = i * 64 + word.trailing_zeros() as usize;
                word &= word - 1;
                if x >= lo && x <= hi && f(x) {
                    return true;
                }
            }
        }
        false
    }

    fn hole_in(&self, lo: usize, h: usize) -> Option<usize> {
        if lo > h {
            return None;
        }
        let mut i = h / 64;
        let keep = h % 64;
        let mut word = !self.w[i];
        if keep < 63 {
            word &= (1u64 << (keep + 1)) - 1;
        }
        loop {
            if word != 0 {
                let x = i * 64 + 63 - word.leading_zeros() as usize;
                return if x >= lo { Some(x) } else { None };
            }
            if i == lo / 64 {
                return None;
            }
            i -= 1;
            word = !self.w[i];
        }
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

// WIDE

#[derive(Debug, Clone, PartialEq, Default)]
struct Lanes {
    seed: usize,
    window: usize,
    pair: usize,
    last: u8,
}

fn lane(a: &[u128], pre: &[u128], z: u128) -> u8 {
    let c = a.partition_point(|&t| t <= z);
    if c == 0 {
        0
    } else if z > pre[c - 1] {
        1
    } else {
        2
    }
}

fn wide(bases: &[u64], k: u32, bits: u32) -> (Verdict, Lanes) {
    wide_with(bases, k, bits, |_, _, _| {})
}

fn wide_with(
    bases: &[u64],
    k: u32,
    bits: u32,
    mut log: impl FnMut(usize, u128, u128),
) -> (Verdict, Lanes) {
    let (num, den) = sigma(bases);
    if num <= den {
        return (Verdict::Cap, Lanes::default());
    }
    let c: u128 = bases
        .iter()
        .map(|&d| (d as u128).pow(k) * (den / (d - 1) as u128))
        .sum();
    let a = elements(bases, k, far());
    let mut pre = vec![0u128];
    for &x in &a {
        pre.push(pre[pre.len() - 1] + x);
    }
    let w = 1usize << bits;
    let wu = w as u128;
    let mut b = Bits::new(w + 64);
    let limit = 1usize << 25;
    let mut sparse: Vec<u128> = vec![];
    let mut s: u128 = 0;
    let mut seen = 0usize;
    let mut top: Option<usize> = None;
    for n in 0..a.len() - 1 {
        let x = a[n];
        let snew = s + x;
        if snew >> 63 != 0 {
            return (Verdict::Cap, Lanes::default());
        }
        if snew / 2 > wu {
            let half = snew / 2;
            if x > s && half > s.max(wu) && half - s.max(wu) > limit as u128 {
                return (Verdict::Cap, Lanes::default());
            }
            let mem = |z: i128| -> bool {
                if z < 0 || z as u128 > s {
                    return false;
                }
                let z = z as u128;
                if z <= wu {
                    b.get(z as usize)
                } else if z >= s - wu {
                    b.get((s - z) as usize)
                } else if 2 * z <= s {
                    sparse.binary_search(&z).is_err()
                } else {
                    sparse.binary_search(&(s - z)).is_err()
                }
            };
            let mut next: Vec<u128> = vec![];
            let mut keep = |z: u128| -> bool {
                if z > wu && z <= half && !mem(z as i128 - x as i128) {
                    next.push(z);
                }
                next.len() > limit
            };
            let mut spill = false;
            for &g in &sparse {
                spill |= keep(g) || keep(s - g);
            }
            if s > wu {
                let d = s as i128 - x as i128;
                let glo = if d > 0 { ((d + 1) / 2) as usize } else { 0 };
                let ghi = w.min((s - wu - 1) as usize);
                spill |= b.each_zero(glo, ghi, |g| keep(s - g as u128));
            }
            if x > s {
                for z in s.max(wu) + 1..=half {
                    spill |= keep(z);
                }
            }
            if spill {
                return (Verdict::Cap, Lanes::default());
            }
            next.sort();
            next.dedup();
            sparse = next;
        }
        if x <= wu {
            b.fold(x as usize, w.min(snew as usize));
        }
        s = snew;
        let h = w.min((s / 2) as usize);
        let fresh = if h >= seen { b.hole_in(seen, h) } else { None };
        top = fresh.or_else(|| top.and_then(|t| b.hole_in(0, t)));
        seen = seen.max(h + 1);
        let t = match sparse.last() {
            Some(&z) => z + 1,
            None => top.map_or(0, |z| z as u128 + 1),
        };
        log(n + 1, s, t);
        if t > a[n + 1] || t == 0 {
            continue;
        }
        if let Some(reach) = grows(&a, n, s, t, (num - den, den, c)) {
            let mut lanes = Lanes::default();
            let mut count = |z: u128| match lane(&a, &pre, z) {
                0 => lanes.seed += 1,
                1 => lanes.window += 1,
                _ => lanes.pair += 1,
            };
            b.each_zero(0, w.min((t - 1) as usize), |z| {
                count(z as u128);
                false
            });
            for &z in &sparse {
                count(z);
            }
            lanes.last = lane(&a, &pre, t - 1);
            let v = Verdict::Found {
                f: t - 1,
                gaps: lanes.seed + lanes.window + lanes.pair,
                n: n + 1,
                last: a[n],
                sum: s,
                reach,
            };
            return (v, lanes);
        }
    }
    (Verdict::Cap, Lanes::default())
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

fn wcontrol(bits: u32) {
    let clock = Instant::now();
    let (mut agree, mut differ, mut capped, mut past) = (0, 0, 0, 0);
    for set in minimal(10) {
        for k in 1..=4 {
            let full = certify(&set, k, 1 << 30);
            if full == Verdict::Cap {
                continue;
            }
            let (v, _) = wide(&set, k, bits);
            match &v {
                Verdict::Cap => capped += 1,
                Verdict::Found { sum, .. } => {
                    past += (*sum > 2u128 << bits) as usize;
                    if v == full {
                        agree += 1;
                    } else {
                        differ += 1;
                        println!("{} k {} full {:?} wide {:?}", show(&set), k, full, v);
                    }
                }
            }
        }
    }
    println!(
        "wide at 2^{} against the full certificate at 2^30, minimal sets in [3, 10], k = 1..4: {} agree, {} differ, {} capped, {} certified with S_n past twice the window; {:.1} s",
        bits, agree, differ, capped, past, clock.elapsed().as_secs_f64()
    );
}

fn split(bases: &[u64], k: u32, x0: u128, count: u128) {
    let clock = Instant::now();
    let terms = elements(bases, k, x0 + count);
    let (l, r) = terms.split_at(terms.len() / 2);
    let sums = |t: &[u128]| -> Vec<u128> {
        let mut v = vec![0u128];
        for &x in t {
            for i in 0..v.len() {
                v.push(v[i] + x);
            }
        }
        v.sort();
        v
    };
    let (ls, rs) = (sums(l), sums(r));
    let hit = |x: u128| -> bool {
        let (mut i, mut j) = (0usize, rs.len());
        while i < ls.len() && j > 0 {
            let v = ls[i] + rs[j - 1];
            if v == x {
                return true;
            }
            if v < x {
                i += 1;
            } else {
                j -= 1;
            }
        }
        false
    };
    let out: Vec<u128> = (x0..x0 + count).filter(|&x| !hit(x)).collect();
    println!(
        "{} k {}: meet in the middle over {} terms, the non-sums in [{}, {}] are {:?}; {:.1} s",
        show(bases),
        k,
        terms.len(),
        x0,
        x0 + count - 1,
        out,
        clock.elapsed().as_secs_f64()
    );
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

type Row = Vec<(u32, Verdict, Lanes, f64)>;

fn census(r: u64, kmax: u32, bits: u32, threads: usize, deep: bool) {
    let clock = Instant::now();
    let sets = minimal(r);
    let next = AtomicUsize::new(0);
    let rows: Mutex<Vec<(usize, Row)>> = Mutex::new(vec![]);
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
                    let (v, lanes) = if deep {
                        wide(&sets[i], k, bits)
                    } else {
                        (certify(&sets[i], k, 1usize << bits), Lanes::default())
                    };
                    let stop = v == Verdict::Cap;
                    row.push((k, v, lanes, t.elapsed().as_secs_f64()));
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
        for (k, v, lanes, t) in row {
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
                    if deep {
                        line += &format!(
                            " lanes {}/{}/{} F {}",
                            lanes.seed,
                            lanes.window,
                            lanes.pair,
                            ["seed", "window", "pair"][lanes.last as usize]
                        );
                    }
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
    if deep {
        for k in 1..=kmax {
            let mut last = [0usize; 3];
            let mut total = [0usize; 3];
            for (_, row) in &rows {
                for (kk, v, lanes, _) in row {
                    if *kk == k && *v != Verdict::Cap {
                        last[lanes.last as usize] += 1;
                        total[0] += lanes.seed;
                        total[1] += lanes.window;
                        total[2] += lanes.pair;
                    }
                }
            }
            println!(
                "k = {}: F below the least term in {} cells, by the window route in {}, by the pair route in {}; non-sums seed/window/pair {}/{}/{}",
                k, last[0], last[1], last[2], total[0], total[1], total[2]
            );
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

fn wcell(bases: &[u64], k: u32, bits: u32) {
    let clock = Instant::now();
    let (v, lanes) = wide(bases, k, bits);
    println!(
        "{} k {} wide 2^{} {:?} {:?} {:.1} s",
        show(bases),
        k,
        bits,
        v,
        lanes,
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

fn kick(n: usize) -> usize {
    2 - (n / 2) % 2
}

fn chainless_terms(cap: usize) -> Vec<usize> {
    let mut a = vec![4usize, 5, 6];
    loop {
        let n = a.len() - 1;
        let v = a[..n].iter().sum::<usize>() - kick(n) - kick(n + 1);
        if v > cap {
            return a;
        }
        a.push(v);
    }
}

fn forced(bits_: u32) {
    let cap = 1usize << bits_;
    for p in [6usize, 8, 10] {
        for c in [2usize, 3] {
            let mut a = vec![4usize, 5, 6];
            loop {
                let n = a.len() - 1;
                let s: usize = a[..n].iter().sum();
                let v = if n >= 6 && n % p == 0 {
                    s + c
                } else {
                    s - kick(n) - kick(n + 1)
                };
                if v > cap {
                    break;
                }
                a.push(v);
            }
            let bits = missing(&a);
            let mut sums = vec![0usize];
            for &x in &a {
                sums.push(sums[sums.len() - 1] + x);
            }
            let least = (1..a.len())
                .map(|n| 2 * sums[n] as i64 - 3 * a[n] as i64)
                .min()
                .unwrap();
            let open: Vec<usize> = (2..a.len() - 1).filter(|&n| a[n + 1] > sums[n]).collect();
            let mut chains = 0;
            for &m in &open {
                for &n in &open {
                    if n > m && sums[n] - a[n] < a[m + 1] && sums[m] < a[n + 1] - a[n] {
                        chains += 1;
                    }
                }
            }
            let top = a[a.len() - 1];
            let mut total = 0;
            bits.each_zero(1, top - 1, |_| {
                total += 1;
                false
            });
            let mut late = 0;
            bits.each_zero(a[a.len() - 13], top - 1, |_| {
                late += 1;
                false
            });
            println!(
                "E with a_(n+2) = S_n + {} at n >= 6, n = 0 mod {}: {} terms to 2^{}, {} open windows at n >= 2, {} chained pairs, least 2 (S_n - a_(n+1)) - a_(n+1) = {}; {} non-sums below the last term, {} in the last 12 term intervals",
                c, p, a.len(), bits_, open.len(), chains, least, total, late
            );
        }
    }
}

fn chainless(bits_: u32) {
    let a = chainless_terms(1 << bits_);
    let bits = missing(&a);
    let mut sums = vec![0usize];
    for &x in &a {
        sums.push(sums[sums.len() - 1] + x);
    }
    let top = a[a.len() - 1];
    let early = (0..2).filter(|&n| a[n + 1] > sums[n]).count();
    let late = (2..a.len() - 1).filter(|&n| a[n + 1] > sums[n]).count();
    let least = (1..a.len())
        .map(|n| 2 * sums[n] as i64 - 3 * a[n] as i64)
        .min()
        .unwrap();
    let mut kicks: Vec<i64> = (2..a.len() - 2)
        .map(|n| a[n + 2] as i64 - a[n + 1] as i64 - a[n] as i64)
        .collect();
    kicks.sort();
    kicks.dedup();
    let chain: Vec<usize> = (3..a.len() - 1)
        .filter(|&n| a[n] + kick(n) < a[n + 1])
        .map(|n| a[n] + kick(n))
        .collect();
    let held = chain.iter().filter(|&&x| !bits.get(x)).count();
    let wide_a: Vec<u128> = a.iter().map(|&x| x as u128).collect();
    let pre: Vec<u128> = sums.iter().map(|&x| x as u128).collect();
    let mut lanes = [0usize; 3];
    bits.each_zero(1, top - 1, |z| {
        lanes[lane(&wide_a, &pre, z as u128) as usize] += 1;
        false
    });
    let low: Vec<usize> = (1..200).filter(|&x| !bits.get(x)).collect();
    println!(
        "a_1..a_3 = 4, 5, 6 and a_(n+2) = S_n - y_n - y_(n+1), y_n = 2 - (floor(n/2) mod 2), {} terms to 2^{}: {:?} ...; windows a_(n+2) > S_n at n < 2: {}, at n >= 2: {}; least 2 (S_n - a_(n+1)) - a_(n+1) = {}; a_(n+3) - a_(n+2) - a_(n+1) over n >= 2 takes {:?}; chain a_(n+1) + y_n, n >= 3: {} of {} non-sums, first {:?}; non-sums below {} by lane seed/window/pair: {}/{}/{}; non-sums below 200: {:?}",
        a.len(), bits_, &a[..12], early, late, least, kicks, held, chain.len(), &chain[..8], top, lanes[0], lanes[1], lanes[2], low
    );
}

fn windowed_terms(cap: usize) -> (Vec<usize>, Vec<i64>) {
    let mut a = vec![4usize, 5, 6];
    let mut u = vec![0i64, 0];
    let rule = |n: usize, a: &[usize], u: &[i64]| -> i64 {
        let x = |m: usize| a[m] as i64 + u[m];
        match n % 5 {
            0 if n >= 5 => 2,
            1 if n >= 6 => -x(n - 2),
            2 if n >= 7 => x(n - 3),
            _ => 1,
        }
    };
    loop {
        let n = a.len() - 1;
        while u.len() <= n + 1 {
            let v = rule(u.len(), &a, &u);
            u.push(v);
        }
        let v = a[..n].iter().sum::<usize>() as i64 - u[n] - u[n + 1];
        if v as usize > cap {
            return (a, u);
        }
        a.push(v as usize);
    }
}

fn windowed(bits_: u32) {
    let (a, u) = windowed_terms(1 << bits_);
    let bits = missing(&a);
    let mut sums = vec![0usize];
    for &x in &a {
        sums.push(sums[sums.len() - 1] + x);
    }
    let len = a.len();
    let top = a[len - 1];
    let sorted = a.windows(2).all(|w| w[0] <= w[1]);
    let open: Vec<usize> = (2..len - 1).filter(|&n| a[n + 1] > sums[n]).collect();
    let fifth = open.iter().all(|&n| n % 5 == 0)
        && (2..len - 1).filter(|&n| n % 5 == 0).count() == open.len();
    let size = open
        .iter()
        .map(|&n| 1000 * (a[n + 1] - sums[n]) / a[n + 1])
        .min()
        .unwrap();
    let mut chains = 0;
    for &m in &open {
        for &n in &open {
            if n > m && sums[n] - a[n] < a[m + 1] && sums[m] < a[n + 1] - a[n] {
                chains += 1;
            }
        }
    }
    let held = open
        .iter()
        .filter(|&&n| {
            let mut hit = false;
            bits.each_zero(sums[n] + 1, a[n + 1] - 1, |_| {
                hit = true;
                true
            });
            hit
        })
        .count();
    let least = (3..len)
        .map(|n| 12 * (sums[n] as i64 - a[n] as i64) - a[n] as i64)
        .min()
        .unwrap();
    let steep = (1..len)
        .map(|n| a[n] as i64 - 3 * a[n - 1] as i64)
        .max()
        .unwrap();
    let mut kicks: Vec<i64> = (3..len - 2)
        .filter(|&n| n % 5 == 3)
        .map(|n| a[n + 2] as i64 - a[n + 1] as i64 - a[n] as i64)
        .collect();
    kicks.sort();
    kicks.dedup();
    let chain: Vec<usize> = (3..len - 1)
        .filter(|&n| ((a[n] as i64 + u[n]) as usize) < a[n + 1])
        .map(|n| (a[n] as i64 + u[n]) as usize)
        .collect();
    let gone = chain.iter().filter(|&&x| !bits.get(x)).count();
    let wide_a: Vec<u128> = a.iter().map(|&x| x as u128).collect();
    let pre: Vec<u128> = sums.iter().map(|&x| x as u128).collect();
    let mut lanes = [0usize; 3];
    bits.each_zero(1, top - 1, |z| {
        lanes[lane(&wide_a, &pre, z as u128) as usize] += 1;
        false
    });
    println!(
        "V: a_1..a_3 = 4, 5, 6, a_(n+2) = S_n - u_n - u_(n+1), u_n = 2, -x_(n-2), x_(n-3) at n = 0, 1, 2 mod 5 from n = 5, 6, 7, else 1, x_n = a_(n+1) + u_n; {} terms to 2^{}: {:?} ...; sorted {}; windows a_(n+2) > S_n at n >= 2: {} at {:?} ..., exactly n = 0 mod 5: {}; least (a_(n+2) - S_n)/a_(n+2) over them {}/1000; chained pairs {}; windows holding a non-sum {} of {}; least 12 (S_n - a_(n+1)) - a_(n+1) over n >= 3: {}; largest a_(n+1) - 3 a_n: {}; a_(n+3) - a_(n+2) - a_(n+1) at n = 3 mod 5, n >= 3, takes {:?}; chain x_n, 3 <= n, x_n below a_(n+2): {} of {} non-sums, first {:?}; non-sums below {}: {}, by lane seed/window/pair: {}/{}/{}",
        len, bits_, &a[..14], sorted, open.len(), &open[..open.len().min(6)], fifth, size, chains, held, open.len(), least, steep, kicks, gone, chain.len(), &chain[..8], top, lanes.iter().sum::<usize>(), lanes[0], lanes[1], lanes[2]
    );
}

// DEPTH

fn base_of(bases: &[u64], x: u128) -> u64 {
    for &d in bases {
        let mut y = x;
        while y.is_multiple_of(d as u128) {
            y /= d as u128;
        }
        if y == 1 {
            return d;
        }
    }
    0
}

fn trace(bases: &[u64], k: u32, bits: u32) {
    let a = elements(bases, k, far());
    let mut rows = vec![];
    let (v, _) = wide_with(bases, k, bits, |n, s, t| rows.push((n, s, t)));
    println!("{} k {} wide 2^{} {:?}", show(bases), k, bits, v);
    for (i, &(n, s, t)) in rows.iter().enumerate() {
        let next = a[n];
        let rise = rows.get(i + 1).is_some_and(|r| r.2 > t);
        println!(
            "n {} a_n {} = {}^. S_n {} a_(n+1) {} R_n {} T_n {} T_n/a_(n+1) {}.{:04}{}",
            n,
            a[n - 1],
            base_of(bases, a[n - 1]),
            s,
            next,
            s as i128 - next as i128,
            t,
            t / next,
            t % next * 10000 / next,
            if rise { " rises" } else { "" }
        );
    }
}

#[derive(Default, Clone)]
struct Jumps {
    rises: usize,
    pauses: usize,
    pauses_at_last: usize,
    last: usize,
    run: usize,
    open: usize,
    bad: usize,
    birth: usize,
    y: i128,
    deep: bool,
    fs: (u128, u128),
    peak: (u128, u128, usize),
}

fn frac(p: (u128, u128), places: u32) -> String {
    let m = 10u128.pow(places);
    format!(
        "{}.{:0w$}",
        p.0 / p.1,
        p.0 % p.1 * m / p.1,
        w = places as usize
    )
}

fn jumps_of(
    bases: &[u64],
    k: u32,
    a: &[u128],
    rows: &[(usize, u128, u128)],
    f: Option<u128>,
) -> Jumps {
    let mut j = Jumps {
        peak: (0, 1, 0),
        fs: (1, 1),
        ..Default::default()
    };
    let first = bases.iter().map(|&d| (d as u128).pow(k)).max().unwrap();
    let all = a.partition_point(|&x| x < first) + 1;
    let mut run = 0;
    for i in 0..rows.len() {
        let (n, s, t) = rows[i];
        let next = a[n];
        let d = s as i128 - next as i128;
        let ti = t as i128;
        if n >= all && t * j.peak.1 > j.peak.0 * next {
            j.peak = (t, next, n);
        }
        if i > 0 && t > rows[i - 1].2 && d > 2 * ti - 2 {
            j.bad += 1;
        }
        if let Some(f) = f {
            let s1 = if i + 1 < rows.len() {
                rows[i + 1].1
            } else {
                s + next
            };
            if s < 2 * f && 2 * f <= s1 {
                j.birth = n;
                j.y = f as i128 - next as i128;
                j.deep = ti - 1 > d;
                j.fs = (f, s);
            }
        }
        if i + 1 == rows.len() {
            break;
        }
        let (_, s1, t1) = rows[i + 1];
        let rise = t1 > t;
        if n >= all && !rise {
            j.pauses += 1;
        }
        if rise {
            j.last = n;
            j.pauses_at_last = j.pauses;
            j.rises += 1;
            run += 1;
            j.run = j.run.max(run);
            if t1 + t < s + 2 || t1 > s1 / 2 + 1 || d > 2 * ti - 2 {
                j.bad += 1;
            }
        } else {
            run = 0;
        }
        if d < ti - 1 && 2 * (ti - 1) < s as i128 && !rise {
            j.bad += 1;
        }
    }
    j.open = run;
    j
}

type Cell = (usize, u32, Verdict, Lanes, Jumps);

fn jumps(r: u64, kmax: u32, bits: u32, threads: usize) {
    let clock = Instant::now();
    let sets = minimal(r);
    let next = AtomicUsize::new(0);
    let out: Mutex<Vec<Cell>> = Mutex::new(vec![]);
    std::thread::scope(|scope| {
        for _ in 0..threads {
            scope.spawn(|| loop {
                let i = next.fetch_add(1, Ordering::SeqCst);
                if i >= sets.len() {
                    break;
                }
                for k in 1..=kmax {
                    let a = elements(&sets[i], k, far());
                    let mut rows = vec![];
                    let (v, lanes) = wide_with(&sets[i], k, bits, |n, s, t| rows.push((n, s, t)));
                    let f = match v {
                        Verdict::Found { f, .. } => Some(f),
                        Verdict::Cap => None,
                    };
                    let j = jumps_of(&sets[i], k, &a, &rows, f);
                    let stop = v == Verdict::Cap;
                    out.lock().unwrap().push((i, k, v, lanes, j));
                    if stop {
                        break;
                    }
                }
            });
        }
    });
    let mut out = out.into_inner().unwrap();
    out.sort_by_key(|x| (x.0, x.1));
    let mut bad = 0;
    let mut worst: Option<(u128, u128, String)> = None;
    let mut low: Option<(u128, u128, String)> = None;
    let mut gaps = vec![0usize; 64];
    let mut born = 0;
    let mut caps = (0usize, usize::MAX, 0usize, 0usize);
    let mut tally = vec![[0usize; 7]; kmax as usize + 1];
    for (i, k, v, lanes, j) in &out {
        let name = format!("{} k {}", show(&sets[*i]), k);
        bad += j.bad;
        match v {
            Verdict::Found { f, n, .. } => {
                if worst
                    .as_ref()
                    .is_none_or(|w| j.peak.0 * w.1 > w.0 * j.peak.1)
                {
                    worst = Some((j.peak.0, j.peak.1, name.clone()));
                }
                if *k >= 2 && low.as_ref().is_none_or(|w| j.peak.0 * w.1 < w.0 * j.peak.1) {
                    low = Some((j.peak.0, j.peak.1, name.clone()));
                }
                gaps[(n - j.last).min(63)] += 1;
                born += (j.birth == j.last) as usize;
                let row = &mut tally[*k as usize];
                row[0] += 1;
                row[1] += (j.y >= 0) as usize;
                row[2] += (lanes.last == 2) as usize;
                row[3] += (lanes.last == 1 && j.deep) as usize;
                row[5] += j.deep as usize;
                row[6] += (j.pauses_at_last == 0) as usize;
                row[4] +=
                    (j.y >= 0 && lanes.last != 2) as usize + (lanes.last == 1 && !j.deep) as usize;
                println!(
                    "{} F {} N {} rises {} longest run {}, steps without a rise from the entry of the last base to the last rise at m {}: {}; F born at b {}, F - a_(b+1) {}, T_b - 1 {} R_b, F/S_b {}, F by {}; peak T_n/a_(n+1) {} at n {}; bad {}",
                    name, f, n, j.rises, j.run, j.last, j.pauses_at_last, j.birth, j.y, if j.deep { ">" } else { "<=" }, frac(j.fs, 4), ["seed", "window", "pair"][lanes.last as usize], frac((j.peak.0, j.peak.1), 4), j.peak.2, j.bad
                );
            }
            Verdict::Cap => {
                caps.0 += 1;
                caps.1 = caps.1.min(j.open);
                caps.2 = caps.2.max(j.open);
                caps.3 += (j.open == j.rises) as usize;
                println!(
                "{} cap 2^{}: rises {} longest run {}, run open at the cap {}; peak T_n/a_(n+1) {} at n {}; bad {}",
                name, bits, j.rises, j.run, j.open, frac((j.peak.0, j.peak.1), 4), j.peak.2, j.bad
                )
            }
        }
        bad += tally[*k as usize][4];
        tally[*k as usize][4] = 0;
    }
    for (k, row) in tally.iter().enumerate().skip(1) {
        println!(
            "k = {}: {} certified cells; T rises at every step from the entry of the last base to the last rise in {}; T_b - 1 > R_b at the birth b of F in {}; F >= a_(b+1) in {}; F by the pair route in {}; F by the window route, each with T_b - 1 > R_b, in {}",
            k, row[0], row[6], row[5], row[1], row[2], row[3]
        );
    }
    if let Some(w) = worst {
        println!(
            "largest peak T_n/a_(n+1) after every base enters: {} at {}",
            frac((w.0, w.1), 4),
            w.2
        );
    }
    if let Some(w) = low {
        println!(
            "least peak T_n/a_(n+1) after every base enters over k >= 2: {} at {}",
            frac((w.0, w.1), 4),
            w.2
        );
    }
    let close: Vec<String> = gaps
        .iter()
        .enumerate()
        .filter(|x| *x.1 > 0)
        .map(|(g, c)| format!("{} steps in {}", g, c))
        .collect();
    println!("the certificate closes after the last rise of T: {}; F is born at the last rise in {} cells", close.join(", "), born);
    println!(
        "capped cells {}: every rise in one run open at the cap in {}, open runs of {} to {} rises",
        caps.0, caps.3, caps.1, caps.2
    );
    println!(
        "cells run {}; violations of the jump law and the route lemma {}; {:.1} s",
        out.len(),
        bad,
        clock.elapsed().as_secs_f64()
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
        "census" => census(num(2, 10), num(3, 4) as u32, num(4, 31) as u32, num(5, 4) as usize, false),
        "deep" => census(num(2, 10), num(3, 5) as u32, num(4, 31) as u32, num(5, 4) as usize, true),
        "wcell" => {
            let bases: Vec<u64> = args[2].split(',').map(|x| x.parse().unwrap()).collect();
            wcell(&bases, num(3, 1) as u32, num(4, 31) as u32)
        }
        "chainless" => chainless(num(2, 27) as u32),
        "forced" => forced(num(2, 26) as u32),
        "windowed" => windowed(num(2, 27) as u32),
        "jumps" => jumps(num(2, 10), num(3, 5) as u32, num(4, 31) as u32, num(5, 4) as usize),
        "trace" => {
            let bases: Vec<u64> = args[2].split(',').map(|x| x.parse().unwrap()).collect();
            trace(&bases, num(3, 1) as u32, num(4, 31) as u32)
        }
        "wcontrol" => wcontrol(num(2, 16) as u32),
        "split" => {
            let bases: Vec<u64> = args[2].split(',').map(|x| x.parse().unwrap()).collect();
            let x0: u128 = args[4].parse().unwrap();
            split(&bases, num(3, 1) as u32, x0, num(5, 64) as u128)
        }
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
        _ => println!("verbs census R K BITS THREADS, deep R K BITS THREADS, jumps R K BITS THREADS, trace D K BITS, windowed BITS, cell D K BITS, wcell D K BITS, chainless BITS, forced BITS, wcontrol BITS, split D K X COUNT, set D K B, windows D K E F, graham T P Q BITS, refute BITS, ternary N HI THREADS, probe N TOP THREADS, band LO HI W THREADS, offsets LO HI, control, route R E"),
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
    fn the_wide_certificate_matches_the_full_one_past_its_window() {
        let mut truncated = 0;
        for (bases, k) in [
            (&[3u64, 4, 5][..], 2),
            (&[3, 4, 6], 2),
            (&[3, 5, 6, 7], 2),
            (&[3, 4, 7, 8], 2),
        ] {
            let full = certify(bases, k, 1 << 26);
            let (v, lanes) = wide(bases, k, 20);
            assert_eq!(v, full);
            let Verdict::Found { gaps, sum, .. } = v else {
                panic!()
            };
            assert_eq!(gaps, lanes.seed + lanes.window + lanes.pair);
            truncated += (sum > 1 << 21) as usize;
        }
        assert!(truncated > 0);
    }

    #[test]
    fn a_multiset_with_surplus_full_spectrum_and_no_window_is_incomplete() {
        let a = chainless_terms(1 << 22);
        assert_eq!(&a[..10], &[4, 5, 6, 7, 12, 18, 31, 50, 80, 129]);
        let bits = missing(&a);
        let mut sums = vec![0usize];
        for &x in &a {
            sums.push(sums[sums.len() - 1] + x);
        }
        for n in 2..a.len() - 1 {
            assert!(a[n + 1] < sums[n]);
        }
        for n in 3..a.len() - 1 {
            assert!(a[n] + kick(n) < a[n + 1] && !bits.get(a[n] + kick(n)));
        }
    }

    #[test]
    fn windows_at_every_fifth_step_leave_the_chain_open() {
        let (a, u) = windowed_terms(1 << 22);
        assert_eq!(&a[..10], &[4, 5, 6, 7, 13, 19, 47, 54, 86, 153]);
        let bits = missing(&a);
        let mut sums = vec![0usize];
        for &x in &a {
            sums.push(sums[sums.len() - 1] + x);
        }
        for n in 2..a.len() - 1 {
            assert_eq!(a[n + 1] > sums[n], n % 5 == 0);
            assert!(12 * sums[n + 1] + 2 >= 13 * a[n + 1]);
        }
        for n in 3..a.len() - 1 {
            let x = (a[n] as i64 + u[n]) as usize;
            assert!(x < a[n + 1] && !bits.get(x));
        }
    }

    #[test]
    fn the_deepest_hole_rises_only_past_the_middle() {
        for (bases, k) in [(&[3u64, 4, 5][..], 3), (&[3, 4, 6], 2), (&[3, 5, 6, 7], 2)] {
            let a = elements(bases, k, far());
            let mut rows = vec![];
            let (v, lanes) = wide_with(bases, k, 20, |n, s, t| rows.push((n, s, t)));
            let Verdict::Found { f, .. } = v else {
                panic!()
            };
            let j = jumps_of(bases, k, &a, &rows, Some(f));
            assert_eq!(j.bad, 0);
            assert!(j.rises > 10);
            assert!(j.y < 0 || lanes.last == 2);
            assert!(lanes.last != 1 || j.deep);
        }
    }

    #[test]
    fn a_set_below_the_threshold_never_certifies() {
        assert_eq!(certify(&[3, 4], 1, 1 << 22), Verdict::Cap);
        assert_eq!(certify(&[3, 6, 9, 12, 15, 21], 1, 1 << 22), Verdict::Cap);
    }
}
