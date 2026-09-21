use std::collections::HashSet;
use std::env;
use std::time::Instant;

// BITSET

struct Bits {
    top: u64,
    w: Vec<u64>,
}

impl Bits {
    fn new(top: u64) -> Bits {
        let words = (top / 64 + 1) as usize;
        Bits {
            top,
            w: vec![0; words],
        }
    }

    fn set(&mut self, x: u64) {
        self.w[(x / 64) as usize] |= 1u64 << (x % 64);
    }

    fn get(&self, x: u64) -> bool {
        (self.w[(x / 64) as usize] >> (x % 64)) & 1 == 1
    }

    fn mask(&mut self) {
        let r = self.top % 64;
        let last = self.w.len() - 1;
        if r != 63 {
            self.w[last] &= (1u64 << (r + 1)) - 1;
        }
    }

    fn or_shift(&mut self, sh: u64) {
        if sh > self.top {
            return;
        }
        let wsh = (sh / 64) as usize;
        let bsh = (sh % 64) as u32;
        let len = self.w.len();
        for i in (wsh..len).rev() {
            let src = i - wsh;
            let mut v = self.w[src] << bsh;
            if bsh > 0 && src > 0 {
                v |= self.w[src - 1] >> (64 - bsh);
            }
            self.w[i] |= v;
        }
        self.mask();
    }

    fn count(&self) -> u64 {
        self.w.iter().map(|x| x.count_ones() as u64).sum()
    }

    fn count_to(&self, d: u64) -> u64 {
        let full = (d / 64) as usize;
        let head: u64 = self.w[..full].iter().map(|x| x.count_ones() as u64).sum();
        let r = d % 64;
        let tail = if r == 63 {
            self.w[full]
        } else {
            self.w[full] & ((1u64 << (r + 1)) - 1)
        };
        head + tail.count_ones() as u64
    }
}

// DESIGNS

fn powers(base: u64, top: u64) -> Vec<u64> {
    let mut v = vec![1u64];
    while v
        .last()
        .unwrap()
        .checked_mul(base)
        .map_or(false, |p| p <= top)
    {
        v.push(v.last().unwrap() * base);
    }
    v
}

fn members(base: u64, top: u64) -> Vec<u64> {
    let mut list = vec![0u64];
    for p in powers(base, top) {
        let n = list.len();
        for i in 0..n {
            let x = list[i] + p;
            if x <= top {
                list.push(x);
            }
        }
    }
    list.sort_unstable();
    list
}

fn sumset(top: u64, direct: u64, shifted: u64) -> Bits {
    let mut s = Bits::new(top);
    for a in members(direct, top) {
        s.set(a);
    }
    for p in powers(shifted, top) {
        s.or_shift(p);
    }
    s
}

fn sumset_direct(top: u64) -> Bits {
    let mut s = Bits::new(top);
    let a = members(3, top);
    let b = members(4, top);
    for &x in &a {
        for &y in &b {
            if x + y <= top {
                s.set(x + y);
            }
        }
    }
    s
}

// SCAN

#[derive(Clone, Copy)]
struct Ext {
    c: u64,
    x: u64,
}

impl Ext {
    fn none() -> Ext {
        Ext { c: 0, x: 0 }
    }

    fn above(&self, c: u64, x: u64) -> bool {
        self.x == 0 || (c as u128) * (self.x as u128) > (self.c as u128) * (x as u128)
    }

    fn below(&self, c: u64, x: u64) -> bool {
        self.x == 0 || (c as u128) * (self.x as u128) < (self.c as u128) * (x as u128)
    }
}

#[derive(Clone, Copy)]
struct Window {
    lo: u64,
    hi: u64,
    max: Ext,
    min: Ext,
}

impl Window {
    fn new(lo: u64, hi: u64) -> Window {
        Window {
            lo,
            hi,
            max: Ext::none(),
            min: Ext::none(),
        }
    }

    fn feed(&mut self, c: u64, x: u64, up: bool, down: bool) {
        if up && self.max.above(c, x) {
            self.max = Ext { c, x };
        }
        if down && self.min.below(c, x) {
            self.min = Ext { c, x };
        }
    }
}

