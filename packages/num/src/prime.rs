use crate::factor::{divisors, factorize_wide};
use crate::series::li;
use serde::{Deserialize, Serialize};

pub fn is_prime(number: usize) -> bool {
    if number < 2 {
        return false;
    }
    if number < 4 {
        return true;
    }
    if number.is_multiple_of(2) || number.is_multiple_of(3) {
        return false;
    }
    let mut step = 5;
    while step <= number / step {
        if number.is_multiple_of(step) || number.is_multiple_of(step + 2) {
            return false;
        }
        step += 6;
    }
    true
}

pub fn prime_from(number: usize) -> usize {
    let mut n = number.max(2);
    while !is_prime(n) {
        n += 1;
    }
    n
}

pub fn rectangles(number: usize) -> Vec<(usize, usize)> {
    divisors(number as u64)
        .into_iter()
        .map(|side| side as usize)
        .filter(|&a| a <= number / a)
        .map(|a| (a, number / a))
        .collect()
}

pub fn splits(number: usize) -> Vec<(usize, usize)> {
    let mut out = Vec::new();
    if number < 4 {
        return out;
    }
    for p in primes(number / 2) {
        if is_prime(number - p) {
            out.push((p, number - p));
        }
    }
    out
}

pub fn squares(number: usize) -> Option<(usize, usize)> {
    let mut a = 1;
    while 2 * a * a <= number {
        let rest = number - a * a;
        let b = rest.isqrt();
        if b * b == rest {
            return Some((a, b));
        }
        a += 1;
    }
    None
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Prime {
    pub value: usize,
    pub index: usize,
    pub gap: usize,
    pub twin: bool,
    pub squares: Option<(usize, usize)>,
}

pub fn study(limit: usize) -> Vec<Prime> {
    let list = primes(limit);
    list.iter()
        .enumerate()
        .map(|(i, &value)| Prime {
            value,
            index: i + 1,
            gap: if i == 0 { 0 } else { value - list[i - 1] },
            twin: is_prime(value - 2) || is_prime(value + 2),
            squares: squares(value),
        })
        .collect()
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Sieve {
    types: Vec<u8>,
    at: usize,
    rank: usize,
    struck: usize,
    done: bool,
}

impl Sieve {
    pub fn new(limit: usize) -> Sieve {
        let mut sieve = Sieve {
            types: vec![0; limit + 1],
            at: 2,
            rank: 0,
            struck: 0,
            done: false,
        };
        sieve.settle();
        sieve
    }
    fn settle(&mut self) {
        while self.at < self.types.len() && self.types[self.at] != 0 {
            self.at += 1;
        }
        if self.at * self.at >= self.types.len() {
            for number in 2..self.types.len() {
                if self.types[number] == 0 {
                    self.types[number] = 1;
                }
            }
            self.done = true;
        }
    }
    pub fn step(&mut self) -> usize {
        if self.done {
            return 0;
        }
        let prime = self.at;
        self.rank += 1;
        self.types[prime] = 1;
        self.struck = 0;
        let mark = (self.rank + 1).min(255) as u8;
        let mut multiple = prime * prime;
        while multiple < self.types.len() {
            if self.types[multiple] == 0 {
                self.types[multiple] = mark;
                self.struck += 1;
            }
            multiple += prime;
        }
        self.settle();
        prime
    }
    pub fn finish(&mut self) {
        while !self.done {
            self.step();
        }
    }
    pub fn done(&self) -> bool {
        self.done
    }
    pub fn types(&self) -> &[u8] {
        &self.types
    }
    pub fn count(&self) -> usize {
        self.types.iter().filter(|&&t| t == 1).count()
    }
    pub fn struck(&self) -> usize {
        self.struck
    }
    pub fn rank(&self) -> usize {
        self.rank
    }
}

pub fn flags(limit: usize) -> Vec<bool> {
    let mut sieve = Sieve::new(limit);
    sieve.finish();
    sieve.types().iter().map(|&t| t == 1).collect()
}

pub fn primes(limit: usize) -> Vec<usize> {
    flags(limit)
        .iter()
        .enumerate()
        .filter(|&(_, &prime)| prime)
        .map(|(number, _)| number)
        .collect()
}

pub fn prime_count(n: usize) -> usize {
    primes(n).len()
}

fn pairs(flag: &[bool], number: usize) -> usize {
    (2..=number / 2)
        .filter(|&p| flag[p] && flag[number - p])
        .count()
}

pub fn goldbach(number: usize) -> usize {
    if number < 4 {
        return 0;
    }
    pairs(&flags(number), number)
}

pub fn goldbach_record(top: usize) -> Vec<usize> {
    if top < 4 {
        return Vec::new();
    }
    let flag = flags(top);
    (2..=top / 2).map(|k| pairs(&flag, 2 * k)).collect()
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Pile {
    pub number: u64,
    pub factors: Vec<(u64, u32)>,
    pub prime: bool,
    pub rectangles: Vec<(u64, u64)>,
}

pub fn pile(number: u64) -> Pile {
    let factors = factorize_wide(number);
    let rectangles = divisors(number)
        .into_iter()
        .filter(|&a| a <= number / a)
        .map(|a| (a, number / a))
        .collect();
    Pile {
        number,
        prime: factors.len() == 1 && factors[0].1 == 1,
        factors,
        rectangles,
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct Reading {
    pub x: usize,
    pub pi: usize,
    pub ratio: f64,
    pub li: f64,
}

pub fn chart(top: usize, bins: usize) -> Vec<Reading> {
    let list = primes(top);
    let step = (top / bins.max(1)).max(1);
    let mut out = Vec::new();
    let mut x = step;
    while x <= top {
        if x >= 2 {
            out.push(Reading {
                x,
                pi: list.partition_point(|&p| p <= x),
                ratio: x as f64 / (x as f64).ln(),
                li: li(x as f64),
            });
        }
        if x == top {
            break;
        }
        x = (x + step).min(top);
    }
    out
}
