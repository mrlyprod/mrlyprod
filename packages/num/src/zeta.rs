use crate::prime::primes;
use crate::series::bernoulli;
use serde::{Deserialize, Serialize};
use std::f64::consts::PI;
use std::ops::{Add, Div, Mul, Neg, Sub};

pub const JOIN: f64 = 20.0;
const SHIFT: usize = 10;
const TAIL: usize = 7;
const STEPS: usize = 10;
const STENCIL: f64 = 0.02;
const TOLERANCE: f64 = 1e-9;
const NODES: usize = 4096;
const REMAINDER: f64 = 1.0;

#[derive(Clone, Copy, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Complex {
    pub re: f64,
    pub im: f64,
}

impl Complex {
    pub const fn new(re: f64, im: f64) -> Complex {
        Complex { re, im }
    }
    pub fn abs(z: Complex) -> f64 {
        z.re.hypot(z.im)
    }
    pub fn arg(z: Complex) -> f64 {
        z.im.atan2(z.re)
    }
    pub fn exp(z: Complex) -> Complex {
        let r = z.re.exp();
        Complex::new(r * z.im.cos(), r * z.im.sin())
    }
    pub fn ln(z: Complex) -> Complex {
        Complex::new(Complex::abs(z).ln(), Complex::arg(z))
    }
    pub fn turn(angle: f64) -> Complex {
        Complex::new(angle.cos(), angle.sin())
    }
}

impl Add for Complex {
    type Output = Complex;
    fn add(self, other: Complex) -> Complex {
        Complex::new(self.re + other.re, self.im + other.im)
    }
}

impl Add<f64> for Complex {
    type Output = Complex;
    fn add(self, other: f64) -> Complex {
        Complex::new(self.re + other, self.im)
    }
}

impl Sub for Complex {
    type Output = Complex;
    fn sub(self, other: Complex) -> Complex {
        Complex::new(self.re - other.re, self.im - other.im)
    }
}

impl Sub<f64> for Complex {
    type Output = Complex;
    fn sub(self, other: f64) -> Complex {
        Complex::new(self.re - other, self.im)
    }
}

impl Mul for Complex {
    type Output = Complex;
    fn mul(self, other: Complex) -> Complex {
        Complex::new(
            self.re * other.re - self.im * other.im,
            self.re * other.im + self.im * other.re,
        )
    }
}

impl Mul<f64> for Complex {
    type Output = Complex;
    fn mul(self, other: f64) -> Complex {
        Complex::new(self.re * other, self.im * other)
    }
}

impl Div for Complex {
    type Output = Complex;
    fn div(self, other: Complex) -> Complex {
        let d = other.re * other.re + other.im * other.im;
        Complex::new(
            (self.re * other.re + self.im * other.im) / d,
            (self.im * other.re - self.re * other.im) / d,
        )
    }
}

impl Neg for Complex {
    type Output = Complex;
    fn neg(self) -> Complex {
        Complex::new(-self.re, -self.im)
    }
}

pub fn raise(base: f64, exponent: Complex) -> Complex {
    Complex::exp(exponent * base.ln())
}

pub fn kernel(p: f64) -> f64 {
    let d = if p < 0.5 { 0.25 - p } else { p - 0.75 };
    let below = (2.0 * PI * d).sin();
    if below == 0.0 {
        return 0.5;
    }
    (PI * d * (1.0 + 2.0 * d)).sin() / below
}

fn difference(order: usize, p: f64, step: f64) -> f64 {
    let mut sum = 0.0;
    let mut binomial = 1.0;
    for k in 0..=order {
        let offset = (order as f64 / 2.0 - k as f64) * step;
        let sign = if k.is_multiple_of(2) { 1.0 } else { -1.0 };
        sum += sign * binomial * kernel(p + offset);
        binomial = binomial * (order - k) as f64 / (k + 1) as f64;
    }
    sum / step.powi(order as i32)
}

fn derivative(order: usize, p: f64) -> f64 {
    (4.0 * difference(order, p, STENCIL) - difference(order, p, 2.0 * STENCIL)) / 3.0
}