struct Scan {
    marks: Vec<(u64, u64)>,
    windows3: Vec<Window>,
    windows2: Vec<Window>,
    global: Window,
}

fn window_list(base: u64, top: u64) -> Vec<Window> {
    let p = powers(base, top);
    let mut v = Vec::new();
    for i in 0..p.len() {
        let lo = p[i];
        let hi = if i + 1 < p.len() { p[i + 1] - 1 } else { top };
        v.push(Window::new(lo, hi));
    }
    v
}

fn scan(s: &Bits, marks: &[u64]) -> Scan {
    let top = s.top;
    let windows3 = window_list(3, top);
    let windows2 = window_list(2, top);
    let mut special: HashSet<u64> = HashSet::new();
    for &m in marks {
        special.insert(m / 64);
    }
    for w in windows3.iter().chain(windows2.iter()) {
        special.insert(w.lo / 64);
        special.insert(w.hi / 64);
        if w.lo > 0 {
            special.insert((w.lo - 1) / 64);
        }
    }
    special.insert(0);
    special.insert(top / 64);
    let mut markset: Vec<u64> = marks.to_vec();
    markset.sort_unstable();
    markset.dedup();
    let mut sc = Scan {
        marks: Vec::new(),
        windows3,
        windows2,
        global: Window::new(1, top),
    };
    let mut i3 = 0usize;
    let mut i2 = 0usize;
    let mut mi = 0usize;
    let mut c: u64 = 0;
    for (wi, &word) in s.w.iter().enumerate() {
        let base = (wi as u64) * 64;
        if !special.contains(&(wi as u64)) {
            if word == 0 {
                let x = base + 63;
                sc.windows3[i3].feed(c, x, false, true);
                sc.windows2[i2].feed(c, x, false, true);
                sc.global.feed(c, x, false, true);
            } else if word == u64::MAX {
                c += 64;
                let x = base + 63;
                sc.windows3[i3].feed(c, x, true, false);
                sc.windows2[i2].feed(c, x, true, false);
                sc.global.feed(c, x, true, false);
            } else {
                for j in 0..64u64 {
                    let x = base + j;
                    let on = (word >> j) & 1 == 1;
                    if on {
                        c += 1;
                    }
                    sc.windows3[i3].feed(c, x, on, !on);
                    sc.windows2[i2].feed(c, x, on, !on);
                    sc.global.feed(c, x, on, !on);
                }
            }
            continue;
        }
        for j in 0..64u64 {
            let x = base + j;
            if x == 0 {
                continue;
            }
            if x > top {
                break;
            }
            if (word >> j) & 1 == 1 {
                c += 1;
            }
            while i3 + 1 < sc.windows3.len() && x >= sc.windows3[i3 + 1].lo {
                i3 += 1;
            }
            while i2 + 1 < sc.windows2.len() && x >= sc.windows2[i2 + 1].lo {
                i2 += 1;
            }
            sc.windows3[i3].feed(c, x, true, true);
            sc.windows2[i2].feed(c, x, true, true);
            sc.global.feed(c, x, true, true);
            while mi < markset.len() && markset[mi] == x {
                sc.marks.push((x, c));
                mi += 1;
            }
        }
    }
    sc
}

// PRINT

fn dens(c: u64, x: u64) -> String {
    let q = (c as u128) * 1_000_000u128 / (x as u128);
    format!("{}.{:06}", q / 1_000_000, q % 1_000_000)
}

fn expo(c: u64, x: u64) -> String {
    if x < 2 || c == 0 {
        return "-".to_string();
    }
    let e = (c as f64).ln() / (x as f64).ln();
    let t = (e * 1_000_000.0).floor() / 1_000_000.0;
    format!("{:.6}", t)
}

fn print_window(tag: &str, i: usize, w: &Window) {
    println!(
        "{} {:>2} [{}, {}] max {} at x = {} c = {} min {} at x = {} c = {}",
        tag,
        i,
        w.lo,
        w.hi,
        dens(w.max.c, w.max.x),
        w.max.x,
        w.max.c,
        dens(w.min.c, w.min.x),
        w.min.x,
        w.min.c
    );
}

