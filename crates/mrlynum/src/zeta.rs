use crate::classics::primes;
use crate::series::bernoulli;
use std::f64::consts::PI;
use std::ops::{Add, Div, Mul, Neg, Sub};

/// The t where the walk hands over from Euler-Maclaurin to Riemann-Siegel.
pub const JOIN: f64 = 20.0;
const SHIFT: usize = 10;
const TAIL: usize = 7;
const STEPS: usize = 10;
const STENCIL: f64 = 0.02;
const TOLERANCE: f64 = 1e-9;

/// A complex number: a real and an imaginary part.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Complex {
    /// The real part.
    pub re: f64,
    /// The imaginary part.
    pub im: f64,
}

impl Complex {
    /// Builds a complex number from its parts.
    pub const fn new(re: f64, im: f64) -> Complex {
        Complex { re, im }
    }
    /// Returns the modulus.
    pub fn abs(self) -> f64 {
        self.re.hypot(self.im)
    }
    /// Returns the principal argument.
    pub fn arg(self) -> f64 {
        self.im.atan2(self.re)
    }
    /// Returns the exponential.
    pub fn exp(self) -> Complex {
        let r = self.re.exp();
        Complex::new(r * self.im.cos(), r * self.im.sin())
    }
    /// Returns the principal logarithm.
    pub fn ln(self) -> Complex {
        Complex::new(self.abs().ln(), self.arg())
    }
    /// Returns a unit complex number at the given angle.
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

/// Returns a positive real base raised to a complex exponent.
pub fn raise(base: f64, exponent: Complex) -> Complex {
    (exponent * base.ln()).exp()
}

/// Returns the Riemann-Siegel kernel, the cosine ratio that leads the remainder, in the form that stays finite at its removable points.
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

/// Returns the first four Riemann-Siegel corrections at the fractional part p: the kernel and its derivatives by central differences with one Richardson step.
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

/// The critical line: the Bernoulli numbers and the Euler-Maclaurin weights the two engines share, built once.
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
    /// Builds the line: the even Bernoulli numbers through the fourteenth and their Euler-Maclaurin weights.
    pub fn new() -> Line {
        let fractions = bernoulli(2 * TAIL + 1);
        let bern: Vec<f64> = (0..=TAIL)
            .map(|k| {
                let (num, den) = fractions[2 * k];
                num as f64 / den as f64
            })
            .collect();
        let mut factorial = 1.0;
        let tail = (0..=TAIL)
            .map(|k| {
                if k > 0 {
                    factorial *= ((2 * k - 1) * 2 * k) as f64;
                }
                bern[k] / factorial
            })
            .collect();
        Line { bern, tail }
    }
    /// Returns the Riemann-Siegel theta: the argument of gamma at one quarter plus i t over two, less t ln pi over two, by Stirling's series after a shift of ten.
    pub fn theta(&self, t: f64) -> f64 {
        let z = Complex::new(0.25, 0.5 * t);
        let w = z + SHIFT as f64;
        let mut arg = ((w - 0.5) * w.ln() - w).im;
        for k in 0..SHIFT {
            arg -= (z + k as f64).arg();
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
    /// Returns zeta at one half plus i t by the complex Euler-Maclaurin sum: t plus ten terms and seven Bernoulli corrections.
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
    /// Returns Z(t) from the Euler-Maclaurin value turned onto the real axis.
    pub fn exact(&self, t: f64) -> f64 {
        (Complex::turn(self.theta(t)) * self.maclaurin(t)).re
    }
    /// Returns Z(t) by the Riemann-Siegel formula: the main sum and the first four corrections.
    pub fn siegel(&self, t: f64) -> f64 {
        let a = (t / (2.0 * PI)).sqrt();
        let whole = a.floor();
        let theta = self.theta(t);
        let mut sum = 0.0;
        for k in 1..=whole as usize {
            let kf = k as f64;
            sum += (theta - t * kf.ln()).cos() / kf.sqrt();
        }
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
        2.0 * sum + sign * rest
    }
    /// Returns Z(t): Euler-Maclaurin below the join, Riemann-Siegel above.
    pub fn z(&self, t: f64) -> f64 {
        if t < JOIN {
            self.exact(t)
        } else {
            self.siegel(t)
        }
    }
    /// Returns zeta on the line and Z(t) together, from the engine that serves the t.
    pub fn point(&self, t: f64) -> (Complex, f64) {
        if t < JOIN {
            let value = self.maclaurin(t);
            (value, (Complex::turn(self.theta(t)) * value).re)
        } else {
            let z = self.siegel(t);
            (Complex::turn(-self.theta(t)) * z, z)
        }
    }
    /// Returns the largest gap between the two engines over the t range on a grid.
    pub fn seam(&self, t0: f64, t1: f64, steps: usize) -> f64 {
        (0..=steps)
            .map(|k| {
                let t = t0 + (t1 - t0) * k as f64 / steps as f64;
                (self.siegel(t) - self.exact(t)).abs()
            })
            .fold(0.0, f64::max)
    }
    /// Returns the n-th Gram point, where theta is n pi, by Newton from the right.
    pub fn gram(&self, n: i64) -> f64 {
        let target = n as f64 * PI;
        let mut t = 2.0 * PI * (n as f64 + 2.0).max(3.0);
        for _ in 0..100 {
            let step = (self.theta(t) - target) / (0.5 * (t / (2.0 * PI)).ln());
            t -= step;
            if step.abs() < 1e-12 {
                break;
            }
        }
        t
    }
    fn brackets(&self, limit: f64, count: usize, exact: bool) -> Vec<(f64, f64)> {
        let z = |t: f64| if exact { self.exact(t) } else { self.z(t) };
        let mut out = Vec::new();
        let mut n = -1;
        let mut left = self.gram(n);
        let mut previous = left;
        let mut before = z(left);
        'walk: while out.len() < count && left < limit {
            let right = self.gram(n + 1);
            for k in 1..=STEPS {
                let t = (left + (right - left) * k as f64 / STEPS as f64).min(limit);
                let now = z(t);
                if before * now < 0.0 {
                    out.push((previous, t));
                }
                previous = t;
                before = now;
                if t >= limit {
                    break 'walk;
                }
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
    /// Returns the first zeros on the line: sign changes of Z between Gram points, refined by bisection on Euler-Maclaurin to a billionth.
    pub fn zeros(&self, count: usize) -> Vec<f64> {
        self.brackets(f64::INFINITY, count, true)
            .into_iter()
            .map(|pair| self.bisect(pair))
            .collect()
    }
    /// Counts the zeros on the line below t.
    pub fn count(&self, t: f64) -> usize {
        self.brackets(t, usize::MAX, false).len()
    }
}

/// Returns the Chebyshev staircase at every whole number from one to x: the sum of ln p over the prime powers up to each.
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

/// Returns the von Mangoldt explicit formula at x over the zeros at the given ordinates and their mirrors: x less the sum of x to the rho over rho, less ln two pi, less half the ln of one minus x to the minus two.
pub fn psi_formula(x: f64, gammas: &[f64]) -> f64 {
    let log = x.ln();
    let waves: f64 = gammas
        .iter()
        .map(|&g| (0.5 * (g * log).cos() + g * (g * log).sin()) / (0.25 + g * g))
        .sum();
    x - 2.0 * x.sqrt() * waves - (2.0 * PI).ln() - 0.5 * (1.0 - 1.0 / (x * x)).ln()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn classic(t: f64) -> f64 {
        0.5 * t * (t / (2.0 * PI)).ln() - 0.5 * t - PI / 8.0
            + 1.0 / (48.0 * t)
            + 7.0 / (5760.0 * t.powi(3))
            + 31.0 / (80640.0 * t.powi(5))
    }

    #[test]
    fn the_complex_arithmetic_round_trips() {
        let z = Complex::new(-1.5, 2.25);
        let back = z.ln().exp();
        assert!((back - z).abs() < 1e-14);
        assert!((z / z - Complex::new(1.0, 0.0)).abs() < 1e-15);
        assert!((raise(2.0, Complex::new(3.0, 0.0)).re - 8.0).abs() < 1e-13);
        assert!((raise(4.0, Complex::new(0.5, 0.0)) - Complex::new(2.0, 0.0)).abs() < 1e-14);
    }

    #[test]
    fn theta_meets_the_asymptotic_series_and_the_first_gram_points() {
        let line = Line::new();
        for t in [20.0, 50.0, 100.0, 200.0] {
            assert!((line.theta(t) - classic(t)).abs() < 1e-9, "t {t}");
        }
        assert!(line.theta(0.0).abs() < 1e-12);
        assert!((line.gram(-1) - 9.666_908).abs() < 1e-6);
        assert!((line.gram(0) - 17.845_600).abs() < 1e-6);
        assert!((line.gram(1) - 23.170_283).abs() < 1e-6);
        assert!(line.theta(line.gram(2)).abs() - 2.0 * PI < 1e-10);
    }

    #[test]
    fn maclaurin_meets_the_known_values_on_the_line() {
        let line = Line::new();
        assert!((line.maclaurin(0.0).re + 1.460_354_508_809_586_8).abs() < 1e-10);
        assert!(line.maclaurin(0.0).im.abs() < 1e-12);
        let one = line.maclaurin(1.0);
        assert!((one.re - 0.143_936_427_077_189).abs() < 1e-9);
        assert!((one.im + 0.722_099_743_531_673).abs() < 1e-9);
        assert!(line.maclaurin(14.134_725).abs() < 1e-5);
        for t in [3.0, 25.0, 140.0] {
            let value = line.maclaurin(t);
            assert!(
                (Complex::turn(line.theta(t)) * value).im.abs() < 1e-9,
                "t {t}"
            );
        }
    }

    #[test]
    fn the_kernel_pins_its_centre_and_its_removable_points() {
        assert!((kernel(0.5) - (3.0 * PI / 8.0).cos()).abs() < 1e-15);
        assert!((kernel(0.0) - (PI / 8.0).cos()).abs() < 1e-15);
        assert_eq!(kernel(0.25), 0.5);
        assert_eq!(kernel(0.75), 0.5);
        assert!((kernel(0.25 + 1e-9) - 0.5).abs() < 1e-7);
        assert!((kernel(0.75 - 1e-9) - 0.5).abs() < 1e-7);
        let direct = |p: f64| (2.0 * PI * (p * p - p - 1.0 / 16.0)).cos() / (2.0 * PI * p).cos();
        for p in [0.05, 0.1, 0.4, 0.5, 0.6, 0.9, 0.95] {
            assert!((kernel(p) - direct(p)).abs() < 1e-13, "p {p}");
        }
        assert!(corrections(0.5)[1].abs() < 1e-9);
    }

    #[test]
    fn siegel_meets_maclaurin_beyond_the_join() {
        let line = Line::new();
        assert!(line.seam(JOIN, 60.0, 800) < 5e-5);
        assert!(line.seam(60.0, 250.0, 1900) < 5e-6);
        assert!((line.z(JOIN) - line.exact(JOIN)).abs() < 5e-5);
    }

    #[test]
    fn the_zeros_and_their_count_are_the_classic_ones() {
        let line = Line::new();
        let first = line.zeros(5);
        let known = [14.134_725, 21.022_040, 25.010_858, 30.424_876, 32.935_062];
        for (got, want) in first.iter().zip(known) {
            assert!((got - want).abs() < 1e-6, "{got} {want}");
        }
        assert_eq!(line.count(100.0), 29);
        assert_eq!(line.count(200.0), 79);
        assert_eq!(line.count(10.0), 0);
        let hundred = line.zeros(100);
        assert_eq!(hundred.len(), 100);
        assert!((hundred[99] - 236.524_230).abs() < 1e-5);
        assert!(hundred.windows(2).all(|w| w[1] > w[0]));
    }

    #[test]
    fn psi_pins_the_staircase_and_the_smooth_guess() {
        let stair = psi_stair(100);
        assert!((stair[9] - 7.832_0).abs() < 1e-4);
        assert!((stair[99] - 94.045_311).abs() < 1e-5);
        assert_eq!(stair[0], 0.0);
        assert!((stair[7] - 3.0 * 2f64.ln() - 3f64.ln() - 5f64.ln() - 7f64.ln()).abs() < 1e-12);
        assert!((psi_formula(10.0, &[]) - 8.167_1).abs() < 1e-4);
        let line = Line::new();
        let zeros = line.zeros(100);
        let close = psi_formula(100.0, &zeros);
        assert!((close - stair[99]).abs() < 1.0, "{close}");
        assert!((close - stair[99]).abs() < (psi_formula(100.0, &[]) - stair[99]).abs());
    }
}