pub fn corrections(p: f64) -> [f64; 4] {
    let (pi2, pi4, pi6) = (PI * PI, PI.powi(4), PI.powi(6));
    [
        kernel(p),
        -derivative(3, p) / (96.0 * pi2),
        derivative(6, p) / (18_432.0 * pi4) + derivative(2, p) / (64.0 * pi2),
        -derivative(9, p) / (5_308_416.0 * pi6)
            - derivative(5, p) / (3_840.0 * pi4)
            - derivative(1, p) / (64.0 * pi2),
    ]
}

#[derive(Serialize, Deserialize)]
pub struct Line {
    bern: Vec<f64>,
    tail: Vec<f64>,
}

impl Default for Line {
    fn default() -> Line {
        Line::new()
    }
}

impl Line {
    pub fn new() -> Line {
        let fractions = bernoulli(2 * TAIL + 1).unwrap_or_default();
        let bern: Vec<f64> = fractions
            .iter()
            .step_by(2)
            .map(|&(num, den)| num as f64 / den as f64)
            .collect();
        let mut factorial = 1.0;
        let tail = bern
            .iter()
            .enumerate()
            .map(|(k, &value)| {
                if k > 0 {
                    factorial *= ((2 * k - 1) * 2 * k) as f64;
                }
                value / factorial
            })
            .collect();
        Line { bern, tail }
    }
    pub fn theta(&self, t: f64) -> f64 {
        let z = Complex::new(0.25, 0.5 * t);
        let w = z + SHIFT as f64;
        let mut arg = ((w - 0.5) * Complex::ln(w) - w).im;
        for k in 0..SHIFT {
            arg -= Complex::arg(z + k as f64);
        }
        let inverse = Complex::new(1.0, 0.0) / w;
        let square = inverse * inverse;
        let mut power = inverse;
        for k in 1..=TAIL {
            arg += self.bern[k] / ((2 * k * (2 * k - 1)) as f64) * power.im;
            power = power * square;
        }
        arg - 0.5 * t * PI.ln()
    }
    pub fn maclaurin(&self, t: f64) -> Complex {
        let s = Complex::new(0.5, t);
        let count = t.abs() as usize + SHIFT;
        let mut sum = Complex::default();
        for k in 1..=count {
            sum = sum + raise(k as f64, -s);
        }
        let base = count as f64;
        let mut out = sum + raise(base, -s + 1.0) / (s - 1.0) - raise(base, -s) * 0.5;
        let mut rising = s;
        let mut power = raise(base, -s - 1.0);
        let square = 1.0 / (base * base);
        for k in 1..=TAIL {
            out = out + rising * power * self.tail[k];
            rising = rising * (s + (2 * k - 1) as f64) * (s + (2 * k) as f64);
            power = power * square;
        }
        out
    }
    pub fn pair(&self, s: Complex) -> (Complex, Complex) {
        let one = Complex::new(1.0, 0.0);
        let count = s.im.abs() as usize + SHIFT;
        let mut value = Complex::default();
        let mut slope = Complex::default();
        for k in 1..=count {
            let kf = k as f64;
            let term = raise(kf, -s);
            value = value + term;
            slope = slope - term * kf.ln();
        }
        let base = count as f64;
        let log = base.ln();
        let head = raise(base, -s + 1.0) / (s - 1.0);
        let half = raise(base, -s) * 0.5;
        value = value + head - half;
        slope = slope - head * log - head / (s - 1.0) + half * log;
        let mut rising = s;
        let mut ratio = one / s;
        let mut power = raise(base, -s - 1.0);
        let square = 1.0 / (base * base);
        for (k, &weight) in self.tail.iter().enumerate().skip(1) {
            let term = rising * power * weight;
            value = value + term;
            slope = slope + term * (ratio - log);
            let (a, b) = (s + (2 * k - 1) as f64, s + (2 * k) as f64);
            rising = rising * a * b;
            ratio = ratio + one / a + one / b;
            power = power * square;
        }
        (value, slope)
    }
    pub fn novelty_coefficients(&self, gammas: &[f64]) -> Vec<Complex> {
        gammas
            .iter()
            .map(|&g| {
                let rho = Complex::new(0.5, g);
                let (left, _) = self.pair(rho - 1.0);
                let (_, prime) = self.pair(rho);
                mellin(rho) * left / prime
            })
            .collect()
    }
    pub fn exact(&self, t: f64) -> f64 {
        (Complex::turn(self.theta(t)) * self.maclaurin(t)).re
    }
    pub fn siegel(&self, t: f64) -> f64 {
        let whole = (t / (2.0 * PI)).sqrt().floor();
        let theta = self.theta(t);
        let mut sum = 0.0;
        for k in 1..=whole as usize {
            let kf = k as f64;
            sum += (theta - t * kf.ln()).cos() / kf.sqrt();
        }
        2.0 * sum + remainder(t)
    }
    fn turned(&self, t: f64, turns: &[Complex], roots: &[f64]) -> f64 {
        let a = (t / (2.0 * PI)).sqrt();
        let (sin, cos) = self.theta(t).sin_cos();
        let sum: f64 = turns[..a as usize]
            .iter()
            .zip(roots)
            .map(|(turn, root)| (cos * turn.re + sin * turn.im) / root)
            .sum();
        if 2.0 * sum.abs() * a.sqrt() > REMAINDER {
            sum
        } else {
            2.0 * sum + remainder(t)
        }
    }
    pub fn z(&self, t: f64) -> f64 {
        if t < JOIN {
            self.exact(t)
        } else {
            self.siegel(t)
        }
    }
    pub fn point(&self, t: f64) -> (Complex, f64) {
        if t < JOIN {
            let value = self.maclaurin(t);
            (value, (Complex::turn(self.theta(t)) * value).re)
        } else {
            let z = self.siegel(t);
            (Complex::turn(-self.theta(t)) * z, z)
        }
    }
    pub fn seam(&self, t0: f64, t1: f64, steps: usize) -> f64 {
        (0..=steps)
            .map(|k| {
                let t = t0 + (t1 - t0) * k as f64 / steps as f64;
                (self.siegel(t) - self.exact(t)).abs()
            })
            .fold(0.0, f64::max)
    }
    pub fn gram(&self, n: i64) -> f64 {
        self.newton(n, 2.0 * PI * (n as f64 + 2.0).max(3.0), 1e-12)
    }
    fn newton(&self, n: i64, mut t: f64, tolerance: f64) -> f64 {
        let target = n as f64 * PI;
        for _ in 0..100 {
            let step = (self.theta(t) - target) / (0.5 * (t / (2.0 * PI)).ln());
            t -= step;
            if step.abs() < tolerance {
                break;
            }
        }
        t
    }
    fn brackets(&self, count: usize) -> Vec<(f64, f64)> {
        let mut out = Vec::new();
        let mut n = -1;
        let mut left = self.gram(n);
        let mut previous = left;
        let mut before = self.exact(left);
        while out.len() < count {
            let right = self.gram(n + 1);
            for k in 1..=STEPS {
                let t = left + (right - left) * k as f64 / STEPS as f64;
                let now = self.exact(t);
                if before * now < 0.0 {
                    out.push((previous, t));
                }
                previous = t;
                before = now;
            }
            left = right;
            n += 1;
        }
        out.truncate(count);
        out
    }
    fn bisect(&self, (mut a, mut b): (f64, f64)) -> f64 {
        let mut fa = self.exact(a);
        while b - a > TOLERANCE {
            let mid = 0.5 * (a + b);
            let fm = self.exact(mid);
            if fa * fm <= 0.0 {
                b = mid;
            } else {
                a = mid;
                fa = fm;
            }
        }
        0.5 * (a + b)
    }
    pub fn zeros(&self, count: usize) -> Vec<f64> {
        self.brackets(count)
            .into_iter()
            .map(|pair| self.bisect(pair))
            .collect()
    }
    pub fn count(&self, t: f64) -> usize {
        let terms = (t.max(0.0) / (2.0 * PI)).sqrt() as usize + 1;
        let logs: Vec<f64> = (1..=terms).map(|k| (k as f64).ln()).collect();
        let roots: Vec<f64> = (1..=terms).map(|k| (k as f64).sqrt()).collect();
        let mut turns = vec![Complex::default(); terms];
        let mut steps = vec![Complex::default(); terms];
        let mut found = 0;
        let mut n = -1;
        let mut left = self.gram(n);
        let mut before = self.z(left);
        while left < t {
            let guess = left + PI / (0.5 * (left / (2.0 * PI)).ln());
            let right = self.newton(n + 1, guess, 1e-12 * left);
            let stride = (right - left) / STEPS as f64;
            let high = left + stride >= JOIN;
            if high {
                for (k, log) in logs.iter().enumerate() {
                    turns[k] = Complex::turn((left + stride) * log);
                    steps[k] = Complex::turn(stride * log);
                }
            }
            for k in 1..=STEPS {
                let at = (left + (right - left) * k as f64 / STEPS as f64).min(t);
                let now = if high && at < t {
                    self.turned(at, &turns, &roots)
                } else {
                    self.z(at)
                };
                if before * now < 0.0 {
                    found += 1;
                }
                if at >= t {
                    return found;
                }
                before = now;
                for (turn, step) in turns.iter_mut().zip(&steps) {
                    *turn = *turn * *step;
                }
            }
            left = right;
            n += 1;
        }
        found
    }
}