fn centres(top: u64) -> Vec<(u32, u32, u64, bool)> {
    let p3 = powers(3, top * 3);
    let p4 = powers(4, top * 4);
    let mut v = Vec::new();
    for s in 1..p4.len() {
        for r in 1..p3.len() {
            let d = (p3[r] - 1) / 2 + (p4[s] - 1) / 3;
            if d > top {
                continue;
            }
            let clean = p3[r] > d && p4[s] > d;
            if p3[r] < 3 * p4[s] && p4[s] < 3 * p3[r] {
                v.push((r as u32, s as u32, d, clean));
            }
        }
    }
    v.sort_by_key(|t| t.2);
    v
}

fn density(k: u32) {
    let t0 = Instant::now();
    let top = 3u64.pow(k);
    let s = sumset(top, 3, 4);
    let built = t0.elapsed().as_secs_f64();
    let p3 = powers(3, top);
    let p4 = powers(4, top);
    let mut marks: Vec<u64> = Vec::new();
    marks.extend(p3.iter().copied());
    marks.extend(p4.iter().copied());
    marks.extend(p3.iter().map(|p| p / 2));
    marks.extend(p4.iter().map(|p| p / 3));
    let cs = centres(top);
    marks.extend(cs.iter().map(|t| t.2));
    marks.retain(|&x| x >= 1);
    let sc = scan(&s, &marks);
    let at = |x: u64| -> u64 { sc.marks.iter().find(|m| m.0 == x).map(|m| m.1).unwrap() };
    println!("top 3^{} = {}", k, top);
    println!("members of A up to top {}", members(3, top).len());
    println!("members of B up to top {}", members(4, top).len());
    println!("card(S meet [1, top]) {}", s.count() - 1);
    for (i, &x) in p3.iter().enumerate() {
        let c = at(x);
        println!(
            "3^{:<2} x = {} c = {} D = {} exponent {}",
            i,
            x,
            c,
            dens(c, x),
            expo(c, x)
        );
    }
    for (i, &x) in p4.iter().enumerate() {
        let c = at(x);
        println!(
            "4^{:<2} x = {} c = {} D = {} exponent {}",
            i,
            x,
            c,
            dens(c, x),
            expo(c, x)
        );
    }
    for (i, &p) in p3.iter().enumerate().skip(1) {
        let x = p / 2;
        let c = at(x);
        println!("3^{:<2}/2 x = {} c = {} D = {}", i, x, c, dens(c, x));
    }
    for (i, &p) in p4.iter().enumerate().skip(1) {
        let x = p / 3;
        let c = at(x);
        println!("4^{:<2}/3 x = {} c = {} D = {}", i, x, c, dens(c, x));
    }
    for &(r, sx, d, clean) in &cs {
        let c = at(d);
        println!(
            "centre r = {:<2} s = {:<2} 4^s/3^r = {} d = {} c = {} D = {} {}",
            r,
            sx,
            ratio(r, sx),
            d,
            c,
            dens(c, d),
            if clean { "clean" } else { "mixed" }
        );
    }
    for (i, w) in sc.windows3.iter().enumerate() {
        print_window("window3", i, w);
    }
    for (i, w) in sc.windows2.iter().enumerate() {
        print_window("window2", i, w);
    }
    print_window("global", 0, &sc.global);
    println!(
        "built in {:.2} s, scanned in {:.2} s",
        built,
        t0.elapsed().as_secs_f64() - built
    );
}

// ENERGY

fn zeros3(mut t: i64, k: u32) -> Option<u32> {
    let mut z = 0;
    for _ in 0..k {
        let r = t.rem_euclid(3);
        t = (t - r) / 3;
        if r == 2 {
            t += 1;
        } else if r == 0 {
            z += 1;
        }
    }
    if t == 0 {
        Some(z)
    } else {
        None
    }
}

fn energy(k: u32, m: u32) -> u128 {
    let p4: Vec<i64> = (0..m).map(|i| 4i64.pow(i)).collect();
    let mut e: u128 = 0;
    let mut digits = vec![-1i64; m as usize];
    loop {
        let mut t = 0i64;
        let mut z4 = 0u32;
        for i in 0..m as usize {
            t += digits[i] * p4[i];
            if digits[i] == 0 {
                z4 += 1;
            }
        }
        if let Some(z3) = zeros3(t, k) {
            e += 1u128 << (z3 + z4);
        }
        let mut i = 0usize;
        loop {
            if i == m as usize {
                return e;
            }
            if digits[i] < 1 {
                digits[i] += 1;
                break;
            }
            digits[i] = -1;
            i += 1;
        }
    }
}

#[cfg(test)]
fn energy_direct(k: u32, m: u32) -> u128 {
    let a = members(3, 3u64.pow(k) - 1);
    let b = members(4, 4u64.pow(m) - 1);
    let d = (3u64.pow(k) - 1) / 2 + (4u64.pow(m) - 1) / 3;
    let mut r = vec![0u64; d as usize + 1];
    for &x in &a {
        for &y in &b {
            r[(x + y) as usize] += 1;
        }
    }
    r.iter().map(|&v| (v as u128) * (v as u128)).sum()
}

fn six(num: u128, den: u128) -> String {
    let q = num * 1_000_000 / den;
    format!("{}.{:06}", q / 1_000_000, q % 1_000_000)
}

fn six_up(num: u128, den: u128) -> String {
    let q = (num * 1_000_000 + den - 1) / den;
    format!("{}.{:06}", q / 1_000_000, q % 1_000_000)
}

fn ratio(r: u32, m: u32) -> String {
    six_up(4u128.pow(m) * 1_000_000, 3u128.pow(r) * 1_000_000)
}

fn energies(kmax: u32) {
    let t0 = Instant::now();
    let top = 3u64.pow(kmax);
    let s = sumset(top, 3, 4);
    let mut first: Vec<Option<(u32, f64)>> = vec![None, None];
    let mut last: Option<(u32, f64)> = None;
    for (r, m, d, clean) in centres(top) {
        if r < 4 {
            continue;
        }
        let e = energy(r, m);
        let pairs = 1u128 << (r + m);
        let card = s.count_to(d) as u128;
        let range = d as u128 + 1;
        println!(
            "k = {:<2} m = {:<2} {} 4^m/3^k = {} d = {} card = {} fill {} energy {} random {} ratio {} bound {}",
            r,
            m,
            if clean { "clean" } else { "mixed" },
            ratio(r, m),
            d,
            card,
            six(card, range),
            e,
            six(pairs * pairs, range),
            six_up(e * range, pairs * pairs),
            six(pairs * pairs, e * range)
        );
        let q = (e as f64) * (range as f64) / (pairs as f64) / (pairs as f64);
        for (i, start) in [6u32, 11].iter().enumerate() {
            if r >= *start && first[i].is_none() {
                first[i] = Some((r, q));
            }
        }
        if r >= 6 {
            last = Some((r, q));
        }
    }
    for f in first {
        if let (Some((k0, q0)), Some((k1, q1))) = (f, last) {
            let eta = (q1 / q0).ln() / 3f64.ln() / ((k1 - k0) as f64);
            println!(
                "growth of Q from k = {} to k = {} as 3^(eta k): eta = {:.6}",
                k0,
                k1,
                (eta * 1_000_000.0).ceil() / 1_000_000.0
            );
        }
    }
    println!("{:.2} s", t0.elapsed().as_secs_f64());
}

// CONTROL

const OEIS_A367090: [u64; 58] = [
    62, 63, 143, 144, 207, 208, 209, 210, 211, 212, 213, 214, 215, 216, 217, 218, 219, 220, 221,
    222, 223, 224, 225, 226, 227, 228, 229, 230, 231, 232, 233, 234, 235, 236, 237, 238, 239, 240,
    241, 242, 463, 464, 465, 466, 467, 468, 469, 470, 471, 472, 473, 474, 475, 476, 477, 478, 479,
    480,
];

fn same(a: &Bits, b: &Bits) -> bool {
    a.top == b.top && a.w == b.w
}

fn symmetric(s: &Bits, d: u64) -> bool {
    (0..=d).all(|x| s.get(x) == s.get(d - x))
}

fn complement(s: &Bits, n: usize) -> Vec<u64> {
    (1..=s.top).filter(|&x| !s.get(x)).take(n).collect()
}