fn remainder(t: f64) -> f64 {
    let a = (t / (2.0 * PI)).sqrt();
    let whole = a.floor();
    let sign = if (whole as u64).is_multiple_of(2) {
        -1.0
    } else {
        1.0
    };
    let mut weight = 1.0 / a.sqrt();
    let mut rest = 0.0;
    for c in corrections(a - whole) {
        rest += c * weight;
        weight /= a;
    }
    sign * rest
}

pub fn psi_stair(x: usize) -> Vec<f64> {
    let mut jumps = vec![0.0; x + 1];
    for p in primes(x) {
        let mut q = p;
        while q <= x {
            jumps[q] += (p as f64).ln();
            q *= p;
        }
    }
    let mut sum = 0.0;
    jumps[1..]
        .iter()
        .map(|jump| {
            sum += jump;
            sum
        })
        .collect()
}

pub fn psi_formula(x: f64, gammas: &[f64]) -> f64 {
    let log = x.ln();
    let waves: f64 = gammas
        .iter()
        .map(|&g| (0.5 * (g * log).cos() + g * (g * log).sin()) / (0.25 + g * g))
        .sum();
    x - 2.0 * x.sqrt() * waves - (2.0 * PI).ln() - 0.5 * (1.0 - 1.0 / (x * x)).ln()
}