fn control() {
    let t0 = Instant::now();
    let top = 3u64.pow(13);
    let direct = sumset_direct(top);
    let shifted = sumset(top, 3, 4);
    println!(
        "double loop against shift-or at 3^13: {}",
        if same(&direct, &shifted) {
            "agree"
        } else {
            "DIFFER"
        }
    );
    let top = 3u64.pow(17);
    let ab = sumset(top, 3, 4);
    let ba = sumset(top, 4, 3);
    println!(
        "A direct with B shifted against B direct with A shifted at 3^17: {}",
        if same(&ab, &ba) { "agree" } else { "DIFFER" }
    );
    let gaps = complement(&ab, OEIS_A367090.len());
    println!(
        "first {} non-members against A367090: {}",
        OEIS_A367090.len(),
        if gaps == OEIS_A367090 {
            "agree"
        } else {
            "DIFFER"
        }
    );
    for (r, s, d, clean) in centres(top) {
        println!(
            "centre r = {:<2} s = {:<2} d = {} {} symmetric {}",
            r,
            s,
            d,
            if clean { "clean" } else { "mixed" },
            symmetric(&ab, d)
        );
    }
    println!("{:.2} s", t0.elapsed().as_secs_f64());
}

fn main() {
    let args: Vec<String> = env::args().collect();
    match args.get(1).map(|s| s.as_str()) {
        Some("density") => density(args.get(2).and_then(|s| s.parse().ok()).unwrap_or(17)),
        Some("control") => control(),
        Some("energy") => energies(args.get(2).and_then(|s| s.parse().ok()).unwrap_or(17)),
        _ => println!("verbs: density K, energy K, control"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shift_or_matches_double_loop() {
        let top = 3u64.pow(9);
        assert!(same(&sumset_direct(top), &sumset(top, 3, 4)));
        assert!(same(&sumset_direct(top), &sumset(top, 4, 3)));
    }

    #[test]
    fn first_gaps_are_a367090() {
        let s = sumset(3u64.pow(7), 3, 4);
        assert_eq!(complement(&s, OEIS_A367090.len()), OEIS_A367090.to_vec());
    }

    #[test]
    fn member_counts_are_powers_of_two() {
        assert_eq!(members(3, 3u64.pow(10) - 1).len(), 1 << 10);
        assert_eq!(members(4, 4u64.pow(6) - 1).len(), 1 << 6);
    }

    #[test]
    fn scan_counts_match_popcount() {
        let top = 3u64.pow(9);
        let s = sumset(top, 3, 4);
        let sc = scan(&s, &[top]);
        assert_eq!(sc.marks, vec![(top, s.count() - 1)]);
    }

    #[test]
    fn scan_extremes_match_direct_search() {
        let top = 3u64.pow(9);
        let s = sumset(top, 3, 4);
        let sc = scan(&s, &[]);
        for w in sc.windows3.iter().chain(sc.windows2.iter()) {
            let mut c = (1..w.lo).filter(|&x| s.get(x)).count() as u64;
            let mut best_max = Ext::none();
            let mut best_min = Ext::none();
            for x in w.lo..=w.hi {
                if s.get(x) {
                    c += 1;
                }
                if best_max.above(c, x) {
                    best_max = Ext { c, x };
                }
                if best_min.below(c, x) {
                    best_min = Ext { c, x };
                }
            }
            assert_eq!((w.max.c, w.max.x), (best_max.c, best_max.x));
            assert_eq!((w.min.c, w.min.x), (best_min.c, best_min.x));
        }
    }

    #[test]
    fn energy_matches_representation_histogram() {
        for (k, m) in [(4, 3), (6, 5), (9, 7)] {
            assert_eq!(energy(k, m), energy_direct(k, m));
        }
    }

    #[test]
    fn prefix_count_matches_bit_scan() {
        let s = sumset(3u64.pow(9), 3, 4);
        for d in [0u64, 63, 64, 705, 15302, 19683] {
            assert_eq!(s.count_to(d), (0..=d).filter(|&x| s.get(x)).count() as u64);
        }
    }

    #[test]
    fn clean_centres_are_symmetric() {
        let top = 3u64.pow(11);
        let s = sumset(top, 3, 4);
        for (_, _, d, clean) in centres(top) {
            if clean {
                assert!(symmetric(&s, d));
            }
        }
    }
}