pub fn psi_gap(x: usize, gammas: &[f64]) -> f64 {
    let stair = psi_stair(x).last().copied().unwrap_or(0.0);
    (psi_formula(x as f64, gammas) - stair).abs()
}

pub fn psi_curve(gammas: &[f64], samples: &[f64]) -> Vec<f64> {
    samples.iter().map(|&x| psi_formula(x, gammas)).collect()
}

pub fn bump(u: f64) -> f64 {
    if u <= 1.0 || u >= 2.0 {
        return 0.0;
    }
    (4.0 - 1.0 / ((u - 1.0) * (2.0 - u))).exp()
}

pub fn mellin(s: Complex) -> Complex {
    let step = 1.0 / NODES as f64;
    let mut sum = Complex::default();
    for k in 1..NODES {
        let u = 1.0 + k as f64 * step;
        sum = sum + raise(u, s - 1.0) * (bump(u) * step);
    }
    sum
}

pub fn novelty_main() -> f64 {
    6.0 / (PI * PI) * mellin(Complex::new(2.0, 0.0)).re
}

pub fn smoothed_novelty(phi: &[u64], y: f64, main: f64) -> f64 {
    let lo = (1.0 / y).ceil() as usize;
    let hi = (2.0 / y).floor() as usize;
    let sum: f64 = phi[lo..=hi]
        .iter()
        .zip(lo..)
        .map(|(&p, n)| p as f64 * bump(n as f64 * y))
        .sum();
    y * y * sum - main
}

pub fn sharp_novelty(prefix: &[u64], y: f64) -> f64 {
    let lo = (1.0 / y).ceil() as usize;
    let hi = (2.0 / y).floor() as usize;
    y * y * (prefix[hi] - prefix[lo - 1]) as f64 - 9.0 / (PI * PI)
}

pub fn novelty_wave(gammas: &[f64], coef: &[Complex], log_y: f64) -> f64 {
    let sum: f64 = gammas
        .iter()
        .zip(coef)
        .map(|(g, c)| (*c * Complex::turn(-g * log_y)).re)
        .sum();
    2.0 * sum
}
